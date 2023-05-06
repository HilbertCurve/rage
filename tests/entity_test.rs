use rage::prelude::*;

fn hearts_start(world: &mut World) -> RageResult {
    let scene = world.new_scene("main")?;

    let hearts: Spritesheet = Spritesheet::from("assets/textures/hearts.png".to_owned(), 16, 16, 0)?;

    let entity = scene.spawn("0")?;
    entity.attach(SpriteRenderer::from(&hearts))?;
    entity.add(Transform::from(
        Vec3::ZERO,
        Vec3::from((250.0, 250.0, 50.0)),
        Vec3::from((0.0, 0.0, 0.0)),
    ))?;

    world.set_scene("main")?;
    world.push_timer("main");

    Ok(())
}

fn hearts_update(world: &mut World) -> RageResult {
    // heart animation using a timer
    if world.get_timer("main")? >= 0.2 {
        world
            .get_scene("main")?
            .get("0")?
            .get_mut::<SpriteRenderer>()?
            .next_wrap();
        world.reset_timer("main")?;
    }

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