mod attributes;
mod context;
mod expressions;
mod hooks;
mod response;
mod state;

pub use hooks::dispatch_hook;

#[cfg(target_arch = "wasm32")]
use extism_pdk::*;
#[cfg(target_arch = "wasm32")]
use fua_plugin_api::{HookRequest, HookResponse};

#[cfg(target_arch = "wasm32")]
#[plugin_fn]
pub fn handle_hook(input: String) -> FnResult<String> {
    let request = match serde_json::from_str::<HookRequest<'static>>(&input) {
        Ok(request) => request,
        Err(_) => return Ok(String::new()),
    };

    let response = dispatch_hook(request);
    match response {
        HookResponse::Continue => Ok(String::new()),
        other => Ok(serde_json::to_string(&other).unwrap_or_default()),
    }
}

#[cfg(test)]
mod tests {
    use super::dispatch_hook;
    use fua_core::config::FormatterConfig;
    use fua_core::formatter::Formatter;
    use fua_core::parser::Parser;
    use fua_core::plugins::{FormatPlugin, PluginHost};
    use fua_core::syntax::SyntaxNode;
    use fua_plugin_api::{
        HookContext, HookRequest, HookResponse, LeadingSpacing, NodeHook, NodePhase, Replacement,
        TokenHook,
    };
    use std::borrow::Cow;

