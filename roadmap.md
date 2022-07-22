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
- [ ] Textures
- [ ] Polished renderer
- [ ] Fonts and text?

Core:
- [x] Keyboard io
- [x] Mouse io
- [x] Window config
- [ ] Renderer config - WIP
- [x] Misc. config through config.rs
- [ ] Scenes - this should wait until ecs is working
- [x] Blocks - data chunks, primarily for renderer

ECS:
- [ ] Game objects
- [ ] Components
- [ ] Basic sprite renderer

Tests:
- [ ] More block tests; edge cases

Misc:
- [ ] Make README
- [ ] Tons of documentation lmao
- [ ] Proper re-exports: certain components really shouldn't be exposed, at least without using the `rage::ext` module; other components have no function outside of the engine (like globals) and shouldn't be exposed at all

Known bugs:
- [ ] Fix gl::Viewport; currently items clip off of the screen easily

