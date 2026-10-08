use soroban_sdk::{contracttype, Env, Vec, Address, Map, BytesN};
use crate::types::{ClaimEvent, AuthorizedPayoutAddress, AddressWhitelistRequest, Nullifier, Commitment, PathPaymentConfig, PathPaymentClaimEvent, LockupConfig, BeneficiaryReassignment, VetoVote, TokenSupplyInfo, LSTConfig, ConfidentialGrant, MasterViewingKey, StreamPause, MasterSchedule};


// Stream pause storage keys for suspicious activity detection

// Emergency pause storage keys

// Cross-project reputation storage keys

// Milestone vesting storage keys

// Zero-Knowledge Privacy Claims storage keys

// Stellar Horizon Path Payment Claim storage keys

// Lock-up period storage keys

// Beneficiary reassignment and governance veto storage keys

// LST Auto-Compounding storage keys (Issue #154)

// Protocol Sunset and Migration storage keys (Issue #280)

/// Packed storage keys for the vesting vault contract.
///
/// Replaces the previous `&str` key constants: every ledger entry key now
/// encodes as a compact `u32` discriminant plus its typed parameters instead
/// of a full-length string, shrinking Soroban ledger entry size and the
/// associated read/write storage fees.
#[contracttype]
#[derive(Clone, Debug)]
pub enum StorageKey {
    ClaimHistory,
    AuthorizedPayoutAddress(Address),
    PendingAddressRequest(Address),
    StreamPauses(u32, Address),
    StreamPauseHistory,
    Auditors,
    AuditorPauseRequests,
    EmergencyPause,
    ReputationBridgeContract,
    ReputationBonusApplied(Address),
    MilestoneConfigs(u32),
    MilestoneStatus(u32),
    NullifierMap(Nullifier),
    CommitmentStorage(BytesN<32>),
    PrivacyClaimHistory,
    MerkleRoots,
    PathPaymentConfig,
    PathPaymentClaimHistory,
    LockupConfigs(u32),
    BeneficiaryReassignments(u32),
    VetoVotes(u32),
    TokenSupplyInfo,
    ReassignmentCounter,
    GovernanceVetoThreshold,
    LstConfigs(u32),
    LstPoolShares,
    UserLstShares,
    UnbondingRequests,
    UnbondingQueue,
    ProtocolSunset,
    MigrationPayloads,
    RelayerMigrations,
    TaxConfigs(u32),
    CumulativeTaxes(u32, Address),
    TaxWithholdingConfig,
    Sep12Oracle,
    TokenMetadata(Address),
    VestingGrant(u32),
    TotalUnvestedBalance(Address),
    AdminDeadManSwitch,
    OraclePriceRecord,
    ContractTotalUnvested,
    ConfidentialGrants(u32),
    MasterViewingKey,
    NullifierSet(BytesN<32>),
    PaginationState,
    ExpiredSchedules,
    MaxAllocationLimit,
    TotalAllocated,
    MasterSchedules(u32),
    MergedSchedules(u32),
    MasterScheduleCounter,
}

// 48 hours in seconds
const TIMELOCK_DURATION: u64 = 172_800;

// 7 days in seconds for emergency pause
const EMERGENCY_PAUSE_DURATION: u64 = 604_800;

pub fn get_claim_history(e: &Env) -> Vec<ClaimEvent> {
    e.storage()
        .instance()
        .get(&StorageKey::ClaimHistory)
        .unwrap_or(Vec::new(e))
}

pub fn set_claim_history(e: &Env, history: &Vec<ClaimEvent>) {
    e.storage().instance().set(&StorageKey::ClaimHistory, history);
}

pub fn get_authorized_payout_address(e: &Env, beneficiary: &Address) -> Option<AuthorizedPayoutAddress> {
    e.storage()
        .instance()
        .get(&StorageKey::AuthorizedPayoutAddress(beneficiary.clone()))
}

pub fn set_authorized_payout_address(e: &Env, beneficiary: &Address, auth_address: &AuthorizedPayoutAddress) {
    e.storage().instance().set(&StorageKey::AuthorizedPayoutAddress(beneficiary.clone()), auth_address);
}

