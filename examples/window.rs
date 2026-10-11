use walzer::{Engine, Error};

fn main() -> Result<(), Error> {
    Engine::builder()
        .title("My WALZER game")
        .window_size(640, 480)
        .build()?
        .run()
}
