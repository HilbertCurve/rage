extern crate gl;

use crate::renderer::shader::Shader;

use std::mem;

static mut VAO: u32 = 0;

const VB: [f32; 42] = [
    0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0,
    0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0,
    0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0,
];
static mut VBO: u32 = 0;

const IB: [u32; 6] = [
    0, 1, 2,
    3, 4, 5,
];
static mut IBO: u32 = 0;

static mut DEFAULT_SHADER: Shader = Shader::new_uninit();

const SOF: usize = mem::size_of::<f32>();

pub fn start() {
    unsafe {
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        gl::GenBuffers(1, &mut IBO);

        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (VB.len() * mem::size_of::<f32>()) as isize,
                       VB.as_ptr().cast(),
                       gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, IBO);
        // actual unsafe
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (IB.len() * mem::size_of::<u32>()) as isize,
                       IB.as_ptr().cast(),
                       gl::STATIC_DRAW);

        DEFAULT_SHADER = Shader::new("assets/shaders/default.vert", "assets/shaders/default.frag");

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, 0, 7 * SOF as i32, (0 * SOF) as * const _);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 4, gl::FLOAT, 0, 7 * SOF as i32, (3 * SOF) as * const _);

        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }
}

pub fn update() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);

        gl::BufferData(gl::ARRAY_BUFFER,
                       (VB.len() * mem::size_of::<f32>()) as isize,
                       VB.as_ptr().cast(),
                       gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, IBO);

        // shader stuff
        DEFAULT_SHADER.attach();

        // vertex attrib pointers
        gl::BindVertexArray(VAO);
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);

        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

        gl::DisableVertexAttribArray(0);
        gl::DisableVertexAttribArray(1);
        gl::BindVertexArray(0);

        DEFAULT_SHADER.detach();
    }
}