pub fn get_pending_address_request(e: &Env, beneficiary: &Address) -> Option<AddressWhitelistRequest> {
    e.storage()
        .instance()
        .get(& StorageKey::PendingAddressRequest(beneficiary.clone()))
}

pub fn set_pending_address_request(e: &Env, beneficiary: &Address, request: &AddressWhitelistRequest) {
    e.storage().instance().set(&StorageKey::PendingAddressRequest(beneficiary.clone()), request);
}

pub fn remove_pending_address_request(e: &Env, beneficiary: &Address) {
    e.storage().instance().remove(&StorageKey::PendingAddressRequest(beneficiary.clone()));
}

pub fn get_timelock_duration() -> u64 {
    TIMELOCK_DURATION
}

// Emergency pause functions
pub fn get_auditors(e: &Env) -> Vec<Address> {
    e.storage()
        .instance()
        .get(&StorageKey::Auditors)
        .unwrap_or(Vec::new(e))
}

pub fn set_auditors(e: &Env, auditors: &Vec<Address>) {
    e.storage().instance().set(&StorageKey::Auditors, auditors);
}

pub fn get_auditor_pause_requests(e: &Env) -> Map<Address, crate::emergency::AuditorPauseRequest> {
    e.storage()
        .instance()
        .get(&StorageKey::AuditorPauseRequests)
        .unwrap_or(Map::new(e))
}

pub fn set_auditor_pause_requests(e: &Env, requests: &Map<Address, crate::emergency::AuditorPauseRequest>) {
    e.storage().instance().set(&StorageKey::AuditorPauseRequests, requests);
}

pub fn get_emergency_pause(e: &Env) -> Option<crate::emergency::EmergencyPause> {
    e.storage().instance().get(&StorageKey::EmergencyPause)
}

pub fn set_emergency_pause(e: &Env, pause: &crate::emergency::EmergencyPause) {
    e.storage().instance().set(&StorageKey::EmergencyPause, pause);
}

pub fn remove_emergency_pause(e: &Env) {
    e.storage().instance().remove(&StorageKey::EmergencyPause);
}

// Cross-project reputation functions
pub fn get_reputation_bridge_contract(e: &Env) -> Option<Address> {
    e.storage().instance().get(&StorageKey::ReputationBridgeContract)
}

pub fn set_reputation_bridge_contract(e: &Env, contract_address: &Address) {
    e.storage().instance().set(&StorageKey::ReputationBridgeContract, contract_address);
}

pub fn has_reputation_bonus_applied(e: &Env, beneficiary: &Address) -> bool {
    e.storage()
        .instance()
        .get(&StorageKey::ReputationBonusApplied(beneficiary.clone()))
        .unwrap_or(false)
}

pub fn set_reputation_bonus_applied(e: &Env, beneficiary: &Address) {
    e.storage().instance().set(&StorageKey::ReputationBonusApplied(beneficiary.clone()), &true);
}

// Milestone vesting functions
pub fn get_milestone_configs(e: &Env, vesting_id: u32) -> Option<Vec<u32>> {
    e.storage().instance().get(&StorageKey::MilestoneConfigs(vesting_id))
}

pub fn set_milestone_configs(e: &Env, vesting_id: u32, milestones: &Vec<u32>) {
    e.storage().instance().set(&StorageKey::MilestoneConfigs(vesting_id), milestones);
}

pub fn get_milestone_status(e: &Env, vesting_id: u32) -> Map<u32, bool> {
    e.storage()
        .instance()
        .get(&StorageKey::MilestoneStatus(vesting_id))
        .unwrap_or(Map::new(e))
}

pub fn set_milestone_status(e: &Env, vesting_id: u32, status: &Map<u32, bool>) {
    e.storage().instance().set(&StorageKey::MilestoneStatus(vesting_id), status);
}

pub fn get_emergency_pause_duration() -> u64 {
    EMERGENCY_PAUSE_DURATION
}

// Zero-Knowledge Privacy Claims functions

