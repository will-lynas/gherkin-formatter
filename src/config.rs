pub enum TrailingNewlineOption {
    Add,
    NoChange,
}

pub struct FormatterConfig {
    pub indent_size: usize,
    pub add_trailing_newline: TrailingNewlineOption,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        FormatterConfig {
            indent_size: 2,
            add_trailing_newline: TrailingNewlineOption::Add,
        }
    }
}
