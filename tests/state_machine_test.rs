use lazy_static::lazy_static;
use rage::prelude::*;

lazy_static! {pub static ref STATES: [State; 2] = [
    State::from("start".to_string(), 
        |entity, _dt| {
            entity.get_mut::<SpriteRenderer>()?.color = vec4(1.0, 1.0, 1.0, 1.0);
            if keyboard::is_pressed(glfw::Key::A) {
                println!("Switched!");
                entity.get_mut::<StateMachine>()?.change_state("next".to_string())?;
            }
            Ok(())
        },
    ),
    State::from("next".to_string(),
        |entity, _dt| {
            entity.get_mut::<SpriteRenderer>()?.color = vec4(0.5, 0.5, 0.5, 1.0);
            if !keyboard::is_pressed(glfw::Key::A) {
                println!("Unswitched!");
                entity.get_mut::<StateMachine>()?.change_state("start".to_string())?;
            }
            Ok(())
        },
    )
];
}

fn state_machine_start(world: &mut World) -> RageResult {
    let scene = world.new_scene("main")?;
    let entity = scene.spawn("main")?;

    entity.attach(StateMachine::from(STATES.to_vec()))?;
    entity.attach(SpriteRenderer::from(vec4(1.0, 1.0, 1.0, 1.0)))?;
    entity.add(Transform::from(Vec3::ZERO, vec3(50.0, 50.0, 50.0), vec3(0.0, 0.0, 0.0)))?;
    world.set_scene("main")?;
    Ok(())
}
fn state_machine_update(_world: &mut World) -> RageResult {
    Ok(())
}

#[test]
fn state_machine_test() -> RageResult {
    World::new()
        .on_start(state_machine_start)
        .on_update(state_machine_update)
        .set_config(Config::default())
        .run()
}