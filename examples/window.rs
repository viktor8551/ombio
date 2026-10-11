use walzer::{Engine, Error};

fn main() -> Result<(), Error> {
    Engine::builder().build()?.run()
}
