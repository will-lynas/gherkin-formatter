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
        "The formatter should leave correct indentation as is."
    );
}

#[test]
fn basic_feature_bad() {
    let input = "\
Feature: Guess the word
Scenario: Maker starts a game
When the Maker starts a game
Then the Maker waits for a Breaker to join";

    let expected = "\
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join"
        .to_string();

    let result = format(input);
    assert_eq!(
        result, expected,
        "The formatter should indent the feature correctly."
    );
}
