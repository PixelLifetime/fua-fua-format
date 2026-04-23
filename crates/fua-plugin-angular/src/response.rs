use fua_plugin_api::{HookResponse, LeadingSpacing, Replacement};

pub(crate) fn replacement(output: impl Into<String>) -> HookResponse {
    HookResponse::replace(Replacement::text(output))
}

pub(crate) fn replacement_with_spacing(
    output: impl Into<String>,
    spacing: LeadingSpacing,
) -> HookResponse {
    HookResponse::replace(Replacement::text(output).with_leading(spacing))
}

pub(crate) fn condition_whitespace_replacement(
    current_indent: usize,
    target_indent: usize,
) -> HookResponse {
    let delta = target_indent as i32 - current_indent as i32;
    HookResponse::replace(
        Replacement::text("")
            .with_indent(delta, -delta)
            .with_leading(LeadingSpacing::LineBreak),
    )
}

pub(crate) trait HookResponseExt {
    fn map_indent(self, indent_before: i32, indent_after: i32) -> Self;
}

impl HookResponseExt for HookResponse {
    fn map_indent(self, indent_before: i32, indent_after: i32) -> Self {
        match self {
            HookResponse::Continue => HookResponse::Continue,
            HookResponse::Replace(replacement) => {
                HookResponse::Replace(replacement.with_indent(indent_before, indent_after))
            }
        }
    }
}
