# Running the rectangle example on Windows

Install Rust with the `x86_64-pc-windows-msvc` toolchain (Rust 1.87 or newer),
and Visual Studio Build Tools with the Desktop development with C++ workload
and a Windows SDK. A working desktop session and a graphics driver supporting
one of wgpu's native backends are required.

From the repository root, run:

```powershell
cargo run --example rectangle
```

An 800 by 600 logical-pixel window opens with a green 160 by 100 rectangle at
position (100, 100) on a black background. Close the window to exit.
This example is static; keyboard movement and browser startup are later tickets.

The game implements `Game::draw` and submits rectangles through `Frame`.
`Frame::viewport()` returns the visible width and height in logical pixels.
Rectangle coordinates start at the top left, with positive x to the right and
positive y downward. Colors are linear RGBA values from 0 to 1.

To check the code and run the GPU rendering test:

```powershell
cargo check --all-targets
cargo test
```

The rendering test requires a GPU adapter. It submits drawing commands through
the game-facing interface and checks actual rendered pixels at 2x display scaling.