    fn token_request(
        kind: &'static str,
        text: &'static str,
        parent_kind: &'static str,
    ) -> HookRequest<'static> {
        HookRequest::Token(TokenHook {
            kind: Cow::Borrowed(kind),
            text: Cow::Borrowed(text),
            context: HookContext::new(parent_kind, None, None, 2, 2, false),
            plugin_options: None,
        })
    }

    #[test]
    fn resets_state_on_root_enter() {
        let request = HookRequest::Node(NodeHook {
            phase: NodePhase::Enter,
            kind: Cow::Borrowed("ROOT"),
            text: Cow::Borrowed(""),
            tag_name: None,
            context: HookContext::new("NONE", None, None, 0, 2, false),
            plugin_options: None,
        });

        let response = dispatch_hook(request);
        assert_eq!(response, HookResponse::Continue);
    }

    #[test]
    fn block_openers_start_on_a_new_line() {
        let _ = dispatch_hook(HookRequest::Node(NodeHook {
            phase: NodePhase::Enter,
            kind: Cow::Borrowed("ROOT"),
            text: Cow::Borrowed(""),
            tag_name: None,
            context: HookContext::new("NONE", None, None, 0, 2, false),
            plugin_options: None,
        }));
        let response = dispatch_hook(token_request("IDENT", "@if", "ROOT"));
        assert_eq!(
            response,
            HookResponse::replace(Replacement::text("@if").with_leading(LeadingSpacing::LineBreak),)
        );
    }

    struct AngularNativePlugin {
        plugin_options: Option<String>,
    }

    impl FormatPlugin for AngularNativePlugin {
        fn handle_hook(&mut self, request: &HookRequest<'_>) -> HookResponse {
            let owned = request
                .clone()
                .with_plugin_options(self.plugin_options.as_deref().map(Cow::Borrowed));
            dispatch_hook(owned.into_owned())
        }
    }

    trait IntoOwnedHookRequest {
        fn into_owned(self) -> HookRequest<'static>;
    }

    impl IntoOwnedHookRequest for HookRequest<'_> {
        fn into_owned(self) -> HookRequest<'static> {
            match self {
                HookRequest::Node(node) => HookRequest::Node(fua_plugin_api::NodeHook {
                    phase: node.phase,
                    kind: Cow::Owned(node.kind.into_owned()),
                    text: Cow::Owned(node.text.into_owned()),
                    tag_name: node.tag_name.map(|value| Cow::Owned(value.into_owned())),
                    context: fua_plugin_api::HookContext {
                        parent_kind: Cow::Owned(node.context.parent_kind.into_owned()),
                        tag_name: node.context.tag_name.map(|value| Cow::Owned(value.into_owned())),
                        attribute_name: node
                            .context
                            .attribute_name
                            .map(|value| Cow::Owned(value.into_owned())),
                        previous_kind: node
                            .context
                            .previous_kind
                            .map(|value| Cow::Owned(value.into_owned())),
                        previous_text: node
                            .context
                            .previous_text
                            .map(|value| Cow::Owned(value.into_owned())),
                        next_kind: node.context.next_kind.map(|value| Cow::Owned(value.into_owned())),
                        next_text: node.context.next_text.map(|value| Cow::Owned(value.into_owned())),
                        current_indent: node.context.current_indent,
                        indent_size: node.context.indent_size,
                        use_tabs: node.context.use_tabs,
                        class_wrap_tokens_min: node.context.class_wrap_tokens_min,
                        class_wrap_tokens_per_line: node.context.class_wrap_tokens_per_line,
                    },
                    plugin_options: node.plugin_options.map(|value| Cow::Owned(value.into_owned())),
                }),
                HookRequest::Token(token) => HookRequest::Token(fua_plugin_api::TokenHook {
                    kind: Cow::Owned(token.kind.into_owned()),
                    text: Cow::Owned(token.text.into_owned()),
                    context: fua_plugin_api::HookContext {
                        parent_kind: Cow::Owned(token.context.parent_kind.into_owned()),
                        tag_name: token.context.tag_name.map(|value| Cow::Owned(value.into_owned())),
                        attribute_name: token
                            .context
                            .attribute_name
                            .map(|value| Cow::Owned(value.into_owned())),
                        previous_kind: token
                            .context
                            .previous_kind
                            .map(|value| Cow::Owned(value.into_owned())),
                        previous_text: token
                            .context
                            .previous_text
                            .map(|value| Cow::Owned(value.into_owned())),
                        next_kind: token.context.next_kind.map(|value| Cow::Owned(value.into_owned())),
                        next_text: token.context.next_text.map(|value| Cow::Owned(value.into_owned())),
                        current_indent: token.context.current_indent,
                        indent_size: token.context.indent_size,
                        use_tabs: token.context.use_tabs,
                        class_wrap_tokens_min: token.context.class_wrap_tokens_min,
                        class_wrap_tokens_per_line: token.context.class_wrap_tokens_per_line,
                    },
                    plugin_options: token.plugin_options.map(|value| Cow::Owned(value.into_owned())),
                }),
            }
        }
    }

    #[derive(Clone, Copy)]
    struct ChunkConfig {
        indent_size: usize,
        use_tabs: bool,
        wrap_conditions_min: usize,
        ngclass_wrap_entries_min: usize,
    }

    struct ChunkScenario<'a> {
        condition_expr: &'a str,
        ngclass_entries: &'a [(&'a str, &'a str)],
        config: ChunkConfig,
    }

    fn format_with_angular_plugin(input: &str, config: ChunkConfig) -> String {
        let parser = Parser::new(input);
        let syntax_node = SyntaxNode::new_root(parser.parse());
        let mut plugin_host = PluginHost::new();
        let plugin_options = format!(
            r#"{{"wrap_conditions_min":{},"wrap_conditions_in_parens":true,"ngclass_wrap_entries_min":{}}}"#,
            config.wrap_conditions_min, config.ngclass_wrap_entries_min
        );
        plugin_host.register(Box::new(AngularNativePlugin {
            plugin_options: Some(plugin_options),
        }));
        Formatter::with_plugin_host(
            FormatterConfig {
                print_width: 200,
                indent_size: config.indent_size,
                use_tabs: config.use_tabs,
                ..FormatterConfig::default()
            },
            plugin_host,
        )
        .format(&syntax_node)
    }

    fn indent(depth: usize, config: ChunkConfig) -> String {
        if config.use_tabs {
            "\t".repeat(depth)
        } else {
            " ".repeat(depth * config.indent_size)
        }
    }

    fn split_condition(expr: &str) -> Vec<(String, String)> {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut chars = expr.chars().peekable();
        let mut pending_op: Option<String> = None;
        while let Some(ch) = chars.next() {
            if (ch == '&' || ch == '|') && chars.peek() == Some(&ch) {
                let expr_part = current.trim();
                if !expr_part.is_empty() {
                    parts.push((pending_op.take().unwrap_or_default(), expr_part.to_string()));
                }
                current.clear();
                pending_op = Some(format!("{ch}{ch}"));
                let _ = chars.next();
            } else {
                current.push(ch);
            }
        }
        if !current.trim().is_empty() {
            parts.push((pending_op.unwrap_or_default(), current.trim().to_string()));
        }
        parts
    }

    fn generate_big_chunk_input(scenario: &ChunkScenario<'_>) -> String {
        let ngclass_body = scenario
            .ngclass_entries
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "<section><div [ngClass]=\"{{{}}}\">@if ({}) {{<span>OK</span>}}</div></section>",
            ngclass_body, scenario.condition_expr
        )
    }

    fn generate_big_chunk_expected(scenario: &ChunkScenario<'_>) -> String {
        let cfg = scenario.config;
        let i1 = indent(1, cfg);
        let i2 = indent(2, cfg);
        let i3 = indent(3, cfg);
        let i4 = indent(4, cfg);

        let condition_parts = split_condition(scenario.condition_expr);
        let should_wrap_condition = condition_parts.len().saturating_sub(1) >= cfg.wrap_conditions_min;

        let should_wrap_ngclass =
            scenario.ngclass_entries.len() >= cfg.ngclass_wrap_entries_min.max(1);

        let mut lines = vec!["<section>".to_string(), format!("{i1}<div")];
        if should_wrap_ngclass {
            lines.push(format!("{i2}[ngClass]=\""));
            lines.push(format!("{i3}{{"));
            for (idx, (key, value)) in scenario.ngclass_entries.iter().enumerate() {
                let suffix = if idx + 1 == scenario.ngclass_entries.len() {
                    ""
                } else {
                    ","
                };
                lines.push(format!("{i4}{key}: {value}{suffix}"));
            }
            lines.push(format!("{i3}}}"));
            lines.push(format!("{i2}\""));
        } else {
            let inline = scenario
                .ngclass_entries
                .iter()
                .map(|(k, v)| format!("{k}: {v}"))
                .collect::<Vec<_>>()
                .join(", ");
            lines.push(format!("{i2}[ngClass]=\"{{{inline}}}\""));
        }
        lines.push(format!("{i1}>"));

        if should_wrap_condition && !condition_parts.is_empty() {
            lines.push(format!("{i2}@if ("));
            lines.push(format!("{i3}   {}", condition_parts[0].1));
            for (op, expr) in condition_parts.iter().skip(1) {
                lines.push(format!("{i3}{op} {expr}"));
            }
            lines.push(format!("{i2}) {{<span>OK</span>"));
        } else {
            lines.push(format!("{i2}@if ({}) {{<span>OK</span>", scenario.condition_expr));
        }

        lines.push(format!("{i2}}}"));
        lines.push(format!("{i1}</div>"));
        lines.push("</section>".to_string());
        lines.join("\n")
    }

    #[test]
    fn big_chunk_generator_matches_full_html_formatting_result() {
        let scenario = ChunkScenario {
            condition_expr: "user && isAdmin || hasFeature",
            ngclass_entries: &[
                ("'border-accent'", "this.hasImageChanges"),
                ("'border-quaternary'", "!this.hasImageChanges"),
            ],
            config: ChunkConfig {
                indent_size: 4,
                use_tabs: true,
                wrap_conditions_min: 2,
                ngclass_wrap_entries_min: 2,
            },
        };
        let input = generate_big_chunk_input(&scenario);
        let expected = generate_big_chunk_expected(&scenario);
        let actual = format_with_angular_plugin(&input, scenario.config);
        assert_eq!(expected, actual);
    }

}
