pub struct FormatterConfig {
    pub indent_size: usize,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        FormatterConfig { indent_size: 2 }
    }
}
