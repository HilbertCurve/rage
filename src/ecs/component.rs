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

pub trait Component: 'static {
    fn start(&self, parent: &Go);
    fn update(&self, parent: &Go);
    fn type_enum(&self) -> ComponentEnum;
}

