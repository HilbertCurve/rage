extern crate gl;
extern crate glfw;

use glfw::Context;

use crate::ecs::prelude::*;
use crate::core::{self, prelude::*};
use crate::core::scene::{Scene, SceneError};
use crate::renderer::renderer;

use std::sync::mpsc::Receiver;
use glam::*;

pub type RageResult = Result<(), Box<dyn std::error::Error>>;
type GlfwConf = (glfw::Glfw, glfw::Window, Receiver<(f64, glfw::WindowEvent)>);

const TITLE: &str = "Rage Game Engine";
pub struct WorldBuilder {
    start_fn: fn(&mut World) -> RageResult,
    update_fn: fn(&mut World) -> RageResult,
    config: Config,
}

impl Default for WorldBuilder {
    fn default() -> WorldBuilder {
        WorldBuilder {
            start_fn: |_| Ok(()),
            update_fn: |_| Ok(()),
            config: Config::default(),
        }
    }
}

impl WorldBuilder {
    pub fn on_start(mut self, start: fn(&mut World) -> RageResult) -> WorldBuilder {
        self.start_fn = start;
        self
    }
    pub fn on_update(mut self, update: fn(&mut World) -> RageResult) -> WorldBuilder {
        self.update_fn = update;
        self
    }
    pub fn set_config(mut self, config: Config) -> WorldBuilder {
        self.config = config;
        self
    }
    pub fn run(self) -> RageResult {
        let built_world: World = World {
            scenes: vec![],
            uptime: 0.0,
            dt: 0.0001,
            fps: 0.0,
            current_scene: String::new(),
            start: self.start_fn,
            update: self.update_fn,
        };
        built_world.run(self.config)
    }
}

pub struct World {
    // SceneManager?
    scenes: Vec<Scene>,
    uptime: f64,
    dt: f64,
    fps: f64,
    current_scene: String,
    start: fn(&mut World) -> RageResult,
    update: fn(&mut World) -> RageResult,
    //...
}

impl World {
    pub fn new() -> WorldBuilder {
        WorldBuilder::default()
    }

    pub fn new_scene(&mut self, name: &str) -> Result<&mut Scene, SceneError> {
        for i in 0..self.scenes.len() {
            if self.scenes[i].name() == name {
                return Err(SceneError::new(
                        &format!("Scene of name: {} already in World.", name)));
            }
        }
        let scene = Scene::new(name.to_owned());

        self.scenes.push(scene);
        Ok(self.scenes.last_mut().expect("stinki2"))
    }

    pub fn get_scene(&mut self, name: &str) -> Result<&mut Scene, SceneError> {
        for i in 0..self.scenes.len() {
            if self.scenes[i].name() == name {
                return Ok(&mut self.scenes[i]);
            }
        }
        Err(SceneError::new(&format!("Scene of name: {} not found in World.", name)))
    }
    pub fn set_scene(&mut self, name: &str) -> Result<(), SceneError> {
        for i in 0..self.scenes.len() {
            if self.scenes[i].name() == name {
                self.current_scene = name.to_owned();
                return Ok(());
            }
        }
        Err(SceneError::new(&format!("Scene of name: {} not found in World.", name)))
    }
    pub fn current_scene(&mut self) -> Result<&mut Scene, SceneError> {
        if self.current_scene == "" {
            Err(SceneError::new("No default scene set!"))
        } else {
            Ok(self.get_scene(&self.current_scene.clone()).expect("stinki!"))
        }
    }
    pub fn fps(&self) -> f64 {
        self.fps
    }
    pub fn dt(&self) -> f64 {
        self.dt
    }

    /// Initializes and runs program, consuming inputted configuration object.
    pub fn run(mut self, config: Config) -> RageResult {
        // set config
        Config::set(config)?;

        // game loop, as specified by current scene
        // TODO: move to window.rs
        let (mut glfw, mut window, events) = World::window_init()?;
        (self.start)(&mut self)?;

        let mut t0: f64;
        let mut t1: f64;
        while !window.should_close() {
            t0 = glfw.get_time();
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                World::handle_window_event(&mut window, event);
            }

            //TODO: toggle which ones update in Config?
            (self.update)(&mut self)?;
            let wdt = self.dt.clone();
            self.current_scene()?.update::<SpriteRenderer>(wdt)?;
          //self.current_scene()?.update::<Collider>()?;
            renderer::update();

            if keyboard::is_pressed(glfw::Key::A) {
                println!("{:?}", mouse::pos());
            }
            if mouse::is_pressed(glfw::MouseButton::Button1) {
                println!("Pressed");
            }

            window.swap_buffers();

            t1 = glfw.get_time();
            let dt = t1 - t0;
            self.uptime += dt;
            self.dt = dt;
            self.fps = 1.0 / dt;
        }

        Ok(())
    }

    fn window_init() -> Result<GlfwConf, String> {
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
        window.set_size_polling(true);
        window.make_current();

        inst.default_window_hints();
        inst.set_swap_interval(glfw::SwapInterval::Sync(1));

        unsafe {
            core::window::set_width_height(conf.window_width, conf.window_height);
            core::window::set_title(String::from(TITLE));
        }

        // gl
        gl::load_with(|s| window.get_proc_address(s) as * const _);

        // start engines
        renderer::start();

        Ok((inst, window, events))
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
                core::window::width_height_event(x as u32, y as u32);
                println!("({},{})", x, y);
                unsafe {
                    gl::Viewport(0, 0, x, y);
                }
            }
            _ => {}
        }
    }
}

