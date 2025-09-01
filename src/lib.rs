//! <code>crispii_errors</code> is a simple crate, consisting of a single <code>CrispiiError</code> enum.
//! 
//! The basic idea is that this enum provides all possible failure outcomes of a function call, which can be returned as the <code>Err(CrispiiError)</code> variant of the <code>Result</code> type.
//! 
//! This pattern provides library creators and consumers with an easy way to share an understanding of how a function call *could* fail, allowing the library's consumer to decide how they wish to handle the failure case.
//! 
//! For example:
//! ```
//! use crispii_errors::{
//!     enums::CrispiiError,
//!     structs::{
//!         ImpossibleOperationError,
//!     },
//! };
//! 
//! /// Adds 1 to num
//! /// Failure cases:
//! /// - ImpossibleOperationError -> Attempted to add 1 to a u8::MAX
//! fn add_one(num: u8) -> Result<u8, CrispiiError> {
//!     if num == u8::MAX {
//!         return Err(CrispiiError::ImpossibleOperation(ImpossibleOperationError::new("'num' is already equal to the maximum possible u8 value")));
//!     }
//!     
//!     Ok(num + 1)
//! }
//! 
//! let result = match add_one(u8::MAX) {
//!     Ok(val) => val,
//!     Err(crispii_error) => match crispii_error {
//!         CrispiiError::ImpossibleOperation(_) => u8::MAX,
//!         _ => panic!("No other variant of this error should be returned."),
//!     }
//! };
//! ```
pub mod enums;

pub mod structs;

mod traits;
