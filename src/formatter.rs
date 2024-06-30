use crate::config::FormatterConfig;

#[derive(PartialEq, Clone)]
enum Context {
    Feature,
    Rule,
    Scenario,
    Example,
    Background,
}

const FINAL_CONTEXTS: [Context; 3] = [Context::Scenario, Context::Background, Context::Example];

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
            let mut new_context_stack = context_stack.clone();

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

            if line.starts_with("Feature:") {
                new_context_stack.push(Context::Feature);
            } else if line.starts_with("Scenario:") {
                if FINAL_CONTEXTS.contains(context_stack.last().unwrap()) {
                    context_stack.pop();
                } else {
                    new_context_stack.push(Context::Scenario);
                }
            } else if line.starts_with("Rule:") {
                if FINAL_CONTEXTS.contains(context_stack.last().unwrap()) {
                    context_stack.pop();
                } else {
                    new_context_stack.push(Context::Rule);
                }
            } else if line.starts_with("Example:") {
                if FINAL_CONTEXTS.contains(context_stack.last().unwrap()) {
                    context_stack.pop();
                } else {
                    new_context_stack.push(Context::Example);
                }
            } else if line.starts_with("Background:") {
                if FINAL_CONTEXTS.contains(context_stack.last().unwrap()) {
                    context_stack.pop();
                } else {
                    new_context_stack.push(Context::Background);
                }
            }

            result_line.push_str(&" ".repeat(self.config.indent_size * context_stack.len()));
            result_line.push_str(line);
            self.result.push(result_line);
            context_stack = new_context_stack;
        }
    }
}
