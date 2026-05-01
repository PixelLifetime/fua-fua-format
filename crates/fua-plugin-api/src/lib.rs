use std::borrow::Cow;

use serde::{Deserialize, Serialize};

pub const HANDLE_HOOK_EXPORT: &str = "handle_hook";

pub type Text<'a> = Cow<'a, str>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodePhase {
    Enter,
    Exit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookContext<'a> {
    pub parent_kind: Text<'a>,
    pub tag_name: Option<Text<'a>>,
    pub attribute_name: Option<Text<'a>>,
    pub previous_kind: Option<Text<'a>>,
    pub previous_text: Option<Text<'a>>,
    pub next_kind: Option<Text<'a>>,
    pub next_text: Option<Text<'a>>,
    pub current_indent: usize,
    pub indent_size: usize,
    pub use_tabs: bool,
    #[serde(default)]
    pub class_wrap_tokens_min: Option<usize>,
    #[serde(default = "default_class_wrap_tokens_per_line")]
    pub class_wrap_tokens_per_line: usize,
}

impl<'a> HookContext<'a> {
    pub fn new(
        parent_kind: &'a str,
        tag_name: Option<&'a str>,
        attribute_name: Option<&'a str>,
        current_indent: usize,
        indent_size: usize,
        use_tabs: bool,
    ) -> Self {
        Self {
            parent_kind: Cow::Borrowed(parent_kind),
            tag_name: tag_name.map(Cow::Borrowed),
            attribute_name: attribute_name.map(Cow::Borrowed),
            previous_kind: None,
            previous_text: None,
            next_kind: None,
            next_text: None,
            current_indent,
            indent_size,
            use_tabs,
            class_wrap_tokens_min: None,
            class_wrap_tokens_per_line: default_class_wrap_tokens_per_line(),
        }
    }

    pub fn with_neighbors(
        mut self,
        previous_kind: Option<&'a str>,
        previous_text: Option<&'a str>,
        next_kind: Option<&'a str>,
        next_text: Option<&'a str>,
    ) -> Self {
        self.previous_kind = previous_kind.map(Cow::Borrowed);
        self.previous_text = previous_text.map(Cow::Borrowed);
        self.next_kind = next_kind.map(Cow::Borrowed);
        self.next_text = next_text.map(Cow::Borrowed);
        self
    }

    pub fn with_class_wrapping(
        mut self,
        class_wrap_tokens_min: Option<usize>,
        class_wrap_tokens_per_line: usize,
    ) -> Self {
        self.class_wrap_tokens_min = class_wrap_tokens_min;
        self.class_wrap_tokens_per_line = class_wrap_tokens_per_line.max(1);
        self
    }
}

fn default_class_wrap_tokens_per_line() -> usize {
    1
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeHook<'a> {
    pub phase: NodePhase,
    pub kind: Text<'a>,
    pub text: Text<'a>,
    pub tag_name: Option<Text<'a>>,
    pub context: HookContext<'a>,
    pub plugin_options: Option<Text<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenHook<'a> {
    pub kind: Text<'a>,
    pub text: Text<'a>,
    pub context: HookContext<'a>,
    pub plugin_options: Option<Text<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "hook", rename_all = "snake_case")]
pub enum HookRequest<'a> {
    Node(NodeHook<'a>),
    Token(TokenHook<'a>),
}

impl<'a> HookRequest<'a> {
    pub fn node(
        phase: NodePhase,
        kind: &'a str,
        text: &'a str,
        tag_name: Option<&'a str>,
        context: HookContext<'a>,
    ) -> Self {
        Self::Node(NodeHook {
            phase,
            kind: Cow::Borrowed(kind),
            text: Cow::Borrowed(text),
            tag_name: tag_name.map(Cow::Borrowed),
            context,
            plugin_options: None,
        })
    }

    pub fn token(kind: &'a str, text: &'a str, context: HookContext<'a>) -> Self {
        Self::Token(TokenHook {
            kind: Cow::Borrowed(kind),
            text: Cow::Borrowed(text),
            context,
            plugin_options: None,
        })
    }

    pub fn with_plugin_options(self, plugin_options: Option<Text<'a>>) -> Self {
        match self {
            Self::Node(mut hook) => {
                hook.plugin_options = plugin_options;
                Self::Node(hook)
            }
            Self::Token(mut hook) => {
                hook.plugin_options = plugin_options;
                Self::Token(hook)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeadingSpacing {
    None,
    Space,
    LineBreak,
    BlankLine,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Replacement {
    pub output: String,
    pub indent_before: i32,
    pub indent_after: i32,
    pub leading_spacing: LeadingSpacing,
}

impl Replacement {
    pub fn text(output: impl Into<String>) -> Self {
        Self {
            output: output.into(),
            indent_before: 0,
            indent_after: 0,
            leading_spacing: LeadingSpacing::None,
        }
    }

    pub fn with_leading(mut self, leading_spacing: LeadingSpacing) -> Self {
        self.leading_spacing = leading_spacing;
        self
    }

    pub fn with_indent(mut self, indent_before: i32, indent_after: i32) -> Self {
        self.indent_before = indent_before;
        self.indent_after = indent_after;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum HookResponse {
    Continue,
    Replace(Replacement),
}

impl HookResponse {
    pub fn replace(replacement: Replacement) -> Self {
        Self::Replace(replacement)
    }
}
