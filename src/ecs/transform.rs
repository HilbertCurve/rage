use crate::ecs::component::Component;

use glam::Vec3;

#[derive(Component, Clone, Copy)]
pub struct Transform {
    pub pos: Vec3,
    pub whd: Vec3,
}

impl Transform {
    pub fn from(pos: Vec3, whd: Vec3) -> Transform {
        Transform { pos, whd }
    }
}

