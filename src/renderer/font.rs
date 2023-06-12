extern crate fontdue;

use std::{fs::File, path::Path, collections::HashMap};

use glam::{UVec2, uvec2};
use image::{ImageBuffer, DynamicImage, GrayImage};

use crate::{prelude::Block, impl_error};

pub struct Font {
    id: u32,
    size: f32,
    glyphs: std::collections::HashMap<char, Glyph>,
    fontdue_back: fontdue::Font,
    pub buffer: Block,
}

struct Glyph {
    pub metrics: fontdue::Metrics,
    pub tex_start: UVec2,
    pub tex_wh: UVec2,
}

impl_error!(FontError);

impl Font {
    pub fn from(path: &str, size: f32) -> Result<Self, FontError> {
        // open file
        let mut f = File::open(Path::new(path)).unwrap();
        // load fontdue backend
        let fontdue_back = fontdue::Font::from_bytes(
            Block::from_file(&mut f).unwrap(),
            fontdue::FontSettings { collection_index: 0, scale: size }
        ).unwrap();

        let mut texture: DynamicImage = DynamicImage::ImageRgb8(ImageBuffer::new(512, 512));

        // iterate through chars, pushing newly generated glyphs into a list
        let mut glyphs: HashMap<char, Glyph> = HashMap::new();
        let mut curr_x: u32 = 0;
        let mut curr_y: u32 = 0;
        for c in 'a'..'Z' {
            // get glyph width, height, data and position info
            let (metrics, buffer) = fontdue_back.rasterize(c, size);
            let tex_wh = uvec2(metrics.width as u32, metrics.height as u32);
            
            let g = GrayImage::from_vec(tex_wh.x, tex_wh.y, buffer.clone()).unwrap();
            g.save("a.png").unwrap();

            image::imageops::overlay(
                &mut texture,
                &ImageBuffer::from_vec(
                    tex_wh.x,
                    tex_wh.y,
                    buffer
                ).unwrap(),
                curr_x as i64, curr_y as i64,
            );

            glyphs.insert(c, Glyph { metrics, tex_start: uvec2(curr_x, curr_y), tex_wh });

            curr_x += tex_wh.x;
            if curr_x >= 512 {
                curr_x = 0;
                curr_y += fontdue_back.horizontal_line_metrics(size).unwrap().new_line_size as u32;
            }
        }

        let mut id = 0;
        unsafe {

            // disable byte alignment stuff
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            // generate new texture handle
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

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RED as i32,
                512, 512,
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                texture.as_bytes().as_ptr() as * const _
            );
        };

        texture.save("./assets/textures/test_internal.png");

        Ok(Font {
            id,
            size,
            glyphs,
            fontdue_back,
            buffer: Block::from(texture.as_bytes()),
        })
    }
}

