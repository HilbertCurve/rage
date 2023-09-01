use glam::{Vec3, Vec4};

use crate::{ecs::{
    component::{Component, ComponentError, DynComponent},
    entity::Entity,
    transform::Transform,
}, renderer::texture::Spritesheet};
use crate::renderer::{
    buffer::{VertexBuffer, VProp, VType},
    renderer::{DEFAULT_VB, Renderable, RenderError},
    texture::Texture,
};

#[derive(Component)]
pub struct SpriteRenderer {
    pub color: Vec4,
    pub textures: Vec<Texture>,
    cur_tex: usize,
    trans_cache: Transform,
}

impl SpriteRenderer {
    /// Create a SpriteRenderer from frames `start..end` in a Spritesheet.
    /// 
    /// Warning: if start and end are equal, no textures will be selected.
    /// In addition, if start or end is out of bounds of the Spritesheet's
    /// textures, this will crash.
    /// TODO: bounds check error when `start == end`.
    pub fn slice(color: Vec4, sheet: &Spritesheet, start: usize, end: usize) -> SpriteRenderer {
        SpriteRenderer {
            color,
            textures: sheet.as_vec()[start..end].to_vec(),
            cur_tex: 0,
            trans_cache: Transform::zero(),
        }
    }
    /// Create a SpriteRenderer from `frame`th texture in a Spritesheet.
    /// 
    /// TODO: this function does not check bounds.
    pub fn select(color: Vec4, sheet: &Spritesheet, frame: usize) -> SpriteRenderer {
        SpriteRenderer {
            color,
            textures: vec![sheet.get_texture(frame)],
            cur_tex: 0,
            trans_cache: Transform::zero(),
        }
    }
    /// Advances this SpriteRenderer to the next frame.
    pub fn next_frame(&mut self) -> Result<(), ComponentError> {
        if self.cur_tex + 1 >= self.textures.len() {
            Err(ComponentError::InvalidOp)
        } else {
            self.cur_tex += 1;
            Ok(())
        }
    }
    /// Backtracks this SpriteRenderer to the previous frame.
    pub fn prev_frame(&mut self) -> Result<(), ComponentError> {
        if self.cur_tex - 1 <= 0 {
            Err(ComponentError::InvalidOp)
        } else {
            self.cur_tex -= 1;
            Ok(())
        }
    }
    /// Advances this SpriteRenderer to the next frame, looping to 
    /// the first frame if it overflows.
    #[inline]
    pub fn next_wrap(&mut self) {
        if let Err(_) = self.next_frame() {
            self.cur_tex = 0;
        }
    }
    /// Backtracks this SpriteRenderer to the previous frame, looping
    /// to the last frame if it underflows.
    #[inline]
    pub fn prev_wrap(&mut self) {
        if let Err(_) = self.prev_frame() {
            self.cur_tex = self.textures.len() - 1;
        }
    }
    /// Sets this SpriteRenderer to the first frame of it's animation.
    #[inline]
    pub fn first_frame(&mut self) {
        self.cur_tex = 0;
    }
    /// Sets this SpriteRenderer to the last frame of it's animation.
    #[inline]
    pub fn last_frame(&mut self) {
        self.cur_tex = self.textures.len() - 1;
    }
    #[inline]
    pub fn curr_frame(&self) -> usize {
        self.cur_tex
    }
}

unsafe impl Send for SpriteRenderer {}

impl From<Vec4> for SpriteRenderer {
    fn from(color: Vec4) -> Self {
        SpriteRenderer {
            color: color,
            textures: vec![Spritesheet::empty_tex()],
            cur_tex: 0,
            trans_cache: Transform::zero()
        }
    }
}

impl From<&Spritesheet> for SpriteRenderer {
    fn from(sheet: &Spritesheet) -> Self {
        SpriteRenderer {
            color: Vec4::ONE,
            textures: sheet.as_vec(),
            cur_tex: 0,
            trans_cache: Transform::zero()
        }
    }
}

impl DynComponent for SpriteRenderer {
    unsafe fn start(&mut self, _parent: *mut Entity) -> Result<(), ComponentError> {
        Ok(())
    }
    unsafe fn update(&mut self, _dt: f64, parent: *mut Entity) -> Result<(), ComponentError> {
        self.trans_cache = (*parent).get::<Transform>()?.clone();
        if let Err(err) = self.to_buffer(&mut DEFAULT_VB) {
            Err(ComponentError::BadUpdate(format!("Buffering failed: {}", err)))
        } else {
            Ok(())
        }
    }
    unsafe fn stop(&mut self, _parent: *mut Entity) -> Result<(), ComponentError> {
        //TODO: is there something to clean here???
        Ok(())
    }
}

impl Renderable for SpriteRenderer {
    fn to_buffer(&self, buf: &mut VertexBuffer) -> Result<(), RenderError> {
        // get transform
        let trans = self.trans_cache;

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
            Vec3::new( trans.whd.x,  trans.whd.y, 0.0) / 2.0,
            Vec3::new(-trans.whd.x,  trans.whd.y, 0.0) / 2.0,
            Vec3::new(-trans.whd.x, -trans.whd.y, 0.0) / 2.0,
            Vec3::new( trans.whd.x, -trans.whd.y, 0.0) / 2.0,
        ];

        // rotation: gimbal rotation, from x to y to z
        let rotation = (glam::Mat4::from_rotation_x(trans.rot.x) * 
                       glam::Mat4::from_rotation_y(trans.rot.y) * 
                       glam::Mat4::from_rotation_z(trans.rot.z)).inverse();
        let translation = glam::Mat4::from_translation(trans.pos);

        let mut acc = 0;
        for mut corner in corners {
            // buffer vertex

            corner = (translation * rotation * Vec4::from((corner, 1.0))).truncate();

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
                    self.textures[self.cur_tex].uvs[acc+i])
                    .or(Err(RenderError::from(&format!("bad block insertion"))))?;
            }
            buf.vb.set(
                offset + tid_pos * tid_type_enum.size_bytes(),
                self.textures[self.cur_tex].id as f32)
                .or(Err(RenderError::from(&format!("bad block insertion"))))?;

            offset += buf.layout_len();
            acc += 2;
        }

        buf.size += 1;

        Ok(())
    }
}
