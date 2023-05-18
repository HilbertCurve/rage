use rage::prelude::*;

static mut LIVES: usize = 3;
static mut KILLS: usize = 0;

pub fn is_inside(t: Transform, v: Vec2) -> bool {
    // check x:
    t.pos.x - t.whd.x / 2.0 <= v.x && v.x <= t.pos.x + t.whd.x / 2.0 &&
    // check y:
    t.pos.y - t.whd.y / 2.0 <= v.y && v.y <= t.pos.y + t.whd.y / 2.0
}

#[derive(Component)]
struct Button {
    trans_cache: Transform,
}

impl Button {
    pub fn new() -> Button {
        Button {
            trans_cache: Transform::zero(),
        }
    }
    pub fn is_pressed(&self) -> bool {
        is_inside(self.trans_cache, mouse_pos::mouse_pos()) && mouse::is_pressed(glfw::MouseButton::Button1)
    }
}

impl DynComponent for Button {
    unsafe fn start(&mut self, parent: *mut Entity) -> Result<(), ComponentError> {
        Ok(())
    }
    unsafe fn update(&mut self, dt: f64, parent: *mut Entity) -> Result<(), ComponentError> {
        self.trans_cache = (&*parent).get::<Transform>()?.clone();

        Ok(())
    }
    unsafe fn stop(&mut self, parent: *mut Entity) -> Result<(), ComponentError> {
        Ok(())
    }
}

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
    play.attach(Button::new())?;

    let quit = s_start.spawn("quit")?;
    quit.attach(SpriteRenderer::select(Vec4::ONE, &ui, 3))?;
    quit.add(Transform::from(
        vec3(-200.0, -300.0, 0.0),
        vec3(400.0, 100.0, 0.0),
        Vec3::ZERO,
    ))?;
    quit.attach(Button::new())?;

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

    for i in 0..unsafe{LIVES} {
        let e = s_status.spawn(&format!("life{}", i))?;
        e.attach(SpriteRenderer::from(Vec4::ONE))?;
        e.add(Transform::from(
            vec3(-50.0 + 50.0 * i as f32, -150.0, 0.0),
            vec3(50.0, 50.0, 0.0),
            Vec3::ZERO,
        ))?;
    }

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

    /////////////////
    // Death Scene //
    /////////////////
    let s_death = world.new_scene("death")?;

    let title0 = s_death.spawn("title0")?;
    title0.attach(SpriteRenderer::select(Vec4::ONE, &ui, 8))?;
    title0.add()?;

    ///////////
    // Misc. //
    ///////////
    world.set_scene("start")?;

    world.push_timer("timer")?;

    Ok(())
}

fn start_update(world: &mut World) -> RageResult {
    if world.get_scene("start")?.get("play")?.get::<Button>()?.is_pressed() {
        world.set_scene("game")?;
    }
    Ok(())
}

fn status_update(world: &mut World) -> RageResult {
    Ok(())
}

fn game_update(world: &mut World) -> RageResult {
    Ok(())
}

fn death_update(world: &mut World) -> RageResult {
    Ok(())
}

fn update(world: &mut World) -> RageResult {
    // update new DynComponent
    // TODO: add plugins to `World`
    let dt = world.dt().clone();
    world.current_scene()?.update::<Button>(dt)?;

    // update each scene, depending on the current scene
    let name = world.current_scene()?.name();
    if name == "start" {
        start_update(world)
    } else if name == "status" {
        status_update(world)
    } else if name == "game" {
        game_update(world)
    } else if name == "death" {
        death_update(world)
    } else {
        Ok(())
    }


    // debug
    /*
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
    */
}

#[test]
fn georg_washingmachine() -> RageResult {
    World::new()
        .on_start(start)
        .on_update(update)
        .set_config(Config::default())
        .run()
}

