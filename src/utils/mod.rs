use std::fmt;
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

#[derive(Debug, Error)]
pub struct Error {
    pub what: String,
}

impl Error {
    pub fn new(s: &str) -> Error {
        let what = s.to_string();
        Error { what }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.what)
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub what: String,
}

impl ParseError {
    pub fn new(s: &str) -> ParseError {
        let what = s.to_string();
        ParseError { what }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.what)
    }
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        "Error while parsing input"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

impl From<ParseIntError> for ParseError {
    fn from(_error: ParseIntError) -> Self {
        ParseError::new("Unable to parse integer")
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(_error: ParseFloatError) -> Self {
        ParseError::new("Unable to parse float")
    }
}

impl From<ParseError> for Error {
    fn from(_error: ParseError) -> Self {
        Error::new("Unable to parse something")
    }
}
