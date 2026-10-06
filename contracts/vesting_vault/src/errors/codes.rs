use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
/// Stable error codes returned by vesting-vault entry points and validation helpers.
pub enum Error {
    // 🔐 General (100s)
    /// Indicates unauthorized.
    Unauthorized = 100,
    /// Indicates invalid input.
    InvalidInput = 101,
    /// #13: Address is the zero address -- not allowed as a participant
    ZeroAddress = 102,
    /// #13: Duplicate addresses found where uniqueness is required
    DuplicateAuditor = 103,
    /// #13: Amount must be strictly positive (> 0)
    AmountMustBePositive = 104,
    /// #13: Vesting schedule ID must be strictly positive (> 0)
    InvalidVestingId = 105,
    /// #13: Timestamp exceeds the maximum reasonable date bound
    TimestampTooLarge = 106,

    // ⏳ Vesting (200s)
    /// Indicates vesting not found.
    VestingNotFound = 200,
    /// Indicates vault not found.
    VaultNotFound = 201,
    /// Indicates already initialized.
    AlreadyInitialized = 202,
    /// Indicates not initialized.
    NotInitialized = 203,
    /// Indicates contract paused.
    ContractPaused = 204,
    /// Indicates cliff not reached.
    CliffNotReached = 205,
    /// Indicates nothing to claim.
    NothingToClaim = 206,
    /// Indicates already fully claimed.
    AlreadyFullyClaimed = 207,
    /// Indicates vault revoked.
    VaultRevoked = 208,
    /// Indicates vault frozen.
    VaultFrozen = 209,
    /// Indicates invalid schedule.
    InvalidSchedule = 210,
    /// Indicates milestone not completed.
    MilestoneNotCompleted = 211,
    /// Indicates invalid amount.
    InvalidAmount = 212,
    /// Indicates vault not initialized.
    VaultNotInitialized = 213,

    // 💰 Financial (300s)
    /// Indicates insufficient balance.
    InsufficientBalance = 300,
    /// Indicates insufficient funds.
    InsufficientFunds = 301,
    /// Indicates transfer failed.
    TransferFailed = 302,

    // 🧪 LST Auto-Compounding (310s)
    /// #154: LST not configured for this vesting schedule
    LSTNotConfigured = 310,
    /// #154: LST auto-compounding not enabled
    LSTNotEnabled = 311,
    /// #154: LST pool shares not initialized
    LSTPoolNotInitialized = 312,
    /// #154: User has no shares in the LST pool
    NoUserShares = 313,
    /// #154: No shares to unbond
    NoSharesToUnbond = 314,
    /// #154: Unbonding already pending for this user
    UnbondingAlreadyPending = 315,
    /// #154: Unbonding queue is full (rate limit)
    UnbondingQueueFull = 316,
    /// #154: Unbonding period has not elapsed yet
    UnbondingPeriodNotElapsed = 317,
    /// #154: No unbonding request found
    NoUnbondingRequest = 318,
    /// #154: Exchange rate manipulation suspected
    ExchangeRateManipulationSuspected = 319,

