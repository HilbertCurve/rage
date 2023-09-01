use crate::renderer::{
    buffer::VertexBuffer,
    camera::Camera,
    primitive::{self},
    shader::Shader,
    texture::Spritesheet,
};

use std::error::Error;
use std::fmt::{self, Display};
use std::ptr;
use std::sync::Mutex;

use super::{model::{MODEL_SHADER, MODEL_VB}, buffer::{VAttrib, VProp, VType}};

// global variable, bc idk, probably best
pub static TEX_POOL: Mutex<Vec<Spritesheet>> = Mutex::new(vec![]);

pub fn gl_err_check(line: u32) {
    let err = unsafe { gl::GetError() };
    if err != 0 {
        panic!("error here at line: {}! {}", line, err.to_string())
    }
}
pub fn gl_err_clear() {
    while unsafe { gl::GetError() } != 0 {}
}

#[derive(Debug)]
pub struct RenderError {
    what: String,
}

impl Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.what)
    }
}

impl Error for RenderError {}

impl RenderError {
    pub fn from(message: &str) -> RenderError {
        RenderError { what: String::from(message) }
    }
}

// TODO: render targets; no more `pub static VertexBuffer`s
pub enum RenderTarget {
    Sprite,
    Model,
}

pub trait Renderable {
    fn to_buffer(&self, buf: &mut VertexBuffer) -> Result<(), RenderError>;
}

pub static mut DEFAULT_SHADER: Shader = Shader::new_uninit();

const VERT_CODE: &str = 
"#version 330 core
#ifdef GL_ES
 precision mediump float;
#endif

layout (location=0) in vec3 aPos;
layout (location=1) in vec4 aColor;
layout (location=2) in vec2 aTexUV;
layout (location=3) in float aTexID;

uniform mat4 uProjection;
uniform mat4 uView;

out vec4 fPos;
out vec4 fColor;
out vec2 fTexUV;
out float fTexID;

void main()
{
    fPos = uProjection * uView * vec4(aPos, 1.0);
    fColor = aColor;
    fTexUV = aTexUV;
    fTexID = aTexID;
    gl_Position = uProjection * uView * vec4(aPos, 1.0);
}";
const FRAG_CODE: &str = 
"#version 330 core

in vec4 fPos;
in vec4 fColor;
in vec2 fTexUV;
in float fTexID;

uniform sampler2D uTextures[8];
// uniform float uTime;

out vec4 color;
void main()                          
{
    /*
    if (fTexID != 0.0)
    {
        color = fColor * texture(uTextures[int(fTexID - 1)], fTexUV);
    }
    */
    if (fTexID == 1.0)
    {
        color = fColor * texture(uTextures[0], fTexUV);
    }
    else if (fTexID == 2.0)
    {
        color = fColor * texture(uTextures[1], fTexUV);
    }
    else if (fTexID == 3.0)
    {
        color = fColor * texture(uTextures[2], fTexUV);
    }
    else if (fTexID == 4.0)
    {
        color = fColor * texture(uTextures[3], fTexUV);
    }
    else if (fTexID == 5.0)
    {
        color = fColor * texture(uTextures[4], fTexUV);
    }
    else if (fTexID == 6.0)
    {
        color = fColor * texture(uTextures[5], fTexUV);
    }
    else if (fTexID == 7.0)
    {
        color = fColor * texture(uTextures[6], fTexUV);
    }
    else if (fTexID == 8.0)
    {
        color = fColor * texture(uTextures[7], fTexUV);
    }
    else
    {
        color = fColor;
    }
}";

pub static mut DEFAULT_VB: VertexBuffer = VertexBuffer::new();

pub fn start() {
    unsafe {
        // this spot is for initializing vertex buffers, data buffers,
        // and shaders, along with some gl settings

        DEFAULT_VB.set_layout(&VertexBuffer::DEFAULT_ATTRIBS);
        DEFAULT_VB.set_primitive(&primitive::QUAD);
        DEFAULT_SHADER = Shader::new(VERT_CODE.to_owned(), FRAG_CODE.to_owned());

        MODEL_VB.set_layout(&[
            VAttrib { v_prop: VProp::Position, v_type: VType::Float, v_count: 3 },
            VAttrib { v_prop: VProp::Color, v_type: VType::Float, v_count: 4 },
            VAttrib { v_prop: VProp::Other, v_type: VType::Float, v_count: 3 },
            VAttrib { v_prop: VProp::TexUV, v_type: VType::Float, v_count: 2 },
            VAttrib { v_prop: VProp::TexID, v_type: VType::Float, v_count: 1 },
        ]);
        MODEL_VB.set_primitive(&primitive::MODEL);
        MODEL_SHADER = Shader::new(super::model::VERT_CODE.to_owned(), super::model::FRAG_CODE.to_owned());

        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }
}

pub fn render(buffer: &mut VertexBuffer, shader: &mut Shader) {
    // TODO: static vec of data buffers, render each according to their primitive
    unsafe {
        gl_err_clear();
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl_err_check(line!());

        buffer.bind();
        gl_err_check(line!());
        buffer.refresh();
        gl_err_check(line!());

        // attach textures
        for tex in TEX_POOL.try_lock().unwrap().iter() {
            gl::ActiveTexture(gl::TEXTURE0 + tex.get_id());
            gl::BindTexture(gl::TEXTURE_2D, tex.get_id());
        }

        // shader stuff
        shader.attach();
        gl_err_check(line!());
        shader.set_uniform_mat4("uProjection", Camera::get().projection_mat());
        gl_err_check(line!());
        shader.set_uniform_mat4("uView", Camera::get().view_mat());
        gl_err_check(line!());
        // there must be a better way to do this
        let mut ids;
        {
            let tpl = TEX_POOL.try_lock().unwrap();
            ids = vec![0i32;tpl.len()];
            let mut i = 0;
            for id in ids.iter_mut() {
                *id = tpl.get(i).unwrap().get_id() as i32;
                i += 1;
            }
        }
        // make sure to attach integer values to the uTextures as well!!!
        shader.set_uniform_i32_array(
            "uTextures",
            TEX_POOL.try_lock().unwrap().len() as i32,
            ids.as_ptr(),
        );

        // vertex attrib pointers
        buffer.enable_attribs();
        gl_err_check(line!());

        // draw
        gl::DrawElements(
            buffer.prim.gl_prim,
            buffer.ib.len() as i32 / std::mem::size_of::<u32>() as i32,
            gl::UNSIGNED_INT,
            ptr::null()
            );
        gl_err_check(line!());

        buffer.disable_attribs();
        gl_err_check(line!());

        buffer.unbind();
        gl_err_check(line!());
        buffer.clear();
        gl_err_check(line!());
        shader.detach();
        gl_err_check(line!());

        // detach textures
        for tex in TEX_POOL.try_lock().unwrap().iter() {
            gl::ActiveTexture(gl::TEXTURE0 + tex.get_id());
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

