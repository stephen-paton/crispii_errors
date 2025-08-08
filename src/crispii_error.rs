use crate::OfficialCrispiiError;

/// An empty marker trait for CrispiiErrors
#[allow(private_bounds)]
pub trait CrispiiError : OfficialCrispiiError {}
