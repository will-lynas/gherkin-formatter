use crate::config;
use crate::config::FormatterConfig;

pub struct Formatter {
    pub input: String,
    pub config: FormatterConfig,
}

impl Formatter {
    pub fn new(input: &str, config: FormatterConfig) -> Self {
        Self {
            input: input.to_string(),
            config,
        }
    }
    pub fn format(self) -> String {
        let has_trailing_newline = self.input.ends_with('\n');
        let mut result = String::new();
        for line in self.input.lines() {
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
