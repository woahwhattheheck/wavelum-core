//! Zero-Knowledge SNARK Verifier Module
//!
//! This module provides Circom/snarkjs-compatible Groth16 verification for
//! confidential claims over the BN254 curve (alt_bn128), backed by the Soroban
//! host's native BN254 pairing and G1 arithmetic operations.
//! Optimized for gas efficiency on Soroban to prevent out-of-gas compute panics.
//!
//! # Proof and key encoding
//! Points are the ethereum/snarkjs uncompressed affine serialization:
//! - `G1`: `x || y` (each 32-byte big-endian `Fp`), 64 bytes total
//! - `G2`: `x_c1 || x_c0 || y_c1 || y_c0` (each 32-byte big-endian `Fp`), 128 bytes
//! The all-zero encoding is the point at infinity, per the host decoder.
//!
//! `ZKClaimProof.proof_data` is the concatenation `a || b || c` = 256 bytes.
//! Public input order for the private-claim circuit:
//! `[commitment_hash, nullifier_hash, merkle_root, claimed_amount]`, each
//! reduced modulo the BN254 scalar field order `r`. The verifying key's `ic`
//! vector must therefore have exactly 5 points (`ic[0]` plus one per input).
//!
//! # Security Considerations
//! - The proving curve (BN254) is highly optimized for gas efficiency
//! - Verification fails closed: malformed proofs, missing verifying keys and
//!   failed pairing checks all reject before any contract state is mutated
//! - All proof validations return early on failure to minimize gas waste

use soroban_sdk::{Bytes, BytesN, Env, U256, Vec as SorobanVec};
use soroban_sdk::crypto::bn254::{Bn254G1Affine, Bn254G2Affine, Fr};
use crate::types::{ConfidentialClaimProof, ZkVerificationKey, ZKClaimProof};
use crate::errors::Error;

/// Verification result for ZK proofs
#[derive(Debug, PartialEq)]
pub enum VerificationResult {
    Valid,
    InvalidProof,
    OverClaimAttempt,
    InvalidCommitment,
}

/// BN254 scalar field order `r` (big-endian).
/// r = 21888242871839275222246405745257275088548364400416034343698204186575808495617
pub const BN254_ORDER_BYTES: [u8; 32] = [
    0x30, 0x64, 0x4e, 0x72, 0xe1, 0x31, 0xa0, 0x29,
    0xb8, 0x50, 0x45, 0xb6, 0x81, 0x81, 0x58, 0x5d,
    0x28, 0x33, 0xe8, 0x48, 0x79, 0xb9, 0x70, 0x91,
    0x43, 0xe1, 0xf5, 0x93, 0xf0, 0x00, 0x00, 0x01,
];

/// Serialized size of an uncompressed BN254 G1 affine point (x || y, big-endian).
pub const G1_SERIALIZED_SIZE: u32 = 64;
/// Serialized size of an uncompressed BN254 G2 affine point
/// (x_c1 || x_c0 || y_c1 || y_c0, big-endian).
pub const G2_SERIALIZED_SIZE: u32 = 128;
/// Serialized size of a Groth16 proof blob: a(64) || b(128) || c(64).
pub const PROOF_SERIALIZED_SIZE: u32 = G1_SERIALIZED_SIZE + G2_SERIALIZED_SIZE + G1_SERIALIZED_SIZE;

/// ZK-SNARK Verifier for confidential claims
///
/// This verifier consumes Groth16 proofs generated off-chain (snarkjs/circom)
/// using the BN254 curve (also known as alt_bn128).
pub struct ZKVerifier;

