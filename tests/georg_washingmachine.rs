use rage::prelude::*;

const START_LIVES: usize = 3;
static mut LIVES: usize = START_LIVES;
static mut KILLS: usize = 0;

pub fn is_inside(t: Transform, v: Vec2) -> bool {
    // check x:
    t.pos.x - t.whd.x / 2.0 <= v.x && v.x <= t.pos.x + t.whd.x / 2.0 &&
    // check y:
    t.pos.y - t.whd.y / 2.0 <= v.y && v.y <= t.pos.y + t.whd.y / 2.0
}

/// checks if two transforms are colliding
/// NOTE: only works if t0 and t1 are AABBs, extended support
/// to be added later
pub fn colliding(t0: &Transform, t1: &Transform) -> bool {
    // check if ends of t0 and t1 overlap
    // for x:
    t0.pos.x - t0.whd.x.abs() / 2.0 <= t1.pos.x + t1.whd.x.abs() &&
    t0.pos.x + t0.whd.x.abs() / 2.0 >= t1.pos.x - t1.whd.x.abs() &&
    // for y:
    t0.pos.y - t0.whd.y.abs() / 2.0 <= t1.pos.y + t1.whd.y.abs() &&
    t0.pos.y + t0.whd.y.abs() / 2.0 >= t1.pos.y - t1.whd.y.abs()
}

#[derive(Component)]
#[allow(dead_code)]
struct Velocity {
    pub x: f32,
    pub y: f32,
}
#[deny(dead_code)]

#[derive(Component)]
struct Enemy {
    x_motion: f32,
}

impl Enemy {
    pub fn new(x_motion: f32) -> Enemy {
        Enemy { x_motion }
    }
}

impl DynComponent for Enemy {
    unsafe fn start(&mut self, _parent: *mut Entity) -> Result<(), ComponentError> {
        Ok(())
    }
    unsafe fn update(&mut self, dt: f64, parent: *mut Entity) -> Result<(), ComponentError> {
        let t: &mut Transform = (&mut *parent).get_mut::<Transform>()?;
        t.pos.x += self.x_motion * dt as f32;
        // if at edge, bounce
        if (t.pos.x <= -300.0 && self.x_motion < 0.0) ||
           (t.pos.x >= 300.0 && self.x_motion > 0.0) {
            self.x_motion *= -1.0;
        }

        Ok(())
    }
    unsafe fn stop(&mut self, _parent: *mut Entity) -> Result<(), ComponentError> {
        Ok(())
    }
}

#[derive(Component)]
struct Spawner {
    inc: usize,
    alive: Vec<usize>,
}

impl Spawner {
    pub fn new() -> Spawner {
        Spawner { inc: 0, alive: vec![] }
    }
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
        is_inside(self.trans_cache, rage::utils::mouse_pos::mouse_pos()) && mouse::is_pressed(glfw::MouseButton::Button1)
    }
}

impl DynComponent for Button {
    unsafe fn start(&mut self, _parent: *mut Entity) -> Result<(), ComponentError> {
        Ok(())
    }
    unsafe fn update(&mut self, _dt: f64, parent: *mut Entity) -> Result<(), ComponentError> {
        self.trans_cache = (&*parent).get::<Transform>()?.clone();

        Ok(())
    }
    unsafe fn stop(&mut self, _parent: *mut Entity) -> Result<(), ComponentError> {
        Ok(())
    }
}

lazy_static::lazy_static! { pub static ref STATES: [State; 3] = [
    State::from("grounded", 
        |entity, dt| {
            if keyboard::is_pressed(glfw::Key::Left) {
                entity.get_mut::<Transform>()?.pos.x -= 200.0 * dt as f32;
            }
            if keyboard::is_pressed(glfw::Key::Right) {
                entity.get_mut::<Transform>()?.pos.x += 200.0 * dt as f32;
            }
            if keyboard::is_pressed(glfw::Key::Space) {
                entity.get_mut::<Velocity>()?.y = 700.0;
                entity.get_mut::<StateMachine>()?.change_state("jumping")?;
            }
            Ok(())
        },
    ),
    State::from("jumping",
        |entity, dt| {
            if keyboard::is_pressed(glfw::Key::Left) {
                entity.get_mut::<Transform>()?.pos.x -= 200.0 * dt as f32;
            }
            if keyboard::is_pressed(glfw::Key::Right) {
                entity.get_mut::<Transform>()?.pos.x += 200.0 * dt as f32;
            }

            entity.get_mut::<Velocity>()?.y -= 1300.0 * dt as f32;
            
            let v = entity.get::<Velocity>()?.y.clone();
            entity.get_mut::<Transform>()?.pos.y += v * dt as f32;

            if entity.get_mut::<Transform>()?.pos.y <= -250.0 {
                entity.get_mut::<Transform>()?.pos.y = -250.0;
                entity.get_mut::<Velocity>()?.y = 0.0;
                entity.get_mut::<StateMachine>()?.change_state("grounded")?;
            }

            Ok(())
        },
    ),
    State::from("dead",
        |entity, dt| {
            // state is manually changed here in the game scene, and manually changed out
            // when the game starts
            entity.get_mut::<Transform>()?.pos.y -= 200.0 * dt as f32;

            Ok(())
        },
    ),
];
}

