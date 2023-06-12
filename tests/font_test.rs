extern crate rage;

use std::error::Error;

use image::{ImageBuffer, DynamicImage};
use rage::prelude::{Font};

#[test]
fn font_test() -> Result<(), Box<dyn Error>> {
    let f: Font = Font::from("./assets/fonts/Arialn.ttf", 13.0)?;

    let t: DynamicImage = DynamicImage::ImageLuma8(ImageBuffer::from_vec(512, 512, f.buffer.to_vec()).unwrap());

    t.save("./assets/textures/test.png")?;

    Ok(())
}