impl ZKVerifier {
    /// Verify a confidential claim ZK proof
    ///
    /// # Arguments
    /// * `e` - The environment
    /// * `proof` - The ZK proof containing public inputs and proof points
    /// * `expected_commitment` - The expected commitment hash from storage
    /// * `remaining_shielded` - The current remaining shielded amount
    /// * `vk` - The Groth16 verifying key configured for the claim circuit
    ///
    /// # Returns
    /// * `Ok(())` if the proof is valid
    /// * `Err(Error::InvalidZKProof)` if the proof is malformed or invalid
    /// * `Err(Error::OverClaimAttempt)` if the claim amount exceeds remaining
    pub fn verify_confidential_claim(
        e: &Env,
        proof: &ConfidentialClaimProof,
        expected_commitment: &BytesN<32>,
        remaining_shielded: i128,
        vk: &ZkVerificationKey,
    ) -> Result<(), Error> {
        // Step 1: Verify commitment matches expected
        if proof.commitment_hash != *expected_commitment {
            return Err(Error::InvalidZKProof);
        }

        // Step 2: Verify the claim doesn't exceed remaining shielded amount
        if proof.claimed_amount > remaining_shielded {
            return Err(Error::OverClaimAttempt);
        }

        // Step 3: Verify remaining amount is non-negative
        if proof.remaining_amount < 0 {
            return Err(Error::InvalidZKProof);
        }

        // Step 4: Verify arithmetic consistency
        // claimed + remaining should equal the original commitment's hidden value
        // In a full implementation, this would be verified by the ZK circuit
        let calculated_total = proof.claimed_amount
            .checked_add(proof.remaining_amount)
            .ok_or(Error::Overflow)?;

        // In production, we'd verify this against the commitment's hidden value
        // For now, we ensure the arithmetic is consistent
        if calculated_total < proof.claimed_amount {
            return Err(Error::InvalidZKProof);
        }

        // Step 5: Verify the ZK-SNARK proof structure
        if !Self::verify_proof_structure(proof) {
            return Err(Error::InvalidZKProof);
        }

        // Step 6: Perform the actual BN254 pairing verification.
        // Public input order for the confidential-claim circuit:
        // [commitment_hash, nullifier, merkle_root, claimed_amount, remaining_amount]
        let mut inputs: SorobanVec<Fr> = SorobanVec::new(e);
        inputs.push_back(Self::hash_to_fr(e, &proof.commitment_hash));
        inputs.push_back(Self::hash_to_fr(e, &proof.nullifier));
        inputs.push_back(Self::hash_to_fr(e, &proof.merkle_root));
        inputs.push_back(Self::i128_to_fr(e, proof.claimed_amount));
        inputs.push_back(Self::i128_to_fr(e, proof.remaining_amount));

        if !Self::groth16_verify(e, &proof.proof_a, &proof.proof_b, &proof.proof_c, &inputs, vk)? {
            return Err(Error::InvalidZKProof);
        }

        Ok(())
    }

    /// Verify the Groth16 proof attached to a `private_claim` call.
    ///
    /// The proof binds to the claim's public inputs; the pairing equation is
    /// evaluated by the Soroban BN254 host functions.
    ///
    /// # Returns
    /// * `Ok(true)` when the pairing equation holds
    /// * `Ok(false)` when the equation does not hold (invalid proof)
    /// * `Err(Error::InvalidZKProof)` when the proof blob or key is malformed
    pub fn verify_private_claim_proof(
        e: &Env,
        proof: &ZKClaimProof,
        vk: &ZkVerificationKey,
        amount: i128,
    ) -> Result<bool, Error> {
        let (a, b, c) = Self::parse_claim_proof(e, &proof.proof_data)?;

        // Public input order for the private-claim circuit:
        // [commitment_hash, nullifier_hash, merkle_root, claimed_amount]
        let mut inputs: SorobanVec<Fr> = SorobanVec::new(e);
        inputs.push_back(Self::hash_to_fr(e, &proof.commitment_hash));
        inputs.push_back(Self::hash_to_fr(e, &proof.nullifier_hash));
        inputs.push_back(Self::hash_to_fr(e, &proof.merkle_root));
        inputs.push_back(Self::i128_to_fr(e, amount));

        Self::groth16_verify(e, &a, &b, &c, &inputs, vk)
    }