    // 📜 Compliance (400s)
    /// Indicates kyc not completed.
    KycNotCompleted = 400,
    /// Indicates kyc expired.
    KycExpired = 401,
    /// Indicates address sanctioned.
    AddressSanctioned = 402,
    /// Indicates jurisdiction restricted.
    JurisdictionRestricted = 403,
    /// Indicates legal signature missing.
    LegalSignatureMissing = 404,
    /// Indicates legal signature invalid.
    LegalSignatureInvalid = 405,
    /// Indicates compliance check failed.
    ComplianceCheckFailed = 406,
    /// Indicates aml threshold exceeded.
    AmlThresholdExceeded = 407,
    /// Indicates risk rating too high.
    RiskRatingTooHigh = 408,
    /// Indicates document verification failed.
    DocumentVerificationFailed = 409,
    /// Indicates accreditation status invalid.
    AccreditationStatusInvalid = 410,
    /// Indicates tax compliance failed.
    TaxComplianceFailed = 411,
    /// Tax liquidation (swap/transfer) failed during claim processing
    TaxLiquidationFailed = 421,
    /// Indicates regulatory block active.
    RegulatoryBlockActive = 412,
    /// Indicates whitelist not approved.
    WhitelistNotApproved = 413,
    /// Indicates blacklist violation.
    BlacklistViolation = 414,
    /// Indicates geofencing restriction.
    GeofencingRestriction = 415,
    /// Indicates identity verification expired.
    IdentityVerificationExpired = 416,
    /// Indicates source of funds not verified.
    SourceOfFundsNotVerified = 417,
    /// Indicates beneficial owner not verified.
    BeneficialOwnerNotVerified = 418,
    /// Indicates politically exposed person.
    PoliticallyExposedPerson = 419,
    /// Indicates sanctions list hit.
    SanctionsListHit = 420,

    // 🚨 Security (450s)
    /// Stream paused due to suspicious activity detection
    StreamPaused = 450,

    // ⚙️ System (900s)
    /// Indicates overflow.
    Overflow = 900,
    /// #9: Integer arithmetic overflow detected in financial computation
    ArithmeticOverflow = 901,
    /// #9: Integer arithmetic underflow detected in financial computation
    ArithmeticUnderflow = 902,

    // 🗳️ Governance / DAO (500s)
    /// #223: No unvested balance found for the queried address
    NoUnvestedBalance = 500,
    /// Indicates already voted.
    AlreadyVoted = 501,
    /// Indicates voting period ended.
    VotingPeriodEnded = 502,
    /// Indicates quorum not met.
    QuorumNotMet = 503,
    /// Timelock period has not yet elapsed
    TimelockNotElapsed = 504,
    /// Indicates path payment not configured.
    PathPaymentNotConfigured = 505,
    /// Indicates path payment disabled.
    PathPaymentDisabled = 506,
    /// Indicates insufficient liquidity.
    InsufficientLiquidity = 507,

    // 🔑 Admin Recovery (600s)
    /// #226: Admin dead-man's switch not configured
    AdminSwitchNotConfigured = 600,
    /// #226: Admin inactivity timeout has not elapsed yet
    AdminInactivityNotElapsed = 601,
    /// #226: Admin switch already triggered
    AdminSwitchAlreadyTriggered = 602,
    /// #226: Recovery address cannot be the same as admin
    RecoveryAddressInvalid = 603,

    // 🔮 Oracle (700s)
    /// #228: Oracle circuit breaker is currently tripped — vault is frozen
    OracleCircuitBreakerActive = 700,
    /// #228: Price deviation exceeds the 30% threshold
    OraclePriceDeviationTooHigh = 701,

    // 🛡️ Self-Destruct Prevention (800s)
    /// #231: Cannot upgrade/delete contract while unvested balance > 0
    UpgradeBlockedByUnvestedFunds = 800,

    // 🔐 Zero-Knowledge Privacy (1000s)
    /// #269: ZK proof verification failed
    InvalidZKProof = 1000,
    /// #269: Attempted to claim more than the shielded amount
    OverClaimAttempt = 1001,
    /// #269: Master viewing key not authorized for clawback
    ViewingKeyUnauthorized = 1002,

    // 🔄 Schedule Consolidation (1100s)
    /// #276: Asset mismatch between schedules - cannot merge
    AssetMismatch = 1100,
    /// #276: Schedule IDs must belong to the calling user
    UnauthorizedScheduleAccess = 1101,
    /// #276: Cannot merge - would artificially accelerate unlock dates
    UnlockDateAcceleration = 1102,
    /// #276: At least 2 schedules required for merging
    InsufficientSchedules = 1103,
    /// #276: Schedule already merged or inactive
    ScheduleNotActive = 1104,
}