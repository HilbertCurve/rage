extern crate glam;

use crate::core::config::Config;
use crate::core::window;

use glam::{Vec3, Quat, Mat4};
use std::sync::Once;
use std::mem::{MaybeUninit};

const PI: f32 = 3.14159265359;

#[derive(Copy, Clone)]
pub enum CameraMode {
    Orthographic,
    Perspective,
}

pub struct Camera {
    pos: Vec3,
    orient: Quat,
    fov: f32,
    zoom: f32,
    mode: CameraMode,
}

impl Camera {
    fn new() -> Camera {
        Camera {
            pos: Vec3::new(0.0, 0.0, 1.0),
            orient: Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
            // TODO: specify these in config.rs
            fov: PI / 3.0,
            zoom: 1.0,
            mode: Config::get().proj_mode,
        }
    }

    pub fn get() -> &'static mut Camera {
        static mut SINGLETON: MaybeUninit<Camera> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                let singleton = Camera::new();
                SINGLETON.write(singleton);
            });

            SINGLETON.assume_init_mut()
        }
    }

    pub fn projection_mat(&self) -> Mat4 {
        let (win_w, win_h) = window::get_width_height();
        let (win_w, win_h) = (win_w as f32, win_h as f32);
        match &self.mode {
            CameraMode::Orthographic => {
                // TODO: idk if zoom is necessary ??
                Mat4::orthographic_rh_gl(
                    -win_w * self.zoom, win_w * self.zoom,
                    -win_h * self.zoom, win_h * self.zoom,
                    0.01, 100.0)
            }
            CameraMode::Perspective => {
                Mat4::perspective_rh_gl(self.fov, win_w / win_h, 0.01, 100.0)
            }
        }
    }

    pub fn view_mat(&self) -> Mat4 {
        let ret = Mat4::from_translation(self.pos) * Mat4::from_quat(self.orient);
        Mat4::inverse(&ret)
    }
}

