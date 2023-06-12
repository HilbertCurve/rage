use std::error::Error;

#[derive(Error)]
pub struct UnsupportedError {
    pub what: String,
}

impl UnsupportedError {
    pub fn new(what: &str) -> UnsupportedError {
        UnsupportedError { what: String::from(what) }
    }
}

#[macro_export]
macro_rules! impl_error {
    ($name:ident) => {
        use std::error::Error;
        #[derive(Error)]
        pub struct $name {
            what: String,
        }
    }
}
