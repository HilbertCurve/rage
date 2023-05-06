use std::error::Error;

use rodio::PlayError;

#[derive(Error)]
pub struct AudioError {
    what: String,
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
impl From<std::io::Error> for AudioError {
    fn from(err: std::io::Error) -> Self {
        AudioError {
            what: err.to_string()
        }
    }
}

pub fn update(_dt: f64) -> Result<(), AudioError> {
    Ok(())
}
