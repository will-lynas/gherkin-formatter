pub mod config;
pub mod formatter;

use formatter::Formatter;
use std::io::{self, Read, Write};

use config::FormatterConfig;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let config = FormatterConfig::default();
    let formatter = Formatter::new(&buffer, config);
    let formatted = formatter.format();
    io::stdout().write_all(formatted.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests;
