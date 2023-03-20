#[macro_use]
extern crate lazy_static;
#[macro_use]
pub extern crate rage_macros;

pub mod core;
pub mod ecs;
pub mod renderer;
pub mod utils;
// re-exports here

extern crate glam;

#[cfg(test)]
mod tests {
    use super::core::prelude::*;
    use super::core;
    use super::core::app::{World, RageResult};
    use super::utils::block::{Block, BlockError};
    use super::renderer::texture::Spritesheet;
    use super::ecs::prelude::*;
    use super::core::scene::Scene;
    use glam::*;

    fn s_init(world: &mut World) -> RageResult {
        let spritesheet: Spritesheet = Spritesheet::from(String::from("./assets/textures/test.png"), 16, 16, 0)?;

        let r_player: SpriteRenderer = SpriteRenderer::from(
            vec4(1.0, 1.0, 1.0, 1.0),
            spritesheet.get_texture(0));
        let t_player: Transform = Transform::from(vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0));

        let mut scene_id = world.new_scene();
        world.set_scene(scene_id)?;
        let mut scene = world.get_scene(scene_id)?;

        let e_ref = unsafe {
            scene.spawn()
        };
        e_ref.attach(r_player)?;
        e_ref.add(t_player)?;

        Ok(())
    }
    fn s_update(world: &mut World, dt: f64) -> RageResult {
        Ok(())
    }

    #[test]
    pub fn pog() -> RageResult {
        // Setup
        let mut config: Config = Config::default();

        World::new()
            .on_start(s_init)
            .on_update(s_update)
            .set_config(config)
            .run()
    }






    //#[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        /*
        let mut app: World = World::new();

        let mut s_main: Scene = Scene::new();
        s_main.set_start(s_init);

        let mut config: Config = Config::default();
        config.window_height = 600;

        //app.add_scene(s_main);
        app.run(config)
        */
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

