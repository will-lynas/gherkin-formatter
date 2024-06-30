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
        let lines = self.input.as_ref().unwrap().split('\n');
        for mut line in lines {
            let mut result_line = String::new();
            if line.trim_start().starts_with('#') {
                result_line.push_str(line);
            } else {
                line = line.trim_start();
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
            }
            self.result.push(result_line);
        }
    }
}
