use extism::{Manifest, Plugin, Wasm};
use serde::{Deserialize, Serialize};

// ── Data contract shared between host and all guest plugins ──────────────────

/// Serialized and sent to the WASM plugin for every token the host wants
/// to offer for pre-emption.  The plugin returns [`PluginResult`] if it
/// claims the token, or an empty string / error to pass through.
#[derive(Debug, Serialize)]
pub struct NodeData<'a> {
    /// SyntaxKind as a human-readable string: `"IDENT"`, `"TEXT"`, `"STRING_DOUBLE"`, …
    pub kind: &'a str,
    /// Raw source text of the token (e.g. `"@if"`, `"{"`, `"\"value\""`).
    pub text: &'a str,
    /// SyntaxKind of the **parent** node: `"ELEMENT"`, `"ROOT"`, `"OPEN_TAG"`, …
    pub parent_kind: &'a str,
    /// For tokens inside a tag, the name of the attribute this token belongs to
    /// (e.g. `"[ngClass]"`).  Empty string when not inside an attribute.
    pub attribute_name: &'a str,
    /// Current indentation depth at the point this token is about to be emitted.
    pub current_indent: usize,
    /// Spaces per indent level (only meaningful when `use_tabs` is false).
    pub indent_size: usize,
    /// Whether the formatter is configured to use tab characters.
    pub use_tabs: bool,
    /// JSON string of plugin-specific options from `config.plugin.options`.
    /// `None` when no plugin section is present in the config.
    pub plugin_options: Option<&'a str>,
}

/// What the WASM plugin tells the formatter to do for a given token.
///
/// If the plugin returns an empty string the formatter falls back to its
/// built-in logic.  Otherwise it deserialises this struct and hands control
/// to [`Formatter::emit_plugin_result`].
#[derive(Debug, Deserialize)]
pub struct PluginResult {
    /// The literal text to push into the formatter output for this token.
    pub output: String,
    /// Signed indent adjustment applied **around** `output`:
    ///   - negative → decrement `current_indent` *before* emitting (e.g. closing `}`)
    ///   - positive → increment `current_indent` *after* emitting (e.g. opening `{`)
    pub indent_delta: i32,
    /// When `true` the formatter calls `push_newlines_with_indent(1)` before
    /// pushing `output`, ensuring a clean newline + correct indentation prefix.
    pub prepend_newline: bool,
    /// When `true` the formatter ensures at least one space immediately before
    /// `output` (idempotent — will not double-up an existing trailing space).
    pub prepend_space: bool,
}

// ── Plugin host ───────────────────────────────────────────────────────────────

pub struct PluginManager {
    plugin: Option<Plugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugin: None }
    }

    /// Returns `true` if a WASM plugin is currently loaded.
    pub fn has_plugin(&self) -> bool {
        self.plugin.is_some()
    }

    /// Load a compiled `.wasm` plugin from `path`.
    /// The plugin must export a function named `format_hook`.
    pub fn load_wasm(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let manifest = Manifest::new([Wasm::file(path)]);
        let plugin = Plugin::new(&manifest, [], true)?;
        self.plugin = Some(plugin);
        Ok(())
    }

    /// Offer `node_data` to the loaded plugin.
    ///
    /// Returns `Some(PluginResult)` when the plugin claims the token and
    /// provides formatting instructions.  Returns `None` (pass-through) when:
    ///   - no plugin is loaded
    ///   - the plugin function returns an empty string
    ///   - any serialisation / call error occurs
    pub fn call_format_hook(&mut self, node_data: &NodeData<'_>) -> Option<PluginResult> {
        let plugin = self.plugin.as_mut()?;
        let input = serde_json::to_string(node_data).ok()?;
        let response: String = plugin.call("format_hook", input).ok()?;
        if response.is_empty() {
            return None;
        }
        serde_json::from_str(&response).ok()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
