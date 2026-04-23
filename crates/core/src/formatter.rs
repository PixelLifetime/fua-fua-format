mod content;
mod context;
mod hooks;
mod output;
mod tags;
mod traversal;

use crate::config::FormatterConfig;
use crate::plugins::PluginHost;
use crate::syntax::SyntaxNode;
use fua_plugin_api::Replacement;
use std::collections::HashMap;

pub struct Formatter {
    config: FormatterConfig,
    plugin_host: PluginHost,
}

struct FormatSession {
    config: FormatterConfig,
    plugin_host: PluginHost,
    output: String,
    current_indent: usize,
    cached_token_replacements: HashMap<(u32, u32), Option<Replacement>>,
}

impl Formatter {
    pub fn new(config: FormatterConfig) -> Self {
        Self::with_plugin_host(config, PluginHost::new())
    }

    pub fn with_plugin_host(config: FormatterConfig, plugin_host: PluginHost) -> Self {
        Self {
            config,
            plugin_host,
        }
    }

    pub fn format(self, root: &SyntaxNode) -> String {
        FormatSession::new(self.config, self.plugin_host).format(root)
    }
}

impl FormatSession {
    fn new(config: FormatterConfig, plugin_host: PluginHost) -> Self {
        Self {
            config,
            plugin_host,
            output: String::new(),
            current_indent: 0,
            cached_token_replacements: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::FormatterConfig;
    use crate::parser::Parser;
    use crate::plugins::{FormatPlugin, PluginHost};
    use crate::syntax::SyntaxNode;
    use fua_plugin_api::{HookRequest, HookResponse, Replacement};

    fn format_with_config(input: &str, config: FormatterConfig) -> String {
        let parser = Parser::new(input);
        let syntax_node = SyntaxNode::new_root(parser.parse());
        Formatter::new(config).format(&syntax_node)
    }

    #[test]
    fn formats_plain_html_without_plugins() {
        let input = "<div id=\"app\">   <p>Hello <span>world</span></p></div>";
        let output = format_with_config(input, FormatterConfig::default());
        let expected = "\n<div id=\"app\">\n  <p>\n    Hello <span>world</span>\n  </p>\n</div>";
        assert_eq!(output, expected);
    }

    struct UppercaseTextPlugin;

    impl FormatPlugin for UppercaseTextPlugin {
        fn handle_hook(&mut self, request: &HookRequest<'_>) -> HookResponse {
            match request {
                HookRequest::Token(token) if token.kind == "IDENT" && token.text == "Hello" => {
                    HookResponse::replace(Replacement::text("HELLO"))
                }
                _ => HookResponse::Continue,
            }
        }
    }

    #[test]
    fn plugin_hooks_only_apply_when_a_plugin_is_registered() {
        let input = "<p>Hello</p>";
        let parser = Parser::new(input);
        let syntax_node = SyntaxNode::new_root(parser.parse());

        let without_plugins = Formatter::new(FormatterConfig::default()).format(&syntax_node);
        assert_eq!(without_plugins, "<p>Hello</p>");

        let mut plugin_host = PluginHost::new();
        plugin_host.register(Box::new(UppercaseTextPlugin));
        let with_plugins = Formatter::with_plugin_host(FormatterConfig::default(), plugin_host)
            .format(&syntax_node);

        assert_eq!(with_plugins, "<p>HELLO</p>");
    }

    struct AttributeValuePlugin;

    impl FormatPlugin for AttributeValuePlugin {
        fn handle_hook(&mut self, request: &HookRequest<'_>) -> HookResponse {
            match request {
                HookRequest::Token(token)
                    if token.kind == "STRING_DOUBLE"
                        && token.context.attribute_name.as_deref() == Some("data-role") =>
                {
                    HookResponse::replace(Replacement::text("\"widget\""))
                }
                _ => HookResponse::Continue,
            }
        }
    }

    #[test]
    fn attribute_value_hooks_receive_the_current_attribute_name() {
        let input = r#"<div id="app" data-role="card"></div>"#;
        let parser = Parser::new(input);
        let syntax_node = SyntaxNode::new_root(parser.parse());

        let mut plugin_host = PluginHost::new();
        plugin_host.register(Box::new(AttributeValuePlugin));
        let output = Formatter::with_plugin_host(FormatterConfig::default(), plugin_host)
            .format(&syntax_node);

        assert_eq!(output, "<div id=\"app\" data-role=\"widget\"></div>");
    }

    struct MultilineClassPlugin;

    impl FormatPlugin for MultilineClassPlugin {
        fn handle_hook(&mut self, request: &HookRequest<'_>) -> HookResponse {
            match request {
                HookRequest::Token(token)
                    if token.kind == "STRING_DOUBLE"
                        && token.context.attribute_name.as_deref() == Some("class") =>
                {
                    HookResponse::replace(Replacement::text("\"\n    one\n    two\n  \""))
                }
                _ => HookResponse::Continue,
            }
        }
    }

    #[test]
    fn keeps_short_simple_nodes_inline_when_they_fit_print_width() {
        let doctype = "<!doctype html>";
        let title = "<title>Short title</title>";
        let link = "<a href=\"/x\">Link</a>";
        let span = "<span>Hello</span>";

        assert_eq!(
            format_with_config(doctype, FormatterConfig::default()),
            doctype
        );
        assert_eq!(format_with_config(title, FormatterConfig::default()), title);
        assert_eq!(format_with_config(link, FormatterConfig::default()), link);
        assert_eq!(format_with_config(span, FormatterConfig::default()), span);

        let input = "<title>Short title</title><a href=\"/x\">Link</a><span>Hello</span>";
        let output = format_with_config(input, FormatterConfig::default());

        assert_eq!(
            output,
            "<title>Short title</title><a href=\"/x\">Link</a><span>Hello</span>"
        );
    }

    #[test]
    fn wraps_long_nodes_when_the_inline_rendering_exceeds_print_width() {
        let input = "<title>A title that is definitely too long to stay on one line</title>";
        let config = FormatterConfig {
            print_width: 30,
            ..FormatterConfig::default()
        };

        let output = format_with_config(input, config);

        assert_eq!(
            output,
            "\n<title>\n  A title that is definitely too long to stay on one line\n</title>"
        );
    }

    #[test]
    fn wraps_long_tags_with_mixed_attributes_across_multiple_lines() {
        let input = r#"<a href="/x" data-id="123" aria-label="A descriptive label">Link</a>"#;
        let config = FormatterConfig {
            print_width: 32,
            ..FormatterConfig::default()
        };

        let output = format_with_config(input, config);

        assert_eq!(
            output,
            "\n<a\n  href=\"/x\"\n  data-id=\"123\"\n  aria-label=\"A descriptive label\"\n>\n  Link\n</a>"
        );
    }

    #[test]
    fn keeps_nodes_inline_when_their_length_is_exactly_the_print_width() {
        let input = "<span>Hello</span>";
        let config = FormatterConfig {
            print_width: input.chars().count(),
            ..FormatterConfig::default()
        };

        let output = format_with_config(input, config);

        assert_eq!(output, input);
    }

    #[test]
    fn multiline_attribute_replacements_force_the_tag_and_element_to_wrap() {
        let input = r#"<a href="/settings" class="one two">Link</a>"#;
        let parser = Parser::new(input);
        let syntax_node = SyntaxNode::new_root(parser.parse());

        let mut plugin_host = PluginHost::new();
        plugin_host.register(Box::new(MultilineClassPlugin));

        let output = Formatter::with_plugin_host(
            FormatterConfig {
                print_width: 200,
                ..FormatterConfig::default()
            },
            plugin_host,
        )
        .format(&syntax_node);

        assert_eq!(
            output,
            "\n<a\n  href=\"/settings\"\n  class=\"\n    one\n    two\n  \"\n>\n  Link\n</a>"
        );
    }

    #[test]
    fn preserves_script_content_verbatim() {
        let input = r#"<script>
  const sample = {
    title: "Keep JS raw",
    nested: { value: 123 }
  };

  function noop() {
    return sample;
  }
</script>"#;

        let output = format_with_config(input, FormatterConfig::default());

        assert_eq!(output, format!("\n{input}"));
    }

    #[test]
    fn preserves_style_content_verbatim() {
        let input = r#"<style>.custom-debug-box { outline: 2px solid hotpink;
}</style>"#;

        let output = format_with_config(input, FormatterConfig::default());

        assert_eq!(output, format!("\n{input}"));
    }

    #[test]
    fn rebases_raw_text_block_indentation_to_match_html_nesting() {
        let input = r#"<div>
				<script>
        const sample = {
          title: "Keep JS raw",
          nested: { value: 123 }
        };

        function noop() {
          return sample;
        }
      </script>

				<style>
        .custom-debug-box {
          outline: 2px solid hotpink;
        }
      </style>
</div>"#;

        let output = format_with_config(input, FormatterConfig::default());

        assert_eq!(
            output,
            "\n<div>\n  <script>\n    const sample = {\n      title: \"Keep JS raw\",\n      nested: { value: 123 }\n    };\n\n    function noop() {\n      return sample;\n    }\n  </script>\n\n  <style>\n    .custom-debug-box {\n      outline: 2px solid hotpink;\n    }\n  </style>\n</div>"
        );
    }
}
