use std::error::Error;
use std::fmt::{self, Display};

use rodio::PlayError;

#[derive(Debug)]
pub struct AudioError {
    what: String,
}

impl Error for AudioError {  }

impl Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error with audio engine: {}", self.what)
    }
}

impl From<PlayError> for AudioError {
    fn from(err: PlayError) -> Self {
        AudioError {
            what: err.to_string()
        }
    }
}

impl From<String> for AudioError {
    fn from(str: String) -> Self {
        AudioError {
            what: str
        }
    }
}
impl From<&str> for AudioError {
    fn from(str: &str) -> Self {
        AudioError {
            what: str.to_owned()
        }
    }
}

pub fn update(_dt: f64) -> Result<(), AudioError> {
    Ok(())
}
