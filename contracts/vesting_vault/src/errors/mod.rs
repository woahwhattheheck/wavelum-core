/// Stable contract error codes exposed by the vesting vault.
pub mod codes;
/// Small result helpers for converting validation failures into contract errors.
pub mod helpers;

pub use codes::Error;
pub use helpers::*;