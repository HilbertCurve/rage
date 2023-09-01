use components::player::Player;
use rage::prelude::*;

mod components;

fn start(world: &mut World) -> RageResult {
    let s = world.new_scene("name")?;
    let p = s.spawn("player")?;
    p.add(Transform::from(
        Vec3::ZERO,
        vec3(25.0, 45.0, 0.0),
        Vec3::ZERO,
    ))?;
    p.attach(SpriteRenderer::from(Vec4::ONE))?;
    p.attach(StateMachine::from(vec![
        State::from("base", |_, _| {
            Ok(())
        })
    ]))?;
    world.set_scene("name")?;
    Ok(())
}

fn update(world: &mut World) -> RageResult {
    let dt = world.dt().clone();
    world.current_scene()?.update::<Player>(dt)?;
    Ok(())
}

#[test]
fn game() -> RageResult {
    World::new()
        .on_start(start)
        .on_update(update)
        .set_config(Config::default())
        .run()
}
