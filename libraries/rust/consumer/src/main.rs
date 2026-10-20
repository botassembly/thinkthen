//! Installed Rust packet dumper through the public session API.
mod dump;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let output = dump::run(&input)?;
    writeln!(io::stdout().lock(), "{output}")?;
    Ok(())
}
