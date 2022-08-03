use glam::Vec4;

use crate::ecs::{
    component::{Component, ComponentEnum},
    go::Go,
};
use crate::renderer::{
    buffer::VertexBuffer,
    renderer::{Renderable, RenderError},
    texture::Texture,
};

pub struct SpriteRenderer {
    color: Vec4,
    texture: Texture,
}

impl Component for SpriteRenderer {
    // you never call these directly; these are called automatically by the parent game object
    fn start(&self, parent: &Go) {

    }
    fn update(&self, parent: &Go) {

    }
    fn type_enum(&self) -> ComponentEnum {
        ComponentEnum::SpriteRenderer
    }
}

impl Renderable for SpriteRenderer {
    fn to_buffer(&self, buf: &mut VertexBuffer, pos: u32) -> Result<(), RenderError> {
        // ensure parent has Transform component

        Ok(())
    }
}
