#[macro_use]
pub extern crate rage_macros;

pub mod audio;
pub mod core;
pub mod ecs;
pub mod renderer;
pub mod utils;

pub mod prelude {
    pub use crate::audio::prelude::*;
    pub use crate::core::prelude::*;
    pub use crate::ecs::prelude::*;
    pub use crate::renderer::prelude::*;
    pub use crate::utils::prelude::*;
    pub use rage_macros::*;

    pub use glam::{Vec2, Vec3, Vec4, vec2, vec3, vec4};
}

extern crate glam;
