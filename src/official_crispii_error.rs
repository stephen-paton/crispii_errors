use std::error::Error;

/// An internal-only marker trait intended to prevent library users from creating their own "Crispii" errors by marking them with the CrispiiError trait 
pub(crate) trait OfficialCrispiiError : Error {}