// Nullifier map functions - prevent double-spending in private claims
pub fn is_nullifier_used(e: &Env, nullifier: &Nullifier) -> bool {
    e.storage()
        .instance()
        .get(&StorageKey::NullifierMap(nullifier.clone()))
        .unwrap_or(false)
}

pub fn set_nullifier_used(e: &Env, nullifier: &Nullifier) {
    e.storage().instance().set(&StorageKey::NullifierMap(nullifier.clone()), &true);
}

// Commitment storage functions
pub fn get_commitment(e: &Env, commitment_hash: &BytesN<32>) -> Option<Commitment> {
    e.storage().instance().get(&StorageKey::CommitmentStorage(commitment_hash.clone()))
}

pub fn set_commitment(e: &Env, commitment_hash: &BytesN<32>, commitment: &Commitment) {
    e.storage().instance().set(&StorageKey::CommitmentStorage(commitment_hash.clone()), commitment);
}

pub fn mark_commitment_used(e: &Env, commitment_hash: &BytesN<32>) {
    if let Some(mut commitment) = get_commitment(e, commitment_hash) {
        commitment.is_used = true;
        set_commitment(e, commitment_hash, &commitment);
    }
}

// Privacy claim history functions
pub fn get_privacy_claim_history(e: &Env) -> Vec<crate::types::PrivacyClaimEvent> {
    e.storage()
        .instance()
        .get(&StorageKey::PrivacyClaimHistory)
        .unwrap_or(Vec::new(e))
}

pub fn add_privacy_claim_event(e: &Env, event: &crate::types::PrivacyClaimEvent) {
    let mut history = get_privacy_claim_history(e);
    history.push_back(event.clone());
    e.storage().instance().set(&StorageKey::PrivacyClaimHistory, &history);
}

// Merkle root management for ZK proofs
pub fn add_merkle_root(e: &Env, merkle_root: &BytesN<32>) {
    let mut roots = get_merkle_roots(e);
    roots.push_back(merkle_root.clone());
    e.storage().instance().set(&StorageKey::MerkleRoots, &roots);
}

pub fn get_merkle_roots(e: &Env) -> Vec<BytesN<32>> {
    e.storage()
        .instance()
        .get(&StorageKey::MerkleRoots)
        .unwrap_or(Vec::new(e))
}

pub fn is_valid_merkle_root(e: &Env, merkle_root: &BytesN<32>) -> bool {
    let roots = get_merkle_roots(e);
    roots.contains(merkle_root.clone())
}

// Stellar Horizon Path Payment Claim storage functions
pub fn get_path_payment_config(e: &Env) -> Option<PathPaymentConfig> {
    e.storage().instance().get(&StorageKey::PathPaymentConfig)
}

pub fn set_path_payment_config(e: &Env, config: &PathPaymentConfig) {
    e.storage().instance().set(&StorageKey::PathPaymentConfig, config);
}

pub fn get_path_payment_claim_history(e: &Env) -> Vec<PathPaymentClaimEvent> {
    e.storage()
        .instance()
        .get(&StorageKey::PathPaymentClaimHistory)
        .unwrap_or(Vec::new(e))
}

pub fn add_path_payment_claim_event(e: &Env, event: &PathPaymentClaimEvent) {
    let mut history = get_path_payment_claim_history(e);
    history.push_back(event.clone());
    e.storage().instance().set(&StorageKey::PathPaymentClaimHistory, &history);
}

// Lock-up period storage functions
pub fn get_lockup_config(e: &Env, vesting_id: u32) -> Option<LockupConfig> {
    e.storage().instance().get(&StorageKey::LockupConfigs(vesting_id))
}

pub fn set_lockup_config(e: &Env, vesting_id: u32, config: &LockupConfig) {
    e.storage().instance().set(&StorageKey::LockupConfigs(vesting_id), config);
}

pub fn remove_lockup_config(e: &Env, vesting_id: u32) {
    e.storage().instance().remove(&StorageKey::LockupConfigs(vesting_id));
}

// ========== TAX CONFIGURATION STORAGE ==========

pub fn get_tax_config(e: &Env, vesting_id: u32) -> Option<crate::types::TaxConfig> {
    e.storage().instance().get(&StorageKey::TaxConfigs(vesting_id))
}

