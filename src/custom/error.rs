use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct GenerateError {
    details: String,
}

#[allow(dead_code)]
impl GenerateError {
    pub fn new<S: Into<String>>(msg: S) -> GenerateError {
        GenerateError {
            details: msg.into(),
        }
    }
}

impl fmt::Display for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for GenerateError {
    fn description(&self) -> &str {
        &self.details
    }
}

