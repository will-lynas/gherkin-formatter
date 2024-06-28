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
    fn basic_feature_good() {
        let input = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join";

        let expected = input.to_string();
        let result = format(input);
        assert_eq!(
            result, expected,
            "The format function should leave correct indentation as is."
        );
    }
}
