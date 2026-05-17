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
    /// as a JSON string on each generic hook request.
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
    pub class_wrap_tokens_min: Option<usize>,
    pub class_wrap_tokens_per_line: usize,
    /// Glob patterns for files to include when using `--all` or `--only-staged`.
    /// Default: all files (`["*"]`).
    pub include: Vec<String>,
    /// Regex patterns (or plain path substrings) for files to skip.
    pub exclude: Vec<String>,
    /// Preferred plugin list for the formatter host.
    pub plugins: Vec<PluginConfig>,
    /// Legacy single-plugin configuration kept for backward compatibility.
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
            class_wrap_tokens_min: None,
            class_wrap_tokens_per_line: 1,
            include: vec!["*".to_string()],
            exclude: Vec::new(),
            plugins: Vec::new(),
            plugin: PluginConfig::default(),
        }
    }
}

impl FormatterConfig {
    pub fn plugin_configs(&self) -> Vec<PluginConfig> {
        let mut plugins = self.plugins.clone();
        if self.plugin.path.is_some() {
            plugins.push(self.plugin.clone());
        }
        plugins
    }
}
