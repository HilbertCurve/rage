use glam::{Vec3, Vec4};

use crate::ecs::{
    component::{Component, ComponentError, DynComponent},
    entity::Entity,
    transform::Transform,
};
use crate::renderer::{
    buffer::{VertexBuffer, VProp, VType},
    renderer::{DEFAULT_VB, Renderable, RenderError},
    texture::Texture,
};

#[derive(Component)]
pub struct SpriteRenderer {
    pub color: Vec4,
    pub texture: Texture,
    parent: *const Entity,
}

impl SpriteRenderer {
    pub fn from(color: Vec4, texture: Texture) -> SpriteRenderer {
        SpriteRenderer {
            color,
            texture,
            parent: std::ptr::null(),
        }
    }
}

unsafe impl Send for SpriteRenderer {}

impl DynComponent for SpriteRenderer {
    fn get_parent(&self) -> Option<&Entity> {
        if self.parent.is_null() {
            None
        } else {
            unsafe { Some(&*self.parent) }
        }
    }
    fn set_parent(&mut self, parent: &Entity) {
        self.parent = parent as *const Entity;
    }
    fn detach(&mut self) {
        self.parent = std::ptr::null();
    }
    fn update(&mut self) -> Result<(), ComponentError> {
        if let Err(err) = unsafe { self.to_buffer(&mut DEFAULT_VB) } {
            Err(ComponentError::BadUpdate(format!("Buffering failed: {}", err)))
        } else {
            Ok(())
        }
    }
}

impl Renderable for SpriteRenderer {
    fn to_buffer(&self, buf: &mut VertexBuffer) -> Result<(), RenderError> {
        // get transform
        let trans = *self
            .get_parent().expect("how was this updated if it had no parent?")
            .get::<Transform>()
            .or(Err(RenderError::from("No transform")))?;

        let mut offset = buf.vb.len();

        // get attrib info and do checks
        let (pos_pos, pos_size, pos_type_enum) = buf.attrib_metadata(VProp::Position)?;
        let (col_pos, col_size, col_type_enum) = buf.attrib_metadata(VProp::Color)?;
        let (tuv_pos, tuv_size, tuv_type_enum) = buf.attrib_metadata(VProp::TexUV)?;
        let (tid_pos, _, tid_type_enum) = buf.attrib_metadata(VProp::TexID)?;
        if pos_size < 3 || pos_type_enum != VType::Float {
            return Err(RenderError::from(
                    &format!("bad position layout, got {} of type {:?}", pos_size, pos_type_enum)));
        }
        if col_size < 4 || col_type_enum != VType::Float {
            return Err(RenderError::from(
                    &format!("bad color layout, got {} of type {:?}", col_size, col_type_enum)));
        }
        if tuv_size < 2 || tuv_type_enum != VType::Float {
            return Err(RenderError::from(
                    &format!("bad tex uv layout, got {} of type {:?}", tuv_size, tuv_type_enum)));
        }
        if tid_type_enum != VType::Float {
            return Err(RenderError::from(
                    &format!("bad tex id layout, got {} of type {:?}", tuv_size, tuv_type_enum)));
        }

        let corners: [Vec3; 4] = [
            trans.pos + Vec3::new( trans.whd.x,  trans.whd.y, 0.0) / 2.0,
            trans.pos + Vec3::new(-trans.whd.x,  trans.whd.y, 0.0) / 2.0,
            trans.pos + Vec3::new(-trans.whd.x, -trans.whd.y, 0.0) / 2.0,
            trans.pos + Vec3::new( trans.whd.x, -trans.whd.y, 0.0) / 2.0,
        ];

        let mut acc = 0;
        for corner in corners {
            // buffer vertex

            // NOTE: it's ok to set a float to 0x00000000, that evaluates to 0.0
            for _ in 0..buf.layout_len() {
                buf.vb.push(0u8);
            }

            for i in 0..3 {
                buf.vb.set(
                    offset + (pos_pos + i) * pos_type_enum.size_bytes(),
                    corner[i])
                    .or(Err(RenderError::from(&format!("bad block insertion"))))?;
            }
            for i in 0..4 {
                buf.vb.set(
                    offset + (col_pos + i) * col_type_enum.size_bytes(),
                    self.color[i])
                    .or(Err(RenderError::from(&format!("bad block insertion"))))?;
            }
            for i in 0..2 {
                buf.vb.set(
                    offset + (tuv_pos + i) * tuv_type_enum.size_bytes(),
                    self.texture.uvs[acc+i])
                    .or(Err(RenderError::from(&format!("bad block insertion"))))?;
            }
            buf.vb.set(
                offset + tid_pos * tid_type_enum.size_bytes(),
                self.texture.id as f32)
                .or(Err(RenderError::from(&format!("bad block insertion"))))?;

            offset += buf.layout_len() as usize;
            acc += 2;
        }

        buf.size += 2;

        Ok(())
    }
}

