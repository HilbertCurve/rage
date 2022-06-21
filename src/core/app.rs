extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key};
use crate::renderer;

use std::sync::mpsc::Receiver;

pub struct Window {
    width: i32,
    height: i32,
    title: String,
}

// ik it's global state: cry about it
static mut MAIN_WIN: Option<Window> = None;
const TITLE: &str = "Rage Game Engine";

fn init() -> (glfw::Glfw, glfw::Window, Receiver<(f64, glfw::WindowEvent)>) {
    // starting window

    let mut inst = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = inst.create_window(300, 300, TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    window.set_key_polling(true);
    window.make_current();

    inst.default_window_hints();
    inst.set_swap_interval(glfw::SwapInterval::Sync(1));
    
    unsafe {
        MAIN_WIN = Some(Window { 
            width: 300,
            height: 300,
            title: String::from(TITLE),
        });
    }

    // gl
    gl::load_with(|s| window.get_proc_address(s) as * const _);

    renderer::start();

    (inst, window, events)
}

pub fn run() {
    // game loop, as specified by current scene
    let (mut glfw, mut window, events) = init();
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
 
        renderer::update();
 
        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}

