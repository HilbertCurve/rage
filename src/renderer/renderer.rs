use crate::renderer::{
    buffer::VertexBuffer,
    camera::Camera,
    primitive::{self},
    shader::Shader,
    texture::Spritesheet,
};

use std::error::Error;
use std::fmt::{self, Display};
use std::mem;
use std::ptr;
use std::sync::Mutex;

// global variable, bc idk, probably best
lazy_static! {
    pub static ref TEX_POOL: Mutex<Vec<Spritesheet>> = Mutex::new(vec![]);
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

pub trait Renderable {
    fn to_buffer(&self, buf: &mut VertexBuffer) -> Result<(), RenderError>;
}

const VB: [f32; 80] = [
    0.0, 0.0, -10.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0,
    0.0, 0.5, -10.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0,
    0.5, 0.5, -10.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    0.5, 0.0, -10.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0,
    0.5, 0.5, -10.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0,
    0.5, 1.0, -10.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0,
    1.0, 1.0, -10.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 0.5, -10.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0,
];

static mut IB: Vec<u32> = vec![];

static mut DEFAULT_SHADER: Shader = Shader::new_uninit();
pub static mut DEFAULT_VB: VertexBuffer = VertexBuffer::new();

pub fn start() {
    unsafe {
        // this spot is for initializing default vertex buf, data buf,
        // and shader, along with some gl settings
        IB = vec![
            0, 0, 0,
            0, 0, 0,
            0, 0, 0,
            0, 0, 0,
        ];

        DEFAULT_VB.set_layout(&VertexBuffer::DEFAULT_ATTRIBS);
        DEFAULT_VB.set_primitive(&primitive::QUAD);
        DEFAULT_VB.init(&VB, &IB);
        DEFAULT_VB.bind();
        DEFAULT_VB.refresh();
        DEFAULT_SHADER = Shader::new("assets/shaders/default.vert", "assets/shaders/default.frag");
        DEFAULT_VB.enable_attribs();


        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }
}

pub fn update() {
    // TODO: static vec of data buffers, render each according to their primitive
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);

        DEFAULT_VB.bind();
        DEFAULT_VB.refresh();

        // attach textures
        for tex in &*TEX_POOL.try_lock().unwrap() {
            gl::ActiveTexture(gl::TEXTURE0 + tex.get_id());
            gl::BindTexture(gl::TEXTURE_2D, tex.get_id());
        }

        // shader stuff
        DEFAULT_SHADER.attach();
        DEFAULT_SHADER.set_uniform_mat4("uProjection", Camera::get().projection_mat());
        DEFAULT_SHADER.set_uniform_mat4("uView", Camera::get().view_mat());

        // vertex attrib pointers
        DEFAULT_VB.enable_attribs();

        let quad = &primitive::QUAD;
        gl::DrawElements(
            quad.gl_prim,
            DEFAULT_VB.ib.len() as i32 / mem::size_of::<u32>() as i32,
            gl::UNSIGNED_INT,
            ptr::null()
            );

        DEFAULT_VB.disable_attribs();

        DEFAULT_VB.unbind();
        DEFAULT_SHADER.detach();

        // detach textures
        for tex in &*TEX_POOL.try_lock().unwrap() {
            gl::ActiveTexture(gl::TEXTURE0 + tex.get_id());
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
