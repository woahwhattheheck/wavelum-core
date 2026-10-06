//! Storage accessors and namespace keys for the vesting vault.
//!
//! Each accessor makes its storage class explicit in code (instance, persistent, or
//! temporary). Tuple keys are built from the documented namespace plus the function
//! parameters, keeping key lifecycle and replacement/removal behavior centralized.

use soroban_sdk::{Env, Vec, Address, Map, BytesN};
use crate::types::{ClaimEvent, AuthorizedPayoutAddress, AddressWhitelistRequest, Nullifier, Commitment, PathPaymentConfig, PathPaymentClaimEvent, LockupConfig, BeneficiaryReassignment, VetoVote, TokenSupplyInfo, LSTConfig, ConfidentialGrant, MasterViewingKey, StreamPause, MasterSchedule};

/// Namespace key for claim history records in instance storage.
pub const CLAIM_HISTORY: &str = "CLAIM_HISTORY";
/// Namespace key for authorized payout address records in instance storage.
/// Composite entries are additionally keyed by `beneficiary`, `auth_address`.
pub const AUTHORIZED_PAYOUT_ADDRESS: &str = "AUTHORIZED_PAYOUT_ADDRESS";
/// Namespace key for pending address request records in instance storage.
/// Composite entries are additionally keyed by `beneficiary`.
pub const PENDING_ADDRESS_REQUEST: &str = "PENDING_ADDRESS_REQUEST";

// Stream pause storage keys for suspicious activity detection
/// Namespace key for stream pauses records in instance storage.
/// Composite entries are additionally keyed by `vesting_id`, `beneficiary`.
pub const STREAM_PAUSES: &str = "STREAM_PAUSES";
/// Namespace key for stream pause history records in instance storage.
pub const STREAM_PAUSE_HISTORY: &str = "STREAM_PAUSE_HISTORY";

// Emergency pause storage keys
/// Namespace key for auditors records in instance storage.
/// Composite entries are additionally keyed by `auditors`.
pub const AUDITORS: &str = "AUDITORS";
/// Namespace key for auditor pause requests records in instance storage.
/// Composite entries are additionally keyed by `requests`, `crate`.
pub const AUDITOR_PAUSE_REQUESTS: &str = "AUDITOR_PAUSE_REQUESTS";
/// Namespace key for emergency pause records in instance storage.
pub const EMERGENCY_PAUSE: &str = "EMERGENCY_PAUSE";

// Cross-project reputation storage keys
/// Namespace key for reputation bridge contract records in instance storage.
pub const REPUTATION_BRIDGE_CONTRACT: &str = "REPUTATION_BRIDGE_CONTRACT";
/// Namespace key for reputation bonus applied records in instance storage.
/// Composite entries are additionally keyed by `beneficiary`.
pub const REPUTATION_BONUS_APPLIED: &str = "REPUTATION_BONUS_APPLIED";

// Milestone vesting storage keys
/// Namespace key for milestone configs records in instance storage.
/// Composite entries are additionally keyed by `vesting_id`.
pub const MILESTONE_CONFIGS: &str = "MILESTONE_CONFIGS";
/// Namespace key for milestone status records in instance storage.
/// Composite entries are additionally keyed by `vesting_id`, `bool>`.
pub const MILESTONE_STATUS: &str = "MILESTONE_STATUS";

// Zero-Knowledge Privacy Claims storage keys
/// Namespace key for nullifier map records in instance storage.
/// Composite entries are additionally keyed by `nullifier`.
pub const NULLIFIER_MAP: &str = "NULLIFIER_MAP";
/// Namespace key for commitment storage records in instance storage.
/// Composite entries are additionally keyed by `commitment_hash`, `commitment`.
pub const COMMITMENT_STORAGE: &str = "COMMITMENT_STORAGE";
/// Namespace key for privacy claim history records in instance storage.
pub const PRIVACY_CLAIM_HISTORY: &str = "PRIVACY_CLAIM_HISTORY";
/// Namespace key for merkle roots records in instance storage.
/// Composite entries are additionally keyed by `merkle_root`.
pub const MERKLE_ROOTS: &str = "MERKLE_ROOTS";

// Stellar Horizon Path Payment Claim storage keys
/// Namespace key for path payment config records in instance storage.
pub const PATH_PAYMENT_CONFIG: &str = "PATH_PAYMENT_CONFIG";
/// Namespace key for path payment claim history records in instance storage.
pub const PATH_PAYMENT_CLAIM_HISTORY: &str = "PATH_PAYMENT_CLAIM_HISTORY";

// Lock-up period storage keys
/// Namespace key for lockup configs records in instance storage.
/// Composite entries are additionally keyed by `vesting_id`.
pub const LOCKUP_CONFIGS: &str = "LOCKUP_CONFIGS";

