use crate::attributes::process_attribute_string;
use crate::context::{is_angular_binding, is_block_opener, is_content_context, is_else_like};
use crate::expressions::split_on_top_level_ops;
use crate::response::{HookResponseExt, replacement, replacement_with_spacing};
use crate::state::{read_state, reset_state, with_state};
use fua_plugin_api::{
    HookRequest, HookResponse, LeadingSpacing, NodeHook, NodePhase, Replacement, TokenHook,
};
use serde_json::{Value, json};

pub fn dispatch_hook(request: HookRequest<'static>) -> HookResponse {
    match request {
        HookRequest::Node(node) => handle_node_hook(&node),
        HookRequest::Token(token) => handle_token_hook(&token),
    }
}

fn handle_node_hook(node: &NodeHook<'_>) -> HookResponse {
    if node.phase == NodePhase::Enter && node.kind == "ROOT" {
        reset_state();
    }
    HookResponse::Continue
}

fn handle_token_hook(token: &TokenHook<'_>) -> HookResponse {
    if should_process_attribute_string(token) {
        return handle_attribute_string(token);
    }

    if !is_content_context(token.context.parent_kind.as_ref()) {
        return HookResponse::Continue;
    }

    if let Some(response) = maybe_capture_condition_token(token) {
        return response;
    }

    match token.kind.as_ref() {
        "IDENT" => handle_content_ident(token),
        "WHITESPACE" => handle_content_whitespace(token),
        "TEXT" => handle_content_text(token),
        _ => HookResponse::Continue,
    }
}

fn should_process_attribute_string(token: &TokenHook<'_>) -> bool {
    matches!(token.kind.as_ref(), "STRING_DOUBLE" | "STRING_SINGLE")
        && matches!(
            token.context.parent_kind.as_ref(),
            "OPEN_TAG" | "SELF_CLOSING_TAG"
        )
        && token
            .context
            .attribute_name
            .as_deref()
            .is_some_and(is_angular_binding)
}

fn handle_attribute_string(token: &TokenHook<'_>) -> HookResponse {
    let options = plugin_options(token.plugin_options.as_deref());
    let attr_name = token.context.attribute_name.as_deref().unwrap_or_default();

    if let Some(output) = process_attribute_string(
        token.text.as_ref(),
        attr_name,
        &options,
        token.context.current_indent,
        token.context.indent_size,
        token.context.use_tabs,
    ) {
        replacement(output)
    } else {
        HookResponse::Continue
    }
}

fn handle_content_ident(token: &TokenHook<'_>) -> HookResponse {
    if is_block_opener(token.text.as_ref()) {
        let _ = with_state(|state| {
            state.awaiting_condition_start = true;
        });
        return replacement_with_spacing(token.text.as_ref(), LeadingSpacing::LineBreak);
    }

    if is_else_like(token.text.as_ref()) {
        return replacement_with_spacing(token.text.as_ref(), LeadingSpacing::Space);
    }

    if let Some(response) = maybe_emit_first_condition_token(token.text.as_ref()) {
        return response;
    }

    HookResponse::Continue
}

fn handle_content_whitespace(token: &TokenHook<'_>) -> HookResponse {
    let inside_condition = read_state(|state| state.condition_depth > 0).unwrap_or(false);
    if !inside_condition {
        return HookResponse::Continue;
    }

    if token.context.next_text.as_deref() == Some("&&")
        || token.context.next_text.as_deref() == Some("||")
        || token.context.next_text.as_deref() == Some(")")
    {
        return replacement("");
    }

    if token.context.previous_text.as_deref() == Some("&&")
        || token.context.previous_text.as_deref() == Some("||")
    {
        return replacement("");
    }

    if matches!(
        token.context.next_kind.as_deref(),
        Some("STRING_SINGLE") | Some("STRING_DOUBLE")
    ) {
        return replacement(" ");
    }

    if token.text.contains('\n') || token.text.contains('\r') {
        return replacement("");
    }

    replacement(" ")
}

