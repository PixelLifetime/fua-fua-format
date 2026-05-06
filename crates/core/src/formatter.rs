mod classes;
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
        let expected = "<div id=\"app\">\n  <p>\n    Hello <span>world</span>\n  </p>\n</div>";
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
            "<title>\n  A title that is definitely too long to stay on one line\n</title>"
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
            "<a\n  href=\"/x\"\n  data-id=\"123\"\n  aria-label=\"A descriptive label\"\n>\n  Link\n</a>"
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
            "<a\n  href=\"/settings\"\n  class=\"\n    one\n    two\n  \"\n>\n  Link\n</a>"
        );
    }

    #[test]
    fn wraps_class_tokens_using_configured_tokens_per_line() {
        let input = r#"<button class="flex flex-col items-center px-4 py-2 text-sm"></button>"#;
        let config = FormatterConfig {
            print_width: 200,
            class_wrap_tokens_min: Some(6),
            class_wrap_tokens_per_line: 2,
            ..FormatterConfig::default()
        };

        let output = format_with_config(input, config);

        assert_eq!(
            output,
            "<button\n  class=\"\n    flex flex-col\n    items-center px-4\n    py-2 text-sm\n  \"\n>\n</button>"
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

        assert_eq!(output, input);
    }

    #[test]
    fn preserves_style_content_verbatim() {
        let input = r#"<style>.custom-debug-box { outline: 2px solid hotpink;
}</style>"#;

        let output = format_with_config(input, FormatterConfig::default());

        assert_eq!(output, input);
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
            "<div>\n  <script>\n    const sample = {\n      title: \"Keep JS raw\",\n      nested: { value: 123 }\n    };\n\n    function noop() {\n      return sample;\n    }\n  </script>\n\n  <style>\n    .custom-debug-box {\n      outline: 2px solid hotpink;\n    }\n  </style>\n</div>"
        );
    }

    #[test]
    fn collapses_blank_lines_immediately_after_open_tag() {
        let input = "<span>\n\n\n{{ 'playlist' | translate }}\n</span>";
        let output = format_with_config(
            input,
            FormatterConfig {
                print_width: 20,
                ..FormatterConfig::default()
            },
        );

        assert_eq!(output, "<span>\n  {{ 'playlist' | translate }}\n</span>");
    }

    #[test]
    fn keeps_class_interpolation_unchanged_when_class_wrapping_is_enabled() {
        let input = r#"<div class="{{ this.getPositionClass() }} fixed p-1.5"></div>"#;
        let output = format_with_config(
            input,
            FormatterConfig {
                print_width: 200,
                class_wrap_tokens_min: Some(1),
                class_wrap_tokens_per_line: 2,
                ..FormatterConfig::default()
            },
        );

        assert!(output.contains("{{ this.getPositionClass() }}"));
        assert!(!output.contains("{{\n"));
        assert!(!output.contains("}}\n"));
    }

    #[test]
    fn preserves_multiline_html_comment_block() {
        let input = r#"<div></div>
<!--
<div class="flex gap-4 p-4">
  <button (click)="warning.visible = true">warn</button>
</div>
-->
<p>after</p>"#;
        let output = format_with_config(input, FormatterConfig::default());

        assert!(output.contains("<!--"));
        assert!(output.contains("<button (click)=\"warning.visible = true\">warn</button>"));
        assert!(output.contains("-->"));
    }

    struct ClassWrapScenario {
        input_tokens: &'static [&'static str],
        expected_token_order: &'static [&'static str],
        tokens_per_line: usize,
        indent_size: usize,
        use_tabs: bool,
    }

    fn join_chunks(tokens: &[&str], tokens_per_line: usize) -> Vec<String> {
        tokens
            .chunks(tokens_per_line.max(1))
            .map(|chunk| chunk.join(" "))
            .collect()
    }

    fn indent(depth: usize, indent_size: usize, use_tabs: bool) -> String {
        if use_tabs {
            "\t".repeat(depth)
        } else {
            " ".repeat(depth * indent_size)
        }
    }

    fn generate_expected_button_with_class(lines: &[String], indent_size: usize, use_tabs: bool) -> String {
        let tag_indent = indent(1, indent_size, use_tabs);
        let class_indent = indent(2, indent_size, use_tabs);
        let close_quote_indent = tag_indent.clone();

        let mut out = String::from("<button\n");
        out.push_str(&tag_indent);
        out.push_str("class=\"\n");
        for line in lines {
            out.push_str(&class_indent);
            out.push_str(line);
            out.push('\n');
        }
        out.push_str(&close_quote_indent);
        out.push_str("\"\n>\n</button>");
        out 
    }

    fn run_class_wrap_scenario(s: ClassWrapScenario) {
        let input = format!(r#"<button class="{}"></button>"#, s.input_tokens.join(" "));
        let config = FormatterConfig {
            print_width: 200,
            class_wrap_tokens_min: Some(1),
            class_wrap_tokens_per_line: s.tokens_per_line,
            indent_size: s.indent_size,
            use_tabs: s.use_tabs,
            ..FormatterConfig::default()
        };
        let actual = format_with_config(&input, config);
        let generated_lines = join_chunks(s.expected_token_order, s.tokens_per_line);
        let expected =
            generate_expected_button_with_class(&generated_lines, s.indent_size, s.use_tabs);
        assert_eq!(expected, actual);
    }

    #[test]
    fn generative_class_wrapping_scenarios() {
        let unsorted = &["py-2", "flex", "items-center", "text-sm", "flex-col", "px-4"];

        for tokens_per_line in 1..=3 {
            run_class_wrap_scenario(ClassWrapScenario {
                input_tokens: unsorted,
                expected_token_order: unsorted,
                tokens_per_line,
                indent_size: 2,
                use_tabs: false,
            });
        }
    }

    #[test]
    fn generative_class_wrapping_with_tabs() {
        let unsorted = &["items-center", "text-sm", "py-2", "flex-col", "px-4", "flex"];

        run_class_wrap_scenario(ClassWrapScenario {
            input_tokens: unsorted,
            expected_token_order: unsorted,
            tokens_per_line: 2,
            indent_size: 4,
            use_tabs: true,
        });
    }

    #[test]
    fn generative_class_wrapping_with_non_divisible_chunk_size() {
        let unsorted = &["w-full", "flex", "items-center", "justify-center", "px-4"];

        run_class_wrap_scenario(ClassWrapScenario {
            input_tokens: unsorted,
            expected_token_order: unsorted,
            tokens_per_line: 4,
            indent_size: 2,
            use_tabs: false,
        });
    }

    #[test]
    fn generative_class_wrapping_for_deep_indent_context() {
        // Realistic template payload: class wrapping inside nested element trees should
        // still chunk lines correctly regardless of nesting depth in the output.
        let input_tokens = &[
            "grid",
            "grid-cols-12",
            "gap-4",
            "md:grid-cols-6",
            "lg:grid-cols-4",
            "items-start",
            "w-full",
        ];
        let generated_lines = join_chunks(input_tokens, 3);
        let expected = generate_expected_button_with_class(&generated_lines, 2, false);
        let input = format!(r#"<button class="{}"></button>"#, input_tokens.join(" "));
        let actual = format_with_config(
            &input,
            FormatterConfig {
                print_width: 200,
                class_wrap_tokens_min: Some(1),
                class_wrap_tokens_per_line: 3,
                indent_size: 2,
                use_tabs: false,
                ..FormatterConfig::default()
            },
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn generative_class_wrapping_respects_min_threshold_toggle() {
        let tokens = &["flex", "items-center", "gap-2"];
        let input = format!(r#"<button class="{}"></button>"#, tokens.join(" "));
        let config_no_wrap = FormatterConfig {
            print_width: 200,
            class_wrap_tokens_min: Some(10),
            class_wrap_tokens_per_line: 2,
            ..FormatterConfig::default()
        };
        let not_wrapped = format_with_config(&input, config_no_wrap);
        assert_eq!(not_wrapped, format!(r#"<button class="{}"></button>"#, tokens.join(" ")));

        let config_wrap = FormatterConfig {
            print_width: 200,
            class_wrap_tokens_min: Some(1),
            class_wrap_tokens_per_line: 2,
            ..FormatterConfig::default()
        };
        let wrapped = format_with_config(&input, config_wrap);
        let expected_lines = join_chunks(tokens, 2);
        let expected = generate_expected_button_with_class(&expected_lines, 2, false);
        assert_eq!(expected, wrapped);
    }

    fn generate_button_block(tokens: &[&str], tokens_per_line: usize, indent_size: usize) -> String {
        let line_indent = " ".repeat(indent_size * 2);
        let class_indent = " ".repeat(indent_size * 3);
        let close_quote_indent = line_indent.clone();
        let class_lines = tokens
            .chunks(tokens_per_line.max(1))
            .map(|chunk| chunk.join(" "))
            .collect::<Vec<_>>();

        let mut out = String::from("  <button\n");
        out.push_str(&line_indent);
        out.push_str("class=\"\n");
        for line in class_lines {
            out.push_str(&class_indent);
            out.push_str(&line);
            out.push('\n');
        }
        out.push_str(&close_quote_indent);
        out.push_str("\"\n  >\n  </button>");
        out
    }

    #[test]
    fn generative_chunk_formats_multiple_wrapped_nodes_consistently() {
        let first = &["flex", "items-center", "justify-center", "px-4"];
        let second = &["grid", "grid-cols-2", "gap-2"];
        let input = format!(
            "<section><button class=\"{}\"></button><button class=\"{}\"></button></section>",
            first.join(" "),
            second.join(" ")
        );
        let config = FormatterConfig {
            print_width: 200,
            class_wrap_tokens_min: Some(1),
            class_wrap_tokens_per_line: 2,
            indent_size: 2,
            use_tabs: false,
            ..FormatterConfig::default()
        };

        let actual = format_with_config(&input, config);
        let expected = format!(
            "<section>\n{}\n{}\n</section>",
            generate_button_block(first, 2, 2),
            generate_button_block(second, 2, 2)
        );

        assert_eq!(expected, actual);
    }
}
