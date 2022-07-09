extern crate gl;

pub struct Primitive {
    pub vert_count: u32,
    pub index_count: u32,
    pub gl_prim: gl::types::GLenum,
    pub gen_indices: fn(&mut [u32], u32),
}

pub const QUAD: Primitive = Primitive {
    vert_count: 4,
    index_count: 6,
    gl_prim: gl::TRIANGLES,
    gen_indices: |elements, location| {
        let offset: usize = location as usize * 6;
        let index: u32 = location * 4;

        elements[offset + 0] = 3 + index;
        elements[offset + 1] = 2 + index;
        elements[offset + 2] = 0 + index;

        elements[offset + 3] = 0 + index;
        elements[offset + 4] = 2 + index;
        elements[offset + 5] = 1 + index;

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

