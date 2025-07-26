//! Errors intended for use in Crispii
mod crispii_error;
pub use crispii_error::CrispiiError;

mod invalid_argument_error;
pub use invalid_argument_error::InvalidArgumentError;

mod impossible_operation_error;
pub use impossible_operation_error::ImpossibleOperationError;
