use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub(crate) struct TailwindOptions {
    pub(crate) class_wrap_tokens_min: Option<usize>,
    pub(crate) class_wrap_tokens_per_line: Option<usize>,
    pub(crate) group_blank_lines: bool,
}

impl Default for TailwindOptions {
    fn default() -> Self {
        Self {
            class_wrap_tokens_min: None,
            class_wrap_tokens_per_line: None,
            group_blank_lines: true,
        }
    }
}

impl TailwindOptions {
    pub(crate) fn from_plugin_options(plugin_options: Option<&str>) -> Self {
        plugin_options
            .and_then(|options| serde_json::from_str::<Value>(options).ok())
            .and_then(|value| serde_json::from_value(value).ok())
            .unwrap_or_default()
    }
}
