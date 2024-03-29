use image::DynamicImage;

use crate::core::assets::Asset;
use crate::renderer::renderer::TEX_POOL;
use crate::utils::block::Block;
use crate::utils::error::UnsupportedError;

use std::error;

#[derive(Copy, Clone)]
pub struct Texture {
    pub id: u32,
    pub uvs: [f32; 8],
}

#[derive(Clone)]
pub struct Spritesheet {
    /// ID number used with OpenGL
    id: u32,
    /// Source of spritesheet, or empty string if there is none
    src: String,
    /// Total width of spritesheet
    width: u32,
    /// Total height of spritesheet
    height: u32,
    /// Width of a single sprite in this spritesheet
    s_width: u32,
    /// Height of a single sprite in this spritesheet
    s_height: u32,
    /// Width/height between each sprite in this spritesheet
    padding: u32,
}

impl Asset for Spritesheet {
    fn new() -> Self where Self: Sized {
        Spritesheet::empty()
    }
    fn clear(&mut self) -> crate::prelude::RageResult {
        // remove from TEX_POOL
        // TODO: TEX_POOL is a horrible idea now that we have assets
        unsafe {
            let mut t_ref = TEX_POOL.try_lock()?;
            for i in 0..t_ref.len() {
                if self.id == t_ref.get_unchecked(i).id {
                    t_ref.remove(i);
                    return Ok(())
                }
            }
        }
        unreachable!()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
    fn type_str() -> &'static str where Self: Sized {
        "Spritesheet"
    }
}

// returns copy of created sprite sheet (bc they aren't really mutable anyways)
impl Spritesheet {
    pub const fn empty() -> Spritesheet {
        Spritesheet {
            id: 0,
            src: String::new(),
            width: 0,
            height: 0,
            s_width: 0,
            s_height: 0,
            padding: 0,
        }
    }
    pub fn from(src: &str, s_width: u32, s_height: u32, padding: u32) -> Result<Spritesheet, Box<dyn error::Error>> {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::ActiveTexture(gl::TEXTURE0 + id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            // Repeat image in both directions
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // When stretching the image, pixelate
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            // When shrinking an image, pixelate
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }

        let texture = image::open(&src)?.flipv();

        let (pixels, channels, width, height) = match texture {
            DynamicImage::ImageRgb8(img) => (Block::from_vec(img.as_raw()), 3, img.width(), img.height()),
            DynamicImage::ImageRgba8(img) => (Block::from_vec(img.as_raw()), 4, img.width(), img.height()),
            DynamicImage::ImageRgb16(img) => (Block::from_vec(img.as_raw()), 3, img.width(), img.height()),
            DynamicImage::ImageRgba16(img) => (Block::from_vec(img.as_raw()), 4, img.width(), img.height()),
            fmt => return Err(Box::new(
                    UnsupportedError { what: format!("Unsupported image format: {:?}", fmt) }))
        };

        unsafe {
            match channels {
                3 => gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, width as i32, height as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, pixels.as_ptr() as * const _),
                4 => gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width as i32, height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, pixels.as_ptr() as * const _),
                _ => return Err(Box::new(
                    UnsupportedError { what: format!("Unsupported number of channels: {}", channels) }))
            }

            TEX_POOL.try_lock()?.push(Spritesheet { id, src: src.to_string(), width, height, s_width, s_height, padding });
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Ok(Spritesheet { id, src: src.to_string(), width, height, s_width, s_height, padding })
    }

    /// Padding as such: <br>
    /// `- | +`: padding <br>
    /// `.`: image pixels <br>
    /// ```
    /// ...|...|...|...|...|...|
    /// ...|...|...|...|...|...|
    /// ---+---+---+---+---+---+
    /// ...|...|...|...|...|...|
    /// ...|...|...|...|...|...|
    /// ---+---+---+---+---+---+
    /// ...|...|...|...|...|...|
    /// ...|...|...|...|...|...|
    /// ---+---+---+---+---+---+
    /// ```
    pub fn get_texture(&self, index: usize) -> Texture {
        // get offset from top
        let from_top = ((index as f32 * self.s_width as f32 / self.width as f32).floor()
                        * (self.s_height as f32 + self.padding as f32)) as u32;
        // get offset from left
        let from_left = (index as u32 * (self.s_width + self.padding)) % self.width;

        let mut pos1 = (from_left as f32, from_top as f32);
        let mut pos2 = ((from_left + self.s_width) as f32, (from_top + self.s_height) as f32);

        pos1.1 = self.height as f32 - pos1.1;
        pos1.0 /= self.width as f32;
        pos1.1 /= self.height as f32;

        pos2.1 = self.height as f32 - pos2.1;
        pos2.0 /= self.width as f32;
        pos2.1 /= self.height as f32;

        // insert texcoords
        let coords: [f32; 8] = [
            pos2.0, pos1.1,
            pos1.0, pos1.1,  
            pos1.0, pos2.1,
            pos2.0, pos2.1    
        ];

        Texture { id: self.id, uvs: coords }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_source(&self) -> &str {
        &self.src
    }

    pub fn empty_tex() -> Texture {
        Texture {
            id: 0,
            uvs: [0.0;8],
        }
    }

    pub fn as_vec(&self) -> Vec<Texture> {
        let width_count = self.width / (self.s_width + self.padding);
        let height_count = self.height / (self.s_height + self.padding);

        let mut ret: Vec<Texture> = vec![];
        for i in 0..width_count * height_count {
            ret.push(self.get_texture(i as usize))
        }

        ret
    }
}
