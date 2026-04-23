use crate::config::{FormatterConfig, PluginConfig};
use crate::formatter::Formatter;
use crate::parser::Parser;
use crate::plugins::PluginHost;
use crate::syntax::SyntaxNode;

pub struct FormatEngine {
    config: FormatterConfig,
    plugin_host: PluginHost,
}

impl FormatEngine {
    pub fn new(config: FormatterConfig) -> Self {
        Self {
            config,
            plugin_host: PluginHost::new(),
        }
    }

    pub fn load_plugin(
        &mut self,
        path: &str,
        plugin_options: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.plugin_host.load_wasm(path, plugin_options)
    }

    pub fn load_plugin_config(
        &mut self,
        plugin: &PluginConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let Some(path) = plugin.path.as_deref() else {
            return Ok(());
        };

        self.load_plugin(path, plugin.options.as_ref().map(|value| value.to_string()))
    }

    pub fn format(self, input: &str) -> String {
        let syntax_root = parse_syntax_tree(input);
        self.into_formatter().format(&syntax_root)
    }

    fn into_formatter(self) -> Formatter {
        Formatter::with_plugin_host(self.config, self.plugin_host)
    }
}

fn parse_syntax_tree(input: &str) -> SyntaxNode {
    let parser = Parser::new(input);
    SyntaxNode::new_root(parser.parse())
}
