use rage::prelude::*;

static life_count: usize = 3;
static kill_count: usize = 0;

fn start(world: &mut World) -> RageResult {
    let ui: Spritesheet = Spritesheet::from("./assets/textures/ui.png", 256, 64, 0)?;
    let georg: Spritesheet = Spritesheet::from("./assets/textures/georg1.png", 225, 225, 0)?;

    ////////////////////
    // Starting Scene //
    ////////////////////
    let s_start = world.new_scene("start")?;

    let title0 = s_start.spawn("title0")?;
    title0.attach(SpriteRenderer::select(Vec4::ONE, &ui, 0))?;
    title0.add(Transform::from(
        vec3(-200.0, 300.0, 0.0),
        vec3(400.0, 100.0, 0.0),
        Vec3::ZERO,
    ))?;

    let title1 = s_start.spawn("title1")?;
    title1.attach(SpriteRenderer::select(Vec4::ONE, &ui, 1))?;
    title1.add(Transform::from(
        vec3(200.0, 300.0, 0.0),
        vec3(400.0, 100.0, 0.0),
        Vec3::ZERO,
    ))?;

    let play = s_start.spawn("play")?;
    play.attach(SpriteRenderer::select(Vec4::ONE, &ui, 2))?;
    play.add(Transform::from(
        vec3(200.0, -300.0, 0.0),
        vec3(400.0, 100.0, 0.0),
        Vec3::ZERO,
    ))?;

    let quit = s_start.spawn("quit")?;
    quit.attach(SpriteRenderer::select(Vec4::ONE, &ui, 3))?;
    quit.add(Transform::from(
        vec3(-200.0, -300.0, 0.0),
        vec3(400.0, 100.0, 0.0),
        Vec3::ZERO,
    ))?;

    //////////////////
    // Status Scene //
    //////////////////
    let s_status = world.new_scene("status")?;

    let defeated0 = s_status.spawn("defeated0")?;
    defeated0.attach(SpriteRenderer::select(Vec4::ONE, &ui, 10))?;
    defeated0.add(Transform::from(
        vec3(-200.0, 300.0, 0.0),
        vec3(400.0, 100.0, 0.0),
        Vec3::ZERO,
    ))?;

    let defeated1 = s_status.spawn("defeated1")?;
    defeated1.attach(SpriteRenderer::select(Vec4::ONE, &ui, 11))?;
    defeated1.add(Transform::from(
        vec3(200.0, 300.0, 0.0),
        vec3(400.0, 100.0, 0.0),
        Vec3::ZERO,
    ))?;

    let lives = s_status.spawn("lives")?;
    lives.attach(SpriteRenderer::select(Vec4::ONE, &ui, 12))?;
    lives.add(Transform::from(
        vec3(0.0, -100.0, 0.0),
        vec3(400.0, 100.0, 0.0),
        Vec3::ZERO,
    ))?;

    ////////////////
    // Game Scene //
    ////////////////
    let s_game = world.new_scene("game")?;

    let player = s_game.spawn("player")?;
    player.attach(SpriteRenderer::select(Vec4::ONE, &georg, 0))?;
    player.add(Transform::from(
        vec3(-300.0, -250.0, 0.0),
        vec3(-225.0, 225.0, 0.0),
        Vec3::ZERO,
    ))?;

    world.set_scene("start")?;
    Ok(())
}

fn update(world: &mut World) -> RageResult {
    // debug
    if keyboard::is_pressed(glfw::Key::Num1) {
        world.set_scene("start")?;
    }
    if keyboard::is_pressed(glfw::Key::Num2) {
        world.set_scene("status")?;
    }
    if keyboard::is_pressed(glfw::Key::Num3) {
        world.set_scene("game")?;
    }
    if keyboard::is_pressed(glfw::Key::Num4) {
        world.set_scene("death")?;
    }
    Ok(())
}

#[test]
fn georg_washingmachine() -> RageResult {
    World::new()
        .on_start(start)
        .on_update(update)
        .set_config(Config::default())
        .run()
}

