use std::{error::Error, fmt};

use crate::traits::CrispiiError;

/// To indicate to the caller of a function that the operation that they attempted to perform using the function was impossible, and an explanation as to why it was impossible
/// ```
///     use crispii_errors::{
///         enums::CrispiiError,
///         structs::ImpossibleOperationError,
///     };
/// 
///     fn add_one(num: u8) -> Result<u8, CrispiiError> {
///         if num == u8::MAX {
///             return Err(CrispiiError::ImpossibleOperation(ImpossibleOperationError::new("'num' is already equal to the maximum possible u8 value")));
///         }
///         
///         Ok(num + 1)
///     }
/// ```
#[derive(Debug)]
pub struct ImpossibleOperationError {
    explanation: String,
}

impl ImpossibleOperationError {
    pub fn new(explanation: &str) -> Self {
        Self {
            explanation: explanation.to_string(),
        }
    }
}

impl Error for ImpossibleOperationError {}
impl CrispiiError for ImpossibleOperationError {}

impl fmt::Display for ImpossibleOperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}!", self.explanation)
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_message() {
        let error = ImpossibleOperationError::new("'num' is already equal to the maximum possible u8 value");

        assert_eq!(error.to_string(), "'num' is already equal to the maximum possible u8 value!");
    }
}