fn handle_content_text(token: &TokenHook<'_>) -> HookResponse {
    if let Some(response) = maybe_condition_operator(token) {
        return response;
    }

    if let Some(response) = maybe_emit_first_condition_token(token.text.as_ref()) {
        return response;
    }

    match token.text.as_ref() {
        "{" => replacement_with_spacing("{", LeadingSpacing::Space).map_indent(0, 1),
        "}" => HookResponse::replace(
            Replacement::text("}")
                .with_indent(-1, 0)
                .with_leading(LeadingSpacing::LineBreak),
        ),
        _ => HookResponse::Continue,
    }
}

fn maybe_emit_first_condition_token(text: &str) -> Option<HookResponse> {
    with_state(|state| {
        if !state.first_condition_token || state.condition_depth == 0 {
            return None;
        }
        state.first_condition_token = false;
        Some(replacement(format!("   {text}")))
    })
    .flatten()
}

fn maybe_condition_operator(token: &TokenHook<'_>) -> Option<HookResponse> {
    let inside_condition = read_state(|state| state.condition_depth > 0).unwrap_or(false);
    if !inside_condition {
        return None;
    }

    let op = token.text.trim();
    if op != "&&" && op != "||" {
        return None;
    }

    let _ = with_state(|state| {
        state.first_condition_token = false;
    });

    let inner = inner_indent(
        token.context.current_indent,
        token.context.indent_size,
        token.context.use_tabs,
    );
    Some(replacement(format!("\n{inner}{op} ")))
}

fn maybe_capture_condition_token(token: &TokenHook<'_>) -> Option<HookResponse> {
    let should_start = read_state(|state| state.awaiting_condition_start).unwrap_or(false);
    let currently_capturing = read_state(|state| state.capturing_condition).unwrap_or(false);

    if should_start && token.text.contains('(') {
        let options = plugin_options(token.plugin_options.as_deref());
        let min_ops = options
            .get("wrap_conditions_min")
            .and_then(Value::as_u64)
            .unwrap_or(2)
            .saturating_sub(1) as usize;

        let text = token.text.as_ref();
        let initial_depth = paren_delta(text, 0);
        let _ = with_state(|state| {
            state.awaiting_condition_start = false;
            state.capturing_condition = true;
            state.captured_condition.clear();
            state.captured_condition.push_str(text);
            state.condition_depth = initial_depth;
            state.condition_base_indent = token.context.current_indent;
            state.first_condition_token = initial_depth > 0;
        });

        if initial_depth == 0 {
            let output = format_captured_condition(
                text,
                min_ops,
                token.context.current_indent,
                token.context.indent_size,
                token.context.use_tabs,
            );
            let _ = with_state(|state| {
                state.capturing_condition = false;
                state.captured_condition.clear();
                state.condition_depth = 0;
            });
            return Some(replacement(output));
        }

        return Some(replacement(""));
    }

    if !currently_capturing {
        return None;
    }

    let text = token.text.as_ref();
    let (next_depth, captured) = with_state(|state| {
        state.captured_condition.push_str(text);
        state.condition_depth = paren_delta(text, state.condition_depth);
        (state.condition_depth, state.captured_condition.clone())
    })?;

    if next_depth == 0 {
        let options = plugin_options(token.plugin_options.as_deref());
        let min_ops = options
            .get("wrap_conditions_min")
            .and_then(Value::as_u64)
            .unwrap_or(2)
            .saturating_sub(1) as usize;

        let output = format_captured_condition(
            &captured,
            min_ops,
            token.context.current_indent,
            token.context.indent_size,
            token.context.use_tabs,
        );
        let _ = with_state(|state| {
            state.capturing_condition = false;
            state.captured_condition.clear();
            state.condition_depth = 0;
        });
        return Some(replacement(output));
    }

    Some(replacement(""))
}

fn paren_delta(text: &str, initial_depth: usize) -> usize {
    let bytes = text.as_bytes();
    let mut depth: i32 = initial_depth as i32;
    let mut in_quote: Option<u8> = None;
    let mut i = 0usize;

    while i < bytes.len() {
        match in_quote {
            Some(quote) => {
                if bytes[i] == b'\\' {
                    i += 2;
                    continue;
                }
                if bytes[i] == quote {
                    in_quote = None;
                }
            }
            None => match bytes[i] {
                b'"' | b'\'' | b'`' => in_quote = Some(bytes[i]),
                b'(' => depth += 1,
                b')' => depth = (depth - 1).max(0),
                _ => {}
            },
        }
        i += 1;
    }

    depth as usize
}