// Beneficiary reassignment and governance veto storage keys
/// Namespace key for beneficiary reassignments records in instance storage.
/// Composite entries are additionally keyed by `reassignment_id`, `reassignment`.
pub const BENEFICIARY_REASSIGNMENTS: &str = "BENEFICIARY_REASSIGNMENTS";
/// Namespace key for veto votes records in instance storage.
/// Composite entries are additionally keyed by `reassignment_id`.
pub const VETO_VOTES: &str = "VETO_VOTES";
/// Namespace key for token supply info records in instance storage.
pub const TOKEN_SUPPLY_INFO: &str = "TOKEN_SUPPLY_INFO";
/// Namespace key for reassignment counter records in instance storage.
pub const REASSIGNMENT_COUNTER: &str = "REASSIGNMENT_COUNTER";
/// Namespace key for governance veto threshold records in instance storage.
pub const GOVERNANCE_VETO_THRESHOLD: &str = "GOVERNANCE_VETO_THRESHOLD"; // Percentage (e.g., 5 for 5%)
/// Namespace key for lST configs records in instance storage.
/// Composite entries are additionally keyed by `vesting_id`.
pub const LST_CONFIGS: &str = "LST_CONFIGS";

// LST Auto-Compounding storage keys (Issue #154)
/// Namespace key for lST pool shares records in contract storage.
pub const LST_POOL_SHARES: &str = "LST_POOL_SHARES";
/// Namespace key for user LST shares records in contract storage.
pub const USER_LST_SHARES: &str = "USER_LST_SHARES";
/// Namespace key for unbonding requests records in contract storage.
pub const UNBONDING_REQUESTS: &str = "UNBONDING_REQUESTS";
/// Namespace key for unbonding queue records in contract storage.
pub const UNBONDING_QUEUE: &str = "UNBONDING_QUEUE";

// Protocol Sunset and Migration storage keys (Issue #280)
/// Namespace key for protocol sunset records in contract storage.
pub const PROTOCOL_SUNSET: &str = "PROTOCOL_SUNSET";
/// Namespace key for migration payloads records in contract storage.
pub const MIGRATION_PAYLOADS: &str = "MIGRATION_PAYLOADS";
/// Namespace key for relayer migrations records in contract storage.
pub const RELAYER_MIGRATIONS: &str = "RELAYER_MIGRATIONS";

// 48 hours in seconds
const TIMELOCK_DURATION: u64 = 172_800;

// 7 days in seconds for emergency pause
const EMERGENCY_PAUSE_DURATION: u64 = 604_800;

/// Reads claim history from instance storage.
pub fn get_claim_history(e: &Env) -> Vec<ClaimEvent> {
    e.storage()
        .instance()
        .get(&CLAIM_HISTORY)
        .unwrap_or(Vec::new(e))
}

/// Stores claim history in instance storage, replacing the value for the same key.
/// Key inputs: `history`.
pub fn set_claim_history(e: &Env, history: &Vec<ClaimEvent>) {
    e.storage().instance().set(&CLAIM_HISTORY, history);
}

/// Reads authorized payout address from instance storage.
/// Key inputs: `beneficiary`.
pub fn get_authorized_payout_address(e: &Env, beneficiary: &Address) -> Option<AuthorizedPayoutAddress> {
    e.storage()
        .instance()
        .get(&(AUTHORIZED_PAYOUT_ADDRESS, beneficiary))
}

/// Stores authorized payout address in instance storage, replacing the value for the same key.
/// Key inputs: `beneficiary`, `auth_address`.
pub fn set_authorized_payout_address(e: &Env, beneficiary: &Address, auth_address: &AuthorizedPayoutAddress) {
    e.storage().instance().set(&(AUTHORIZED_PAYOUT_ADDRESS, beneficiary), auth_address);
}

/// Reads pending address request from instance storage.
/// Key inputs: `beneficiary`.
pub fn get_pending_address_request(e: &Env, beneficiary: &Address) -> Option<AddressWhitelistRequest> {
    e.storage()
        .instance()
        .get(& (PENDING_ADDRESS_REQUEST, beneficiary))
}

/// Stores pending address request in instance storage, replacing the value for the same key.
/// Key inputs: `beneficiary`, `request`.
pub fn set_pending_address_request(e: &Env, beneficiary: &Address, request: &AddressWhitelistRequest) {
    e.storage().instance().set(&(PENDING_ADDRESS_REQUEST, beneficiary), request);
}

/// Removes pending address request from instance storage.
/// Key inputs: `beneficiary`.
pub fn remove_pending_address_request(e: &Env, beneficiary: &Address) {
    e.storage().instance().remove(&(PENDING_ADDRESS_REQUEST, beneficiary));
}

/// Returns the configured timelock duration in ledger seconds.
pub fn get_timelock_duration() -> u64 {
    TIMELOCK_DURATION
}

// Emergency pause functions
/// Reads auditors from instance storage.
pub fn get_auditors(e: &Env) -> Vec<Address> {
    e.storage()
        .instance()
        .get(&AUDITORS)
        .unwrap_or(Vec::new(e))
}

/// Stores auditors in instance storage, replacing the value for the same key.
/// Key inputs: `auditors`.
pub fn set_auditors(e: &Env, auditors: &Vec<Address>) {
    e.storage().instance().set(&AUDITORS, auditors);
}