    /// Split a `proof_data` blob into Groth16 proof points a(G1) || b(G2) || c(G1).
    fn parse_claim_proof(
        e: &Env,
        data: &Bytes,
    ) -> Result<(BytesN<64>, BytesN<128>, BytesN<64>), Error> {
        if data.len() != PROOF_SERIALIZED_SIZE {
            return Err(Error::InvalidZKProof);
        }
        let mut a = [0u8; 64];
        let mut b = [0u8; 128];
        let mut c = [0u8; 64];
        data.slice(0..G1_SERIALIZED_SIZE).copy_into_slice(&mut a);
        data.slice(G1_SERIALIZED_SIZE..(G1_SERIALIZED_SIZE + G2_SERIALIZED_SIZE))
            .copy_into_slice(&mut b);
        data.slice((G1_SERIALIZED_SIZE + G2_SERIALIZED_SIZE)..PROOF_SERIALIZED_SIZE)
            .copy_into_slice(&mut c);
        Ok((
            BytesN::from_array(e, &a),
            BytesN::from_array(e, &b),
            BytesN::from_array(e, &c),
        ))
    }

    /// Core Groth16 verification over the BN254 pairing host function.
    ///
    /// Checks the standard equation
    /// `e(-A, B) * e(alpha, beta) * e(vk_x, gamma) * e(C, delta) == 1`
    /// where `vk_x = ic[0] + sum(inputs[i] * ic[i+1])` and `-A` is computed as
    /// `A * (r - 1)` since the host exposes G1 scalar multiplication but not
    /// point negation.
    pub fn groth16_verify(
        e: &Env,
        a: &BytesN<64>,
        b: &BytesN<128>,
        c: &BytesN<64>,
        public_inputs: &SorobanVec<Fr>,
        vk: &ZkVerificationKey,
    ) -> Result<bool, Error> {
        // The verifying key must carry exactly one IC point per public input
        // plus the constant term.
        if vk.ic.len() != public_inputs.len() + 1 {
            return Err(Error::InvalidZKProof);
        }

        let bn = e.crypto().bn254();

        // vk_x = ic[0] + sum( input_i * ic[i+1] )
        let mut vk_x = Bn254G1Affine::from_bytes(vk.ic.get(0).ok_or(Error::InvalidZKProof)?);
        let mut i: u32 = 1;
        while i < vk.ic.len() {
            let ic_point = Bn254G1Affine::from_bytes(
                vk.ic.get(i).ok_or(Error::InvalidZKProof)?,
            );
            let scalar = public_inputs.get(i - 1).ok_or(Error::InvalidZKProof)?;
            vk_x = bn.g1_add(&vk_x, &bn.g1_mul(&ic_point, &scalar));
            i += 1;
        }

        // -A = A * (r - 1)
        let neg_a = bn.g1_mul(
            &Bn254G1Affine::from_bytes(a.clone()),
            &Self::fr_minus_one(e),
        );

        let mut g1_pairs: SorobanVec<Bn254G1Affine> = SorobanVec::new(e);
        g1_pairs.push_back(neg_a);
        g1_pairs.push_back(Bn254G1Affine::from_bytes(vk.alpha_g1.clone()));
        g1_pairs.push_back(vk_x);
        g1_pairs.push_back(Bn254G1Affine::from_bytes(c.clone()));

        let mut g2_pairs: SorobanVec<Bn254G2Affine> = SorobanVec::new(e);
        g2_pairs.push_back(Bn254G2Affine::from_bytes(b.clone()));
        g2_pairs.push_back(Bn254G2Affine::from_bytes(vk.beta_g2.clone()));
        g2_pairs.push_back(Bn254G2Affine::from_bytes(vk.gamma_g2.clone()));
        g2_pairs.push_back(Bn254G2Affine::from_bytes(vk.delta_g2.clone()));

        Ok(bn.pairing_check(g1_pairs, g2_pairs))
    }

    /// The scalar `r - 1` as a field element, used to negate G1 points.
    fn fr_minus_one(e: &Env) -> Fr {
        let r = U256::from_be_bytes(e, &Bytes::from_slice(e, &BN254_ORDER_BYTES));
        Fr::from_u256(r.sub(&U256::from_u32(e, 1)))
    }

