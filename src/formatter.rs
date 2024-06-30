use crate::config::FormatterConfig;

#[derive(PartialEq)]
enum Context {
    Feature,
    Scenario,
}

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
        let mut context_stack: Vec<Context> = Vec::new();

        for mut line in lines {
            if line.trim_start().starts_with('#') {
                self.result.push(line.to_string());
                continue;
            }

            line = line.trim_start();

            if line.is_empty() {
                self.result.push("".to_string());
                continue;
            }

            let mut result_line = String::new();

            result_line.push_str(&" ".repeat(self.config.indent_size * context_stack.len()));
            result_line.push_str(line);

            if line.starts_with("Feature:") {
                context_stack.push(Context::Feature);
            } else if line.starts_with("Scenario:")
                && context_stack.last().unwrap() != &Context::Scenario
            {
                context_stack.push(Context::Scenario);
            }

            self.result.push(result_line);
        }
    }
}