/// Reads auditor pause requests from instance storage.
pub fn get_auditor_pause_requests(e: &Env) -> Map<Address, crate::emergency::AuditorPauseRequest> {
    e.storage()
        .instance()
        .get(&AUDITOR_PAUSE_REQUESTS)
        .unwrap_or(Map::new(e))
}

/// Stores auditor pause requests in instance storage, replacing the value for the same key.
/// Key inputs: `requests`, `crate`.
pub fn set_auditor_pause_requests(e: &Env, requests: &Map<Address, crate::emergency::AuditorPauseRequest>) {
    e.storage().instance().set(&AUDITOR_PAUSE_REQUESTS, requests);
}

/// Reads emergency pause from instance storage.
pub fn get_emergency_pause(e: &Env) -> Option<crate::emergency::EmergencyPause> {
    e.storage().instance().get(&EMERGENCY_PAUSE)
}

/// Stores emergency pause in instance storage, replacing the value for the same key.
/// Key inputs: `pause`.
pub fn set_emergency_pause(e: &Env, pause: &crate::emergency::EmergencyPause) {
    e.storage().instance().set(&EMERGENCY_PAUSE, pause);
}

/// Removes emergency pause from instance storage.
pub fn remove_emergency_pause(e: &Env) {
    e.storage().instance().remove(&EMERGENCY_PAUSE);
}

// Cross-project reputation functions
/// Reads reputation bridge contract from instance storage.
pub fn get_reputation_bridge_contract(e: &Env) -> Option<Address> {
    e.storage().instance().get(&REPUTATION_BRIDGE_CONTRACT)
}

/// Stores reputation bridge contract in instance storage, replacing the value for the same key.
/// Key inputs: `contract_address`.
pub fn set_reputation_bridge_contract(e: &Env, contract_address: &Address) {
    e.storage().instance().set(&REPUTATION_BRIDGE_CONTRACT, contract_address);
}

/// Returns whether reputation bonus applied is present or active in instance storage.
/// Key inputs: `beneficiary`.
pub fn has_reputation_bonus_applied(e: &Env, beneficiary: &Address) -> bool {
    e.storage()
        .instance()
        .get(&(REPUTATION_BONUS_APPLIED, beneficiary))
        .unwrap_or(false)
}

/// Stores reputation bonus applied in instance storage, replacing the value for the same key.
/// Key inputs: `beneficiary`.
pub fn set_reputation_bonus_applied(e: &Env, beneficiary: &Address) {
    e.storage().instance().set(&(REPUTATION_BONUS_APPLIED, beneficiary), &true);
}

// Milestone vesting functions
/// Reads milestone configs from instance storage.
/// Key inputs: `vesting_id`.
pub fn get_milestone_configs(e: &Env, vesting_id: u32) -> Option<Vec<u32>> {
    e.storage().instance().get(&(MILESTONE_CONFIGS, vesting_id))
}

/// Stores milestone configs in instance storage, replacing the value for the same key.
/// Key inputs: `vesting_id`, `milestones`.
pub fn set_milestone_configs(e: &Env, vesting_id: u32, milestones: &Vec<u32>) {
    e.storage().instance().set(&(MILESTONE_CONFIGS, vesting_id), milestones);
}

/// Reads milestone status from instance storage.
/// Key inputs: `vesting_id`.
pub fn get_milestone_status(e: &Env, vesting_id: u32) -> Map<u32, bool> {
    e.storage()
        .instance()
        .get(&(MILESTONE_STATUS, vesting_id))
        .unwrap_or(Map::new(e))
}

/// Stores milestone status in instance storage, replacing the value for the same key.
/// Key inputs: `vesting_id`, `status`, `bool>`.
pub fn set_milestone_status(e: &Env, vesting_id: u32, status: &Map<u32, bool>) {
    e.storage().instance().set(&(MILESTONE_STATUS, vesting_id), status);
}

/// Returns the configured emergency pause duration in ledger seconds.
pub fn get_emergency_pause_duration() -> u64 {
    EMERGENCY_PAUSE_DURATION
}

// Zero-Knowledge Privacy Claims functions

// Nullifier map functions - prevent double-spending in private claims
/// Returns whether nullifier used is present or active in instance storage.
/// Key inputs: `nullifier`.
pub fn is_nullifier_used(e: &Env, nullifier: &Nullifier) -> bool {
    e.storage()
        .instance()
        .get(&(NULLIFIER_MAP, nullifier.clone()))
        .unwrap_or(false)
}

/// Stores nullifier used in instance storage, replacing the value for the same key.
/// Key inputs: `nullifier`.
pub fn set_nullifier_used(e: &Env, nullifier: &Nullifier) {
    e.storage().instance().set(&(NULLIFIER_MAP, nullifier.clone()), &true);
}

// Commitment storage functions
/// Reads commitment from instance storage.
/// Key inputs: `commitment_hash`.
pub fn get_commitment(e: &Env, commitment_hash: &BytesN<32>) -> Option<Commitment> {
    e.storage().instance().get(&(COMMITMENT_STORAGE, commitment_hash.clone()))
}