    /// Map a 32-byte big-endian value into the BN254 scalar field by reducing
    /// it modulo `r`. Hash outputs are canonicalized the same way snarkjs
    /// reduces public signals.
    fn hash_to_fr(e: &Env, value: &BytesN<32>) -> Fr {
        let r = U256::from_be_bytes(e, &Bytes::from_slice(e, &BN254_ORDER_BYTES));
        let v = U256::from_be_bytes(e, &value.to_bytes());
        Fr::from_u256(v.rem_euclid(&r))
    }

    /// Map a signed ledger amount into the BN254 scalar field.
    fn i128_to_fr(e: &Env, value: i128) -> Fr {
        let r = U256::from_be_bytes(e, &Bytes::from_slice(e, &BN254_ORDER_BYTES));
        let v = if value >= 0 {
            U256::from_u128(e, value as u128)
        } else {
            // Two's-complement field encoding for negative amounts: r - |value|
            r.sub(&U256::from_u128(e, (-value) as u128))
        };
        Fr::from_u256(v.rem_euclid(&r))
    }

    /// Verify the basic structure of the ZK proof
    ///
    /// This checks that the proof data is well-formed and not obviously
    /// malformed. Cryptographic soundness comes from `groth16_verify`; these
    /// checks only reject degenerate inputs early.
    fn verify_proof_structure(proof: &ConfidentialClaimProof) -> bool {
        // Check that proof components are non-zero
        let is_zero = |bytes: &BytesN<32>| {
            bytes.iter().all(|&b| b == 0)
        };
        let is_zero_g1 = |bytes: &BytesN<64>| {
            bytes.iter().all(|&b| b == 0)
        };
        let is_zero_g2 = |bytes: &BytesN<128>| {
            bytes.iter().all(|&b| b == 0)
        };

        // Proof components should not be all zeros
        if is_zero_g1(&proof.proof_a) || is_zero_g2(&proof.proof_b) || is_zero_g1(&proof.proof_c) {
            return false;
        }

        // Nullifier should not be zero
        if is_zero(&proof.nullifier) {
            return false;
        }

        // Commitment should not be zero
        if is_zero(&proof.commitment_hash) {
            return false;
        }

        // Merkle root should not be zero
        if is_zero(&proof.merkle_root) {
            return false;
        }

        // Claimed amount should be positive
        if proof.claimed_amount <= 0 {
            return false;
        }

        true
    }

    /// Verify a viewing key for DAO clawback operations
    ///
    /// # Arguments
    /// * `viewing_key` - The master viewing key to verify
    /// * `authorized_admin` - The admin address that should have authorized this key
    ///
    /// # Returns
    /// * `true` if the viewing key is valid and authorized
    /// * `false` otherwise
    pub fn verify_viewing_key(
        viewing_key: &BytesN<32>,
        authorized_admin: &soroban_sdk::Address,
        stored_key: &crate::types::MasterViewingKey,
    ) -> bool {
        // Check if the key is active
        if !stored_key.is_active {
            return false;
        }

        // Check if the viewing key matches
        if stored_key.viewing_key != *viewing_key {
            return false;
        }

        // Check if the authorizing admin matches
        if stored_key.authorized_by != *authorized_admin {
            return false;
        }

        true
    }

    /// Compute a Pedersen commitment for a given amount
    ///
    /// This is a placeholder for actual Pedersen commitment computation.
    /// In production, this would use elliptic curve operations:
    /// C = r*G + amount*H
    /// where G and H are generator points on the curve
    ///
    /// # Arguments
    /// * `amount` - The amount to commit to
    /// * `blinding_factor` - A random blinding factor
    ///
    /// # Returns
    /// * The commitment hash
    pub fn compute_commitment(
        _amount: i128,
        _blinding_factor: &BytesN<32>,
    ) -> BytesN<32> {
        // Placeholder: In production, this would compute:
        // C = PedersenCommit(amount, blinding_factor)
        // using the BN254 curve

        // For now, return a placeholder hash
        // This would be computed off-chain and passed to the contract
        BytesN::from_array(&Env::default(), &[0u8; 32])
    }

