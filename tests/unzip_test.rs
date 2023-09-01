use std::collections::HashSet;

use rage::prelude::*;

fn start(world: &mut World) -> RageResult {
    let s = world.new_scene("main")?;
    s.spawn("0")?.add(Transform::zero())?;
    s.spawn("1")?.add(Transform::zero())?;
    s.spawn("2")?.add(Transform::zero())?;

    world.set_scene("main")?;
    Ok(())
}

fn update(world: &mut World) -> RageResult {
    let s = world.get_scene_mut("main")?;
    let mut entities = s.unzip(HashSet::from(["0", "1", "2"]))?;

    let e0 = unsafe { &mut *entities[0] };
    let e1 = unsafe { &mut *entities[1] };
    let e2 = unsafe { &mut *entities[2] };

    e0.get_mut::<Transform>()?;
    e2.get_mut::<Transform>()?;
    e1.get_mut::<Transform>()?;

    Ok(())
}

#[test]
fn unzip_test() -> RageResult {
    World::new()
        .on_start(start)
        .on_update(update)
        .set_config(Config::default())
        .run()
}