/// Stores commitment in instance storage, replacing the value for the same key.
/// Key inputs: `commitment_hash`, `commitment`.
pub fn set_commitment(e: &Env, commitment_hash: &BytesN<32>, commitment: &Commitment) {
    e.storage().instance().set(&(COMMITMENT_STORAGE, commitment_hash.clone()), commitment);
}

/// Marks commitment used in contract storage.
/// Key inputs: `commitment_hash`.
pub fn mark_commitment_used(e: &Env, commitment_hash: &BytesN<32>) {
    if let Some(mut commitment) = get_commitment(e, commitment_hash) {
        commitment.is_used = true;
        set_commitment(e, commitment_hash, &commitment);
    }
}

// Privacy claim history functions
/// Reads privacy claim history from instance storage.
pub fn get_privacy_claim_history(e: &Env) -> Vec<crate::types::PrivacyClaimEvent> {
    e.storage()
        .instance()
        .get(&PRIVACY_CLAIM_HISTORY)
        .unwrap_or(Vec::new(e))
}

/// Adds privacy claim event to the corresponding record in instance storage.
/// Key inputs: `event`.
pub fn add_privacy_claim_event(e: &Env, event: &crate::types::PrivacyClaimEvent) {
    let mut history = get_privacy_claim_history(e);
    history.push_back(event.clone());
    e.storage().instance().set(&PRIVACY_CLAIM_HISTORY, &history);
}

// Merkle root management for ZK proofs
/// Adds merkle root to the corresponding record in instance storage.
/// Key inputs: `merkle_root`.
pub fn add_merkle_root(e: &Env, merkle_root: &BytesN<32>) {
    let mut roots = get_merkle_roots(e);
    roots.push_back(merkle_root.clone());
    e.storage().instance().set(&MERKLE_ROOTS, &roots);
}

/// Reads merkle roots from instance storage.
pub fn get_merkle_roots(e: &Env) -> Vec<BytesN<32>> {
    e.storage()
        .instance()
        .get(&MERKLE_ROOTS)
        .unwrap_or(Vec::new(e))
}

/// Returns whether valid merkle root is present or active in contract storage.
/// Key inputs: `merkle_root`.
pub fn is_valid_merkle_root(e: &Env, merkle_root: &BytesN<32>) -> bool {
    let roots = get_merkle_roots(e);
    roots.contains(merkle_root.clone())
}

// Stellar Horizon Path Payment Claim storage functions
/// Reads path payment config from instance storage.
pub fn get_path_payment_config(e: &Env) -> Option<PathPaymentConfig> {
    e.storage().instance().get(&PATH_PAYMENT_CONFIG)
}

/// Stores path payment config in instance storage, replacing the value for the same key.
/// Key inputs: `config`.
pub fn set_path_payment_config(e: &Env, config: &PathPaymentConfig) {
    e.storage().instance().set(&PATH_PAYMENT_CONFIG, config);
}

/// Reads path payment claim history from instance storage.
pub fn get_path_payment_claim_history(e: &Env) -> Vec<PathPaymentClaimEvent> {
    e.storage()
        .instance()
        .get(&PATH_PAYMENT_CLAIM_HISTORY)
        .unwrap_or(Vec::new(e))
}

/// Adds path payment claim event to the corresponding record in instance storage.
/// Key inputs: `event`.
pub fn add_path_payment_claim_event(e: &Env, event: &PathPaymentClaimEvent) {
    let mut history = get_path_payment_claim_history(e);
    history.push_back(event.clone());
    e.storage().instance().set(&PATH_PAYMENT_CLAIM_HISTORY, &history);
}

// Lock-up period storage functions
/// Reads lockup config from instance storage.
/// Key inputs: `vesting_id`.
pub fn get_lockup_config(e: &Env, vesting_id: u32) -> Option<LockupConfig> {
    e.storage().instance().get(&(LOCKUP_CONFIGS, vesting_id))
}

/// Stores lockup config in instance storage, replacing the value for the same key.
/// Key inputs: `vesting_id`, `config`.
pub fn set_lockup_config(e: &Env, vesting_id: u32, config: &LockupConfig) {
    e.storage().instance().set(&(LOCKUP_CONFIGS, vesting_id), config);
}

/// Removes lockup config from instance storage.
/// Key inputs: `vesting_id`.
pub fn remove_lockup_config(e: &Env, vesting_id: u32) {
    e.storage().instance().remove(&(LOCKUP_CONFIGS, vesting_id));
}

// ========== TAX CONFIGURATION STORAGE ==========
/// Namespace key for tax configs records in instance storage.
/// Composite entries are additionally keyed by `vesting_id`.
pub const TAX_CONFIGS: &str = "TAX_CONFIGS"; // keyed by vesting_id
/// Namespace key for cumulative taxes records in instance storage.
/// Composite entries are additionally keyed by `vesting_id`, `authority`.
pub const CUMULATIVE_TAXES: &str = "CUMULATIVE_TAXES"; // keyed by (vesting_id, authority)

