use std::fmt;

/// A simple error struct for use in the engine.
#[derive(Debug)]
pub struct BaseError(String);

impl BaseError {
    /// Create a new BaseError.
    pub fn new(message: &str) -> BaseError {
        BaseError(message.to_string())
    }
}

impl fmt::Display for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BaseError: {}", self.0)
    }
}
