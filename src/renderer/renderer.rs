extern crate gl;

use crate::renderer::{
    buffer::VertexBuffer,
    camera::Camera,
    primitive::{self},
    shader::Shader,
};

use std::ptr;

pub struct RenderError {
    what: String,
}

impl RenderError {
    pub fn from(message: &str) -> RenderError {
        RenderError { what: String::from(message) }
    }
}

pub trait Renderable {
    fn to_buffer(buf: &mut VertexBuffer, pos: u32) -> Result<(), RenderError>;
}

const VB: [f32; 56] = [
    0.0, 0.0, -10.0, 1.0, 1.0, 0.0, 1.0,
    0.0, 0.5, -10.0, 1.0, 0.0, 1.0, 1.0,
    0.5, 0.5, -10.0, 0.0, 1.0, 0.0, 1.0,
    0.5, 0.0, -10.0, 0.0, 1.0, 1.0, 1.0,
    0.5, 0.5, -10.0, 0.0, 0.0, 1.0, 1.0,
    0.5, 1.0, -10.0, 0.0, 0.0, 1.0, 1.0,
    1.0, 1.0, -10.0, 1.0, 0.0, 1.0, 1.0,
    1.0, 0.5, -10.0, 1.0, 0.0, 0.0, 1.0,
];

static mut IB: Vec<u32> = vec![];

static mut DEFAULT_SHADER: Shader = Shader::new_uninit();
static mut DEFAULT_VB: VertexBuffer = VertexBuffer::new();

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

        // vb.bind(); ...
        DEFAULT_VB.bind();
        DEFAULT_VB.refresh();

        // shader stuff
        // vb.shader.attach(); ...
        DEFAULT_SHADER.attach();
        DEFAULT_SHADER.set_uniform_mat4("uProjection", Camera::get().projection_mat());
        DEFAULT_SHADER.set_uniform_mat4("uView", Camera::get().view_mat());

        // vertex attrib pointers
        DEFAULT_VB.enable_attribs();

        let quad = &primitive::QUAD;
        // ...(vb.prim.gl_prim, vb.prim.index_count * vb.len, gl::UNSIGNED_INT, ptr::null());
        gl::DrawElements(quad.gl_prim, 12, gl::UNSIGNED_INT, ptr::null());

        DEFAULT_VB.disable_attribs();

        DEFAULT_VB.unbind();
        DEFAULT_SHADER.detach();
    }
}