/// Reads tax config from instance storage.
/// Key inputs: `vesting_id`.
pub fn get_tax_config(e: &Env, vesting_id: u32) -> Option<crate::types::TaxConfig> {
    e.storage().instance().get(&(TAX_CONFIGS, vesting_id))
}

/// Stores tax config in instance storage, replacing the value for the same key.
/// Key inputs: `vesting_id`, `cfg`.
pub fn set_tax_config(e: &Env, vesting_id: u32, cfg: &crate::types::TaxConfig) {
    e.storage().instance().set(&(TAX_CONFIGS, vesting_id), cfg);
}

/// Reads cumulative taxes from instance storage.
/// Key inputs: `vesting_id`, `authority`.
pub fn get_cumulative_taxes(e: &Env, vesting_id: u32, authority: &Address) -> i128 {
    e.storage()
        .instance()
        .get(&(CUMULATIVE_TAXES, vesting_id, authority.clone()))
        .unwrap_or(0i128)
}

/// Adds cumulative taxes to the corresponding record in instance storage.
/// Key inputs: `vesting_id`, `authority`, `amount`.
pub fn add_cumulative_taxes(e: &Env, vesting_id: u32, authority: &Address, amount: i128) {
    let prev = get_cumulative_taxes(e, vesting_id, authority);
    e.storage().instance().set(&(CUMULATIVE_TAXES, vesting_id, authority.clone()), &(prev + amount));
}

// Beneficiary reassignment and governance veto storage functions
/// Reads reassignment counter from instance storage.
pub fn get_reassignment_counter(e: &Env) -> u32 {
    e.storage()
        .instance()
        .get(&REASSIGNMENT_COUNTER)
        .unwrap_or(0)
}

/// Stores reassignment counter in instance storage, replacing the value for the same key.
/// Key inputs: `counter`.
pub fn set_reassignment_counter(e: &Env, counter: u32) {
    e.storage().instance().set(&REASSIGNMENT_COUNTER, &counter);
}

/// Reads beneficiary reassignment from instance storage.
/// Key inputs: `reassignment_id`.
pub fn get_beneficiary_reassignment(e: &Env, reassignment_id: u32) -> Option<BeneficiaryReassignment> {
    e.storage().instance().get(&(BENEFICIARY_REASSIGNMENTS, reassignment_id))
}

/// Stores beneficiary reassignment in instance storage, replacing the value for the same key.
/// Key inputs: `reassignment_id`, `reassignment`.
pub fn set_beneficiary_reassignment(e: &Env, reassignment_id: u32, reassignment: &BeneficiaryReassignment) {
    e.storage().instance().set(&(BENEFICIARY_REASSIGNMENTS, reassignment_id), reassignment);
}

/// Removes beneficiary reassignment from instance storage.
/// Key inputs: `reassignment_id`.
pub fn remove_beneficiary_reassignment(e: &Env, reassignment_id: u32) {
    e.storage().instance().remove(&(BENEFICIARY_REASSIGNMENTS, reassignment_id));
}

/// Reads veto votes from instance storage.
/// Key inputs: `reassignment_id`.
pub fn get_veto_votes(e: &Env, reassignment_id: u32) -> Vec<VetoVote> {
    e.storage()
        .instance()
        .get(&(VETO_VOTES, reassignment_id))
        .unwrap_or(Vec::new(e))
}

/// Stores veto votes in instance storage, replacing the value for the same key.
/// Key inputs: `reassignment_id`, `votes`.
pub fn set_veto_votes(e: &Env, reassignment_id: u32, votes: &Vec<VetoVote>) {
    e.storage().instance().set(&(VETO_VOTES, reassignment_id), votes);
}

/// Adds veto vote to the corresponding record in contract storage.
/// Key inputs: `reassignment_id`, `vote`.
pub fn add_veto_vote(e: &Env, reassignment_id: u32, vote: &VetoVote) {
    let mut votes = get_veto_votes(e, reassignment_id);
    votes.push_back(vote.clone());
    set_veto_votes(e, reassignment_id, &votes);
}

/// Reads token supply info from instance storage.
pub fn get_token_supply_info(e: &Env) -> TokenSupplyInfo {
    e.storage()
        .instance()
        .get(&TOKEN_SUPPLY_INFO)
        .unwrap_or(TokenSupplyInfo {
            total_supply: 0,
            last_updated: 0,
        })
}

/// Stores token supply info in instance storage, replacing the value for the same key.
/// Key inputs: `supply_info`.
pub fn set_token_supply_info(e: &Env, supply_info: &TokenSupplyInfo) {
    e.storage().instance().set(&TOKEN_SUPPLY_INFO, supply_info);
}

/// Reads governance veto threshold from instance storage.
pub fn get_governance_veto_threshold(e: &Env) -> u32 {
    e.storage()
        .instance()
        .get(&GOVERNANCE_VETO_THRESHOLD)
        .unwrap_or(5) // Default 5% threshold
}

