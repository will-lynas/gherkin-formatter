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

#[test]
fn rules_and_examples() {
    let input = "\
Feature: Highlander

Rule: There can be only One

Example: Only One -- More than one alive
Given there are 3 ninjas
And there are more than one ninja alive
When 2 ninjas meet, they will fight
Then one ninja dies (but not me)
And there is one ninja less alive

Example: Only One -- One alive
Given there is only 1 ninja alive
Then he (or she) will live forever ;-)
";
    let expected = "\
Feature: Highlander

  Rule: There can be only One

    Example: Only One -- More than one alive
      Given there are 3 ninjas
      And there are more than one ninja alive
      When 2 ninjas meet, they will fight
      Then one ninja dies (but not me)
      And there is one ninja less alive

    Example: Only One -- One alive
      Given there is only 1 ninja alive
      Then he (or she) will live forever ;-)
";
    let mut formatter = Formatter::default();
    let result = formatter.format(input);
    assert_eq!(
        result, expected,
        "Rules and examples should be formatted correctly."
    );
}

#[test]
fn background() {
    let input = r#"\
Feature: Multiple site support
Only blog owners can post to a blog, except administrators,
who can post to all blogs.

Background:
Given a global administrator named "Greg"
And a blog named "Greg's anti-tax rants"
And a customer named "Dr. Bill"
And a blog named "Expensive Therapy" owned by "Dr. Bill"

Scenario: Dr. Bill posts to his own blog
Given I am logged in as Dr. Bill
When I try to post to "Expensive Therapy"
Then I should see "Your article was published."

Scenario: Dr. Bill tries to post to somebody else's blog, and fails
Given I am logged in as Dr. Bill
When I try to post to "Greg's anti-tax rants"
Then I should see "Hey! That's not your blog!"

Scenario: Greg posts to a client's blog
Given I am logged in as Greg
When I try to post to "Expensive Therapy"
Then I should see "Your article was published."
"#;
    let expected = r#"\
Feature: Multiple site support
  Only blog owners can post to a blog, except administrators,
  who can post to all blogs.

  Background:
    Given a global administrator named "Greg"
    And a blog named "Greg's anti-tax rants"
    And a customer named "Dr. Bill"
    And a blog named "Expensive Therapy" owned by "Dr. Bill"

  Scenario: Dr. Bill posts to his own blog
    Given I am logged in as Dr. Bill
    When I try to post to "Expensive Therapy"
    Then I should see "Your article was published."

  Scenario: Dr. Bill tries to post to somebody else's blog, and fails
    Given I am logged in as Dr. Bill
    When I try to post to "Greg's anti-tax rants"
    Then I should see "Hey! That's not your blog!"

  Scenario: Greg posts to a client's blog
    Given I am logged in as Greg
    When I try to post to "Expensive Therapy"
    Then I should see "Your article was published."
"#;
    let mut formatter = Formatter::default();
    let result = formatter.format(input);
    assert_eq!(
        result, expected,
        "Backgrounds should be formatted correctly."
    );
}
