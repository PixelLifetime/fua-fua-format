use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Plugin-specific section of the formatter config.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PluginConfig {
    /// Path to the compiled `.wasm` plugin file.
    /// When present the CLI will load it automatically (no `--plugin` flag needed).
    pub path: Option<String>,
    /// Arbitrary plugin-specific options forwarded verbatim to the WASM guest
    /// as a JSON string via `NodeData.plugin_options`.
    pub options: Option<Value>,
}

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
    pub indent_condition_groups: bool,
    pub inline_short_elements_max_len: usize,
    /// Optional WASM plugin configuration.
    pub plugin: PluginConfig,
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
            indent_condition_groups: false,
            inline_short_elements_max_len: 80,
            plugin: PluginConfig::default(),
        }
    }
}
