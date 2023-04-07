use openal;

use std::error::Error;
use std::fmt::{self, Display};
use std::sync::Mutex;

static mut AL_LISTENER: Option<Mutex<openal::Listener<'static>>> = None;

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

impl From<openal::Error> for AudioError {
    fn from(e: openal::Error) -> Self {
        AudioError { what: format!("OpenAL error: {}", e) }
    }
}

pub fn start() -> Result<(), AudioError> {
    unsafe {
        AL_LISTENER = Some(Mutex::new(openal::listener::default(&openal::listener::Attributes::default())?))
    }

    Ok(())
}

pub fn update() -> Result<(), AudioError> {
    Ok(())
}