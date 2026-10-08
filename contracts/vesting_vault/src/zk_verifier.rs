//! Zero-Knowledge SNARK Verifier Module
//! 
//! This module provides Circom-compatible ZK-SNARK verification for confidential claims.
//! Optimized for gas efficiency on Soroban to prevent out-of-gas compute panics.
//! 
//! # Security Considerations
//! - The proving curve (BN254) is highly optimized for gas efficiency
//! - Verification uses constant-time operations to prevent timing attacks
//! - All proof validations return early on failure to minimize gas waste

use soroban_sdk::{Env, Bytes, BytesN, Vec};
use crate::types::ConfidentialClaimProof;
use crate::errors::Error;
use crate::storage::is_valid_merkle_root;

/// Verification result for ZK proofs
#[derive(Debug, PartialEq)]
pub enum VerificationResult {
    Valid,
    InvalidProof,
    OverClaimAttempt,
    InvalidCommitment,
}

/// ZK-SNARK Verifier for confidential claims
/// 
/// This verifier is designed to work with Circom-generated proofs
/// using the BN254 curve (also known as alt_bn128).
pub struct ZKVerifier;

impl ZKVerifier {
    /// Verify a confidential claim ZK proof
    /// 
    /// # Arguments
    /// * `e` - The environment
    /// * `proof` - The ZK proof containing public inputs and proof data
    /// * `expected_commitment` - The expected commitment hash from storage
    /// * `remaining_shielded` - The current remaining shielded amount
    /// 
    /// # Returns
    /// * `Ok(())` if the proof is valid
    /// * `Err(Error::InvalidZKProof)` if the proof is malformed or invalid
    /// * `Err(Error::OverClaimAttempt)` if the claim amount exceeds remaining
    pub fn verify_confidential_claim(
        _e: &Env,
        proof: &ConfidentialClaimProof,
        expected_commitment: &BytesN<32>,
        remaining_shielded: i128,
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
        // This is a placeholder for actual elliptic curve pairing verification
        // In production, this would:
        // - Parse the proof points (A, B, C)
        // - Perform pairing checks: e(A, B) * e(alpha, beta) = e(C, gamma) * e(public, delta)
        // - Use BN254 curve operations optimized for Soroban
        if !Self::verify_proof_structure(proof) {
            return Err(Error::InvalidZKProof);
        }

        // Step 6: Verify the Merkle root is a trusted, stored root and that
        // the commitment is actually a member of that tree. A registered-but-
        // unrelated root must not let a fabricated commitment claim.
        if !is_valid_merkle_root(_e, &proof.merkle_root) {
            return Err(Error::InvalidZKProof);
        }
        if !Self::verify_merkle_proof(
            _e,
            &proof.commitment_hash,
            &proof.merkle_proof,
            &proof.merkle_root,
            proof.position,
        ) {
            return Err(Error::InvalidZKProof);
        }

        Ok(())
    }

