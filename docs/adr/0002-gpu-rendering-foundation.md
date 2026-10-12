# GPU rendering foundation

WALZER uses GPU rendering through `wgpu`. GPU rendering provides the foundation for future sprite and texture rendering.

Applications supply an object implementing the public `Game` trait, whose initial interface is `render(&mut self, frame: &mut Frame)`. `run(game)` takes ownership of that object and replaces the argument-free `run()`. WALZER invokes the render method when a redraw is needed and manages GPU resources and presentation. The object allows the application to retain its own state. Engine preparation and execution remain separate as defined in ADR 0001.