fn format_captured_condition(
    captured: &str,
    min_ops: usize,
    current_indent: usize,
    indent_size: usize,
    use_tabs: bool,
) -> String {
    let Some(open_idx) = captured.find('(') else {
        return captured.to_string();
    };
    let Some(close_idx) = find_matching_paren(captured, open_idx) else {
        return captured.to_string();
    };

    let inner = captured[open_idx + 1..close_idx]
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let suffix = captured[close_idx + 1..].trim();
    let parts = split_on_top_level_ops(&inner);

    if parts.len() < 2 || parts.len().saturating_sub(1) < min_ops {
        if suffix.is_empty() {
            return format!("({inner})");
        }
        return format!("({inner}) {suffix}");
    }

    let inner_indent = inner_indent(current_indent, indent_size, use_tabs);
    let base = base_indent(current_indent, indent_size, use_tabs);

    let op_width = parts.get(1).map(|(op, _)| op.len() + 1).unwrap_or(3);
    let first_pad = " ".repeat(op_width);

    let mut lines = Vec::with_capacity(parts.len() + 2);
    lines.push("(".to_string());
    lines.push(format!("{inner_indent}{first_pad}{}", parts[0].1));
    for (op, expr) in parts.into_iter().skip(1) {
        lines.push(format!("{inner_indent}{op} {expr}"));
    }

    if suffix.is_empty() {
        lines.push(format!("{base})"));
    } else {
        lines.push(format!("{base}) {suffix}"));
    }

    lines.join("\n")
}

fn find_matching_paren(text: &str, open_index: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut depth: i32 = 0;
    let mut in_quote: Option<u8> = None;
    let mut i = open_index;

    while i < bytes.len() {
        match in_quote {
            Some(quote) => {
                if bytes[i] == b'\\' {
                    i += 2;
                    continue;
                }
                if bytes[i] == quote {
                    in_quote = None;
                }
            }
            None => match bytes[i] {
                b'"' | b'\'' | b'`' => in_quote = Some(bytes[i]),
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i);
                    }
                }
                _ => {}
            },
        }
        i += 1;
    }

    None
}

fn base_indent(current_indent: usize, indent_size: usize, use_tabs: bool) -> String {
    if use_tabs {
        "\t".repeat(current_indent)
    } else {
        " ".repeat(current_indent * indent_size.max(1))
    }
}

fn inner_indent(current_indent: usize, indent_size: usize, use_tabs: bool) -> String {
    if use_tabs {
        "\t".repeat(current_indent + 1)
    } else {
        " ".repeat((current_indent + 1) * indent_size.max(1))
    }
}

fn plugin_options(plugin_options: Option<&str>) -> Value {
    plugin_options
        .and_then(|options| serde_json::from_str(options).ok())
        .unwrap_or_else(|| json!({}))
}

#[cfg(test)]
mod generative_tests {
    use super::{format_captured_condition, split_on_top_level_ops};

    struct ConditionWrapScenario {
        input: &'static str,
        min_ops: usize,
        current_indent: usize,
        indent_size: usize,
        use_tabs: bool,
    }

    fn indent(depth: usize, indent_size: usize, use_tabs: bool) -> String {
        if use_tabs {
            "\t".repeat(depth)
        } else {
            " ".repeat(depth * indent_size.max(1))
        }
    }

    fn parse_condition_parts(input: &str) -> (String, String) {
        let open_idx = input
            .find('(')
            .expect("input must contain opening parenthesis");
        let close_idx = input
            .rfind(')')
            .expect("input must contain closing parenthesis");
        let inner = input[open_idx + 1..close_idx]
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let suffix = input[close_idx + 1..].trim().to_string();
        (inner, suffix)
    }

