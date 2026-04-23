use crate::attributes::process_attribute_string;
use crate::context::{
    is_angular_binding, is_block_opener, is_class_attr, is_content_context, is_else_like,
};
use crate::response::{
    HookResponseExt, condition_whitespace_replacement, replacement, replacement_with_spacing,
};
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
            .is_some_and(|name| is_angular_binding(name) || is_class_attr(name))
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

    clear_pending_condition_start();
    HookResponse::Continue
}

fn handle_content_whitespace(token: &TokenHook<'_>) -> HookResponse {
    let options = plugin_options(token.plugin_options.as_deref());
    let Some(target_indent) = read_state(|state| {
        if state.condition_depth == 0 || !token.text.contains('\n') {
            return None;
        }

        let next_is_close_paren = token.context.next_kind.as_deref() == Some("TEXT")
            && token.context.next_text.as_deref() == Some(")");
        let target_indent = if options
            .get("indent_condition_groups")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            state.condition_base_indent + state.condition_depth - usize::from(next_is_close_paren)
        } else if next_is_close_paren {
            state.condition_base_indent
        } else {
            state.condition_base_indent + 1
        };

        Some(target_indent)
    })
    .flatten() else {
        return HookResponse::Continue;
    };

    condition_whitespace_replacement(token.context.current_indent, target_indent)
}

fn handle_content_text(token: &TokenHook<'_>) -> HookResponse {
    match token.text.as_ref() {
        "(" => open_condition_group(token.context.current_indent),
        ")" => close_condition_group(),
        "{" => replacement_with_spacing("{", LeadingSpacing::Space).map_indent(0, 1),
        "}" => HookResponse::replace(
            Replacement::text("}")
                .with_indent(-1, 0)
                .with_leading(LeadingSpacing::LineBreak),
        ),
        _ => {
            clear_pending_condition_start();
            HookResponse::Continue
        }
    }
}

fn open_condition_group(current_indent: usize) -> HookResponse {
    let response = with_state(|state| {
        if state.awaiting_condition_start {
            state.awaiting_condition_start = false;
            state.condition_depth = 1;
            state.condition_base_indent = current_indent;
            return Some(replacement("("));
        }

        if state.condition_depth > 0 {
            state.condition_depth += 1;
            return Some(replacement("("));
        }

        None
    })
    .flatten();

    response.unwrap_or(HookResponse::Continue)
}

fn close_condition_group() -> HookResponse {
    let response = with_state(|state| {
        if state.condition_depth > 0 {
            state.condition_depth = state.condition_depth.saturating_sub(1);
            Some(replacement(")"))
        } else {
            None
        }
    })
    .flatten();

    response.unwrap_or(HookResponse::Continue)
}

fn clear_pending_condition_start() {
    let _ = with_state(|state| {
        if state.awaiting_condition_start {
            state.awaiting_condition_start = false;
        }
    });
}

fn plugin_options(plugin_options: Option<&str>) -> Value {
    plugin_options
        .and_then(|options| serde_json::from_str(options).ok())
        .unwrap_or_else(|| json!({}))
}
