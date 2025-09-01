use std::error::Error;

/// An empty marker trait for CrispiiErrors
#[allow(dead_code)]
pub(crate) trait CrispiiError : Error {}
