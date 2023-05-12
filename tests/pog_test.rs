use rage::{prelude::*, utils::mouse_pos};

// Outline:
// Scene 0 is the title screen. There will be a button that advances the user to
// the game scene.
// Scene 1 is the game scree. There will be two paddles and a ball going back and
// forth on the screen.
// When the game ends (first to five or smth), scene switches to title.

#[derive(Component)]
struct Score(usize);

pub fn is_inside(t: Transform, v: Vec2) -> bool {
    // check x:
    t.pos.x - t.whd.x / 2.0 <= v.x && v.x <= t.pos.x + t.whd.x / 2.0 &&
    // check y:
    t.pos.y - t.whd.y / 2.0 <= v.y && v.y <= t.pos.y + t.whd.y / 2.0
}

fn start(world: &mut World) -> RageResult {
    let button_sheet = Spritesheet::from("assets/pog/pog_button.png".to_owned(), 20, 6, 0)?;
    
    let text_sheet = Spritesheet::from("assets/pog/pog_title.png".to_owned(), 20, 12, 0)?;
    
    // Scene 0
    let scene_0 = world.new_scene("scene_0")?;

    let text = scene_0.spawn("title")?;

    text.attach(SpriteRenderer::select(Vec4::ONE, &text_sheet, 0))?;

    text.add(Transform::from(
        vec3(0.0, 200.0, 0.0),
        vec3(20.0*50.0, 12.0*50.0, 1.0),
        Vec3::ZERO
    ))?;

    let button = scene_0.spawn("button")?;

    button.attach(SpriteRenderer::select(Vec4::ONE, &button_sheet, 0))?;

    button.add(Transform::from(
        vec3(0.0, -200.0, 0.0),
        vec3(20.0*50.0, 6.0*50.0, 1.0),
        Vec3::ZERO
    ))?;
    // Scene 1

    let scene_1 = world.new_scene("scene_1")?;
    let paddle_top = scene_1.spawn("paddle_top")?;
    paddle_top.attach(SpriteRenderer::from(Vec4::ONE))?;
    paddle_top.add(Transform::from(
        vec3(0.0, 400.0, 0.0),
        vec3(100.0, 25.0, 1.0),
        Vec3::ZERO,
    ))?;

    let paddle_bottom = scene_1.spawn("paddle_bottom")?;
    paddle_bottom.attach(SpriteRenderer::from(Vec4::ONE))?;
    paddle_bottom.add(Transform::from(
        vec3(0.0, -400.0, 0.0),
        vec3(100.0, 25.0, 1.0),
        Vec3::ZERO,
    ))?;

    world.set_scene("scene_0")?;

    Ok(())
}

fn s0_update(world: &mut World) -> RageResult {
    let button = world
        .get_scene("scene_0")?
        .get("button")?;

    if is_inside(*button.get::<Transform>()?, mouse_pos::mouse_pos()) {
        button.get_mut::<SpriteRenderer>()?.color = Vec4::ONE / 2.0;
        if mouse::is_pressed(glfw::MouseButton::Button1) {
            world.set_scene("scene_1")?;
        }
    } else {
        button.get_mut::<SpriteRenderer>()?.color = Vec4::ONE;
    }

    Ok(())
}

fn s1_update(world: &mut World) -> RageResult {
    // paddle_top behavior
    let paddle_top = world.get_scene("scene_1")?.get("paddle_top")?;
    if keyboard::is_pressed(glfw::Key::Right) {
        paddle_top.get_mut::<Transform>()?.pos.x += 10.0;
    }
    if keyboard::is_pressed(glfw::Key::Left) {
        paddle_top.get_mut::<Transform>()?.pos.x -= 10.0;
    }
    // paddle_bottom behavior
    let paddle_bottom = world.get_scene("scene_1")?.get("paddle_bottom")?;
    if keyboard::is_pressed(glfw::Key::D) {
        paddle_bottom.get_mut::<Transform>()?.pos.x += 10.0;
    }
    if keyboard::is_pressed(glfw::Key::A) {
        paddle_bottom.get_mut::<Transform>()?.pos.x -= 10.0;
    }
    // ball behavior

    Ok(())
}

fn update(world: &mut World) -> RageResult {
    let name = world.current_scene()?.name();
    if name == "scene_0" {
        s0_update(world)?;
    }
    if name == "scene_1" {
        s1_update(world)?;
    }

    Ok(())
}

#[test]
pub fn pog() -> RageResult {
    // Setup
    let config: Config = Config::default();

    World::new()
        .on_start(start)
        .on_update(update)
        .set_config(config)
        .run()
}