pub fn set_tax_config(e: &Env, vesting_id: u32, cfg: &crate::types::TaxConfig) {
    e.storage().instance().set(&StorageKey::TaxConfigs(vesting_id), cfg);
}

pub fn get_cumulative_taxes(e: &Env, vesting_id: u32, authority: &Address) -> i128 {
    e.storage()
        .instance()
        .get(&StorageKey::CumulativeTaxes(vesting_id, authority.clone()))
        .unwrap_or(0i128)
}

pub fn add_cumulative_taxes(e: &Env, vesting_id: u32, authority: &Address, amount: i128) {
    let prev = get_cumulative_taxes(e, vesting_id, authority);
    e.storage().instance().set(&StorageKey::CumulativeTaxes(vesting_id, authority.clone()), &(prev + amount));
}

// Beneficiary reassignment and governance veto storage functions
pub fn get_reassignment_counter(e: &Env) -> u32 {
    e.storage()
        .instance()
        .get(&StorageKey::ReassignmentCounter)
        .unwrap_or(0)
}

pub fn set_reassignment_counter(e: &Env, counter: u32) {
    e.storage().instance().set(&StorageKey::ReassignmentCounter, &counter);
}

pub fn get_beneficiary_reassignment(e: &Env, reassignment_id: u32) -> Option<BeneficiaryReassignment> {
    e.storage().instance().get(&StorageKey::BeneficiaryReassignments(reassignment_id))
}

pub fn set_beneficiary_reassignment(e: &Env, reassignment_id: u32, reassignment: &BeneficiaryReassignment) {
    e.storage().instance().set(&StorageKey::BeneficiaryReassignments(reassignment_id), reassignment);
}

pub fn remove_beneficiary_reassignment(e: &Env, reassignment_id: u32) {
    e.storage().instance().remove(&StorageKey::BeneficiaryReassignments(reassignment_id));
}

pub fn get_veto_votes(e: &Env, reassignment_id: u32) -> Vec<VetoVote> {
    e.storage()
        .instance()
        .get(&StorageKey::VetoVotes(reassignment_id))
        .unwrap_or(Vec::new(e))
}

pub fn set_veto_votes(e: &Env, reassignment_id: u32, votes: &Vec<VetoVote>) {
    e.storage().instance().set(&StorageKey::VetoVotes(reassignment_id), votes);
}

pub fn add_veto_vote(e: &Env, reassignment_id: u32, vote: &VetoVote) {
    let mut votes = get_veto_votes(e, reassignment_id);
    votes.push_back(vote.clone());
    set_veto_votes(e, reassignment_id, &votes);
}

pub fn get_token_supply_info(e: &Env) -> TokenSupplyInfo {
    e.storage()
        .instance()
        .get(&StorageKey::TokenSupplyInfo)
        .unwrap_or(TokenSupplyInfo {
            total_supply: 0,
            last_updated: 0,
        })
}

pub fn set_token_supply_info(e: &Env, supply_info: &TokenSupplyInfo) {
    e.storage().instance().set(&StorageKey::TokenSupplyInfo, supply_info);
}

pub fn get_governance_veto_threshold(e: &Env) -> u32 {
    e.storage()
        .instance()
        .get(&StorageKey::GovernanceVetoThreshold)
        .unwrap_or(5) // Default 5% threshold
}

pub fn set_governance_veto_threshold(e: &Env, threshold: u32) {
    e.storage().instance().set(&StorageKey::GovernanceVetoThreshold, &threshold);
}

// 7 days in seconds for governance veto period
const GOVERNANCE_VETO_PERIOD: u64 = 604_800;

pub fn get_governance_veto_period() -> u64 {
    GOVERNANCE_VETO_PERIOD
}

// LST Deposit support
pub fn get_lst_config(e: &Env, vesting_id: u32) -> Option<LSTConfig> {
    e.storage().instance().get(&StorageKey::LstConfigs(vesting_id))
}

pub fn set_lst_config(e: &Env, vesting_id: u32, config: &LSTConfig) {
    e.storage().instance().set(&StorageKey::LstConfigs(vesting_id), config);
}

