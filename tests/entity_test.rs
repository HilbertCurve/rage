use std::sync::Mutex;

use rage::{prelude::*, utils::mouse_pos};

static mut HEARTS: Mutex<Spritesheet> = Mutex::new(Spritesheet::empty());

fn hearts_start(world: &mut World) -> RageResult {
    world.add_asset("hearts", Spritesheet::from("assets/textures/hearts.png", 16, 16, 0)?)?;

    let scene = world.new_scene("main")?;

    let entity = scene.spawn("0")?;
    //entity.attach(SpriteRenderer::slice(Vec4::ONE, world.get_asset("hearts".to_string())?, 0, 4))?;
    entity.add(Transform::from(
        Vec3::ZERO,
        Vec3::from((125.0, 125.0, 50.0)),
        Vec3::from((0.0, 0.0, 0.0)),
    ))?;

    world.set_scene("main")?;
    world.push_timer("main");
    world.push_timer("names");

    Ok(())
}

fn hearts_update(world: &mut World) -> RageResult {

    let h_ref = unsafe { HEARTS.get_mut()? };

    // heart animation using a timer
    if world.get_timer("main")? >= 0.02 && mouse::is_pressed(glfw::MouseButton::Button1) {
        world
            .get_scene_mut("main")?
            .get_mut("0")?
            .get_mut::<SpriteRenderer>()?
            .next_wrap();

        // spawn new entity
        let frame = world.get_scene("main")?.get("0")?.get::<SpriteRenderer>()?.curr_frame();
        let timer = world.get_timer("names")?.clone();
        let new_entity = world.get_scene_mut("main")?.spawn(&format!("{}", timer))?;
        new_entity.attach(SpriteRenderer::select(Vec4::ONE, h_ref, frame))?;
        new_entity.add(Transform::from(
            Vec3::from((mouse_pos::mouse_pos(), 0.0)),
            Vec3::from((125.0, 125.0, 50.0)),
            Vec3::ZERO,
        ))?;

        world.reset_timer("main")?;
    }
    world.get_scene_mut("main")?["0"].get_mut::<Transform>()?.pos = Vec3::from((mouse_pos::mouse_pos(), 0.0));

    Ok(())
}

#[test]
pub fn hearts_test() -> RageResult {
    World::new()
        .on_start(hearts_start)
        .on_update(hearts_update)
        .set_config(Config::default())
        .run()
}

const GRAPH: [u8;42] = [
    0, 1, 1, 0, 1, 1, 0,
    1, 2, 2, 1, 2, 2, 1,
    1, 2, 2, 2, 2, 2, 1,
    0, 1, 2, 2, 2, 1, 0,
    0, 0, 1, 2, 1, 0, 0,
    0, 0, 0, 1, 0, 0, 0,
];

fn many_entity_init(world: &mut World) -> RageResult {
    let scene = world.new_scene("main")?;
    for i in 0..6 {
        for j in 0..7 {
            if GRAPH[i*7+j] != 0 {
                let mut col: Vec3 = Vec3::ZERO;
                if GRAPH[i*7+j] == 2 {
                    col = Vec3::new(1.0, 0.1, 0.45); 
                } else if GRAPH[i*7+j] == 1 {
                    col = Vec3::new(1.0, 0.1, 0.2);
                }
                let r_entity = SpriteRenderer::from(col.extend(1.0));
                let t_entity = Transform::from(
                    vec3(j as f32 * 50.0 - 50.0 * 3.5, -(i as f32 * 50.0) + (50.0 * 3.0), 0.0),
                    vec3(50.0, 50.0, 0.0),
                    Vec3::ZERO,
                );
                let entity = scene.spawn(&format!("{}", i*10+j))?;
                entity.add(t_entity)?;
                entity.attach(r_entity)?;
                entity.get::<Transform>()?;
                scene.get(&format!("{}", i*10+j))?.get::<Transform>()?;
            }
        }
    }

    world.set_scene("main")?;

    Ok(())
}

fn many_entity_update(world: &mut World) -> RageResult {
    println!("Current fps: {}", world.fps());
    Ok(())
}

#[test]
pub fn many_entity_test() -> RageResult {
    World::new()
        .on_start(many_entity_init)
        .on_update(many_entity_update)
        .set_config(Config::default())
        .run()
}

#[test]
fn entity_test_attach_detach() -> RageResult {
    let mut entity: Entity = Entity::new("test".to_owned());
    let sprite_renderer: SpriteRenderer = SpriteRenderer::from(Vec4::ONE);
    entity.attach(sprite_renderer)?;
    entity.detach::<SpriteRenderer>()?;

    let transform: Transform = Transform::zero();

    let sprite_renderer: SpriteRenderer = SpriteRenderer::from(Vec4::ONE);
    entity.attach(sprite_renderer)?;
    entity.add(transform)?;

    entity.detach::<SpriteRenderer>()?;
    entity.remove::<Transform>()?;

    let sprite_renderer: SpriteRenderer = SpriteRenderer::from(Vec4::ONE);
    entity.attach(sprite_renderer)?;
    entity.add(transform)?;

    entity.remove::<Transform>()?;
    entity.detach::<SpriteRenderer>()?;

    Ok(())
}
