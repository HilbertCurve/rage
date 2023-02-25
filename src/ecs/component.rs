pub use rage_macros::Component as component_derive;

use super::entity::Entity;

use std::error::Error;
use std::fmt::{self, Display, Formatter};

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

pub trait Component: 'static + Send {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    // TODO: flawed: use better type checking
    fn type_str() -> &'static str where Self: Sized;
}

pub trait DynComponent: 'static + Component {
    fn get_parent(&self) -> Option<&Entity>;
    fn set_parent(&mut self, parent: &Entity);
    fn detach(&mut self);
    fn update(&mut self) -> Result<(), ComponentError>;
}

