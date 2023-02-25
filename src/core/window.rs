use crate::core::scene::Scene;

// TODO: add more exhaustive traits for window (maximized, window pos, etc)
struct Window {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub scene: Scene,
}

impl Window {
    pub fn get_scene(&mut self) -> &mut Scene {
        &mut self.scene
    }
}

