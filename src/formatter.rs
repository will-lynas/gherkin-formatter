use crate::config;
use crate::config::FormatterConfig;

pub struct Formatter {
    pub config: FormatterConfig,
}

impl Formatter {
    pub fn new(config: FormatterConfig) -> Self {
        Self { config }
    }

    pub fn format(self, input: &str) -> String {
        let has_trailing_newline = input.ends_with('\n');
        let mut result = String::new();
        for line in input.lines().map(|line| line.trim_start()) {
            if line.starts_with("Feature:") {
                result.push_str(line);
            } else if line.starts_with("Scenario:") {
                result.push_str(&" ".repeat(self.config.indent_size));
                result.push_str(line);
            } else if line.starts_with("When") || line.starts_with("Then") {
                result.push_str(&" ".repeat(self.config.indent_size * 2));
                result.push_str(line);
            } else {
                result.push_str(line);
            }
            result.push('\n');
        }
        if let Some('\n') = result.chars().last() {
            result.pop();
        }
        match (has_trailing_newline, &self.config.add_trailing_newline) {
            (_, config::TrailingNewlineOption::Add) => {
                result.push('\n');
            }
            (true, config::TrailingNewlineOption::NoChange) => {
                result.push('\n');
            }
            (false, config::TrailingNewlineOption::NoChange) => {}
        }
        result
    }
}
