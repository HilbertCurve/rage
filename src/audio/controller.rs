use std::error::Error;
use std::fmt::{self, Display};
use std::sync::{Mutex, Arc};

use glam::Vec3;
use rodio::{OutputStreamHandle, OutputStream, PlayError};

struct AudioController {
    /// position of "listener"
    /// TODO: implement this in audio mixing
    pos: Vec3,
}

impl AudioController {
    pub fn new() -> AudioController {
        AudioController {
            pos: Vec3::ZERO,
        }
    }
}

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

pub fn update(_dt: f64) -> Result<(), AudioError> {
    Ok(())
}