    fn generate_expected(s: &ConditionWrapScenario) -> String {
        let (inner, suffix) = parse_condition_parts(s.input);
        let parts = split_on_top_level_ops(&inner);

        if parts.len() < 2 || parts.len().saturating_sub(1) < s.min_ops {
            if suffix.is_empty() {
                return format!("({inner})");
            }
            return format!("({inner}) {suffix}");
        }

        let inner_indent = indent(s.current_indent + 1, s.indent_size, s.use_tabs);
        let base_indent = indent(s.current_indent, s.indent_size, s.use_tabs);
        let op_width = parts
            .get(1)
            .map(|(op, _)| op.len() + 1)
            .unwrap_or(3);
        let first_pad = " ".repeat(op_width);

        let mut lines = Vec::new();
        lines.push("(".to_string());
        lines.push(format!("{inner_indent}{first_pad}{}", parts[0].1));
        for (op, expr) in parts.into_iter().skip(1) {
            lines.push(format!("{inner_indent}{op} {expr}"));
        }
        if suffix.is_empty() {
            lines.push(format!("{base_indent})"));
        } else {
            lines.push(format!("{base_indent}) {suffix}"));
        }
        lines.join("\n")
    }

    fn run(s: ConditionWrapScenario) {
        let expected = generate_expected(&s);
        let actual = format_captured_condition(
            s.input,
            s.min_ops,
            s.current_indent,
            s.indent_size,
            s.use_tabs,
        );
        assert_eq!(expected, actual, "failed for input {}", s.input);
    }

    #[test]
    fn wraps_conditions_across_config_sweep() {
        let input = "(user && isAdmin || hasFeature && isActive) {";
        for min_ops in 0..=2 {
            for current_indent in 0..=2 {
                for indent_size in [2, 4] {
                    run(ConditionWrapScenario {
                        input,
                        min_ops,
                        current_indent,
                        indent_size,
                        use_tabs: false,
                    });
                }
            }
        }
    }

    #[test]
    fn wraps_conditions_with_tabs() {
        run(ConditionWrapScenario {
            input: "(flagA && flagB || flagC) {",
            min_ops: 1,
            current_indent: 1,
            indent_size: 2,
            use_tabs: true,
        });
    }

    #[test]
    fn keeps_short_condition_single_line_when_below_threshold() {
        run(ConditionWrapScenario {
            input: "(isReady && hasAccess) {",
            min_ops: 2,
            current_indent: 0,
            indent_size: 2,
            use_tabs: false,
        });
    }

    #[test]
    fn preserves_suffix_after_wrapped_condition() {
        run(ConditionWrapScenario {
            input: "(user && isAdmin || hasFeature) // trailing",
            min_ops: 1,
            current_indent: 1,
            indent_size: 2,
            use_tabs: false,
        });
    }

    #[test]
    fn handles_nested_parentheses_without_splitting_inner_expression() {
        run(ConditionWrapScenario {
            input: "((user && profile.active) || (isAdmin && hasFeature)) {",
            min_ops: 1,
            current_indent: 0,
            indent_size: 2,
            use_tabs: false,
        });
    }

    #[test]
    fn normalizes_irregular_whitespace_before_wrapping() {
        run(ConditionWrapScenario {
            input: "( user   &&   isAdmin   ||    hasFeature ) {",
            min_ops: 1,
            current_indent: 2,
            indent_size: 2,
            use_tabs: false,
        });
    }
}

#[cfg(test)]
mod chunk_generative_tests {
    use super::dispatch_hook;
    use fua_plugin_api::{
        HookContext, HookRequest, HookResponse, LeadingSpacing, NodeHook, NodePhase, Replacement,
        TokenHook,
    };
    use std::borrow::Cow;

    struct ChunkScenario {
        condition_expr: &'static str,
        ngclass_entries: &'static [(&'static str, &'static str)],
        wrap_conditions_min: usize,
        ngclass_wrap_entries_min: usize,
        indent_size: usize,
        use_tabs: bool,
    }

