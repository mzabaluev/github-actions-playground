use hello_world::say_hello;
use std::io::{stdout, Result};

fn main() -> Result<()> {
    say_hello(stdout())
}
