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

#[test]
fn comments_dont_change() {
    let input = "\
Feature: Guess the word
       # comment
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join
";
    let expected = "\
Feature: Guess the word
       # comment
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join
";
    let mut formatter = Formatter::default();
    let result = formatter.format(input);
    assert_eq!(result, expected, "Comments are not changed.");
}

#[test]
fn top_level_tag_is_not_indented() {
    let input = "\
    @tag
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join
";
    let expected = "\
@tag
Feature: Guess the word
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join
";
    let mut formatter = Formatter::default();
    let result = formatter.format(input);
    assert_eq!(
        result, expected,
        "A top level tag should have no indentation."
    );
}

#[test]
fn scenario_tag_is_indented() {
    let input = "\
Feature: Guess the word
@tag
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join
";
    let expected = "\
Feature: Guess the word
  @tag
  Scenario: Maker starts a game
    When the Maker starts a game
    Then the Maker waits for a Breaker to join
";
    let mut formatter = Formatter::default();
    let result = formatter.format(input);
    assert_eq!(
        result, expected,
        "A scenario tag should be intended to the same level as the scenario."
    );
}

#[test]
fn feature_with_body() {
    let input = "\
Feature: Guess the word

The word guess game is a turn-based game for two players.
The Maker makes a word for the Breaker to guess. The game
is over when the Breaker guesses the Maker's word.
";
    let expected = "\
Feature: Guess the word

  The word guess game is a turn-based game for two players.
  The Maker makes a word for the Breaker to guess. The game
  is over when the Breaker guesses the Maker's word.
";
    let mut formatter = Formatter::default();
    let result = formatter.format(input);
    assert_eq!(
        result, expected,
        "A feature with a body should be indented correctly"
    );
}