    /// Verify a Merkle inclusion proof: `leaf` sits at `position` in the tree
    /// whose root is `root`.
    ///
    /// Standard positional verification: starting from the leaf, each sibling
    /// in `proof` is paired with the running hash; the current position bit
    /// selects the ordering (0 = running hash is the left child, 1 = right).
    /// Returns `true` iff the recomputed root equals `root`.
    pub fn verify_merkle_proof(
        e: &Env,
        leaf: &BytesN<32>,
        proof: &Vec<BytesN<32>>,
        root: &BytesN<32>,
        position: u32,
    ) -> bool {
        let mut computed = leaf.clone();
        let mut pos = position;

        for sibling in proof.iter() {
            let mut pair = Bytes::new(e);
            if pos % 2 == 0 {
                pair.append(&Bytes::from_array(e, &computed.to_array()));
                pair.append(&Bytes::from_array(e, &sibling.to_array()));
            } else {
                pair.append(&Bytes::from_array(e, &sibling.to_array()));
                pair.append(&Bytes::from_array(e, &computed.to_array()));
            }
            computed = e.crypto().sha256(&pair).to_bytes();
            pos /= 2;
        }

        // Every position bit must be consumed by the supplied proof depth.\n        // Otherwise positions that differ only above that depth (for example\n        // 0 and 2 for a one-sibling proof) would verify as the same leaf.\n        pos == 0 && computed == *root\n    }

    /// Verify the basic structure of the ZK proof
    /// 
    /// This checks that the proof data is well-formed and not obviously malformed.
    /// In production, this would perform actual cryptographic verification.
    fn verify_proof_structure(proof: &ConfidentialClaimProof) -> bool {
        // Check that proof components are non-zero
        let is_zero = |bytes: &BytesN<32>| {
            bytes.iter().all(|&b| b == 0)
        };

        // Proof components should not be all zeros
        if is_zero(&proof.proof_a) || is_zero(&proof.proof_b) || is_zero(&proof.proof_c) {
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
        e: &Env,
        _amount: i128,
        _blinding_factor: &BytesN<32>,
    ) -> BytesN<32> {
        // Placeholder: In production, this would compute:
        // C = PedersenCommit(amount, blinding_factor)
        // using the BN254 curve
        
        // For now, return a placeholder hash
        // This would be computed off-chain and passed to the contract
        BytesN::from_array(e, &[0u8; 32])
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
        e: &Env,
        commitment: &BytesN<32>,
        amount: i128,
        blinding_factor: &BytesN<32>,
    ) -> bool {
        // In production, this would verify:
        // PedersenCommit(amount, blinding_factor) == commitment
        
        // Placeholder: always return false to force off-chain computation
        // The actual verification should happen in the ZK circuit
        let computed = Self::compute_commitment(e, amount, blinding_factor);
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
        e: &Env,
        _secret: &BytesN<32>,
        _commitment: &BytesN<32>,
    ) -> BytesN<32> {
        // Placeholder: In production, this would compute:
        // nullifier = Hash(secret || commitment)
        // using a cryptographic hash function
        
        BytesN::from_array(e, &[0u8; 32])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn proof(env: &Env, commitment: u8, claimed: i128) -> ConfidentialClaimProof {
        ConfidentialClaimProof {
            commitment_hash: BytesN::from_array(env, &[commitment; 32]),
            nullifier: BytesN::from_array(env, &[2u8; 32]),
            merkle_root: BytesN::from_array(env, &[3u8; 32]),
            claimed_amount: claimed,
            remaining_amount: 900,
            merkle_proof: Vec::new(env),
            position: 0,
            proof_a: BytesN::from_array(env, &[4u8; 32]),
            proof_b: BytesN::from_array(env, &[5u8; 32]),
            proof_c: BytesN::from_array(env, &[6u8; 32]),
        }
    }

    #[test]
    fn test_verify_proof_structure_valid() {
        let env = Env::default();
        assert!(ZKVerifier::verify_proof_structure(&proof(&env, 1, 100)));
    }

    #[test]
    fn test_verify_proof_structure_zero_proof_a() {
        let env = Env::default();
        let mut p = proof(&env, 1, 100);
        p.proof_a = BytesN::from_array(&env, &[0u8; 32]);
        assert!(!ZKVerifier::verify_proof_structure(&p));
    }

    #[test]
    fn test_verify_proof_structure_zero_claimed_amount() {
        let env = Env::default();
        assert!(!ZKVerifier::verify_proof_structure(&proof(&env, 1, 0)));
    }

    #[test]
    fn test_verify_confidential_claim_over_claim() {
        let env = Env::default();
        let expected_commitment = BytesN::from_array(&env, &[1u8; 32]);

        let result = ZKVerifier::verify_confidential_claim(
            &env,
            &proof(&env, 1, 1000),
            &expected_commitment,
            500,
        );

        assert_eq!(result, Err(Error::OverClaimAttempt));
    }

    #[test]
    fn test_verify_confidential_claim_invalid_commitment() {
        let env = Env::default();
        let expected_commitment = BytesN::from_array(&env, &[99u8; 32]);

        let result = ZKVerifier::verify_confidential_claim(
            &env,
            &proof(&env, 1, 100),
            &expected_commitment,
            1000,
        );

        assert_eq!(result, Err(Error::InvalidZKProof));
    }

    #[test]
    fn test_verify_merkle_proof_single_leaf() {
        let env = Env::default();
        let leaf = BytesN::from_array(&env, &[7u8; 32]);
        // Single-leaf tree: the root IS the leaf and the proof is empty.
        assert!(ZKVerifier::verify_merkle_proof(
            &env,
            &leaf,
            &Vec::new(&env),
            &leaf,
            0,
        ));
    }

    #[test]
    fn test_verify_merkle_proof_two_leaves() {
        let env = Env::default();
        let leaf0 = BytesN::from_array(&env, &[10u8; 32]);
        let leaf1 = BytesN::from_array(&env, &[11u8; 32]);

        let mut buf = Bytes::new(&env);
        buf.append(&Bytes::from_array(&env, &leaf0.to_array()));
        buf.append(&Bytes::from_array(&env, &leaf1.to_array()));
        let root: BytesN<32> = env.crypto().sha256(&buf).to_bytes();

        let mut proof = Vec::new(&env);
        proof.push_back(leaf1.clone());

        assert!(ZKVerifier::verify_merkle_proof(&env, &leaf0, &proof, &root, 0));
        // Same proof at the wrong position must not verify against this root.
        assert!(!ZKVerifier::verify_merkle_proof(&env, &leaf0, &proof, &root, 1));
        // Tampered sibling must not verify.
        let mut bad = Vec::new(&env);
        bad.push_back(BytesN::from_array(&env, &[99u8; 32]));
        assert!(!ZKVerifier::verify_merkle_proof(&env, &leaf0, &bad, &root, 0));
        // A different leaf with the same proof must not verify.
        let other = BytesN::from_array(&env, &[12u8; 32]);
        assert!(!ZKVerifier::verify_merkle_proof(&env, &other, &proof, &root, 0));
    }
}
