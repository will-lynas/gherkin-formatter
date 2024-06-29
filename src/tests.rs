use crate::config::TrailingNewlineOption;

use super::*;

#[test]
fn basic_feature_good() {
    let input = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join
";

    let expected = input.to_string();
    let config = FormatterConfig::default();
    let formatter = Formatter::new(input, config);
    let result = formatter.format();
    assert_eq!(
        result, expected,
        "The formatter should leave correct indentation as is."
    );
}

#[test]
fn basic_feature_bad() {
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
"
    .to_string();
    let config = FormatterConfig::default();
    let formatter = Formatter::new(input, config);
    let result = formatter.format();
    assert_eq!(
        result, expected,
        "The formatter should indent the feature correctly."
    );
}

#[test]
fn newline_unchanged_newline() {
    let input = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join
";
    let expected = input.to_string();
    let config = FormatterConfig {
        add_trailing_newline: TrailingNewlineOption::NoChange,
        ..Default::default()
    };
    let formatter = Formatter::new(input, config);
    let result = formatter.format();
    assert_eq!(
        result, expected,
        "With no change set, a newline should be left unchanged"
    );
}

#[test]
fn newline_unchanged_no_newline() {
    let input = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join";
    let expected = input.to_string();
    let config = FormatterConfig {
        add_trailing_newline: TrailingNewlineOption::NoChange,
        ..Default::default()
    };
    let formatter = Formatter::new(input, config);
    let result = formatter.format();
    assert_eq!(
        result, expected,
        "With no change set, a missing newline should remain missing"
    );
}

#[test]
fn newline_add_newline() {
    let input = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join
";
    let expected = input.to_string();
    let config = FormatterConfig {
        add_trailing_newline: TrailingNewlineOption::Add,
        ..Default::default()
    };
    let formatter = Formatter::new(input, config);
    let result = formatter.format();
    assert_eq!(
        result, expected,
        "With add newline set, a newline should remain unchanged"
    );
}

#[test]
fn newline_add_no_newline() {
    let input = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join";
    let expected = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join
"
    .to_string();
    let config = FormatterConfig {
        add_trailing_newline: TrailingNewlineOption::Add,
        ..Default::default()
    };
    let formatter = Formatter::new(input, config);
    let result = formatter.format();
    assert_eq!(
        result, expected,
        "With add newline set, a missing newline should be added"
    );
}
