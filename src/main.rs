pub mod config;
pub mod formatter;

use formatter::Formatter;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut formatter = Formatter::default();
    let formatted = formatter.format(&buffer);
    io::stdout().write_all(formatted.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests;
