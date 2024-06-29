use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let formatted = format(&buffer);
    io::stdout().write_all(formatted.as_bytes())?;
    Ok(())
}

fn format(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests;
