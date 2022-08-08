pub use rage_macros::Component as component_derive;

use super::entity::Entity;

pub enum ComponentError {
    InvalidParent,
    InvalidOp,
    AlreadyPresent(String),
    NotPresent(String),
    BadUpdate(String),
}

pub trait Component: 'static {
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

