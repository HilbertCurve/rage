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
    fn type_str() -> &'static str where Self: Sized;
}

