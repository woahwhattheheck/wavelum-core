//! Contract data types and event payloads used by the vesting vault.
//!
//! These types form the serialized boundary for storage records, public entry-point
//! results, and contract events. Field documentation describes the on-chain meaning
//! so integrators can decode values without relying on implementation details.

use soroban_sdk::{contracttype, contractevent, Address, Vec, Map, String, BytesN, Bytes};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
/// Enumerates stream pause reason values used by the vesting vault.
pub enum StreamPauseReason {
    /// Suspicious activity.
    SuspiciousActivity,
    /// Anomalous claim pattern.
    AnomalousClaimPattern,
    /// Compromised address.
    CompromisedAddress,
    /// Rapid withdrawal.
    RapidWithdrawal,
    /// Unusual behavior.
    UnusualBehavior,
    /// Manual review.
    ManualReview,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing stream pause.
pub struct StreamPause {
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Ledger timestamp for paused at.
    pub paused_at: u64,
    /// Paused by for this record.
    pub paused_by: Address,
    /// Reason for this record.
    pub reason: StreamPauseReason,
    /// Whether active.
    pub is_active: bool,
    /// Notes for this record.
    pub notes: String,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records stream paused.
pub struct StreamPaused {
    #[topic]
    /// Identifier for vesting.
    pub vesting_id: u32,
    #[topic]
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Ledger timestamp for paused at.
    pub paused_at: u64,
    /// Paused by for this record.
    pub paused_by: Address,
    /// Reason for this record.
    pub reason: StreamPauseReason,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records stream unpaused.
pub struct StreamUnpaused {
    #[topic]
    /// Identifier for vesting.
    pub vesting_id: u32,
    #[topic]
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Ledger timestamp for unpaused at.
    pub unpaused_at: u64,
    /// Unpaused by for this record.
    pub unpaused_by: Address,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing claim event.
pub struct ClaimEvent {
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Amount used for this record.
    pub amount: i128,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
    /// Identifier for vesting.
    pub vesting_id: u32,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing authorized payout address.
pub struct AuthorizedPayoutAddress {
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Address associated with authorized address.
    pub authorized_address: Address,
    /// Ledger timestamp for requested at.
    pub requested_at: u64,
    /// Ledger timestamp for effective at.
    pub effective_at: u64,
    /// Whether active.
    pub is_active: bool,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing address whitelist request.
pub struct AddressWhitelistRequest {
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Address associated with requested address.
    pub requested_address: Address,
    /// Ledger timestamp for requested at.
    pub requested_at: u64,
    /// Ledger timestamp for effective at.
    pub effective_at: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records authorized address set.
pub struct AuthorizedAddressSet {
    #[topic]
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Address associated with authorized address.
    pub authorized_address: Address,
    /// Ledger timestamp for effective at.
    pub effective_at: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records address whitelist requested.
pub struct AddressWhitelistRequested {
    #[topic]
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Address associated with requested address.
    pub requested_address: Address,
    /// Ledger timestamp for requested at.
    pub requested_at: u64,
    /// Ledger timestamp for effective at.
    pub effective_at: u64,
}

// Milestone vesting types
#[contracttype]
#[derive(Clone)]
/// Contract data describing milestone config.
pub struct MilestoneConfig {
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Unlock percentage assigned to each milestone; entries should sum to 100.
    pub milestone_percentages: Vec<u32>, // Percentage for each milestone (e.g., [25, 25, 50])
    /// Total milestones used for this record.
    pub total_milestones: u32,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing milestone status.
pub struct MilestoneStatus {
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Completion map keyed by milestone number.
    pub completed_milestones: Map<u32, bool>, // milestone_number -> completed
    /// Last completed for this record.
    pub last_completed: u32,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records milestone completed.
pub struct MilestoneCompleted {
    #[topic]
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Milestone number for this record.
    pub milestone_number: u32,
    /// Ledger timestamp for completed at.
    pub completed_at: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records milestone configured.
pub struct MilestoneConfigured {
    #[topic]
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Milestone percentages used for this record.
    pub milestone_percentages: Vec<u32>,
    /// Total milestones used for this record.
    pub total_milestones: u32,
    /// Ledger timestamp for configured at.
    pub configured_at: u64,
}

// Simulation types
#[contracttype]
#[derive(Clone)]
/// Contract data describing claim simulation.
pub struct ClaimSimulation {
    /// Tokens to release for this record.
    pub tokens_to_release: i128,
    /// Estimated gas fee used for this record.
    pub estimated_gas_fee: u64,
    /// Tax withholding amount used for this record.
    pub tax_withholding_amount: i128,
    /// Net amount used for this record.
    pub net_amount: i128,
    /// Whether claim.
    pub can_claim: bool,
    /// Reason for this record.
    pub reason: String,
}

// Tax configuration for a vesting schedule
#[contracttype]
#[derive(Clone)]
/// Contract data describing tax config.
pub struct TaxConfig {
    /// Tax rate in basis points, where 10,000 equals 100%.
    pub tax_bps: u32, // basis points (10000 = 100%)
    /// Tax-authority address that receives withheld payments.
    pub authority: Address, // tax authority receiving payments
    /// Optional asset in which tax must be paid; `None` uses the claim asset.
    pub tax_asset: Option<Address>, // if Some, tax must be paid in this asset (may require swap)
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records tax withheld.
pub struct TaxWithheld {
    #[topic]
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Gross amount used for this record.
    pub gross_amount: i128,
    /// Tax amount used for this record.
    pub tax_amount: i128,
    /// Net amount used for this record.
    pub net_amount: i128,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

// Backwards-compatible tax withholding config used by other helpers
#[contracttype]
#[derive(Clone)]
/// Contract data describing tax withholding config.
pub struct TaxWithholdingConfig {
    /// Address associated with tax treasury address.
    pub tax_treasury_address: Address,
    /// Tax withholding basis points for this record.
    pub tax_withholding_bps: u32,
    /// Whether enabled.
    pub enabled: bool,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records tax withholding configured.
pub struct TaxWithholdingConfigured {
    /// Address associated with tax treasury address.
    pub tax_treasury_address: Address,
    /// Tax withholding basis points for this record.
    pub tax_withholding_bps: u32,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records tax withholding disabled.
pub struct TaxWithholdingDisabled {
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

// SEP-12 Oracle config
#[contracttype]
#[derive(Clone)]
/// Contract data describing SEP-12 identity oracle.
pub struct SEP12IdentityOracle {
    /// Address associated with contract address.
    pub contract_address: Address,
    /// Whether enabled.
    pub enabled: bool,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records SEP-12 oracle configured.
pub struct SEP12OracleConfigured {
    /// Address associated with oracle address.
    pub oracle_address: Address,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records SEP-12 kycdisabled.
pub struct SEP12KYCDisabled {
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records kyccheck failed.
pub struct KYCCheckFailed {
    #[topic]
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Reason for this record.
    pub reason: String,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

// Token metadata registry
#[contracttype]
#[derive(Clone)]
/// Contract data describing token metadata.
pub struct TokenMetadata {
    /// Address associated with asset address.
    pub asset_address: Address,
    /// Decimals for this record.
    pub decimals: u32,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records token metadata registered.
pub struct TokenMetadataRegistered {
    /// Address associated with asset address.
    pub asset_address: Address,
    /// Decimals for this record.
    pub decimals: u32,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

// Vesting grant for revocability expiration
#[contracttype]
#[derive(Clone)]
/// Contract data describing vesting grant.
pub struct VestingGrant {
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Ledger timestamp for created at.
    pub created_at: u64,
    /// Whether revocable.
    pub is_revocable: bool,
    /// Ledger timestamp for revocability expires at.
    pub revocability_expires_at: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records vesting grant created.
pub struct VestingGrantCreated {
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Whether revocable.
    pub is_revocable: bool,
    /// Ledger timestamp for revocability expires at.
    pub revocability_expires_at: u64,
    /// Ledger timestamp for created at.
    pub created_at: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records revocability expired.
pub struct RevocabilityExpired {
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Ledger timestamp for expired at.
    pub expired_at: u64,
}

// Reputation bridge types
#[contracttype]
#[derive(Clone)]
/// Contract data describing reputation bonus.
pub struct ReputationBonus {
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Cliff reduction months for this record.
    pub cliff_reduction_months: u32,
    /// Ledger timestamp for applied at.
    pub applied_at: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records reputation bonus applied.
pub struct ReputationBonusApplied {
    #[topic]
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Cliff reduction months for this record.
    pub cliff_reduction_months: u32,
    /// Ledger timestamp for applied at.
    pub applied_at: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records reputation bridge set.
pub struct ReputationBridgeSet {
    #[topic]
    /// Address associated with admin.
    pub admin: Address,
    /// Bridge contract for this record.
    pub bridge_contract: Address,
    /// Ledger timestamp for set at.
    pub set_at: u64,
}

// Zero-Knowledge Privacy Claims types
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
/// Contract data describing nullifier.
pub struct Nullifier {
    /// 256-bit nullifier or commitment hash.
    pub hash: BytesN<32>, // 256-bit hash
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing commitment.
pub struct Commitment {
    /// 256-bit commitment hash used as the storage lookup key.
    pub hash: BytesN<32>, // 256-bit hash
    /// Ledger timestamp for created at.
    pub created_at: u64,
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Amount used for this record.
    pub amount: i128,
    /// Whether used.
    pub is_used: bool,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing zkclaim proof.
pub struct ZKClaimProof {
    /// Commitment hash value used by the contract.
    pub commitment_hash: BytesN<32>,
    /// Nullifier hash value used by the contract.
    pub nullifier_hash: BytesN<32>,
    /// Merkle root value used by the contract.
    pub merkle_root: BytesN<32>,
    /// Serialized zero-knowledge proof bytes supplied to the verifier.
    pub proof_data: Bytes, // Placeholder for actual ZK-SNARK proof
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing privacy claim event.
pub struct PrivacyClaimEvent {
    /// Nullifier value used by the contract.
    pub nullifier: Nullifier,
    /// Amount used for this record.
    pub amount: i128,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Whether private.
    pub is_private: bool,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records commitment created.
pub struct CommitmentCreated {
    #[topic]
    /// Commitment hash value used by the contract.
    pub commitment_hash: BytesN<32>,
    #[topic]
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Amount used for this record.
    pub amount: i128,
    /// Ledger timestamp for created at.
    pub created_at: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records private claim executed.
pub struct PrivateClaimExecuted {
    #[topic]
    /// Nullifier hash value used by the contract.
    pub nullifier_hash: BytesN<32>,
    /// Amount used for this record.
    pub amount: i128,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records merkle root admin added.
pub struct MerkleRootAdminAdded {
    #[topic]
    /// Merkle root value used by the contract.
    pub merkle_root: BytesN<32>,
    /// Address associated with admin.
    pub admin: Address,
    /// Ledger timestamp for added at.
    pub added_at: u64,
}

// Stellar Horizon Path Payment Claim types
#[contracttype]
#[derive(Clone)]
/// Contract data describing path payment config.
pub struct PathPaymentConfig {
    /// Asset that should be received after the path payment.
    pub destination_asset: Address, // USDC or other stablecoin
    /// Min destination amount used for this record.
    pub min_destination_amount: i128,
    /// Ordered asset path used by the swap.
    pub path: Vec<Address>, // Path of assets for the swap
    /// Whether enabled.
    pub enabled: bool,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing path payment claim event.
pub struct PathPaymentClaimEvent {
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Source amount used for this record.
    pub source_amount: i128,
    /// Destination amount used for this record.
    pub destination_amount: i128,
    /// Destination asset for this record.
    pub destination_asset: Address,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
    /// Identifier for vesting.
    pub vesting_id: u32,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing path payment simulation.
pub struct PathPaymentSimulation {
    /// Source amount used for this record.
    pub source_amount: i128,
    /// Estimated destination amount used for this record.
    pub estimated_destination_amount: i128,
    /// Min destination amount used for this record.
    pub min_destination_amount: i128,
    /// Path for this record.
    pub path: Vec<Address>,
    /// Whether execute.
    pub can_execute: bool,
    /// Reason for this record.
    pub reason: String,
    /// Estimated gas fee used for this record.
    pub estimated_gas_fee: u64,
}
#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records path payment configured.
pub struct PathPaymentConfigured {
    /// Destination asset for this record.
    pub destination_asset: Address,
    /// Min destination amount used for this record.
    pub min_destination_amount: i128,
    /// Path for this record.
    pub path: Vec<Address>,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records path payment disabled.
pub struct PathPaymentDisabled {
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records path payment claim executed.
pub struct PathPaymentClaimExecuted {
    #[topic]
    /// Address associated with user.
    pub user: Address,
    /// Source amount used for this record.
    pub source_amount: i128,
    /// Destination amount used for this record.
    pub destination_amount: i128,
    /// Destination asset for this record.
    pub destination_asset: Address,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
    #[topic]
    /// Identifier for vesting.
    pub vesting_id: u32,
}

// Lock-up period types
#[contracttype]
#[derive(Clone)]
/// Contract data describing lockup config.
pub struct LockupConfig {
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Lockup duration seconds used for this record.
    pub lockup_duration_seconds: u64,
    /// Whether enabled.
    pub enabled: bool,
    /// Address associated with lockup token address.
    pub lockup_token_address: Address,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records lockup configured.
pub struct LockupConfigured {
    #[topic]
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Lockup duration seconds used for this record.
    pub lockup_duration_seconds: u64,
    /// Address associated with lockup token address.
    pub lockup_token_address: Address,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records lockup disabled.
pub struct LockupDisabled {
    #[topic]
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records lockup claim executed.
pub struct LockupClaimExecuted {
    #[topic]
    /// Address associated with user.
    pub user: Address,
    #[topic]
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Amount used for this record.
    pub amount: i128,
    /// Address associated with lockup token address.
    pub lockup_token_address: Address,
    /// Ledger timestamp for unlock time.
    pub unlock_time: u64,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

// Beneficiary reassignment types (Issue 114)
#[contracttype]
#[derive(Clone)]
/// Contract data describing beneficiary reassignment.
pub struct BeneficiaryReassignment {
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Current beneficiary for this record.
    pub current_beneficiary: Address,
    /// New beneficiary for this record.
    pub new_beneficiary: Address,
    /// Ledger timestamp for requested at.
    pub requested_at: u64,
    /// Ledger timestamp for effective at.
    pub effective_at: u64,
    /// Total amount used for this record.
    pub total_amount: i128,
    /// Whether requires governance veto.
    pub requires_governance_veto: bool,
    /// Whether executed.
    pub is_executed: bool,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing governance veto.
pub struct GovernanceVeto {
    /// Identifier for reassignment.
    pub reassignment_id: u32,
    /// Veto by for this record.
    pub veto_by: Address,
    /// Ledger timestamp for veto at.
    pub veto_at: u64,
    /// Reason for this record.
    pub reason: String,
    /// Voting power for this record.
    pub voting_power: i128,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing veto vote.
pub struct VetoVote {
    /// Voter for this record.
    pub voter: Address,
    /// Identifier for reassignment.
    pub reassignment_id: u32,
    /// Whether vote for veto.
    pub vote_for_veto: bool,
    /// Voting power for this record.
    pub voting_power: i128,
    /// Ledger timestamp for voted at.
    pub voted_at: u64,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing token supply info.
pub struct TokenSupplyInfo {
    /// Total supply used for this record.
    pub total_supply: i128,
    /// Last updated for this record.
    pub last_updated: u64,
}

// Governance veto events (contracttype for Soroban SDK compatibility)
#[contracttype]
#[derive(Clone)]
/// Contract data describing beneficiary reassignment requested.
pub struct BeneficiaryReassignmentRequested {
    /// Identifier for reassignment.
    pub reassignment_id: u32,
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Current beneficiary for this record.
    pub current_beneficiary: Address,
    /// New beneficiary for this record.
    pub new_beneficiary: Address,
    /// Total amount used for this record.
    pub total_amount: i128,
    /// Ledger timestamp for effective at.
    pub effective_at: u64,
    /// Whether requires governance veto.
    pub requires_governance_veto: bool,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing beneficiary reassignment executed.
pub struct BeneficiaryReassignmentExecuted {
    /// Identifier for reassignment.
    pub reassignment_id: u32,
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Old beneficiary for this record.
    pub old_beneficiary: Address,
    /// New beneficiary for this record.
    pub new_beneficiary: Address,
    /// Ledger timestamp for executed at.
    pub executed_at: u64,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing veto period started.
pub struct VetoPeriodStarted {
    /// Identifier for reassignment.
    pub reassignment_id: u32,
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Veto deadline for this record.
    pub veto_deadline: u64,
    /// Threshold percentage used for this record.
    pub threshold_percentage: u32,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing veto vote cast.
pub struct VetoVoteCast {
    /// Voter for this record.
    pub voter: Address,
    /// Identifier for reassignment.
    pub reassignment_id: u32,
    /// Whether vote for veto.
    pub vote_for_veto: bool,
    /// Voting power for this record.
    pub voting_power: i128,
    /// Ledger timestamp for voted at.
    pub voted_at: u64,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing reassignment vetoed.
pub struct ReassignmentVetoed {
    /// Identifier for reassignment.
    pub reassignment_id: u32,
    /// Veto triggered by for this record.
    pub veto_triggered_by: Address,
    /// Veto power for this record.
    pub veto_power: i128,
    /// Ledger timestamp for vetoed at.
    pub vetoed_at: u64,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing reassignment approved.
pub struct ReassignmentApproved {
    /// Identifier for reassignment.
    pub reassignment_id: u32,
    /// Ledger timestamp for approved at.
    pub approved_at: u64,
    /// Total veto power used for this record.
    pub total_veto_power: i128,
}

// LST Deposit support
#[contracttype]
#[derive(Clone)]
/// Contract data describing lstconfig.
pub struct LSTConfig {
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Whether enabled.
    pub enabled: bool,
    /// Address associated with LST token address.
    pub lst_token_address: Address,
    /// Address associated with base token address.
    pub base_token_address: Address,
    /// Address associated with staking contract address.
    pub staking_contract_address: Address,
    /// Unbonding period seconds for this record.
    pub unbonding_period_seconds: u64,
}

// LST Auto-Compounding types (Issue #154)
#[contracttype]
#[derive(Clone)]
/// Contract data describing lstpool shares.
pub struct LSTPoolShares {
    /// Total shares in the pool (tracks ownership proportionally)
    pub total_shares: i128,
    /// Total underlying tokens in the pool (including compounded rewards)
    pub total_underlying: i128,
    /// Last compounding timestamp
    pub last_compounded_at: u64,
    /// Exchange rate snapshot (for security against manipulation)
    pub exchange_rate_snapshot: i128,
    /// Snapshot timestamp
    pub snapshot_timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing user lstshares.
pub struct UserLSTShares {
    /// User's share balance in the pool
    pub shares: i128,
    /// User's vesting ID
    pub vesting_id: u32,
    /// Whether the user has an unbonding request pending
    pub unbonding_pending: bool,
    /// Unbonding request timestamp (0 if not pending)
    pub unbonding_requested_at: u64,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing unbonding request.
pub struct UnbondingRequest {
    /// Address associated with user.
    pub user: Address,
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Shares used for this record.
    pub shares: i128,
    /// Ledger timestamp for requested at.
    pub requested_at: u64,
    /// Ledger timestamp for unbonding complete at.
    pub unbonding_complete_at: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records lstconfigured.
pub struct LSTConfigured {
    #[topic]
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Address associated with LST token address.
    pub lst_token_address: Address,
    /// Address associated with base token address.
    pub base_token_address: Address,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records lstclaim executed.
pub struct LSTClaimExecuted {
    #[topic]
    /// Address associated with user.
    pub user: Address,
    #[topic]
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Base amount used for this record.
    pub base_amount: i128,
    /// LST amount used for this record.
    pub lst_amount: i128,
    /// Address associated with LST token address.
    pub lst_token_address: Address,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records lstrewards compounded.
pub struct LSTRewardsCompounded {
    #[topic]
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Total yield generated used for this record.
    pub total_yield_generated: i128,
    /// Total shares used for this record.
    pub total_shares: i128,
    /// Exchange rate used for this record.
    pub exchange_rate: i128,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records unbonding requested.
pub struct UnbondingRequested {
    #[topic]
    /// Address associated with user.
    pub user: Address,
    #[topic]
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Shares used for this record.
    pub shares: i128,
    /// Ledger timestamp for unbonding complete at.
    pub unbonding_complete_at: u64,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records unbonding completed.
pub struct UnbondingCompleted {
    #[topic]
    /// Address associated with user.
    pub user: Address,
    #[topic]
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Shares used for this record.
    pub shares: i128,
    /// Underlying amount used for this record.
    pub underlying_amount: i128,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

// ========== ISSUE #223: Cross-Contract balanceOf Adapter for DAO Voting ==========

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records voting power queried.
pub struct VotingPowerQueried {
    #[topic]
    /// Voter for this record.
    pub voter: Address,
    /// Voting power for this record.
    pub voting_power: i128,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

// ========== ISSUE #226: Admin Dead-Man's Switch ==========

/// 365 days in seconds
pub const ADMIN_INACTIVITY_TIMEOUT: u64 = 31_536_000;

#[contracttype]
#[derive(Clone)]
/// Contract data describing admin dead man switch.
pub struct AdminDeadManSwitch {
    /// The recovery address that can claim admin rights after inactivity
    pub recovery_address: Address,
    /// Timestamp of the last admin activity
    pub last_admin_activity: u64,
    /// Whether the switch has been triggered (recovery claimed)
    pub is_triggered: bool,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records admin recovery address set.
pub struct AdminRecoveryAddressSet {
    #[topic]
    /// Address associated with recovery address.
    pub recovery_address: Address,
    /// Ledger timestamp for set at.
    pub set_at: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records admin activity recorded.
pub struct AdminActivityRecorded {
    #[topic]
    /// Address associated with admin.
    pub admin: Address,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records admin recovery claimed.
pub struct AdminRecoveryClaimed {
    #[topic]
    /// Address associated with recovery address.
    pub recovery_address: Address,
    /// Ledger timestamp for claimed at.
    pub claimed_at: u64,
}

// ========== ISSUE #228: Oracle Price Deviation Circuit Breaker ==========

/// 30% deviation threshold (in basis points: 3000 = 30%)
pub const ORACLE_DEVIATION_THRESHOLD_BPS: u32 = 3000;

#[contracttype]
#[derive(Clone)]
/// Contract data describing oracle price record.
pub struct OraclePriceRecord {
    /// Price at the last ledger (scaled by 10^7)
    pub last_price: i128,
    /// Ledger sequence number of the last price update
    pub last_ledger: u32,
    /// Whether the circuit breaker is currently tripped
    pub is_frozen: bool,
    /// Timestamp when the freeze was triggered (0 if not frozen)
    pub frozen_at: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records oracle price updated.
pub struct OraclePriceUpdated {
    /// Old price used for this record.
    pub old_price: i128,
    /// New price used for this record.
    pub new_price: i128,
    /// Ledger for this record.
    pub ledger: u32,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records oracle circuit breaker tripped.
pub struct OracleCircuitBreakerTripped {
    /// Old price used for this record.
    pub old_price: i128,
    /// New price used for this record.
    pub new_price: i128,
    /// Deviation basis points for this record.
    pub deviation_bps: u32,
    /// Ledger timestamp for tripped at.
    pub tripped_at: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records oracle circuit breaker reset.
pub struct OracleCircuitBreakerReset {
    /// Address associated with reset by.
    pub reset_by: Address,
    /// Ledger timestamp for reset at.
    pub reset_at: u64,
}

// ========== ISSUE #231: Self-Destruct Prevention ==========

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records upgrade blocked.
pub struct UpgradeBlocked {
    /// Total unvested balance used for this record.
    pub total_unvested_balance: i128,
    /// Ledger timestamp for blocked at.
    pub blocked_at: u64,
}

// ========== ISSUE #269: Zero-Knowledge Confidential Grant Amounts ==========

/// Confidential grant storing commitment instead of plaintext amount
#[contracttype]
#[derive(Clone)]
pub struct ConfidentialGrant {
    /// Hash commitment of the total grant amount (Pedersen commitment)
    pub commitment_hash: BytesN<32>,
    /// Vesting schedule identifier
    pub vesting_id: u32,
    /// Timestamp when grant was created
    pub created_at: u64,
    /// Whether this grant has been fully claimed
    pub is_fully_claimed: bool,
    /// Remaining shielded amount (encrypted, for internal tracking)
    pub remaining_shielded: i128,
}

/// Master viewing key for DAO clawback operations
#[contracttype]
#[derive(Clone)]
pub struct MasterViewingKey {
    /// Public key for viewing shielded amounts
    pub viewing_key: BytesN<32>,
    /// Admin address that authorized this key
    pub authorized_by: Address,
    /// Timestamp when key was set
    pub set_at: u64,
    /// Whether key is currently active
    pub is_active: bool,
}

/// Enhanced ZK proof for confidential claims (Circom-compatible)
#[contracttype]
#[derive(Clone)]
pub struct ConfidentialClaimProof {
    /// Public inputs for the ZK circuit
    pub commitment_hash: BytesN<32>,
    /// Nullifier to prevent double-spending
    pub nullifier: BytesN<32>,
    /// Merkle root of the commitment tree
    pub merkle_root: BytesN<32>,
    /// Claimed amount (public output)
    pub claimed_amount: i128,
    /// Remaining amount after claim (public output)
    pub remaining_amount: i128,
    /// The actual ZK-SNARK proof (Circom output)
    pub proof_a: BytesN<32>,
    /// Proof b for this record.
    pub proof_b: BytesN<32>,
    /// Proof c for this record.
    pub proof_c: BytesN<32>,
}

/// Event emitted when a confidential claim is executed
#[contractevent]
#[derive(Clone)]
pub struct ConfidentialClaimExecuted {
    /// Nullifier hash (leaks zero metadata about the claimer)
    #[topic]
    pub nullifier_hash: BytesN<32>,
    /// Updated commitment hash after claim
    #[topic]
    pub new_commitment_hash: BytesN<32>,
    /// Timestamp of the claim
    pub timestamp: u64,
}

/// Event emitted when a confidential grant is created
#[contractevent]
#[derive(Clone)]
pub struct ConfidentialGrantCreated {
    /// Vesting ID
    #[topic]
    pub vesting_id: u32,
    /// Commitment hash of the total grant
    #[topic]
    pub commitment_hash: BytesN<32>,
    /// Timestamp of creation
    pub timestamp: u64,
}

/// Event emitted when DAO performs clawback using master viewing key
#[contractevent]
#[derive(Clone)]
pub struct ConfidentialClawbackExecuted {
    /// Vesting ID
    #[topic]
    pub vesting_id: u32,
    /// Amount clawed back
    pub clawed_amount: i128,
    /// Admin who authorized the clawback
    #[topic]
    pub authorized_by: Address,
    /// Timestamp of clawback
    pub timestamp: u64,
}

// ========== ISSUE #295: Temporary Storage for Claim-History Pagination ==========

#[contracttype]
#[derive(Clone)]
/// Contract data describing pagination state.
pub struct PaginationState {
    /// Current page used for this record.
    pub current_page: u32,
    /// Total items used for this record.
    pub total_items: u32,
    /// Last updated for this record.
    pub last_updated: u64,
}

#[contracttype]
#[derive(Clone)]
/// Contract data describing claim history page.
pub struct ClaimHistoryPage {
    /// Page number used for this record.
    pub page_number: u32,
    /// Claims for this record.
    pub claims: Vec<ClaimEvent>,
    /// Whether next.
    pub has_next: bool,
    /// Total pages used for this record.
    pub total_pages: u32,
}

// ========== ISSUE #296: Force-Withdrawal for Expired Schedules ==========

#[contracttype]
#[derive(Clone)]
/// Contract data describing expired schedule.
pub struct ExpiredSchedule {
    /// Identifier for vesting.
    pub vesting_id: u32,
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Total amount used for this record.
    pub total_amount: i128,
    /// Claimed amount used for this record.
    pub claimed_amount: i128,
    /// Ledger timestamp for expires at.
    pub expires_at: u64,
    /// Whether force withdrawn.
    pub is_force_withdrawn: bool,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records force withdrawal executed.
pub struct ForceWithdrawalExecuted {
    #[topic]
    /// Identifier for vesting.
    pub vesting_id: u32,
    #[topic]
    /// Address associated with beneficiary.
    pub beneficiary: Address,
    /// Withdrawn amount used for this record.
    pub withdrawn_amount: i128,
    /// Reason for this record.
    pub reason: String,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

// ========== ISSUE #297: Max-Allocation-Sanity-Check ==========

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records max allocation limit set.
pub struct MaxAllocationLimitSet {
    /// Max limit used for this record.
    pub max_limit: i128,
    /// Address associated with set by.
    pub set_by: Address,
    /// Ledger timestamp for timestamp.
    pub timestamp: u64,
}

#[contractevent]
#[derive(Clone)]
/// Event payload emitted when the contract records allocation limit exceeded.
pub struct AllocationLimitExceeded {
    #[topic]
    /// Attempted allocation for this record.
    pub attempted_allocation: i128,
    /// Max limit used for this record.
    pub max_limit: i128,
    /// Ledger timestamp for rejected at.
    pub rejected_at: u64,
}

// ========== ISSUE #276: Vesting Schedule Consolidation and Mergers ==========

/// Master schedule created from merging multiple schedules
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct MasterSchedule {
    /// Unique identifier for the master schedule
    pub master_id: u32,
    /// Beneficiary address (all merged schedules must belong to same user)
    pub beneficiary: Address,
    /// Asset address (all merged schedules must have same asset)
    pub asset_address: Address,
    /// Total amount across all merged schedules
    pub total_amount: i128,
    /// Amount already claimed from merged schedules
    pub claimed_amount: i128,
    /// Weighted-average start time from all schedules
    pub start_time: u64,
    /// Weighted-average end time from all schedules
    pub end_time: u64,
    /// Weighted-average cliff duration
    pub cliff_duration: u64,
    /// Original schedule IDs that were merged
    pub merged_schedule_ids: Vec<u32>,
    /// Timestamp when master schedule was created
    pub created_at: u64,
    /// Whether this master schedule is active
    pub is_active: bool,
}

/// Event emitted when schedules are successfully consolidated
#[contractevent]
#[derive(Clone)]
pub struct SchedulesConsolidated {
    /// Beneficiary who initiated the merge
    #[topic]
    pub beneficiary: Address,
    /// Original schedule IDs that were burned
    #[topic]
    pub burned_schedule_ids: Vec<u32>,
    /// New master schedule ID created
    #[topic]
    pub master_schedule_id: u32,
    /// Total consolidated amount
    pub total_amount: i128,
    /// Weighted-average end time
    pub new_end_time: u64,
    /// Timestamp of consolidation
    pub timestamp: u64,
}

/// Storage key for master schedules
pub const MASTER_SCHEDULES: &str = "MASTER_SCHEDULES";

/// Storage key for tracking merged schedule relationships
pub const MERGED_SCHEDULES: &str = "MERGED_SCHEDULES";
