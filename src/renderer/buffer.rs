use crate::renderer::primitive;
use crate::utils::block::Block;

use std::mem;

#[derive(Copy, Clone, PartialEq)]
pub enum VProp {
    Position,
    Color,
    TexCoords,
    TexID,
    Other,
}

#[derive(Copy, Clone, PartialEq)]
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
    pub v_prop: VProp,
    pub v_type: VType,
    pub v_count: u32,
}

pub struct VertexBuffer {
    vao: u32,
    vbo: u32,
    ibo: u32,
    pub layout: Vec<VAttrib>, // should this be a slice???
    pub is_used: bool,
    prim: &'static primitive::Primitive,
    /// Number of elements to be rendered, as based on vb.len();
    /// The user is responsible for managing the value of this item,
    /// as doing so automatically could lead to UB
    pub size: u32,
    pub vb: Block,
    pub ib: Block,
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
            prim: &primitive::NONE,
            size: 0,
            vb: Block::empty(),
            ib: Block::empty(),
        }
    }

    pub fn set_layout(&mut self, slice: &[VAttrib]) {
        self.layout = Vec::from(slice);
    }

    pub fn set_primitive(&mut self, prim: &'static primitive::Primitive) {
        self.prim = prim;
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

        self.vb.clear();
        self.ib.clear();
        for item in vb {
            self.vb.push(*item);
        }
        for item in ib {
            self.ib.push(*item);
        }
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
                (self.prim.gen_indices)(&mut self.ib, i);
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

