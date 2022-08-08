extern crate gl;

use crate::utils::block::Block;

use std::mem;

pub struct Primitive {
    pub vert_count: u32,
    pub index_count: u32,
    pub gl_prim: gl::types::GLenum,
    pub gen_indices: fn(&mut Block, u32),
}

pub const QUAD: Primitive = Primitive {
    vert_count: 4,
    index_count: 6,
    gl_prim: gl::TRIANGLES,
    gen_indices: |elements, location| {
        let offset: usize = location as usize * 6 * mem::size_of::<u32>();
        let index: u32 = location * 4;

        elements.set::<u32>(offset + 0,  3 + index)
            .expect(&format!("not enough space: failed on offset {}", offset + 0));
        elements.set::<u32>(offset + 4,  2 + index)
            .expect(&format!("not enough space: failed on offset {}", offset + 4));
        elements.set::<u32>(offset + 8,  0 + index)
            .expect(&format!("not enough space: failed on offset {}", offset + 8));

        elements.set::<u32>(offset + 12, 0 + index)
            .expect(&format!("not enough space: failed on offset {}", offset + 12));
        elements.set::<u32>(offset + 16, 2 + index)
            .expect(&format!("not enough space: failed on offset {}", offset + 16));
        elements.set::<u32>(offset + 20, 1 + index)
            .expect(&format!("not enough space: failed on offset {}", offset + 20));

    },
};

pub const NONE: Primitive = Primitive {
    vert_count: 0,
    index_count: 0,
    gl_prim: 0,
    gen_indices: |_, _| {
        eprintln!("Attempted index buffering on NONE primitive.");
    },
};

