extern crate gl;
extern crate glam;

use glam::Mat4;
use std::ffi::CString;
use std::fs;

pub struct Shader {
    pub v_id: u32,
    pub f_id: u32,
    pub p_id: u32,
    pub is_used: bool,
}

impl Shader {
    pub fn new(vert_path: &str, frag_path: &str) -> Shader {
        let mut vert_code = fs::read_to_string(vert_path)
            .expect("Better errors, please!");
        vert_code.push('\0');
        let vert_len = vert_code.len() as i32 + 1;
        let vert_code: CString = CString::from_vec_with_nul(vert_code.into_bytes())
            .expect("invalid vertex code utf8");

        let mut frag_code = fs::read_to_string(frag_path)
            .expect("Better errors, please...?");
        frag_code.push('\0');
        let frag_len = frag_code.len() as i32 + 1;
        let frag_code: CString = CString::from_vec_with_nul(frag_code.into_bytes())
            .expect("invalid fragment code utf8");

        let v_id: u32 = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        let f_id: u32 = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        let p_id: u32;

        let mut result = 0;
        let mut info_len = 0;

        // unsafe :D
        unsafe {
            // compile vertex shader
            gl::ShaderSource(v_id, 1, &vert_code.as_ptr() as * const _, &vert_len);
            gl::CompileShader(v_id);

            // check vertex compilation
            gl::GetShaderiv(v_id, gl::COMPILE_STATUS, &mut result);
            gl::GetShaderiv(v_id, gl::INFO_LOG_LENGTH, &mut info_len);


            if info_len > 0 {
                let mut v_err: Vec<u8> = Vec::with_capacity(info_len as usize + 1);
                v_err.extend([b' '].iter().cycle().take(info_len as usize));
                let c_err: CString = CString::from_vec_unchecked(v_err);

                // actual unsafe
                gl::GetShaderInfoLog(
                    v_id,
                    info_len,
                    0 as * mut _,
                    c_err.as_ptr() as * mut _);
                eprintln!("{:?}", c_err);
            }

            // compile fragment shader
            gl::ShaderSource(f_id, 1, &frag_code.as_ptr() as * const _, &frag_len);
            gl::CompileShader(f_id);

            // check fragment compilation
            gl::GetShaderiv(f_id, gl::COMPILE_STATUS, &mut result);
            gl::GetShaderiv(f_id, gl::INFO_LOG_LENGTH, &mut info_len);

            if info_len > 0 {
                eprintln!("aaa");
                let mut f_err: Vec<u8> = Vec::with_capacity(info_len as usize + 1);
                f_err.extend([b' '].iter().cycle().take(info_len as usize));
                let c_err: CString = CString::from_vec_unchecked(f_err);

                // actual unsafe
                gl::GetShaderInfoLog(
                    f_id,
                    info_len,
                    0 as * mut _,
                    c_err.as_ptr() as * mut _);
                eprintln!("{:?}", c_err);
            }

            // create and link shader program
            p_id = gl::CreateProgram();

            gl::AttachShader(p_id, v_id);
            gl::AttachShader(p_id, f_id);

            gl::LinkProgram(p_id);

            // check linking
            gl::GetShaderiv(p_id, gl::LINK_STATUS, &mut result);
            gl::GetShaderiv(p_id, gl::INFO_LOG_LENGTH, &mut info_len);

            if info_len > 0 {
                let mut p_err: Vec<u8> = Vec::with_capacity(info_len as usize + 1);
                p_err.extend([b' '].iter().cycle().take(info_len as usize));
                let c_err: CString = CString::from_vec_unchecked(p_err);

                // actual unsafe
                gl::GetShaderInfoLog(
                    p_id,
                    info_len,
                    0 as * mut _,
                    c_err.as_ptr() as * mut _);
                eprintln!("{:?}", c_err);
            }
        }

        Shader { v_id, f_id, p_id, is_used: false }
    }

    pub const fn new_uninit() -> Shader {
        Shader { v_id: 0, f_id: 0, p_id: 0, is_used: false }
    }

    pub fn attach(&mut self) {
        unsafe { gl::UseProgram(self.p_id); }
        self.is_used = true;
    }

    pub fn detach(&mut self) {
        unsafe { gl::UseProgram(self.p_id); }
        self.is_used = true;
    }

    pub fn set_uniform_mat4(&mut self, name: &str, mat: Mat4) {
        let var_loc = unsafe {
            let c_str = CString::new(name).expect("");
            gl::GetUniformLocation(self.p_id, c_str.as_ptr())
        };

        if !self.is_used {
            self.attach();
        }

        unsafe {
            gl::UniformMatrix4fv(var_loc, 1, gl::FALSE, mat.to_cols_array().as_ptr());
        }
    }

    pub fn set_uniform_float(&mut self, name: &str, val: f32) {
        let var_loc = unsafe {
            let c_str = CString::new(name).expect("");
            gl::GetUniformLocation(self.p_id, c_str.as_ptr())
        };

        if !self.is_used {
            self.attach();
        }

        unsafe {
            gl::Uniform1f(var_loc, val);
        }
    }
}

