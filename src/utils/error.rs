use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct UnsupportedError {
    pub what: String,
}

impl UnsupportedError {
    pub fn new(what: &str) -> UnsupportedError {
        UnsupportedError { what: String::from(what) }
    }
}

impl Display for UnsupportedError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.what)
    }
}

impl Error for UnsupportedError {}

