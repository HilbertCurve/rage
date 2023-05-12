use glam::{Vec2, Vec4};

use crate::renderer::camera::Camera;
use crate::core::{mouse, window};

pub fn mouse_pos() -> Vec2 {
    let mut pos: Vec2 = Vec2::new(mouse::pos().0 as f32, mouse::pos().1 as f32);

    // transform pos to OpenGL coords (x: [-1, 1], y: [-1,1])
    let wwh = Vec2::new(window::get_width_height().0 as f32, window::get_width_height().1 as f32);
    pos -= wwh / 2.0;
    pos /= wwh / 2.0;
    // vertical pixels count up from top to bottom; to account for this we need to
    // flip the coords upside down
    pos *= Vec2::new(1.0, -1.0);

    // transform pos using inverse matrices from camera
    let pos4 = Vec4::from((pos, 0.0, 1.0));
    let pos4 = Camera::get().view_mat().inverse() * Camera::get().projection_mat().inverse() * pos4;

    // return x and y components of this transformation
    glam::Vec4Swizzles::xy(pos4)
}
