use super::go::Go;

pub enum ComponentError {
    InvalidParent,
    InvalidOp,
    AlreadyPresent,
    NotPresent,
}

#[derive(PartialEq)]
pub enum ComponentEnum {
    SpriteRenderer,
    Transform,
    TextRenderer,
    Custom(String),
}

// TODO: as_any and as_any_mut
pub trait Component: 'static {

    fn type_enum(&self) -> ComponentEnum;
}

// TODO: set_parent, detach, update
pub trait DynComponent: 'static + Component {
    fn get_parent(&mut self) -> Option<&Entity>;

}