/// Stores governance veto threshold in instance storage, replacing the value for the same key.
/// Key inputs: `threshold`.
pub fn set_governance_veto_threshold(e: &Env, threshold: u32) {
    e.storage().instance().set(&GOVERNANCE_VETO_THRESHOLD, &threshold);
}

// 7 days in seconds for governance veto period
const GOVERNANCE_VETO_PERIOD: u64 = 604_800;

/// Returns the configured governance veto period in ledger seconds.
pub fn get_governance_veto_period() -> u64 {
    GOVERNANCE_VETO_PERIOD
}

// LST Deposit support
/// Reads lST config from instance storage.
/// Key inputs: `vesting_id`.
pub fn get_lst_config(e: &Env, vesting_id: u32) -> Option<LSTConfig> {
    e.storage().instance().get(&(LST_CONFIGS, vesting_id))
}

/// Stores lST config in instance storage, replacing the value for the same key.
/// Key inputs: `vesting_id`, `config`.
pub fn set_lst_config(e: &Env, vesting_id: u32, config: &LSTConfig) {
    e.storage().instance().set(&(LST_CONFIGS, vesting_id), config);
}

// --- Tax withholding backwards-compatible wrappers ---
/// Namespace key for tax withholding config records in instance storage.
pub const TAX_WITHHOLDING_CONFIG: &str = "TAX_WITHHOLDING_CONFIG";

/// Stores tax withholding config in instance storage, replacing the value for the same key.
/// Key inputs: `cfg`.
pub fn set_tax_withholding_config(e: &Env, cfg: &crate::types::TaxWithholdingConfig) {
    e.storage().instance().set(&TAX_WITHHOLDING_CONFIG, cfg);
}

/// Reads tax withholding config from instance storage.
pub fn get_tax_withholding_config(e: &Env) -> Option<crate::types::TaxWithholdingConfig> {
    e.storage().instance().get(&TAX_WITHHOLDING_CONFIG)
}

// --- SEP-12 oracle config storage ---
/// Namespace key for sEP-12 oracle records in instance storage.
pub const SEP12_ORACLE: &str = "SEP12_ORACLE";

/// Stores sEP-12 identity oracle in instance storage, replacing the value for the same key.
/// Key inputs: `oracle`.
pub fn set_sep12_identity_oracle(e: &Env, oracle: &crate::types::SEP12IdentityOracle) {
    e.storage().instance().set(&SEP12_ORACLE, oracle);
}

/// Reads sEP-12 identity oracle from instance storage.
pub fn get_sep12_identity_oracle(e: &Env) -> Option<crate::types::SEP12IdentityOracle> {
    e.storage().instance().get(&SEP12_ORACLE)
}

// --- Token metadata storage ---
/// Namespace key for token metadata records in instance storage.
/// Composite entries are additionally keyed by `asset`.
pub const TOKEN_METADATA: &str = "TOKEN_METADATA";

/// Stores token metadata in instance storage, replacing the value for the same key.
/// Key inputs: `asset`, `metadata`.
pub fn set_token_metadata(e: &Env, asset: &Address, metadata: &crate::types::TokenMetadata) {
    e.storage().instance().set(&(TOKEN_METADATA, asset.clone()), metadata);
}

/// Reads token metadata from instance storage.
/// Key inputs: `asset`.
pub fn get_token_metadata(e: &Env, asset: &Address) -> Option<crate::types::TokenMetadata> {
    e.storage().instance().get(&(TOKEN_METADATA, asset.clone()))
}

// --- Vesting grant storage ---
/// Namespace key for vesting grant records in instance storage.
/// Composite entries are additionally keyed by `vesting_id`.
pub const VESTING_GRANT: &str = "VESTING_GRANT"; // keyed by vesting_id

/// Stores vesting grant in instance storage, replacing the value for the same key.
/// Key inputs: `vesting_id`, `grant`.
pub fn set_vesting_grant(e: &Env, vesting_id: u32, grant: &crate::types::VestingGrant) {
    e.storage().instance().set(&(VESTING_GRANT, vesting_id), grant);
}

/// Reads vesting grant from instance storage.
/// Key inputs: `vesting_id`.
pub fn get_vesting_grant(e: &Env, vesting_id: u32) -> Option<crate::types::VestingGrant> {
    e.storage().instance().get(&(VESTING_GRANT, vesting_id))
}

// ========== ISSUE #223: Voting Power (Total Unvested Balance per address) ==========
/// Namespace key for total unvested balance records in instance storage.
/// Composite entries are additionally keyed by `address`.
pub const TOTAL_UNVESTED_BALANCE: &str = "TOTAL_UNVESTED_BALANCE";

/// Reads unvested balance from instance storage.
/// Key inputs: `address`.
pub fn get_unvested_balance(e: &Env, address: &Address) -> i128 {
    e.storage()
        .instance()
        .get(&(TOTAL_UNVESTED_BALANCE, address))
        .unwrap_or(0i128)
}

/// Stores unvested balance in instance storage, replacing the value for the same key.
/// Key inputs: `address`, `balance`.
pub fn set_unvested_balance(e: &Env, address: &Address, balance: i128) {
    e.storage().instance().set(&(TOTAL_UNVESTED_BALANCE, address), &balance);
}

