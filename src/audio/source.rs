use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;

use rodio::{Sink, OutputStream, OutputStreamHandle};

use crate::ecs::component::{Component, ComponentError, DynComponent};
use crate::ecs::entity::Entity;

use super::controller::AudioError;

#[derive(PartialEq, Clone)]
pub enum AudioSourceState {
    Playing(String),
    Paused(String),
    Stopped,
}

#[derive(Component)]
pub struct AudioSource {
    state: AudioSourceState,
    _rodio_stream: OutputStream,
    rodio_handle: OutputStreamHandle,
    rodio_sink: Option<Sink>,
}

//TODO: move this and WAVFile to an asset manager file/directory
#[derive(Debug)]
pub struct ResourceError {
    what: String,
}

impl ResourceError {
    pub fn new(what: String) -> ResourceError {
        ResourceError { what }
    }
}

impl std::error::Error for ResourceError {  }

impl Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceError: {}", self.what)
    }
}

impl AudioSource {
    pub fn new() -> Result<AudioSource, AudioError> {
        let s = OutputStream::try_default().expect("audio device not found");
        Ok(AudioSource {
            state: AudioSourceState::Stopped,
            _rodio_stream: s.0,
            rodio_handle: s.1,
            rodio_sink: None,
        })
    }
    
    //TODO: looping capability
    pub fn play(&mut self, filepath: String) -> Result<(), AudioError> {
        self.stop()?;

        self.state = AudioSourceState::Playing(filepath.clone());

        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open(filepath)?);

        // using the rodio handle, play the buffered file
        // TODO: pre-buffered files
        self.rodio_sink = Some(self.rodio_handle.play_once(file).unwrap());
        self.get_sink()?.play();

        Ok(())
    }
    pub fn resume(&mut self) -> Result<(), AudioError> {
        match self.state.clone() {
            AudioSourceState::Playing(_) => {
                Ok(())
            }
            AudioSourceState::Paused(fp) => {
                self.get_sink()?.play();
                self.state = AudioSourceState::Playing(fp);
                Ok(())
            }
            AudioSourceState::Stopped => {
                Err(AudioError::from("Cannot pause already stopped AudioSource"))
            }
        }
    }
    pub fn pause(&mut self) -> Result<(), AudioError> {
        match self.state.clone() {
            AudioSourceState::Playing(fp) => {
                self.get_sink()?.pause();
                self.state = AudioSourceState::Paused(fp);
                Ok(())
            }
            AudioSourceState::Paused(_) => {
                Ok(())
            }
            AudioSourceState::Stopped => {
                Err(AudioError::from("Cannot pause already stopped AudioSource"))
            }
        }
    }
    pub fn stop(&mut self) -> Result<(), AudioError> {
        match self.state.clone() {
            AudioSourceState::Playing(_) | AudioSourceState::Paused(_) => {
                self.get_sink()?.stop();
                self.rodio_sink = None;
                self.state = AudioSourceState::Stopped;

                Ok(())
            }
            AudioSourceState::Stopped => {
                Ok(())
            }
        }
        
    }
    fn get_sink(&mut self) -> Result<&mut Sink, AudioError> {
        if let Some(sink) = &mut self.rodio_sink {
            Ok(sink)
        } else {
            Err(AudioError::from("No sink found"))
        }
    }
}

impl DynComponent for AudioSource {
    unsafe fn start(&mut self, _parent: *mut Entity) -> Result<(), ComponentError> {
        // TODO: more audio source detecting, transform position stuff
        
        Ok(())
    }
    unsafe fn update(&mut self, _dt: f64, _parent: *mut Entity) -> Result<(), ComponentError> {
        // audio change depending on distance?
        // update state based on state of the Sink
        // TODO: change unwrap to `fn()?`
        if let AudioSourceState::Playing(_) = self.state {
            if self.get_sink().unwrap().empty() {
                self.stop().unwrap();
            }
        }

        Ok(())
    }
    unsafe fn stop(&mut self, _parent: *mut Entity) -> Result<(), ComponentError> {
        // safe stream dropping

        Ok(())
    }
}