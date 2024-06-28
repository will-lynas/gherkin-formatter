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
mod tests {
    use super::*;

    #[test]
    fn test_format_unchanged() {
        let input = "This is a test input.\nIt will be echoed back.\n";
        let expected = input.to_string();
        let result = format(input);
        assert_eq!(result, expected, "The format function should return the input unchanged.");
    }
}