// ========== ISSUE #226: Admin Dead-Man's Switch ==========
/// Namespace key for admin dead man switch records in instance storage.
/// Composite entries are additionally keyed by `switch`.
pub const ADMIN_DEAD_MAN_SWITCH: &str = "ADMIN_DEAD_MAN_SWITCH";

/// Reads admin dead man switch from instance storage.
pub fn get_admin_dead_man_switch(e: &Env) -> Option<crate::types::AdminDeadManSwitch> {
    e.storage().instance().get(&ADMIN_DEAD_MAN_SWITCH)
}

/// Stores admin dead man switch in instance storage, replacing the value for the same key.
/// Key inputs: `switch`.
pub fn set_admin_dead_man_switch(e: &Env, switch: &crate::types::AdminDeadManSwitch) {
    e.storage().instance().set(&ADMIN_DEAD_MAN_SWITCH, switch);
}

// ========== ISSUE #228: Oracle Price Deviation Circuit Breaker ==========
/// Namespace key for oracle price record records in instance storage.
pub const ORACLE_PRICE_RECORD: &str = "ORACLE_PRICE_RECORD";

/// Reads oracle price record from instance storage.
pub fn get_oracle_price_record(e: &Env) -> Option<crate::types::OraclePriceRecord> {
    e.storage().instance().get(&ORACLE_PRICE_RECORD)
}

/// Stores oracle price record in instance storage, replacing the value for the same key.
/// Key inputs: `record`.
pub fn set_oracle_price_record(e: &Env, record: &crate::types::OraclePriceRecord) {
    e.storage().instance().set(&ORACLE_PRICE_RECORD, record);
}

// ========== ISSUE #231: Total Unvested Balance (contract-wide) ==========
/// Namespace key for contract total unvested records in instance storage.
pub const CONTRACT_TOTAL_UNVESTED: &str = "CONTRACT_TOTAL_UNVESTED";

/// Reads contract total unvested from instance storage.
pub fn get_contract_total_unvested(e: &Env) -> i128 {
    e.storage()
        .instance()
        .get(&CONTRACT_TOTAL_UNVESTED)
        .unwrap_or(0i128)
}

/// Stores contract total unvested in instance storage, replacing the value for the same key.
/// Key inputs: `total`.
pub fn set_contract_total_unvested(e: &Env, total: i128) {
    e.storage().instance().set(&CONTRACT_TOTAL_UNVESTED, &total);
}

// ========== ISSUE #269: Zero-Knowledge Confidential Grant Amounts ==========
/// Namespace key for confidential grants records in instance storage.
/// Composite entries are additionally keyed by `vesting_id`.
pub const CONFIDENTIAL_GRANTS: &str = "CONFIDENTIAL_GRANTS";
/// Namespace key for master viewing key records in instance storage.
pub const MASTER_VIEWING_KEY: &str = "MASTER_VIEWING_KEY";
/// Namespace key for nullifier set records in persistent storage.
/// Composite entries are additionally keyed by `nullifier_hash`.
pub const NULLIFIER_SET: &str = "NULLIFIER_SET";

// ========== ISSUE #295: Temporary Storage for Claim-History Pagination ==========
/// Namespace key for pagination state records in contract storage.
pub const PAGINATION_STATE: &str = "PAGINATION_STATE";
/// Namespace key for claim history page size records in contract storage.
pub const CLAIM_HISTORY_PAGE_SIZE: u32 = 100;

// ========== ISSUE #296: Force-Withdrawal for Expired Schedules ==========
/// Namespace key for expired schedules records in contract storage.
pub const EXPIRED_SCHEDULES: &str = "EXPIRED_SCHEDULES";

// ========== ISSUE #297: Max-Allocation-Sanity-Check ==========
/// Namespace key for max allocation limit records in contract storage.
pub const MAX_ALLOCATION_LIMIT: &str = "MAX_ALLOCATION_LIMIT";
/// Namespace key for total allocated records in contract storage.
pub const TOTAL_ALLOCATED: &str = "TOTAL_ALLOCATED";

// Confidential grant storage functions
/// Reads confidential grant from instance storage.
/// Key inputs: `vesting_id`.
pub fn get_confidential_grant(e: &Env, vesting_id: u32) -> Option<ConfidentialGrant> {
    e.storage().instance().get(&(CONFIDENTIAL_GRANTS, vesting_id))
}

/// Stores confidential grant in instance storage, replacing the value for the same key.
/// Key inputs: `vesting_id`, `grant`.
pub fn set_confidential_grant(e: &Env, vesting_id: u32, grant: &ConfidentialGrant) {
    e.storage().instance().set(&(CONFIDENTIAL_GRANTS, vesting_id), grant);
}

/// Removes confidential grant from instance storage.
/// Key inputs: `vesting_id`.
pub fn remove_confidential_grant(e: &Env, vesting_id: u32) {
    e.storage().instance().remove(&(CONFIDENTIAL_GRANTS, vesting_id));
}

