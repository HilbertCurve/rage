use crate::ecs::component::Component;

use glam::Vec3;

#[derive(Component)]
pub struct Transform {
    pub pos: Vec3,
    pub whd: Vec3,
}

