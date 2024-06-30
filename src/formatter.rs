use crate::config::FormatterConfig;

#[derive(Default)]
pub struct Formatter {
    pub config: FormatterConfig,
    input: Option<String>,
    result: Vec<String>,
}

impl Formatter {
    pub fn format(&mut self, input: &str) -> String {
        self.result = Vec::new();
        self.input = Some(input.to_string());
        self.format_lines();
        self.result.join("\n")
    }

    fn format_lines(&mut self) {
        self.input
            .as_ref()
            .unwrap()
            .split('\n')
            .map(|line| line.trim_start())
            .for_each(|line| {
                let mut result_line = String::new();
                if line.starts_with("Feature:") {
                    result_line.push_str(line);
                } else if line.starts_with("Scenario:") {
                    result_line.push_str(&" ".repeat(self.config.indent_size));
                    result_line.push_str(line);
                } else if line.starts_with("When") || line.starts_with("Then") {
                    result_line.push_str(&" ".repeat(self.config.indent_size * 2));
                    result_line.push_str(line);
                } else {
                    result_line.push_str(line);
                }
                self.result.push(result_line);
            });
    }
}
