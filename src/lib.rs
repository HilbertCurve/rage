#[macro_use]
extern crate lazy_static;
#[macro_use]
pub extern crate rage_macros;

pub mod audio;
pub mod core;
pub mod ecs;
pub mod renderer;
pub mod utils;
// re-exports here

extern crate glam;

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::audio::source::AudioSource;
    use super::core::prelude::*;
    use super::utils::block::{Block, BlockError};
    use super::renderer::texture::Spritesheet;
    use super::ecs::prelude::*;
    use glam::*;

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
    
    fn audio_test_start(world: &mut World) -> RageResult {
        let scene = world.new_scene("main")?;

        let entity = scene.spawn("source")?;
        entity.attach(AudioSource::new()?)?;

        entity.get_mut::<AudioSource>()?.play("/home/ben/Desktop/lakes.wav".to_string())?;

        world.set_scene("main")?;

        Ok(())
    }

    fn audio_test_update(world: &mut World) -> RageResult {
        let entity = world.get_scene("main")?.get("source")?;
        if keyboard::is_pressed(glfw::Key::Space) {
            entity.get_mut::<AudioSource>()?.pause()?;
        } else {
            entity.get_mut::<AudioSource>()?.resume()?;
        }
        Ok(())
    }
    
    #[test]
    fn audio_test() -> RageResult {
        World::new()
            .on_start(audio_test_start)
            .on_update(audio_test_update)
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
        static mut COUNT: usize = 0;
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

    fn s_init(world: &mut World) -> RageResult {
        let spritesheet: Spritesheet = Spritesheet::from(String::from("./assets/textures/test.png"), 8, 8, 0)?;

        ////////////////////////////
        // Scene 0                //
        ////////////////////////////
        let r_player_0: SpriteRenderer = SpriteRenderer::slice(
            vec4(1.0, 1.0, 1.0, 1.0),
            &spritesheet, 0, 0);
        let t_player_0: Transform = Transform::from(vec3(0.0, 0.0, 0.0), vec3(100.0, 100.0, 1.0), Vec3::ZERO);
        let r_side_0: SpriteRenderer = SpriteRenderer::slice(
            vec4(1.0, 1.0, 1.0, 1.0),
            &spritesheet, 0, 0);
        let t_side_0: Transform = Transform::from(vec3(100.0, 0.0, 0.0), vec3(100.0, 100.0, 1.0), Vec3::ZERO);

        let scene = world.new_scene("main")?;

        let e_ref_0 = scene.spawn("center")?;
        e_ref_0.attach(r_player_0)?;
        e_ref_0.add(t_player_0)?;
        let e_side_0 = scene.spawn("right")?;
        e_side_0.attach(r_side_0)?;
        e_side_0.add(t_side_0)?;

        ////////////////////////////
        // Scene 1                //
        ////////////////////////////
        let r_player_1: SpriteRenderer = SpriteRenderer::slice(
            vec4(1.0, 1.0, 1.0, 1.0),
            &spritesheet, 2, 2);
        let t_player_1: Transform = Transform::from(vec3(0.0, 0.0, 0.0), vec3(100.0, 100.0, 1.0), Vec3::ZERO);

        let scene = world.new_scene("next")?;

        let e_ref_1 = scene.spawn("center")?;
        e_ref_1.attach(r_player_1)?;
        e_ref_1.add(t_player_1)?;

        Ok(())
    }
    #[allow(dead_code)]
    fn s_update(world: &mut World) -> RageResult {
        world.set_scene("main")?;
        if keyboard::is_pressed(glfw::Key::Space) {
            world.set_scene("next")?;
        }
        Ok(())
    }

    //#[test]
    #[allow(dead_code)]
    pub fn pog() -> RageResult {
        // Setup
        let config: Config = Config::default();

        World::new()
            .on_start(s_init)
            .on_update(s_update)
            .set_config(config)
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

    #[test]
    fn block_test_push_pop() -> Result<(), BlockError> {
        let mut block: Block = Block::empty();
        block.push(4u32);

        let val: u32 = unsafe {
            block.pop::<u32>()?
        };

        assert_eq!(val, 4u32, "Popped value not expected value: expected 4, got {}", val);
        assert!(block.len() == 0, "Inconsistent block length: expected 0, got {}", block.len());

        block.push(2u32);
        block.push(1f32);

        assert!(block.len() == 8, "Inconsistend block length: expected 8, got {}", block.len());

        let (fval, uval) = unsafe {
            (
                block.pop::<f32>()?,
                block.pop::<u32>()?,
                )
        };

        assert_eq!(fval, 1.0, "Popped signed value not expected value: expected 4, got {}", fval);
        assert_eq!(uval, 2, "Popped unsigned value not expected value: expected 4, got {}", uval);

        Ok(())
    }

    #[test]
    fn block_test_get_set() -> Result<(), BlockError> {
        let mut block: Block = Block::empty();
        let u32_size: usize = std::mem::size_of::<u32>() as usize;

        for i in 0..16u32 {
            block.push(i);
        }

        for i in 0..16u32 {
            let val: &u32 = unsafe {
                block.get::<u32>(i as usize * u32_size)?
            };

            assert_eq!(*val, i, "Received unexpected value: expected {}, got {}", i, val);
        }

        let mut inc = 0;
        for i in 16..32u32 {
            block.set::<u32>(inc * u32_size, i)?;
            inc += 1;
        }

        for i in 0..16u32 {
            let val: &u32 = unsafe {
                block.get::<u32>(i as usize * u32_size)?
            };

            assert_eq!(*val, i + 16, "Received unexpected value: expected {}, got {}", i + 16, val);
        }

        Ok(())
    }
}