// Master viewing key storage functions
/// Reads master viewing key from instance storage.
pub fn get_master_viewing_key(e: &Env) -> Option<MasterViewingKey> {
    e.storage().instance().get(&MASTER_VIEWING_KEY)
}

/// Stores master viewing key in instance storage, replacing the value for the same key.
/// Key inputs: `key`.
pub fn set_master_viewing_key(e: &Env, key: &MasterViewingKey) {
    e.storage().instance().set(&MASTER_VIEWING_KEY, key);
}

/// Removes master viewing key from instance storage.
pub fn remove_master_viewing_key(e: &Env) {
    e.storage().instance().remove(&MASTER_VIEWING_KEY);
}

// Nullifier set in Persistent storage (for permanent tracking)
/// Returns whether nullifier in set is present or active in persistent storage.
/// Key inputs: `nullifier_hash`.
pub fn is_nullifier_in_set(e: &Env, nullifier_hash: &BytesN<32>) -> bool {
    e.storage()
        .persistent()
        .get(&(NULLIFIER_SET, nullifier_hash))
        .unwrap_or(false)
}

/// Adds nullifier to set to the corresponding record in persistent storage.
/// Key inputs: `nullifier_hash`.
pub fn add_nullifier_to_set(e: &Env, nullifier_hash: &BytesN<32>) {
    e.storage().persistent().set(&(NULLIFIER_SET, nullifier_hash), &true);
}

// Stream pause functions for suspicious activity detection
/// Reads stream pause from instance storage.
/// Key inputs: `vesting_id`, `beneficiary`.
pub fn get_stream_pause(e: &Env, vesting_id: u32, beneficiary: &Address) -> Option<StreamPause> {
    e.storage()
        .instance()
        .get(&(STREAM_PAUSES, vesting_id, beneficiary))
}

/// Stores stream pause in instance storage, replacing the value for the same key.
/// Key inputs: `vesting_id`, `beneficiary`, `pause`.
pub fn set_stream_pause(e: &Env, vesting_id: u32, beneficiary: &Address, pause: &StreamPause) {
    e.storage()
        .instance()
        .set(&(STREAM_PAUSES, vesting_id, beneficiary), pause);
}

/// Removes stream pause from instance storage.
/// Key inputs: `vesting_id`, `beneficiary`.
pub fn remove_stream_pause(e: &Env, vesting_id: u32, beneficiary: &Address) {
    e.storage()
        .instance()
        .remove(&(STREAM_PAUSES, vesting_id, beneficiary));
}

/// Returns whether stream paused is present or active in contract storage.
/// Key inputs: `vesting_id`, `beneficiary`.
pub fn is_stream_paused(e: &Env, vesting_id: u32, beneficiary: &Address) -> bool {
    if let Some(pause) = get_stream_pause(e, vesting_id, beneficiary) {
        return pause.is_active;
    }
    false
}

/// Reads stream pause history from instance storage.
pub fn get_stream_pause_history(e: &Env) -> Vec<StreamPause> {
    e.storage()
        .instance()
        .get(&STREAM_PAUSE_HISTORY)
        .unwrap_or(Vec::new(e))
}

/// Adds stream pause to history to the corresponding record in instance storage.
/// Key inputs: `pause`.
pub fn add_stream_pause_to_history(e: &Env, pause: &StreamPause) {
    let mut history = get_stream_pause_history(e);
    history.push_back(pause.clone());
    e.storage().instance().set(&STREAM_PAUSE_HISTORY, &history);
}

// ========== ISSUE #276: Vesting Schedule Consolidation Storage ==========

/// Get master schedule by ID
pub fn get_master_schedule(e: &Env, master_id: u32) -> Option<MasterSchedule> {
    e.storage().instance().get(&(crate::types::MASTER_SCHEDULES, master_id))
}

/// Set master schedule
pub fn set_master_schedule(e: &Env, master_id: u32, schedule: &MasterSchedule) {
    e.storage().instance().set(&(crate::types::MASTER_SCHEDULES, master_id), schedule);
}

/// Remove master schedule
pub fn remove_master_schedule(e: &Env, master_id: u32) {
    e.storage().instance().remove(&(crate::types::MASTER_SCHEDULES, master_id));
}

/// Check if a schedule has been merged
pub fn is_schedule_merged(e: &Env, schedule_id: u32) -> bool {
    e.storage()
        .instance()
        .get(&(crate::types::MERGED_SCHEDULES, schedule_id))
        .unwrap_or(false)
}

/// Mark a schedule as merged
pub fn mark_schedule_merged(e: &Env, schedule_id: u32) {
    e.storage().instance().set(&(crate::types::MERGED_SCHEDULES, schedule_id), &true);
}

/// Get next available master schedule ID
pub fn get_next_master_schedule_id(e: &Env) -> u32 {
    let current_id = e
        .storage()
        .instance()
        .get(&("MASTER_SCHEDULE_COUNTER"))
        .unwrap_or(0u32);
    let next_id = current_id + 1;
    e.storage().instance().set(&("MASTER_SCHEDULE_COUNTER"), &next_id);
    next_id
}
