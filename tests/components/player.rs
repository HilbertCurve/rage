use rage::prelude::*;

#[allow(unused)]
#[derive(Component)]
pub struct Player {
    // position, in tile world
    pos: Vec2,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vec2::ZERO,
        }
    }
}

impl DynComponent for Player {
    unsafe fn start(&mut self, _parent: *mut Entity) -> Result<(), ComponentError> {
        Ok(())
    }
    unsafe fn update(&mut self, _dt: f64, _parent: *mut Entity) -> Result<(), ComponentError> {
        Ok(())
    }
    unsafe fn stop(&mut self, _parent: *mut Entity) -> Result<(), ComponentError> {
        Ok(())
    }
}
