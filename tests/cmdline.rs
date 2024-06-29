use assert_cmd::Command;

#[test]
fn test_stdin_stdout_formatting() {
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

    let mut cmd = Command::cargo_bin("gherkin-formatter").unwrap();
    cmd.write_stdin(input).assert().success().stdout(expected);
}