fn start(world: &mut World) -> RageResult {
    let ui: Spritesheet = Spritesheet::from("./assets/textures/ui.png", 256, 64, 0)?;
    let georg: Spritesheet = Spritesheet::from("./assets/textures/georg1.png", 225, 225, 0)?;
    let soldier: Spritesheet = Spritesheet::from("./assets/textures/soldier.png", 338, 511, 0)?;
    
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

    let defeatedbar = s_status.spawn("defeatedbar")?;
    defeatedbar.attach(SpriteRenderer::from(vec4(0.0, 0.0, 1.0, 1.0)))?;
    defeatedbar.add(Transform::from(
        vec3(0.0, 225.0, 0.0),
        vec3(50.0 * unsafe { KILLS } as f32, 50.0, 0.0),
        Vec3::ZERO,
    ))?;

    let lives = s_status.spawn("lives")?;
    lives.attach(SpriteRenderer::select(Vec4::ONE, &ui, 12))?;
    lives.add(Transform::from(
        vec3(0.0, -100.0, 0.0),
        vec3(400.0, 100.0, 0.0),
        Vec3::ZERO,
    ))?;

    let lifebar = s_status.spawn("lifebar")?;
    lifebar.attach(SpriteRenderer::from(vec4(0.0, 0.0, 1.0, 1.0)))?;
    lifebar.add(Transform::from(
        vec3(0.0, -175.0, 0.0),
        vec3(50.0 * unsafe { LIVES } as f32, 50.0, 0.0),
        Vec3::ZERO,
    ))?;

    ////////////////
    // Game Scene //
    ////////////////
    let s_game = world.new_scene("game")?;

    let player = s_game.spawn("player")?;
    player.attach(SpriteRenderer::select(Vec4::ONE, &georg, 0))?;
    player.attach(StateMachine::from(STATES.to_vec()))?;
    player.add(Transform::from(
        vec3(-300.0, -250.0, 0.0),
        vec3(-200.0, 200.0, 0.0),
        Vec3::ZERO,
    ))?;
    player.add(Velocity { x: 0.0, y: 0.0 })?;

    let spawner = s_game.spawn("spawner")?;
    spawner.add(Spawner::new())?;

    /////////////////
    // Death Scene //
    /////////////////
    let s_death = world.new_scene("death")?;

    let dead0 = s_death.spawn("dead0")?;
    dead0.attach(SpriteRenderer::select(Vec4::ONE, &ui, 8))?;
    dead0.add(Transform::from(
        vec3(-200.0, 300.0, 0.0),
        vec3(400.0, 100.0, 0.0),
        Vec3::ZERO,
    ))?;

    let dead1 = s_death.spawn("dead1")?;
    dead1.attach(SpriteRenderer::select(Vec4::ONE, &ui, 9))?;
    dead1.add(Transform::from(
        vec3(200.0, 300.0, 0.0),
        vec3(400.0, 100.0, 0.0),
        Vec3::ZERO,
    ))?;

    let button = s_death.spawn("button")?;
    button.attach(SpriteRenderer::select(Vec4::ONE, &ui, 7))?;
    button.attach(Button::new())?;
    button.add(Transform::from(
        vec3(0.0, -100.0, 0.0),
        vec3(400.0, 100.0, 0.0),
        Vec3::ZERO,
    ))?;

    ///////////
    // Misc. //
    ///////////
    world.set_scene("start")?;

    world.push_timer("timer");
    world.push_timer("game_time");
    world.add_asset("soldier_sheet", soldier)?;

    Ok(())
}

fn start_update(world: &mut World) -> RageResult {
    if world.get_scene("start")?.get("play")?.get::<Button>()?.is_pressed() {
        world.reset_timer("timer")?;
        world.set_scene("status")?;
    }
    if world.get_scene("start")?.get("quit")?.get::<Button>()?.is_pressed() {
        world.stop();
    }
    Ok(())
}

