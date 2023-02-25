extern crate gl;
extern crate glfw;

#[allow(unused)]
use glfw::{Action, Context, Key};
#[deny(unused)]

use crate::renderer::renderer;
use crate::core::{self, prelude::*};

use std::sync::mpsc::Receiver;

// unused right now
// TODO: move title to config
const TITLE: &str = "Rage Game Engine";

#[warn(unused)]
type GlfwConf = (glfw::Glfw, glfw::Window, Receiver<(f64, glfw::WindowEvent)>);

fn init() -> Result<GlfwConf, String> {
    // starting window

    let mut inst = glfw::init(glfw::FAIL_ON_ERRORS)
        .or(Err("Could not initialize GLFW instance.".to_owned()))?;

    let conf = Config::get();

    let (mut window, events) = inst.create_window(
        conf.window_width,
        conf.window_height,
        &conf.window_title,
        glfw::WindowMode::Windowed)
        .ok_or("Could not initialize GLFW window.".to_owned())?;
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.make_current();

    inst.default_window_hints();
    inst.set_swap_interval(glfw::SwapInterval::Sync(1));

    core::window::set_width_height(conf.window_width, conf.window_height);
    core::window::set_title(String::from(TITLE));
    //core::window::set_scene(DEFAULT_SCENE);

    // gl
    gl::load_with(|s| window.get_proc_address(s) as * const _);

    renderer::start();

    Ok((inst, window, events))
}

use glam::*;
use crate::renderer::texture::Spritesheet;
use crate::ecs::prelude::*;
/// Initializes and runs program, consuming inputted configuration object.
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // set config
    Config::set(config)?;

    // game loop, as specified by current scene
    let (mut glfw, mut window, events) = init()?;

    let spritesheet: Spritesheet = Spritesheet::from(String::from("./assets/textures/test.png"), 16, 16, 0)?;

    let r_player: SpriteRenderer = SpriteRenderer::from(
        vec4(1.0, 1.0, 1.0, 1.0),
        spritesheet.get_texture(0));
    let player: Entity = Entity::new();

    player.attach(r_player);

    core::window::get_scene().add(&mut player);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        core::window::get_scene().update::<SpriteRenderer>();
        renderer::update();
 
        if keyboard::is_pressed(glfw::Key::A) {
            println!("{:?}", mouse::pos());
        }
        if mouse::is_pressed(glfw::MouseButton::Button1) {
            println!("Pressed");
        }

        window.swap_buffers();
    }

    Ok(())
}

fn handle_window_event(_window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(key, _, action, _) => {
            core::keyboard::key_event(key, action);
        }
        glfw::WindowEvent::MouseButton(button, action, _) => {
            core::mouse::mouse_button_event(button, action);
        }
        glfw::WindowEvent::CursorPos(x, y) => {
            core::mouse::mouse_pos_event(x, y);
        }
        glfw::WindowEvent::Scroll(x, y) => {
            core::mouse::mouse_scroll_event(x, y);
        }
        glfw::WindowEvent::Size(x, y) => {
            core::window::set_width_height(x as u32, y as u32);
        }
        _ => {}
    }
}

