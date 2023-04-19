pub use rage_macros::Component as component_derive;

use std::error::Error;
use std::fmt::{self, Display, Formatter};

use super::entity::Entity;

#[derive(Debug)]
pub enum ComponentError {
    InvalidParent,
    InvalidOp,
    AlreadyPresent(String),
    NotPresent(String),
    BadUpdate(String),
}

impl Error for ComponentError {  }

impl Display for ComponentError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidParent => write!(f, "invalid parent"),
            Self::InvalidOp => write!(f, "invalid operation"),
            Self::AlreadyPresent(t) => write!(f, "component already present: {}", t),
            Self::NotPresent(t) => write!(f, "component not present: {}", t),
            Self::BadUpdate(what) => write!(f, "component had a bad update: {}", what),
        }
    }
}

pub trait Component: 'static {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    // TODO: flawed: use better type checking
    fn type_str() -> &'static str where Self: Sized;
}

pub trait DynComponent: 'static + Component {
    /// Starts the component, initializing any resources and running preliminary checks.
    /// 
    /// It is *highly* suggested that you don't reference any of `parent`'s components in 
    /// this function, as it might not be known what components are added in whichever order,
    /// and is therefore undefined behavior.
    unsafe fn start(&mut self, parent: *mut Entity) -> Result<(), ComponentError>;
    /// Updates the component with a given delta time.
    unsafe fn update(&mut self, dt: f64, parent: *mut Entity) -> Result<(), ComponentError>;
    /// Stops the component, releasing any resources and running extra checks.
    /// 
    /// It is *highly* suggested that you don't reference any of `parent`'s components in 
    /// this function, as it might not be known what components are removed in whichever order,
    /// and is therefore undefined behavior.
    unsafe fn stop(&mut self, parent: *mut Entity) -> Result<(), ComponentError>;
}
