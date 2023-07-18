use std::fmt;

pub struct BaseError(String);

impl BaseError {
    pub fn new(message: &str) -> BaseError {
        BaseError(message.to_string())
    }
}

impl fmt::Display for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BaseError: {}", self.0)
    }
}