// --- Tax withholding backwards-compatible wrappers ---

pub fn set_tax_withholding_config(e: &Env, cfg: &crate::types::TaxWithholdingConfig) {
    e.storage().instance().set(&StorageKey::TaxWithholdingConfig, cfg);
}

pub fn get_tax_withholding_config(e: &Env) -> Option<crate::types::TaxWithholdingConfig> {
    e.storage().instance().get(&StorageKey::TaxWithholdingConfig)
}

// --- SEP-12 oracle config storage ---

pub fn set_sep12_identity_oracle(e: &Env, oracle: &crate::types::SEP12IdentityOracle) {
    e.storage().instance().set(&StorageKey::Sep12Oracle, oracle);
}

pub fn get_sep12_identity_oracle(e: &Env) -> Option<crate::types::SEP12IdentityOracle> {
    e.storage().instance().get(&StorageKey::Sep12Oracle)
}

// --- Token metadata storage ---

pub fn set_token_metadata(e: &Env, asset: &Address, metadata: &crate::types::TokenMetadata) {
    e.storage().instance().set(&StorageKey::TokenMetadata(asset.clone()), metadata);
}

pub fn get_token_metadata(e: &Env, asset: &Address) -> Option<crate::types::TokenMetadata> {
    e.storage().instance().get(&StorageKey::TokenMetadata(asset.clone()))
}

// --- Vesting grant storage ---

pub fn set_vesting_grant(e: &Env, vesting_id: u32, grant: &crate::types::VestingGrant) {
    e.storage().instance().set(&StorageKey::VestingGrant(vesting_id), grant);
}

pub fn get_vesting_grant(e: &Env, vesting_id: u32) -> Option<crate::types::VestingGrant> {
    e.storage().instance().get(&StorageKey::VestingGrant(vesting_id))
}

// ========== ISSUE #223: Voting Power (Total Unvested Balance per address) ==========

pub fn get_unvested_balance(e: &Env, address: &Address) -> i128 {
    e.storage()
        .instance()
        .get(&StorageKey::TotalUnvestedBalance(address.clone()))
        .unwrap_or(0i128)
}

pub fn set_unvested_balance(e: &Env, address: &Address, balance: i128) {
    e.storage().instance().set(&StorageKey::TotalUnvestedBalance(address.clone()), &balance);
}

// ========== ISSUE #226: Admin Dead-Man's Switch ==========

pub fn get_admin_dead_man_switch(e: &Env) -> Option<crate::types::AdminDeadManSwitch> {
    e.storage().instance().get(&StorageKey::AdminDeadManSwitch)
}

pub fn set_admin_dead_man_switch(e: &Env, switch: &crate::types::AdminDeadManSwitch) {
    e.storage().instance().set(&StorageKey::AdminDeadManSwitch, switch);
}

// ========== ISSUE #228: Oracle Price Deviation Circuit Breaker ==========

pub fn get_oracle_price_record(e: &Env) -> Option<crate::types::OraclePriceRecord> {
    e.storage().instance().get(&StorageKey::OraclePriceRecord)
}

pub fn set_oracle_price_record(e: &Env, record: &crate::types::OraclePriceRecord) {
    e.storage().instance().set(&StorageKey::OraclePriceRecord, record);
}

// ========== ISSUE #231: Total Unvested Balance (contract-wide) ==========

pub fn get_contract_total_unvested(e: &Env) -> i128 {
    e.storage()
        .instance()
        .get(&StorageKey::ContractTotalUnvested)
        .unwrap_or(0i128)
}

pub fn set_contract_total_unvested(e: &Env, total: i128) {
    e.storage().instance().set(&StorageKey::ContractTotalUnvested, &total);
}

// ========== ISSUE #269: Zero-Knowledge Confidential Grant Amounts ==========

// ========== ISSUE #295: Temporary Storage for Claim-History Pagination ==========
pub const CLAIM_HISTORY_PAGE_SIZE: u32 = 100;

// ========== ISSUE #296: Force-Withdrawal for Expired Schedules ==========

