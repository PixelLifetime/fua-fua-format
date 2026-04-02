use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FormatterConfig {
    pub indent_size: usize,
    pub use_tabs: bool,
    pub print_width: usize,
    pub bracket_same_line: bool,
    pub wrap_attributes: bool,
    pub single_quotes: bool,
    pub wrap_content: bool,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            indent_size: 2,
            use_tabs: false,
            print_width: 80,
            bracket_same_line: false,
            wrap_attributes: false,
            single_quotes: false,
            wrap_content: false,
        }
    }
}
