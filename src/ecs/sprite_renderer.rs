extern crate rage_macros;

use rage_macros::*;

use glam::{Vec2, Vec4};

use crate::ecs::{
    component::{Component, ComponentError},
    entity::Entity,
    transform::Transform,
};
use crate::renderer::{
    buffer::VertexBuffer,
    renderer::{DEFAULT_VB, Renderable, RenderError},
    texture::Texture,
};

#[derive(Component)]
pub struct SpriteRenderer {
    pub color: Vec4,
    pub texture: Texture,
    pos_cache: Vec2,
}

impl SpriteRenderer {
    pub fn new(color: Vec4, texture: Texture) -> SpriteRenderer {
        SpriteRenderer { color, texture, pos_cache: Vec2::new(0.0, 0.0)}
    }
    pub fn update(&mut self, parent: &'static Entity) -> Result<(), ComponentError> {
        // find position of go, if it has transform
        let trans = parent.get::<Transform>()?;

        self.pos_cache = trans.pos;

        unsafe {
            if let Err(err) = self.to_buffer(&mut DEFAULT_VB) {
                Err(ComponentError::BadUpdate(format!("Received render error: {:?}", err)))
            } else {
                Ok(())
            }
        }
    }
}

impl Renderable for SpriteRenderer {
    fn to_buffer(&self, buf: &mut VertexBuffer) -> Result<(), RenderError> {

        Ok(())
    }
}