// ========== ISSUE #297: Max-Allocation-Sanity-Check ==========

// Confidential grant storage functions
pub fn get_confidential_grant(e: &Env, vesting_id: u32) -> Option<ConfidentialGrant> {
    e.storage().instance().get(&StorageKey::ConfidentialGrants(vesting_id))
}

pub fn set_confidential_grant(e: &Env, vesting_id: u32, grant: &ConfidentialGrant) {
    e.storage().instance().set(&StorageKey::ConfidentialGrants(vesting_id), grant);
}

pub fn remove_confidential_grant(e: &Env, vesting_id: u32) {
    e.storage().instance().remove(&StorageKey::ConfidentialGrants(vesting_id));
}

// Master viewing key storage functions
pub fn get_master_viewing_key(e: &Env) -> Option<MasterViewingKey> {
    e.storage().instance().get(&StorageKey::MasterViewingKey)
}

pub fn set_master_viewing_key(e: &Env, key: &MasterViewingKey) {
    e.storage().instance().set(&StorageKey::MasterViewingKey, key);
}

pub fn remove_master_viewing_key(e: &Env) {
    e.storage().instance().remove(&StorageKey::MasterViewingKey);
}

// Nullifier set in Persistent storage (for permanent tracking)
pub fn is_nullifier_in_set(e: &Env, nullifier_hash: &BytesN<32>) -> bool {
    e.storage()
        .persistent()
        .get(&StorageKey::NullifierSet(nullifier_hash.clone()))
        .unwrap_or(false)
}

pub fn add_nullifier_to_set(e: &Env, nullifier_hash: &BytesN<32>) {
    e.storage().persistent().set(&StorageKey::NullifierSet(nullifier_hash.clone()), &true);
}

// Stream pause functions for suspicious activity detection
pub fn get_stream_pause(e: &Env, vesting_id: u32, beneficiary: &Address) -> Option<StreamPause> {
    e.storage()
        .instance()
        .get(&StorageKey::StreamPauses(vesting_id, beneficiary.clone()))
}

pub fn set_stream_pause(e: &Env, vesting_id: u32, beneficiary: &Address, pause: &StreamPause) {
    e.storage()
        .instance()
        .set(&StorageKey::StreamPauses(vesting_id, beneficiary.clone()), pause);
}

pub fn remove_stream_pause(e: &Env, vesting_id: u32, beneficiary: &Address) {
    e.storage()
        .instance()
        .remove(&StorageKey::StreamPauses(vesting_id, beneficiary.clone()));
}

pub fn is_stream_paused(e: &Env, vesting_id: u32, beneficiary: &Address) -> bool {
    if let Some(pause) = get_stream_pause(e, vesting_id, beneficiary) {
        return pause.is_active;
    }
    false
}

pub fn get_stream_pause_history(e: &Env) -> Vec<StreamPause> {
    e.storage()
        .instance()
        .get(&StorageKey::StreamPauseHistory)
        .unwrap_or(Vec::new(e))
}

pub fn add_stream_pause_to_history(e: &Env, pause: &StreamPause) {
    let mut history = get_stream_pause_history(e);
    history.push_back(pause.clone());
    e.storage().instance().set(&StorageKey::StreamPauseHistory, &history);
}

// ========== ISSUE #276: Vesting Schedule Consolidation Storage ==========

/// Get master schedule by ID
pub fn get_master_schedule(e: &Env, master_id: u32) -> Option<MasterSchedule> {
    e.storage().instance().get(&StorageKey::MasterSchedules(master_id))
}

/// Set master schedule
pub fn set_master_schedule(e: &Env, master_id: u32, schedule: &MasterSchedule) {
    e.storage().instance().set(&StorageKey::MasterSchedules(master_id), schedule);
}

/// Remove master schedule
pub fn remove_master_schedule(e: &Env, master_id: u32) {
    e.storage().instance().remove(&StorageKey::MasterSchedules(master_id));
}

/// Check if a schedule has been merged
pub fn is_schedule_merged(e: &Env, schedule_id: u32) -> bool {
    e.storage()
        .instance()
        .get(&StorageKey::MergedSchedules(schedule_id))
        .unwrap_or(false)
}