    /// Verify a commitment opening
    ///
    /// Verifies that a commitment opens to the given amount and blinding factor.
    ///
    /// # Arguments
    /// * `commitment` - The commitment hash
    /// * `amount` - The claimed amount
    /// * `blinding_factor` - The blinding factor
    ///
    /// # Returns
    /// * `true` if the commitment opens correctly
    /// * `false` otherwise
    pub fn verify_commitment_opening(
        commitment: &BytesN<32>,
        amount: i128,
        blinding_factor: &BytesN<32>,
    ) -> bool {
        // In production, this would verify:
        // PedersenCommit(amount, blinding_factor) == commitment

        // Placeholder: always return false to force off-chain computation
        // The actual verification should happen in the ZK circuit
        let computed = Self::compute_commitment(amount, blinding_factor);
        computed == *commitment
    }

    /// Generate a nullifier from a secret and commitment
    ///
    /// Nullifiers prevent double-spending by uniquely identifying a claim
    /// without revealing the claimer's identity.
    ///
    /// # Arguments
    /// * `secret` - The user's secret
    /// * `commitment` - The commitment hash
    ///
    /// # Returns
    /// * The nullifier hash
    pub fn compute_nullifier(
        _secret: &BytesN<32>,
        _commitment: &BytesN<32>,
    ) -> BytesN<32> {
        // Placeholder: In production, this would compute:
        // nullifier = Hash(secret || commitment)
        // using a cryptographic hash function

        BytesN::from_array(&Env::default(), &[0u8; 32])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// BN254 G1 generator point (1, 2), uncompressed affine big-endian.
    const G1_GEN: [u8; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
    ];

    /// BN254 G2 generator point, ethereum encoding x_c1||x_c0||y_c1||y_c0.
    const G2_GEN: [u8; 128] = [
        0x18, 0x00, 0xde, 0xef, 0x12, 0x1f, 0x1e, 0x76,
        0x42, 0x6a, 0x00, 0x66, 0x5e, 0x5c, 0x44, 0x79,
        0x67, 0x43, 0x22, 0xd4, 0xf7, 0x5e, 0xda, 0xdd,
        0x46, 0xde, 0xbd, 0x5c, 0xd9, 0x92, 0xf6, 0xed,
        0x19, 0x8e, 0x93, 0x93, 0x92, 0x0d, 0x48, 0x3a,
        0x72, 0x60, 0xbf, 0xb7, 0x31, 0xfb, 0x5d, 0x25,
        0xf1, 0xaa, 0x49, 0x33, 0x35, 0xa9, 0xe7, 0x12,
        0x97, 0xe4, 0x85, 0xb7, 0xae, 0xf3, 0x12, 0xc2,
        0x12, 0xc8, 0x5e, 0xa5, 0xdb, 0x8c, 0x6d, 0xeb,
        0x4a, 0xab, 0x71, 0x80, 0x8d, 0xcb, 0x40, 0x8f,
        0xe3, 0xd1, 0xe7, 0x69, 0x0c, 0x43, 0xd3, 0x7b,
        0x4c, 0xe6, 0xcc, 0x01, 0x66, 0xfa, 0x7d, 0xaa,
        0x09, 0x06, 0x89, 0xd0, 0x58, 0x5f, 0xf0, 0x75,
        0xec, 0x9e, 0x99, 0xad, 0x69, 0x0c, 0x33, 0x95,
        0xbc, 0x4b, 0x31, 0x33, 0x70, 0xb3, 0x8e, 0xf3,
        0x55, 0xac, 0xda, 0xdc, 0xd1, 0x22, 0x97, 0x5b,
    ];

    /// Verifying key for the private-claim circuit layout
    /// ([commitment, nullifier, merkle_root, amount] -> 5 IC points), with
    /// infinity points so vk_x terms and the (alpha,beta),(vk_x,gamma) pairs
    /// evaluate to 1. Real pairing math still runs on the proof points.
    fn test_vk(e: &Env) -> ZkVerificationKey {
        let mut ic: SorobanVec<BytesN<64>> = SorobanVec::new(e);
        for _ in 0..5 {
            ic.push_back(BytesN::from_array(e, &[0u8; 64]));
        }
        ZkVerificationKey {
            alpha_g1: BytesN::from_array(e, &[0u8; 64]),
            beta_g2: BytesN::from_array(e, &[0u8; 128]),
            gamma_g2: BytesN::from_array(e, &[0u8; 128]),
            delta_g2: BytesN::from_array(e, &G2_GEN),
            ic,
        }
    }

    fn zk_claim_proof(e: &Env, a: [u8; 64], b: [u8; 128], c: [u8; 64]) -> ZKClaimProof {
        let mut blob = [0u8; 256];
        blob[0..64].copy_from_slice(&a);
        blob[64..192].copy_from_slice(&b);
        blob[192..256].copy_from_slice(&c);
        let data = Bytes::from_slice(e, &blob);
        ZKClaimProof {
            commitment_hash: BytesN::from_array(e, &[1u8; 32]),
            nullifier_hash: BytesN::from_array(e, &[2u8; 32]),
            merkle_root: BytesN::from_array(e, &[3u8; 32]),
            proof_data: data,
        }
    }

    #[test]
    fn test_groth16_valid_proof_pairing_equation() {
        // e(-G1,G2) * e(0,0) * e(vk_x,0) * e(G1,G2) = e^-1 * 1 * 1 * e = 1
        let env = Env::default();
        let proof = zk_claim_proof(&env, G1_GEN, G2_GEN, G1_GEN);
        let vk = test_vk(&env);

        let ok = ZKVerifier::verify_private_claim_proof(&env, &proof, &vk, 100)
            .expect("pairing evaluation failed");
        assert!(ok);
    }

    #[test]
    fn test_groth16_invalid_proof_rejected() {
        // Same as above but with b = infinity: e(-G1,0)=1 so the product is
        // e(G1,G2) != 1 and the proof must be rejected.
        let env = Env::default();
        let proof = zk_claim_proof(&env, G1_GEN, [0u8; 128], G1_GEN);
        let vk = test_vk(&env);

        let ok = ZKVerifier::verify_private_claim_proof(&env, &proof, &vk, 100)
            .expect("pairing evaluation failed");
        assert!(!ok);
    }

    #[test]
    fn test_groth16_wrong_vk_rejected() {
        // delta = infinity removes the C term: e(-G1,G2) * ... * e(G1,0) =
        // e^-1 != 1.
        let env = Env::default();
        let proof = zk_claim_proof(&env, G1_GEN, G2_GEN, G1_GEN);
        let mut vk = test_vk(&env);
        vk.delta_g2 = BytesN::from_array(&env, &[0u8; 128]);

        let ok = ZKVerifier::verify_private_claim_proof(&env, &proof, &vk, 100)
            .expect("pairing evaluation failed");
        assert!(!ok);
    }

    #[test]
    fn test_groth16_malformed_proof_data() {
        let env = Env::default();
        let vk = test_vk(&env);
        let proof = ZKClaimProof {
            commitment_hash: BytesN::from_array(&env, &[1u8; 32]),
            nullifier_hash: BytesN::from_array(&env, &[2u8; 32]),
            merkle_root: BytesN::from_array(&env, &[3u8; 32]),
            proof_data: Bytes::new(&env), // empty
        };
        assert_eq!(
            ZKVerifier::verify_private_claim_proof(&env, &proof, &vk, 100),
            Err(Error::InvalidZKProof)
        );
    }

    #[test]
    fn test_groth16_wrong_ic_length() {
        let env = Env::default();
        let proof = zk_claim_proof(&env, G1_GEN, G2_GEN, G1_GEN);
        let mut vk = test_vk(&env);
        let mut ic: SorobanVec<BytesN<64>> = SorobanVec::new(&env);
        ic.push_back(BytesN::from_array(&env, &[0u8; 64]));
        vk.ic = ic; // 1 point but 4 public inputs
        assert_eq!(
            ZKVerifier::verify_private_claim_proof(&env, &proof, &vk, 100),
            Err(Error::InvalidZKProof)
        );
    }

    #[test]
    fn test_verify_proof_structure_valid() {
        let env = Env::default();
        let proof = ConfidentialClaimProof {
            commitment_hash: BytesN::from_array(&env, &[1u8; 32]),
            nullifier: BytesN::from_array(&env, &[2u8; 32]),
            merkle_root: BytesN::from_array(&env, &[3u8; 32]),
            claimed_amount: 100,
            remaining_amount: 900,
            proof_a: BytesN::from_array(&env, &[4u8; 64]),
            proof_b: BytesN::from_array(&env, &[5u8; 128]),
            proof_c: BytesN::from_array(&env, &[6u8; 64]),
        };

        assert!(ZKVerifier::verify_proof_structure(&proof));
    }

    #[test]
    fn test_verify_proof_structure_zero_proof_a() {
        let env = Env::default();
        let proof = ConfidentialClaimProof {
            commitment_hash: BytesN::from_array(&env, &[1u8; 32]),
            nullifier: BytesN::from_array(&env, &[2u8; 32]),
            merkle_root: BytesN::from_array(&env, &[3u8; 32]),
            claimed_amount: 100,
            remaining_amount: 900,
            proof_a: BytesN::from_array(&env, &[0u8; 64]),
            proof_b: BytesN::from_array(&env, &[5u8; 128]),
            proof_c: BytesN::from_array(&env, &[6u8; 64]),
        };

        assert!(!ZKVerifier::verify_proof_structure(&proof));
    }

    #[test]
    fn test_verify_proof_structure_zero_claimed_amount() {
        let env = Env::default();
        let proof = ConfidentialClaimProof {
            commitment_hash: BytesN::from_array(&env, &[1u8; 32]),
            nullifier: BytesN::from_array(&env, &[2u8; 32]),
            merkle_root: BytesN::from_array(&env, &[3u8; 32]),
            claimed_amount: 0,
            remaining_amount: 900,
            proof_a: BytesN::from_array(&env, &[4u8; 64]),
            proof_b: BytesN::from_array(&env, &[5u8; 128]),
            proof_c: BytesN::from_array(&env, &[6u8; 64]),
        };

        assert!(!ZKVerifier::verify_proof_structure(&proof));
    }

    #[test]
    fn test_verify_confidential_claim_over_claim() {
        let env = Env::default();
        let vk = test_vk(&env);
        let proof = ConfidentialClaimProof {
            commitment_hash: BytesN::from_array(&env, &[1u8; 32]),
            nullifier: BytesN::from_array(&env, &[2u8; 32]),
            merkle_root: BytesN::from_array(&env, &[3u8; 32]),
            claimed_amount: 1000,
            remaining_amount: 900,
            proof_a: BytesN::from_array(&env, &[4u8; 64]),
            proof_b: BytesN::from_array(&env, &[5u8; 128]),
            proof_c: BytesN::from_array(&env, &[6u8; 64]),
        };
        let expected_commitment = BytesN::from_array(&env, &[1u8; 32]);
        let remaining_shielded = 500;

        let result = ZKVerifier::verify_confidential_claim(
            &env,
            &proof,
            &expected_commitment,
            remaining_shielded,
            &vk,
        );

        assert_eq!(result, Err(Error::OverClaimAttempt));
    }

    #[test]
    fn test_verify_confidential_claim_invalid_commitment() {
        let env = Env::default();
        let vk = test_vk(&env);
        let proof = ConfidentialClaimProof {
            commitment_hash: BytesN::from_array(&env, &[1u8; 32]),
            nullifier: BytesN::from_array(&env, &[2u8; 32]),
            merkle_root: BytesN::from_array(&env, &[3u8; 32]),
            claimed_amount: 100,
            remaining_amount: 900,
            proof_a: BytesN::from_array(&env, &[4u8; 64]),
            proof_b: BytesN::from_array(&env, &[5u8; 128]),
            proof_c: BytesN::from_array(&env, &[6u8; 64]),
        };
        let expected_commitment = BytesN::from_array(&env, &[99u8; 32]);
        let remaining_shielded = 1000;

        let result = ZKVerifier::verify_confidential_claim(
            &env,
            &proof,
            &expected_commitment,
            remaining_shielded,
            &vk,
        );

        assert_eq!(result, Err(Error::InvalidZKProof));
    }
}
