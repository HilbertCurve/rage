use crate::renderer::camera::CameraMode;

use std::error::Error;
use std::fmt::{Debug, Display};
use std::sync::Once;
use std::mem::MaybeUninit;

// TODO: future config stuff:
//     anti-aliasing?
//
pub struct Config {
    // window stuff
    /// Width of default window at startup.
    pub window_width: u32,
    /// Height of default window at startup.
    pub window_height: u32,
    /// Title of game window.
    pub window_title: String,
    // renderer stuff
    pub proj_mode: CameraMode,
}

#[derive(Debug)]
pub struct ConfigError {
    what: String,
}

impl ConfigError {
    pub fn new(what: &str) -> ConfigError {
        ConfigError { what: String::from(what) }
    }
}
impl Error for ConfigError {}
impl Display for ConfigError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.what)?;
        Ok(())
    }
}

static mut SINGLETON: MaybeUninit<Config> = MaybeUninit::uninit();

impl Config {
    pub fn get() -> &'static Config {
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                let singleton = Config::default();
                SINGLETON.write(singleton);
            });

            SINGLETON.assume_init_ref()
        }
    }

    pub fn set(config: Config) -> Result<(), ConfigError> {
        static ONCE: Once = Once::new();
        static mut CALLED: bool = false;

        unsafe {
            ONCE.call_once(|| {
                let get_mut = {
                    Config::get();
                    SINGLETON.assume_init_mut()
                };

                *get_mut = config;
            });

            if !CALLED {
                CALLED = true;
                Ok(())
            } else {
                Err(ConfigError::new("Cannot set configuration twice; try mutating specific values."))
            }
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            window_width: 800,
            window_height: 450,
            window_title: "rage game engine".to_owned(),
            proj_mode: CameraMode::Orthographic,
        }
    }
}

