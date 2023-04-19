use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;

use rodio::{Sink, Decoder, OutputStream, OutputStreamHandle};

use crate::ecs::component::{Component, ComponentError, DynComponent};
use crate::ecs::entity::Entity;

use super::controller::AudioError;

#[derive(Component)]
pub struct AudioSource {
    playing: Option<String>,
    _rodio_stream: OutputStream,
    rodio_handle: OutputStreamHandle,
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
            playing: None,
            _rodio_stream: s.0,
            rodio_handle: s.1,
        })
    }
    
    //TODO: looping capability
    pub fn play(&mut self, filepath: String) -> Result<(), AudioError> {
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open(filepath).unwrap());

        self.rodio_handle.play_once(file).unwrap().detach();

        Ok(())
    }
    
}

impl DynComponent for AudioSource {
    unsafe fn start(&mut self, _parent: *mut Entity) -> Result<(), ComponentError> {
        // TODO: more audio source detecting, transform position stuff
        
        Ok(())
    }
    unsafe fn update(&mut self, _dt: f64, _parent: *mut Entity) -> Result<(), ComponentError> {
        // audio change depending on distance?

        Ok(())
    }
    unsafe fn stop(&mut self, _parent: *mut Entity) -> Result<(), ComponentError> {
        // safe stream dropping

        Ok(())
    }
}