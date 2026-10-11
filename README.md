# WALZER

A 2D game engine written in Rust.

On Windows, run the minimal window example:

```sh
cargo run --example window
```

The example prepares the engine without showing a window, then opens one blank
window titled `WALZER` with an 800 by 600 logical pixel content area. Close it
with the title-bar close button or Alt+F4 to return from `run()` and exit.
Build and run the engine on the main thread, once per process.

```rust
use walzer::{Engine, Error};

fn main() -> Result<(), Error> {
    Engine::builder().build()?.run()
}
```

Window verification requires an interactive Windows desktop. Compile checks
alone do not verify the lifecycle. Check that building alone displays no window,
then running opens exactly one responsive window with the default title and
content size (accounting for Windows DPI scaling). In separate runs, use each
close action and check that the process exits successfully with no window left.