/// Mark a schedule as merged
pub fn mark_schedule_merged(e: &Env, schedule_id: u32) {
    e.storage().instance().set(&StorageKey::MergedSchedules(schedule_id), &true);
}

/// Get next available master schedule ID
pub fn get_next_master_schedule_id(e: &Env) -> u32 {
    let current_id = e
        .storage()
        .instance()
        .get(&StorageKey::MasterScheduleCounter)
        .unwrap_or(0u32);
    let next_id = current_id + 1;
    e.storage().instance().set(&StorageKey::MasterScheduleCounter, &next_id);
    next_id
}

#[cfg(test)]
mod packed_key_tests {
    extern crate std;
    use std::eprintln;

    use super::StorageKey;
    use crate::types::Nullifier;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::xdr::{Limits, ScVal, WriteXdr};
    use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, IntoVal, String, TryFromVal};

    #[contract]
    struct KeyTestContract;

    #[contractimpl]
    impl KeyTestContract {}

    fn xdr_len(e: &Env, v: &soroban_sdk::Val) -> usize {
        ScVal::try_from_val(e, v)
            .unwrap()
            .to_xdr(Limits::none())
            .unwrap()
            .len()
    }

    /// Measure and report the encoded size delta for unit variants. The
    /// enum wraps its discriminant in a Vec frame, so bare keys trade a few
    /// bytes of bounded overhead for typed key safety; the byte savings
    /// come from the parameterized variants below.
    #[test]
    fn unit_key_size_is_bounded() {
        let e = Env::default();
        let legacy = String::from_str(&e, "CLAIM_HISTORY").into_val(&e);
        let packed = StorageKey::ClaimHistory.into_val(&e);
        let (old_len, new_len) = (xdr_len(&e, &legacy), xdr_len(&e, &packed));
        eprintln!("CLAIM_HISTORY key XDR bytes: legacy={old_len} packed={new_len}");
        assert!(new_len <= old_len + 16);
    }

    #[test]
    fn packed_tuple_key_is_smaller_than_legacy_tuple() {
        let e = Env::default();
        let beneficiary = Address::generate(&e);
        // Legacy encoding: ("AUTHORIZED_PAYOUT_ADDRESS", beneficiary)
        let legacy = (
            String::from_str(&e, "AUTHORIZED_PAYOUT_ADDRESS"),
            beneficiary.clone(),
        )
            .into_val(&e);
        let packed = StorageKey::AuthorizedPayoutAddress(beneficiary).into_val(&e);
        let (old_len, new_len) = (xdr_len(&e, &legacy), xdr_len(&e, &packed));
        eprintln!("AUTHORIZED_PAYOUT_ADDRESS key XDR bytes: legacy={old_len} packed={new_len}");
        assert!(new_len < old_len);

        // A long-named keyed variant shows the larger win.
        let vesting_id = 7u32;
        let legacy2 = (String::from_str(&e, "BENEFICIARY_REASSIGNMENTS"), vesting_id).into_val(&e);
        let packed2 = StorageKey::BeneficiaryReassignments(vesting_id).into_val(&e);
        let (o2, n2) = (xdr_len(&e, &legacy2), xdr_len(&e, &packed2));
        eprintln!("BENEFICIARY_REASSIGNMENTS key XDR bytes: legacy={o2} packed={n2}");
        assert!(n2 < o2);
    }

    /// Keyed variants must round-trip through storage unchanged.
    #[test]
    fn keyed_variants_round_trip() {
        let e = Env::default();
        let id = e.register(KeyTestContract, ());
        e.as_contract(&id, || {
            let key = StorageKey::StreamPauses(7, Address::generate(&e));
            e.storage().instance().set(&key, &true);
            assert!(e.storage().instance().has(&key));

            let nkey = StorageKey::NullifierMap(Nullifier {
                hash: BytesN::from_array(&e, &[9u8; 32]),
            });
            e.storage().instance().set(&nkey, &true);
            assert!(e.storage().instance().has(&nkey));
        });
    }
}
