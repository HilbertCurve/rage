use image::DynamicImage;

use crate::renderer::renderer::TEX_POOL;
use crate::utils::block::Block;
use crate::utils::error::UnsupportedError;

use std::error;

pub struct Texture {
    pub id: u32,
    pub uvs: [f32; 8],
}

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

// returns copy of created sprite sheet (bc they aren't really mutable anyways)
impl Spritesheet {
    pub fn from(src: String, s_width: u32, s_height: u32, padding: u32) -> Result<Spritesheet, Box<dyn error::Error>> {
        let mut id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            // Repeat image in both directions
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // When stretching the image, pixelate
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            // When shrinking an image, pixelate
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }

        let texture = image::open(&src)?;

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

            TEX_POOL.try_lock()?.push(Spritesheet { id, src: src.clone(), width, height, s_width, s_height, padding });
        }

        Ok(Spritesheet { id, src: src.clone(), width, height, s_width, s_height, padding })
    }

    pub fn get_texture(&self, index: usize) -> Texture {
        // get offset from top
        let from_top = ((index as f32 * self.s_width as f32 / self.width as f32)
                        * (self.s_height as f32 + self.padding as f32)).floor() as u32;
        // get offset from left
        let from_left = (index as u32 * self.s_width + self.padding) % self.width;

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
}

