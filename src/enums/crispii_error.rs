use crate::structs::{ImpossibleOperationError, InvalidArgumentError};

/// An enum encapsulating all permutations of error that a function *could* throw (will be expanded as new edge cases are found)
/// ```
/// use crispii_errors::{
///     enums::CrispiiError,
///     structs::{
///         ImpossibleOperationError,
///     },
/// };
/// 
/// /// Adds 1 to num
/// /// Failure cases:
/// /// - ImpossibleOperationError -> Attempted to add 1 to a u8::MAX
/// fn add_one(num: u8) -> Result<u8, CrispiiError> {
///     if num == u8::MAX {
///         return Err(CrispiiError::ImpossibleOperation(ImpossibleOperationError::new("'num' is already equal to the maximum possible u8 value")));
///     }
///     
///     Ok(num + 1)
/// }
/// 
/// let result = match add_one(u8::MAX) {
///     Ok(val) => val,
///     Err(crispii_error) => match crispii_error {
///         CrispiiError::ImpossibleOperation(_) => u8::MAX,
///         _ => panic!("No other variant of this error should be returned."),
///     }
/// };
/// ```
pub enum CrispiiError {
    ImpossibleOperation(ImpossibleOperationError),
    InvalidArgument(InvalidArgumentError),
}
