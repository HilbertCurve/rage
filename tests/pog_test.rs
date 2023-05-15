use rage::{prelude::*, utils::mouse_pos};

// Outline:
// Scene 0 is the title screen. There will be a button that advances the user to
// the game scene.
// Scene 1 is the game scree. There will be two paddles and a ball going back and
// forth on the screen.
// When the game ends (first to five or smth), scene switches to title.

#[derive(Component)]
struct Score(usize);
#[derive(Component, Copy, Clone)]
struct Velocity(f32, f32);

pub fn is_inside(t: Transform, v: Vec2) -> bool {
    // check x:
    t.pos.x - t.whd.x / 2.0 <= v.x && v.x <= t.pos.x + t.whd.x / 2.0 &&
    // check y:
    t.pos.y - t.whd.y / 2.0 <= v.y && v.y <= t.pos.y + t.whd.y / 2.0
}

pub fn t_is_overlap(t0: Transform, t1: Transform) -> bool {
    // increase width-height of transform by 1/2 width-height of other one:
    let t_new: Transform = Transform::from(
        t0.pos,
        t0.whd + t1.whd / 2.0,
        Vec3::ZERO,
    );
    is_inside(t_new, (t1.pos.x, t1.pos.y).into())
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

    let ball = scene_1.spawn("ball")?;
    ball.attach(SpriteRenderer::from(Vec4::from((0.3, 0.9, 0.1, 1.0))))?;
    ball.add(Transform::from(
        Vec3::ZERO,
        Vec3::ONE * 25.0,
        Vec3::ZERO,
    ))?;
    ball.add(Velocity(150.0, 200.0))?;

    world.set_scene("scene_0")?;

    Ok(())
}

fn s0_update(world: &mut World) -> RageResult {
    let button = world
        .get_scene_mut("scene_0")?
        .get_mut("button")?;

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
    // dt up here because rust doesn't like immutable borrows after mutable ones
    let dt = world.dt().clone() as f32;
    
    let scene_1 = world.get_scene_mut("scene_1")?;

    // paddle_top behavior
    let paddle_top = scene_1.get_mut("paddle_top")?;
    if keyboard::is_pressed(glfw::Key::Right) {
        paddle_top.get_mut::<Transform>()?.pos.x += 10.0;
    }
    if keyboard::is_pressed(glfw::Key::Left) {
        paddle_top.get_mut::<Transform>()?.pos.x -= 10.0;
    }
    // paddle_bottom behavior
    let paddle_bottom = scene_1.get_mut("paddle_bottom")?;
    if keyboard::is_pressed(glfw::Key::D) {
        paddle_bottom.get_mut::<Transform>()?.pos.x += 10.0;
    }
    if keyboard::is_pressed(glfw::Key::A) {
        paddle_bottom.get_mut::<Transform>()?.pos.x -= 10.0;
    }

    let paddle_bottom_transform = 
        scene_1
        .get("paddle_bottom")?
        .get::<Transform>()?
        .clone();
    let paddle_top_transform = 
        scene_1
        .get("paddle_top")?
        .get::<Transform>()?
        .clone();

    // ball behavior
    let ball = scene_1.get_mut("ball")?;
    let mut vel = ball.get::<Velocity>()?.clone();
    let mut new_pos = ball.get::<Transform>()?.pos + Vec3::from((vel.0, vel.1, 0.0f32)) * dt;

    // bounce off walls
    if new_pos.x > 600.0 || new_pos.x < -600.0 {
        ball.get_mut::<Velocity>()?.0 *= -1.0;
        vel.0 *= -1.0;
        new_pos += Vec3::from((vel.0, vel.1, 0.0f32)) * dt;
    }
    // reset pos if too far up
    if new_pos.y > 400.0 || new_pos.y < -400.0 {
        ball.get_mut::<Transform>()?.pos = Vec3::ZERO;
        new_pos = Vec3::from((0.0, 0.0, 0.0f32));
    }

    let new_trans = Transform::from(
        new_pos,
        ball.get::<Transform>()?.whd,
        Vec3::ZERO,
    );

    // if touching top paddle, bounce
    if t_is_overlap(
        new_trans.clone(),
        paddle_top_transform
    ) && ball.get::<Velocity>()?.1 > 0.0 {
        ball.get_mut::<Velocity>()?.1 *= -1.0;
        vel.1 *= -1.0;
        new_pos += Vec3::from((vel.0, vel.1, 0.0f32)) * dt;
    }
    // if touching bottom paddle, bounce
    if t_is_overlap(
        new_trans.clone(),
        paddle_bottom_transform
    ) && ball.get::<Velocity>()?.1 < 0.0 {
        ball.get_mut::<Velocity>()?.1 *= -1.0;
        vel.1 *= -1.0;
        new_pos += Vec3::from((vel.0, vel.1, 0.0f32)) * dt;
    }

    ball.get_mut::<Transform>()?.pos = new_pos;

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
