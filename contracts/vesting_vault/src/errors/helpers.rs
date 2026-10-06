use crate::errors::codes::Error;

// Generic condition checker
/// Returns `Ok(())` when `condition` is true, otherwise returns `err`.
///
/// Use this helper for validation branches that should surface a typed contract error
/// instead of panicking.
pub fn ensure(condition: bool, err: Error) -> Result<(), Error> {
    if !condition {
        return Err(err);
    }
    Ok(())
}

// Option → Result converter
/// Converts an optional value into a typed contract result.
///
/// Returns the contained value when present and returns `err` when the option is empty.
pub fn unwrap_or_error<T>(opt: Option<T>, err: Error) -> Result<T, Error> {
    match opt {
        Some(val) => Ok(val),
        None => Err(err),
    }
}