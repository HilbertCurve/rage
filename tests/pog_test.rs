use rage::prelude::*;

fn s_init(world: &mut World) -> RageResult {
    let spritesheet: Spritesheet = Spritesheet::from(String::from("./assets/textures/test.png"), 8, 8, 0)?;

    ////////////////////////////
    // Scene 0                //
    ////////////////////////////
    let r_player_0: SpriteRenderer = SpriteRenderer::slice(
        vec4(1.0, 1.0, 1.0, 1.0),
        &spritesheet, 0, 0);
    let t_player_0: Transform = Transform::from(vec3(0.0, 0.0, 0.0), vec3(100.0, 100.0, 1.0), Vec3::ZERO);
    let r_side_0: SpriteRenderer = SpriteRenderer::slice(
        vec4(1.0, 1.0, 1.0, 1.0),
        &spritesheet, 0, 0);
    let t_side_0: Transform = Transform::from(vec3(100.0, 0.0, 0.0), vec3(100.0, 100.0, 1.0), Vec3::ZERO);

    let scene = world.new_scene("main")?;

    let e_ref_0 = scene.spawn("center")?;
    e_ref_0.attach(r_player_0)?;
    e_ref_0.add(t_player_0)?;
    let e_side_0 = scene.spawn("right")?;
    e_side_0.attach(r_side_0)?;
    e_side_0.add(t_side_0)?;

    ////////////////////////////
    // Scene 1                //
    ////////////////////////////
    let r_player_1: SpriteRenderer = SpriteRenderer::slice(
        vec4(1.0, 1.0, 1.0, 1.0),
        &spritesheet, 2, 2);
    let t_player_1: Transform = Transform::from(vec3(0.0, 0.0, 0.0), vec3(100.0, 100.0, 1.0), Vec3::ZERO);

    let scene = world.new_scene("next")?;

    let e_ref_1 = scene.spawn("center")?;
    e_ref_1.attach(r_player_1)?;
    e_ref_1.add(t_player_1)?;

    Ok(())
}
#[allow(dead_code)]
fn s_update(world: &mut World) -> RageResult {
    world.set_scene("main")?;
    if keyboard::is_pressed(glfw::Key::Space) {
        world.set_scene("next")?;
    }
    Ok(())
}

//#[test]
#[allow(dead_code)]
pub fn pog() -> RageResult {
    // Setup
    let config: Config = Config::default();

    World::new()
        .on_start(s_init)
        .on_update(s_update)
        .set_config(config)
        .run()
}