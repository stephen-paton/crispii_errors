use std::{error::Error, fmt};

use crate::CrispiiError;

/// To indicate to the consumer of a function that the argument they passed into it was invalid, and an explanation as to why it was invalid
/// ```
///     use crispii_errors::{InvalidArgumentError, CrispiiError};
/// 
///     fn set_colour(r: u8, g: u8, b: u8, a: u8) -> Result<(), Box<dyn CrispiiError>> {
///         if a > 100 {
///             return Err(Box::new(InvalidArgumentError::new("a", "Must be between 0 and 100 (inclusive)")));
///         }
/// 
///         // logic to set colour
/// 
///         Ok(())
///     }
/// ```
#[derive(Debug)]
pub struct InvalidArgumentError {
    argument: String,
    explanation: String,
}

impl InvalidArgumentError {
    pub fn new(argument: &str, explanation: &str) -> Self {
        Self {
            argument: argument.to_string(),
            explanation: explanation.to_string(),
        }
    }
}

impl Error for InvalidArgumentError {}

impl CrispiiError for InvalidArgumentError {}

impl fmt::Display for InvalidArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}' is invalid: {}!", self.argument, self.explanation)
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_message() {
        let error = InvalidArgumentError::new( "num", "Value must be less than or equal to 99");

        assert_eq!(error.to_string(), "'num' is invalid: Value must be less than or equal to 99!");
    }
}
