mod classes;
mod options;

use classes::process_class_string;
use fua_plugin_api::{HookRequest, HookResponse, Replacement, TokenHook};
use options::TailwindOptions;

pub fn dispatch_hook(request: HookRequest<'static>) -> HookResponse {
    match request {
        HookRequest::Token(token) => handle_token_hook(&token),
        HookRequest::Node(_) => HookResponse::Continue,
    }
}

fn handle_token_hook(token: &TokenHook<'_>) -> HookResponse {
    if !should_process_class_string(token) {
        return HookResponse::Continue;
    }

    let options = TailwindOptions::from_plugin_options(token.plugin_options.as_deref());
    match process_class_string(token.text.as_ref(), &options, &token.context) {
        Some(output) => HookResponse::replace(Replacement::text(output)),
        None => HookResponse::Continue,
    }
}

fn should_process_class_string(token: &TokenHook<'_>) -> bool {
    matches!(token.kind.as_ref(), "STRING_DOUBLE" | "STRING_SINGLE")
        && matches!(
            token.context.parent_kind.as_ref(),
            "OPEN_TAG" | "SELF_CLOSING_TAG"
        )
        && token
            .context
            .attribute_name
            .as_deref()
            .is_some_and(|name| name.trim().eq_ignore_ascii_case("class"))
}

#[cfg(target_arch = "wasm32")]
use extism_pdk::*;
#[cfg(target_arch = "wasm32")]
use fua_plugin_api::{HookRequest as WasmHookRequest, HookResponse as WasmHookResponse};

#[cfg(target_arch = "wasm32")]
#[plugin_fn]
pub fn handle_hook(input: String) -> FnResult<String> {
    let request = match serde_json::from_str::<WasmHookRequest<'static>>(&input) {
        Ok(request) => request,
        Err(_) => return Ok(String::new()),
    };

    let response = dispatch_hook(request);
    match response {
        WasmHookResponse::Continue => Ok(String::new()),
        other => Ok(serde_json::to_string(&other).unwrap_or_default()),
    }
}

#[cfg(test)]
mod tests {
    use super::dispatch_hook;
    use fua_plugin_api::{HookContext, HookRequest, HookResponse, Replacement, TokenHook};
    use std::borrow::Cow;

    fn class_request(
        text: &'static str,
        context_min: Option<usize>,
        context_per_line: usize,
        plugin_options: Option<&'static str>,
    ) -> HookRequest<'static> {
        HookRequest::Token(TokenHook {
            kind: Cow::Borrowed("STRING_DOUBLE"),
            text: Cow::Borrowed(text),
            context: HookContext::new("OPEN_TAG", Some("button"), Some("class"), 0, 2, false)
                .with_class_wrapping(context_min, context_per_line),
            plugin_options: plugin_options.map(Cow::Borrowed),
        })
    }

    #[test]
    fn sorts_groups_and_wraps_tailwind_classes() {
        let response = dispatch_hook(class_request(
            "\"py-2 flex disabled:opacity-50 flex-col px-4 disabled:cursor-not-allowed items-center\"",
            Some(1),
            2,
            None,
        ));

        assert_eq!(
            response,
            HookResponse::replace(Replacement::text(
                "\"\n    flex flex-col\n    items-center\n\n    px-4 py-2\n\n    disabled:opacity-50\n\n    disabled:cursor-not-allowed\n  \""
            ))
        );
    }

    #[test]
    fn plugin_options_override_context_tokens_per_line() {
        let response = dispatch_hook(class_request(
            "\"flex items-center flex-col\"",
            Some(1),
            3,
            Some(r#"{"class_wrap_tokens_per_line":2}"#),
        ));

        assert_eq!(
            response,
            HookResponse::replace(Replacement::text(
                "\"\n    flex flex-col\n    items-center\n  \""
            ))
        );
    }

    #[test]
    fn sorts_short_classes_without_forcing_wrap() {
        let response = dispatch_hook(class_request("\"py-2 flex px-4\"", Some(6), 2, None));

        assert_eq!(
            response,
            HookResponse::replace(Replacement::text("\"flex px-4 py-2\""))
        );
    }

    #[test]
    fn keeps_interpolation_token_intact_when_processing_class() {
        let response = dispatch_hook(class_request(
            "\"fixed p-1.5 {{ this.getPositionClass() }}\"",
            Some(1),
            2,
            None,
        ));

        match response {
            HookResponse::Replace(Replacement { output, .. }) => {
                assert!(output.contains("{{ this.getPositionClass() }}"));
                assert!(!output.contains("{{\n"));
                assert!(!output.contains("{{ }}"));
            }
            other => panic!("expected replacement, got {other:?}"),
        }
    }
}
