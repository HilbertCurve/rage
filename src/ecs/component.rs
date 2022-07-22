use super::go::Go;

use std::rc::Weak;

pub enum ComponentError {
    InvalidParent,
    InvalidOp,
    AlreadyPresent,
}

pub trait Component {
    fn parent(&mut self) -> Weak<Go>;
    fn set_parent(&mut self, go: &Go);
    fn start(&mut self);
    fn update(&mut self);
    fn type_enum(&self) -> String;
}

