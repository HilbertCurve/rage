use rage::prelude::*;

fn start(world: &mut World) -> RageResult {
    let s = world.new_scene("main")?;

    let monkey = s.spawn("monkey")?;

    let model = Model::from_gltf("./assets/models/monkey.gltf")?;

    monkey.attach(ModelRenderer::from(model))?;

    world.set_scene("main")?;

    Ok(())
}

fn update(_: &mut World) -> RageResult {
    Ok(())
}

#[test]
fn model_test() -> RageResult {
    World::new()
        .on_start(start)
        .on_update(update)
        .set_config(Config::default())
        .run()
}