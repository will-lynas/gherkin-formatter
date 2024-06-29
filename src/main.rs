use std::io::{self, Read, Write};

const INDENT_SIZE: usize = 2;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let formatted = format(&buffer);
    io::stdout().write_all(formatted.as_bytes())?;
    Ok(())
}

fn format(input: &str) -> String {
    let mut result = String::new();
    for line in input.lines() {
        if line.starts_with("Feature:") {
            result.push_str(line);
        } else if line.starts_with("Scenario:") {
            result.push_str(&" ".repeat(INDENT_SIZE));
            result.push_str(line);
        } else if line.starts_with("When") || line.starts_with("Then") {
            result.push_str(&" ".repeat(INDENT_SIZE * 2));
            result.push_str(line);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }
    result
}

#[cfg(test)]
mod tests;
