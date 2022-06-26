extern crate gl;

use crate::renderer::{
    camera::Camera,
    primitive::{self},
    shader::Shader,
};

use std::mem;
use std::ptr;

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

#[derive(Copy, Clone)]
pub enum VProp {
    Position,
    Color,
    TexCoords,
    TexID,
    Other,
}

#[derive(Copy, Clone)]
pub enum VType {
    Byte,
    UByte,
    Short,
    UShort,
    Int,
    UInt,
    Float,
    Double,
}

impl VType {
    pub fn size_bytes(&self) -> usize {
        match &self {
            VType::Byte =>   mem::size_of::<i8>(),
            VType::UByte =>  mem::size_of::<u8>(),
            VType::Short =>  mem::size_of::<i16>(),
            VType::UShort => mem::size_of::<u16>(),
            VType::Int =>    mem::size_of::<i32>(),
            VType::UInt =>   mem::size_of::<u32>(),
            VType::Float =>  mem::size_of::<f32>(),
            VType::Double => mem::size_of::<f64>(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct VAttrib {
    v_prop: VProp,
    v_type: VType,
    v_count: u32,
}

struct VertexBuffer {
    vao: u32,
    vbo: u32,
    ibo: u32,
    layout: Vec<VAttrib>, // should this be a slice???
    is_used: bool,
    // TODO: temporary behavior, will switch to DataBuffer soon
    vb: Vec<f32>,
    ib: Vec<u32>,
}

impl VertexBuffer {
    pub const DEFAULT_ATTRIBS: [VAttrib; 2] = [
        VAttrib { v_prop: VProp::Position, v_type: VType::Float, v_count: 3 },
        VAttrib { v_prop: VProp::Color, v_type: VType::Float, v_count: 4 },
    ];

    pub const fn new() -> VertexBuffer {
        VertexBuffer {
            vao: 0,
            vbo: 0,
            ibo: 0,
            layout: vec![],
            is_used: false,
            vb: Vec::new(),
            ib: Vec::new(),
        }
    }

    pub fn set_layout(&mut self, slice: &[VAttrib]) {
        self.layout = Vec::from(slice);
    }

    pub fn init(&mut self, vb: &[f32], ib: &[u32]) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ibo);

            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vb.len() * mem::size_of::<f32>()) as isize,
                vb.as_ptr().cast(),
                gl::DYNAMIC_DRAW
                );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (ib.len() * mem::size_of::<u32>()) as isize,
                ib.as_ptr().cast(),
                gl::DYNAMIC_DRAW
                );

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        self.vb = Vec::from(vb);
        println!("{}", vb.len());
        self.ib = Vec::from(ib);
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
        }

        self.is_used = true;
    }

    pub fn unbind(&mut self) {
        unsafe {
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        self.is_used = false;
    }

    pub fn refresh(&mut self) {
        if !self.is_used {
            self.bind();
        }

        unsafe {
            // regenerate index buffer to match size, if size has changed
            for i in 0..2 { // TODO: automate size and use dirty flags
                (primitive::QUAD.gen_indices)(&mut self.ib, i);
            }

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vb.len() * mem::size_of::<f32>()) as isize,
                self.vb.as_ptr().cast(),
                gl::DYNAMIC_DRAW
                );
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.ib.len() * mem::size_of::<u32>()) as isize,
                self.ib.as_ptr().cast(),
                gl::DYNAMIC_DRAW
                );
        }
    }

    fn layout_len(&self) -> u32 {
        self.layout.iter().try_fold(
            0u32,
            |acc, x| acc.checked_add(x.v_type.size_bytes() as u32 * x.v_count)
            ).expect("who knows what went wrong")
    }

    pub fn enable_attribs(&mut self) {
        if !self.is_used {
            self.bind();
        }

        let mut acc: usize = 0;
        for i in 0..self.layout.len() {
            let attrib = &self.layout[i];

            unsafe {
                gl::EnableVertexAttribArray(i as u32);
                gl::VertexAttribPointer(
                    i as u32,
                    attrib.v_count as i32,
                    gl::FLOAT, // FIXME: specify gl types
                    0,
                    self.layout_len() as i32,
                    acc as * const _
                    );
            }

            acc += attrib.v_count as usize * attrib.v_type.size_bytes();
        }
    }

    pub fn disable_attribs(&mut self) {
        if !self.is_used {
            self.bind();
        }

        for i in 0..self.layout.len() {
            unsafe {
                gl::DisableVertexAttribArray(i as u32);
            }
        }
    }
}

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

