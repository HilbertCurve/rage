pub mod core;
pub mod renderer;
// re-exports here

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut config: core::Config = core::Config::default();
        //config.proj_mode = renderer::camera::CameraMode::Perspective;
        core::run(config).expect("rage-quit");
    }
}

