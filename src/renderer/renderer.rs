extern crate gl;

use crate::renderer::{shader::Shader, camera::Camera};

use std::mem;

const VB: [f32; 42] = [
    0.0, 0.0, -10.0, 1.0, 1.0, 1.0, 1.0,
    0.0, 1.0, -10.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, -10.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 0.0, -10.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, -10.0, 1.0, 0.0, 1.0, 1.0,
    0.0, 1.0, -10.0, 1.0, 1.0, 0.0, 1.0,
];

const IB: [u32; 6] = [
    0, 1, 2,
    3, 4, 5,
];

static mut DEFAULT_SHADER: Shader = Shader::new_uninit();
static mut DEFAULT_VB: VertexBuffer = VertexBuffer::new();

const SOF: usize = mem::size_of::<f32>();

#[derive(Copy, Clone)]
enum VProp {
    Position,
    Color,
    TexCoords,
    TexID,
    Other,
}

#[derive(Copy, Clone)]
enum VType {
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
struct VAttrib {
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
    vb: &'static [f32],
    ib: &'static [u32],
}

impl VertexBuffer {
    const NO_VB: [f32; 0] = [];
    const NO_IB: [u32; 0] = [];
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
            vb: &VertexBuffer::NO_VB,
            ib: &VertexBuffer::NO_IB,
        }
    }

    pub fn set_layout(&mut self, slice: &[VAttrib]) {
        self.layout = Vec::from(slice);
    }

    pub fn init(&mut self, vb: &'static [f32], ib: &'static [u32]) {
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

        self.vb = vb;
        self.ib = ib;
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
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vb.len() * mem::size_of::<f32>()) as isize,
                self.vb.as_ptr().cast(),
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
        DEFAULT_VB.set_layout(&VertexBuffer::DEFAULT_ATTRIBS);
        DEFAULT_VB.init(&VB, &IB);
        DEFAULT_VB.bind();
        DEFAULT_SHADER = Shader::new("assets/shaders/default.vert", "assets/shaders/default.frag");
        DEFAULT_VB.refresh();
        DEFAULT_VB.enable_attribs();

        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }
}

pub fn update() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);

        DEFAULT_VB.bind();
        DEFAULT_VB.refresh();

        // shader stuff
        DEFAULT_SHADER.attach();
        DEFAULT_SHADER.set_uniform_mat4("uProjection", Camera::get().projection_mat());
        DEFAULT_SHADER.set_uniform_mat4("uView", Camera::get().view_mat());

        // vertex attrib pointers
        DEFAULT_VB.enable_attribs();

        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

        DEFAULT_VB.disable_attribs();

        DEFAULT_VB.unbind();
        DEFAULT_SHADER.detach();
    }
}

