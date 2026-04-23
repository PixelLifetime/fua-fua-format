use std::borrow::Cow;

use extism::{Manifest, Plugin, Wasm};
use fua_plugin_api::{HANDLE_HOOK_EXPORT, HookRequest, HookResponse, Replacement};

pub trait FormatPlugin: Send {
    fn handle_hook(&mut self, request: &HookRequest<'_>) -> HookResponse;
}

struct WasmPluginInstance {
    plugin: Plugin,
    plugin_options: Option<String>,
}

impl WasmPluginInstance {
    fn new(path: &str, plugin_options: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let manifest = Manifest::new([Wasm::file(path)]);
        let plugin = Plugin::new(&manifest, [], true)?;
        Ok(Self {
            plugin,
            plugin_options,
        })
    }
}

impl FormatPlugin for WasmPluginInstance {
    fn handle_hook(&mut self, request: &HookRequest<'_>) -> HookResponse {
        let request = request
            .clone()
            .with_plugin_options(self.plugin_options.as_deref().map(Cow::Borrowed));

        let input = match serde_json::to_string(&request) {
            Ok(input) => input,
            Err(_) => return HookResponse::Continue,
        };

        let response: String = match self.plugin.call(HANDLE_HOOK_EXPORT, input) {
            Ok(response) => response,
            Err(_) => return HookResponse::Continue,
        };

        if response.trim().is_empty() {
            return HookResponse::Continue;
        }

        serde_json::from_str(&response).unwrap_or(HookResponse::Continue)
    }
}

#[derive(Default)]
pub struct PluginHost {
    plugins: Vec<Box<dyn FormatPlugin>>,
}

impl PluginHost {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    pub fn register(&mut self, plugin: Box<dyn FormatPlugin>) {
        self.plugins.push(plugin);
    }

    pub fn load_wasm(
        &mut self,
        path: &str,
        plugin_options: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let plugin = WasmPluginInstance::new(path, plugin_options)?;
        self.register(Box::new(plugin));
        Ok(())
    }

    pub fn dispatch(&mut self, request: &HookRequest<'_>) -> Option<Replacement> {
        for plugin in &mut self.plugins {
            match plugin.handle_hook(request) {
                HookResponse::Continue => {}
                HookResponse::Replace(replacement) => return Some(replacement),
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fua_plugin_api::{HookContext, LeadingSpacing};

    struct PassThroughPlugin;

    impl FormatPlugin for PassThroughPlugin {
        fn handle_hook(&mut self, _request: &HookRequest<'_>) -> HookResponse {
            HookResponse::Continue
        }
    }

    struct ReplacingPlugin;

    impl FormatPlugin for ReplacingPlugin {
        fn handle_hook(&mut self, _request: &HookRequest<'_>) -> HookResponse {
            HookResponse::replace(Replacement::text("handled").with_leading(LeadingSpacing::Space))
        }
    }

    #[test]
    fn dispatch_returns_the_first_replacement() {
        let context = HookContext::new("ROOT", None, None, 0, 2, false);
        let request = HookRequest::token("TEXT", "hello", context);

        let mut host = PluginHost::new();
        host.register(Box::new(PassThroughPlugin));
        host.register(Box::new(ReplacingPlugin));

        let replacement = host.dispatch(&request).expect("replacement");
        assert_eq!(replacement.output, "handled");
        assert_eq!(replacement.leading_spacing, LeadingSpacing::Space);
    }
}
