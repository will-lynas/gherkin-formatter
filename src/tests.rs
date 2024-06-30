use super::*;

#[test]
fn basic_feature_good() {
    let input = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join
";
    let expected = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join
";
    let mut formatter = Formatter::default();
    let result = formatter.format(input);
    assert_eq!(
        result, expected,
        "The formatter should leave correct indentation as is."
    );
}

#[test]
fn basic_feature_no_indents() {
    let input = "\
Feature: Guess the word
Scenario: Maker starts a game
When the Maker starts a game
Then the Maker waits for a Breaker to join
";

    let expected = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join
";
    let mut formatter = Formatter::default();
    let result = formatter.format(input);
    assert_eq!(
        result, expected,
        "The formatter should indent the feature correctly."
    );
}

#[test]
fn basic_feature_bad_indents() {
    let input = "\
      Feature: Guess the word
            Scenario: Maker starts a game
  When the Maker starts a game
      Then the Maker waits for a Breaker to join
";
    let expected = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join
";
    let mut formatter = Formatter::default();
    let result = formatter.format(input);
    assert_eq!(
        result, expected,
        "The formatter should indent the badly formatted feature correctly."
    );
}

#[test]
fn trailing_newline() {
    let input = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join
";
    let expected = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join
";
    let mut formatter = Formatter::default();
    let result = formatter.format(input);
    assert_eq!(result, expected, "A trailing newline should be preserved.");
}

#[test]
fn multiple_trailing_newlines() {
    let input = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join


";
    let expected = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join


";
    let mut formatter = Formatter::default();
    let result = formatter.format(input);
    assert_eq!(
        result, expected,
        "Multiple trailing newlines should be preserved"
    );
}

#[test]
fn no_trailing_newline() {
    let input = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join";
    let expected = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join";
    let mut formatter = Formatter::default();
    let result = formatter.format(input);
    assert_eq!(result, expected, "A trailing newline should not be added.");
}
