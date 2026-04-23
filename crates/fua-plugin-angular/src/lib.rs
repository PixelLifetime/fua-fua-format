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
        let response = dispatch_hook(token_request("IDENT", "@if", "ROOT"));
        assert_eq!(
            response,
            HookResponse::replace(Replacement::text("@if").with_leading(LeadingSpacing::LineBreak),)
        );
    }
}