fn status_update(world: &mut World) -> RageResult {
    // after four seconds of the timer, move on to game scene
    if world.get_timer("timer")? >= 4.0 {
        world.reset_timer("timer")?;
        world.reset_timer("game_time")?;
        world.set_scene("game")?;
    }
    Ok(())
}

fn game_update(world: &mut World) -> RageResult {
    // if we're already dead, play death animation instead, changing to
    // status scene after 5 seconds after resetting game scene and updating
    // status scene
    if world
        .get_scene("game")?
        .get("player")?
        .get::<StateMachine>()?
        .current_state()
        .name() == "dead" /* what a mouthful */ {
        if world.get_timer("game_time")? >= 5.0 {
            if unsafe { LIVES == 0 } {
                world.set_scene("death")?;
                unsafe {
                    LIVES = START_LIVES;
                    KILLS = 0;
                }
            } else {
                world.set_scene("status")?;
            }
            world.reset_timer("game_time")?;
            world.reset_timer("timer")?;

            // reset positions:
            let game = world.get_scene_mut("game")?;
            let player = game.get_mut("player")?;
            player.get_mut::<StateMachine>()?.change_state("grounded")?;
            player.get_mut::<Transform>()?.pos = vec3(-300.0, -250.0, 0.0);

            // despawn all spawned enemies
            let spawned = game.get("spawner")?.get::<Spawner>()?.alive.clone();

            for i in spawned {
                game.despawn(&format!("enemy{i}"))?;
                game.get_mut("spawner")?.get_mut::<Spawner>()?.alive = vec![];
            }

            // reset status screen
            let status = world.get_scene_mut("status")?;
            status.get_mut("defeatedbar")?.get_mut::<Transform>()?.whd.x = 50.0 * unsafe { KILLS } as f32;
            status.get_mut("lifebar")?.get_mut::<Transform>()?.whd.x = 50.0 * unsafe { LIVES } as f32;
        }
        return Ok(())
    }

    // spawn new enemy
    if world.get_timer("timer")? >= 3.0 {
        world.reset_timer("timer")?;

        let soldier = world.get_asset::<Spritesheet>("soldier_sheet")?.clone();
        let time = world.get_timer("game_time")?;

        let s = world.get_scene_mut("game")?;
        let index = s.get_mut("spawner")?.get::<Spawner>()?.inc.clone();

        let enemy = s.spawn(&format!("enemy{index}"))?;

        enemy.attach(SpriteRenderer::select(Vec4::ONE, &soldier, 0))?;
        enemy.add(Transform::from(
            vec3(400.0, -300.0, 0.0),
            vec3((338.0 / 511.0) * 100.0, 100.0, 0.0),
            Vec3::ZERO,
        ))?;
        enemy.add(Enemy::new(-10.0 * time as f32))?;

        let spawner = s.get_mut("spawner")?.get_mut::<Spawner>()?;
        spawner.alive.push(index);
        spawner.inc += 1;
    }

    let game = world.get_scene_mut("game")?;

    // check collisions on each enemy
    let spawned = game.get("spawner")?.get::<Spawner>()?.alive.clone();

    let mut dead = false;

    for i in spawned.clone() {
        if colliding(
            game.get("player")?.get::<Transform>()?,
            game.get(&format!("enemy{i}"))?.get::<Transform>()?
        ) {
            // if we're falling (jumping on enemy)
            if game.get("player")?.get::<Velocity>()?.y < 0.0 {
                game.despawn(&format!("enemy{i}"))?;
                // remove element of item i
                for j in 0..spawned.len() {
                    if game.get_mut("spawner")?.get_mut::<Spawner>()?.alive[j] == i {
                        game.get_mut("spawner")?.get_mut::<Spawner>()?.alive.remove(j);
                        break;
                    }
                }
                game.get_mut("player")?.get_mut::<Velocity>()?.y = 200.0;
                game.get_mut("player")?.get_mut::<Transform>()?.pos.y += 20.0;
                unsafe { KILLS += 1 };
            } else {
                game.get_mut("player")?.get_mut::<StateMachine>()?.change_state("dead")?;
                dead = true;
                unsafe { LIVES -= 1 };
            }
            // only necessary to interract with one spawned enemy
            break;
        }
    }

    if dead {
        world.reset_timer("game_time")?;
    }

    Ok(())
}

fn death_update(world: &mut World) -> RageResult {
    if world.get_scene("death")?.get("button")?.get::<Button>()?.is_pressed() {
        world.set_scene("start")?;
    }

    Ok(())
}

fn update(world: &mut World) -> RageResult {
    // update new DynComponent
    // TODO: add plugins to `World`
    let dt = world.dt().clone();
    world.current_scene()?.update::<Button>(dt)?;
    world.current_scene()?.update::<Enemy>(dt)?;

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
