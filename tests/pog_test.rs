use rage::prelude::*;

// Outline:
// Scene 0 is the title screen. There will be a button that advances the user to
// the game scene.
// Scene 1 is the game scree. There will be two paddles and a ball going back and
// forth on the screen.
// When the game ends (first to five or smth), scene switches to title.

#[derive(Component)]
struct Score(usize);

fn s_init(world: &mut World) -> RageResult {
    let button_sheet = Spritesheet::from("assets/pog/pog_button.png".to_owned(), 20, 6, 0)?;
    
    let text_sheet = Spritesheet::from("assets/pog/pog_title.png".to_owned(), 20, 12, 0)?;
    
    // Scene 0
    let scene_0 = world.new_scene("scene_0")?;

    let text = scene_0.spawn("title")?;

    text.attach(SpriteRenderer::select(Vec4::ONE, &text_sheet, 0))?;

    text.add(Transform::from(
        vec3(0.0, 40.0, 0.0),
        vec3(20.0*10.0, 12.0*10.0, 1.0),
        Vec3::ZERO
    ))?;

    let button = scene_0.spawn("button")?;

    button.attach(SpriteRenderer::select(Vec4::ONE, &button_sheet, 0))?;

    button.add(Transform::from(
        vec3(0.0, -40.0, 0.0),
        vec3(20.0*10.0, 6.0*10.0, 1.0),
        Vec3::ZERO
    ))?;
    // Scene 1

    world.set_scene("scene_0")?;

    Ok(())
}

fn s_update(_world: &mut World) -> RageResult {
    
    Ok(())
}

#[test]
pub fn pog() -> RageResult {
    // Setup
    let config: Config = Config::default();

    World::new()
        .on_start(s_init)
        .on_update(s_update)
        .set_config(config)
        .run()
}