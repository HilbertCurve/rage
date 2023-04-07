use crate::ecs::component::Component;

use glam::Vec3;

/// A component that reflects three-dimensional qualities of the attached Entity.
#[derive(Component, Clone, Copy)]
pub struct Transform {
    /// Position
    pub pos: Vec3,
    /// Width, height, and depth
    pub whd: Vec3,
    /// Orientation in 3D space, rotation on the x, y, and z axes respectively
    pub rot: Vec3,
}

impl Transform {
    pub fn from(pos: Vec3, whd: Vec3, rot: Vec3) -> Transform {
        Transform { pos, whd, rot }
    }
    pub fn zero() -> Transform {
        Transform { pos: Vec3::ZERO, whd: Vec3::ZERO, rot: Vec3::ZERO }
    }
}

