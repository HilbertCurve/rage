use rage::ecs::prelude::*;

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