    fn token_request(
        kind: &'static str,
        text: &'static str,
        parent_kind: &'static str,
        attribute_name: Option<&'static str>,
        plugin_options: Option<&'static str>,
    ) -> HookRequest<'static> {
        HookRequest::Token(TokenHook {
            kind: Cow::Borrowed(kind),
            text: Cow::Borrowed(text),
            context: HookContext::new(parent_kind, Some("div"), attribute_name, 1, 4, true),
            plugin_options: plugin_options.map(Cow::Borrowed),
        })
    }

    fn condition_expected(expr: &str, indent_size: usize, use_tabs: bool) -> String {
        let parts = expr
            .split("&&")
            .flat_map(|segment| {
                segment
                    .split("||")
                    .enumerate()
                    .map(move |(idx, item)| (idx, item.trim().to_string()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let inner_indent = if use_tabs {
            "\t".repeat(2)
        } else {
            " ".repeat((1 + 1) * indent_size)
        };
        let base_indent = if use_tabs {
            "\t".repeat(1)
        } else {
            " ".repeat(indent_size)
        };

        // Keep deterministic operator sequence from expression text.
        let operators = expr
            .split_whitespace()
            .filter(|token| *token == "&&" || *token == "||")
            .collect::<Vec<_>>();
        let mut lines = vec!["(".to_string(), format!("{inner_indent}   {}", parts[0].1)];
        for (idx, part) in parts.iter().skip(1).enumerate() {
            let op = operators.get(idx).copied().unwrap_or("&&");
            lines.push(format!("{inner_indent}{op} {}", part.1));
        }
        lines.push(format!("{base_indent}) {{"));
        lines.join("\n")
    }

    fn ngclass_expected(
        entries: &[(&str, &str)],
        current_indent: usize,
        indent_size: usize,
        use_tabs: bool,
    ) -> String {
        let entry_indent = if use_tabs {
            "\t".repeat(current_indent + 2)
        } else {
            " ".repeat((current_indent + 2) * indent_size)
        };
        let mut lines = vec!["{".to_string()];
        for (index, (key, value)) in entries.iter().enumerate() {
            let suffix = if index + 1 == entries.len() { "" } else { "," };
            lines.push(format!("{entry_indent}{key}: {value}{suffix}"));
        }
        lines.push(format!("{entry_indent}}}"));
        format!("\"{}\"", lines.join("\n"))
    }

    #[test]
    fn generative_chunk_combines_block_condition_and_ngclass_wrapping() {
        let scenario = ChunkScenario {
            condition_expr: "user && isAdmin || hasFeature",
            ngclass_entries: &[
                ("'border-accent'", "this.hasImageChanges"),
                ("'border-quaternary'", "!this.hasImageChanges"),
            ],
            wrap_conditions_min: 1,
            ngclass_wrap_entries_min: 2,
            indent_size: 4,
            use_tabs: true,
        };

        let options = r#"{"wrap_conditions_min":2,"wrap_conditions_in_parens":true,"ngclass_wrap_entries_min":2}"#;

        let root_enter = dispatch_hook(HookRequest::Node(NodeHook {
            phase: NodePhase::Enter,
            kind: Cow::Borrowed("ROOT"),
            text: Cow::Borrowed(""),
            tag_name: None,
            context: HookContext::new("NONE", None, None, 0, 2, false),
            plugin_options: None,
        }));
        assert_eq!(root_enter, HookResponse::Continue);

        let opener = dispatch_hook(token_request("IDENT", "@if", "CONTENT", None, Some(options)));
        assert_eq!(
            opener,
            HookResponse::replace(Replacement::text("@if").with_leading(LeadingSpacing::LineBreak))
        );

        let condition_text: &'static str = Box::leak(format!("({}) {{", scenario.condition_expr).into_boxed_str());
        let condition = dispatch_hook(token_request(
            "TEXT",
            condition_text,
            "CONTENT",
            None,
            Some(options),
        ));
        let expected_condition = condition_expected(
            scenario.condition_expr,
            scenario.indent_size,
            scenario.use_tabs,
        );
        assert_eq!(
            condition,
            HookResponse::replace(Replacement::text(expected_condition))
        );

        let ngclass_input: &'static str = Box::leak(
            format!(
                "\"{{{}}}\"",
                scenario
                    .ngclass_entries
                    .iter()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .into_boxed_str(),
        );
        let ngclass = dispatch_hook(token_request(
            "STRING_DOUBLE",
            ngclass_input,
            "OPEN_TAG",
            Some("[ngClass]"),
            Some(options),
        ));
        let expected_ngclass = ngclass_expected(
            scenario.ngclass_entries,
            1,
            scenario.indent_size,
            scenario.use_tabs,
        );
        assert_eq!(ngclass, HookResponse::replace(Replacement::text(expected_ngclass)));

        assert_eq!(scenario.wrap_conditions_min, 1);
        assert_eq!(scenario.ngclass_wrap_entries_min, 2);
    }
}
