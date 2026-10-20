# GPU rendering foundation

OMBIO uses GPU rendering through `wgpu`. GPU rendering provides the foundation for future sprite and texture rendering.

Applications supply a constructor callback of the form `FnOnce(&Context) -> Result<G, Error>`, where `G` implements the public `Game` trait, so they can write `engine.run(RectangleGame::new)`. Inside `run()`, OMBIO creates the window, graphics, and Context, then calls the constructor once before any game updates or rendering. This lets the application initialize a complete game using the actual window size and available engine resources. Constructor errors end execution and are returned from `run()`; slow initialization can delay the first rendered frame. The constructor remains an ordinary application method rather than a requirement on the Game trait.

The Game object retains the application's state. OMBIO invokes its render method, `render(&mut self, frame: &mut Frame)`, when a redraw is needed and manages GPU resources and presentation. Engine preparation and execution remain separate as defined in ADR 0001.
