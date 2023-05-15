rage game engine progress:
thank goodness rust has an automatic build system - I'm no good at cpp :P
note: for some linux distros, run these commands BEFORE building:
```
sudo apt-get install libasound2-dev
export MESA_GL_VERSION_OVERRIDE=3.3
```

Renderer:
- [x] Get window working
- [x] First triangle
- [x] Shader system
- [ ] Custom shader loading
- [ ] Turn shaders into `Asset`s
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
- [x] `Scene` - this should wait until ecs is working
- [x] `Block` - data chunks, primarily for renderer
- [x] `World` - delta time, configuration, `WorldBuilder`
- [x] `Timer` in world
- [ ] `Asset` in world

ECS: Entity Component System
What if we had scenes create objects that are manipulated on the fly? Kinda like the factory pattern?
- [x] Entities
- [x] Components
- [x] Component derive
- [x] Basic components - transform, etc
- [x] Dynamic components
- [x] Scenes - collection of entities that can be updated en masse
- [x] Basic sprite renderer
- [x] Scene names
- [x] Entity names
- [ ] Refactor `SpriteRenderer` to `renderer` directory
Scene function pointer integration - moved to World

Resources: resource access, freeing, storing, etc.
- [ ] Spritesheet - refactor here
- [ ] WAVFile
- [ ] etc (as they come along)

Audio:
- [x] Temporary rodio implementation
- [ ] Sometime, shift to pure cpal implementation (better portability)

Physics: Main controller -> RigidBody factory. Collider is the component form of a RigidBody, and there's a 1-to-1 correspondence between created components and RigidBody's (to be managed by a custom drop() function).
- [ ] Framework - set up the physics factory
- [ ] Collision detection
- [ ] Collision resolution using conservation of momentum and energy
- [ ] Rotational collision resolution - how does friction play into this?

State Machines:
- [x] StateMachine component
- [x] States
- [x] State Machine test

Tests:
- [ ] More block tests; edge cases
- [x] Scene change test

Misc:
- [x] Error macro
- [ ] Make README
- [ ] Tons of documentation lmao
- [ ] Proper re-exports: certain components really shouldn't be exposed, at least without using the `rage::ext` module; other components have no function outside of the engine (like globals) and shouldn't be exposed at all
ChainError - wouldn't have much use, as many functions fail at library level, and most errors can be debugged without too much trouble

Known bugs:
- [x] Fix gl::Viewport; currently items clip off of the screen easily - `window.set_size_polling(true)` needed
- [x] Fix Spritesheet vertical texture wrapping - improper `.floor()`
- [x] Fix DynComponent null parent problem - passing pointers as function arguments B)
