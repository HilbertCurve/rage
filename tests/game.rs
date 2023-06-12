use rage::prelude::*;

fn start(w: &mut World) -> RageResult {
    let f: Font = Font::from("./assets/fonts/Arialn.ttf", 14.0)?;

    let t: image::DynamicImage = image::DynamicImage::ImageLuma8(image::ImageBuffer::from_vec(512, 512, f.buffer.to_vec()).unwrap());

    t.save("./assets/textures/test.png")?;

    w.new_scene("name")?;
    w.set_scene("name")?;

    Ok(())
}

fn update(_: &mut World) -> RageResult {
    Ok(())
}

#[test]
fn test() -> RageResult {
    World::new()
        .on_start(start)
        .on_update(update)
        .set_config(Config::default())
        .run()
}