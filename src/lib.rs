mod core;
mod renderer;
// re-exports here
pub use crate::core::*;
pub use crate::renderer::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        core::run();
    }
}
