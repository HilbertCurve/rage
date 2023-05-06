pub use rage::prelude::*;

fn audio_test_start(world: &mut World) -> RageResult {
    let scene = world.new_scene("main")?;

    let entity = scene.spawn("source")?;
    entity.attach(AudioSource::new()?)?;

    entity.get_mut::<AudioSource>()?.play("assets/audio/test1.wav".to_string())?;

    world.set_scene("main")?;

    Ok(())
}

fn audio_test_update(world: &mut World) -> RageResult {
    let entity = world.get_scene("main")?.get("source")?;
    if keyboard::is_pressed(glfw::Key::Space) {
        entity.get_mut::<AudioSource>()?.pause()?;
    } else {
        entity.get_mut::<AudioSource>()?.resume()?;
    }
    Ok(())
}

#[test]
fn audio_test() -> RageResult {
    World::new()
        .on_start(audio_test_start)
        .on_update(audio_test_update)
        .set_config(Config::default())
        .run()
}