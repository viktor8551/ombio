# Own the game loop over lower-level platform libraries

Walzer will own the game loop and call game-provided update and drawing functions, using `winit` for windowing and input and `wgpu` for graphics. This keeps platform coordination inside Walzer and gives it its own game-facing API while reusing foundations that support the long-term desktop, mobile, and web targets. We accept the initial graphics setup work instead of adopting an existing game framework's architecture or writing separate platform integrations ourselves; Windows and WebGPU web are the first validation targets.
