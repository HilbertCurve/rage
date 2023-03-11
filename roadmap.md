rage game engine progress:
thank goodness rust has an automatic build system

Renderer:
- [x] Get window working
- [x] First triangle
- [x] Shader system
- [x] Camera
- [x] Abstract vertex buffer object
- [x] First quad (as in primitive abstractions)
- [x] Data buffers
- [x] Textures
- [ ] Fonts and text
- [ ] 3D Rendering
- [ ] Debugging

3D Rendering:
- [ ] glTF Parser
- [ ] Model objects
- [ ] Model primitive and rendering

Core:
- [x] Keyboard io
- [x] Mouse io
- [x] Window config
- [ ] Renderer config - WIP
- [x] Misc. config through config.rs
- [x] Scenes - this should wait until ecs is working
- [x] Blocks - data chunks, primarily for renderer

Example Scene (goal):
```rust 
pub fn start(scene: &mut Scene) -> Result<(), SceneError> {
    scene.spawn(...);
}

pub fn main() {
    // config stuff
    ...

    // App setup
    app::set_scene(s_main);
    app::run();
}
```

ECS: Entity Component System
What if we had scenes create objects that are manipulated on the fly? Kinda like the factory pattern?
- [x] Entities
- [x] Components
- [x] Component derive
- [x] Basic components - transform, etc
- [x] Dynamic components
- [x] Scenes - collection of entities that can be updated en masse
- [x] Basic sprite renderer
- [ ] Tests

Audio:
- [ ] Audio setup (find OpenAL bindings)
- [ ] AudioController
- [ ] AudioListener
- [ ] AudioSource

Physics:
- [ ] Framework - set up the physics factory
- [ ] Collision detection
- [ ] Collision resolution using conservation of momentum and energy
- [ ] Rotational collision resolution - how does friction play into this?

State Machines:
- [ ] Plan

Tests:
- [ ] More block tests; edge cases

Misc:
- [ ] Make README
- [ ] Tons of documentation lmao
- [ ] Proper re-exports: certain components really shouldn't be exposed, at least without using the `rage::ext` module; other components have no function outside of the engine (like globals) and shouldn't be exposed at all
- [-] ChainError - wouldn't have much use, as many functions fail at library level, and most errors can be debugged without too much trouble

Known bugs:
- [ ] Fix gl::Viewport; currently items clip off of the screen easily

