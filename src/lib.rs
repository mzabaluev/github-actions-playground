use std::io::{Write, Result};

pub fn say_hello<W: Write>(mut out: W) -> Result<()> {
    say_hello_internal(&mut out)
}

fn say_hello_internal(out: &mut dyn Write) -> Result<()> {
    writeln!(out, "Hello, world!")
}
