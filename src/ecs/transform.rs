use glam::Vec2;

use crate::ecs::component::Component;
use rage_macros::*;

#[derive(Component)]
pub struct Transform {
    pub pos: Vec2,
}

