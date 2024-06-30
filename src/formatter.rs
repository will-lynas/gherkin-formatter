use crate::config::FormatterConfig;

#[derive(Default)]
pub struct Formatter {
    pub config: FormatterConfig,
    input: Option<String>,
    result: Option<String>,
    has_trailing_newline: Option<bool>,
}

impl Formatter {
    pub fn format(&mut self, input: &str) -> String {
        self.result = Some(String::new());
        self.input = Some(input.to_string());
        self.analyse();
        self.format_lines();
        self.fix_newline();
        self.result.to_owned().unwrap()
    }

    fn analyse(&mut self) {
        self.has_trailing_newline = Some(self.input.as_ref().unwrap().ends_with('\n'));
    }

    fn format_lines(&mut self) {
        let mut result = String::new();
        self.input
            .as_ref()
            .unwrap()
            .lines()
            .map(|line| line.trim_start())
            .for_each(|line| {
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
            });
        self.result = Some(result);
        self.remove_trailing_newline();
    }

    fn remove_trailing_newline(&mut self) {
        if let Some('\n') = self.result.as_mut().unwrap().chars().last() {
            self.result.as_mut().unwrap().pop();
        }
    }

    fn fix_newline(&mut self) {
        if self.has_trailing_newline.unwrap() {
            self.result.as_mut().unwrap().push('\n');
        }
    }
}
