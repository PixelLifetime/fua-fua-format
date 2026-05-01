use crate::options::TailwindOptions;
use fua_plugin_api::HookContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum UtilityGroup {
    Layout,
    FlexGrid,
    Spacing,
    Sizing,
    Typography,
    Visual,
    Effects,
    Unknown,
}

#[derive(Debug, Clone)]
struct ClassToken {
    raw: String,
    original_index: usize,
    sort_key: SortKey,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SortKey {
    group: UtilityGroup,
    utility_rank: usize,
    variant_rank: usize,
    normalized: String,
}

pub(crate) fn process_class_string(
    text: &str,
    options: &TailwindOptions,
    context: &HookContext<'_>,
) -> Option<String> {
    let (quote, inner) = quoted_inner(text)?;
    let tokens = inner
        .split_whitespace()
        .enumerate()
        .map(|(index, token)| ClassToken::new(token, index))
        .collect::<Vec<_>>();

    if tokens.is_empty() {
        return None;
    }

    let original_compact = tokens
        .iter()
        .map(|token| token.raw.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let sorted = sort_tokens(tokens);
    let sorted_compact = sorted
        .iter()
        .map(|token| token.raw.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    let min_tokens = options
        .class_wrap_tokens_min
        .or(context.class_wrap_tokens_min);
    let should_wrap =
        inner.contains('\n') || min_tokens.is_some_and(|min_tokens| sorted.len() >= min_tokens);

    if !should_wrap {
        if sorted_compact == original_compact {
            return None;
        }

        return Some(format!("{quote}{sorted_compact}{quote}"));
    }

    let tokens_per_line = options
        .class_wrap_tokens_per_line
        .unwrap_or(context.class_wrap_tokens_per_line)
        .max(1);
    let token_indent = indent_string(context, context.current_indent + 2);
    let close_indent = indent_string(context, context.current_indent + 1);
    let grouped_lines = grouped_class_lines(&sorted, tokens_per_line, options.group_blank_lines);

    Some(quote_multiline(
        quote,
        &grouped_lines,
        &token_indent,
        &close_indent,
    ))
}

impl ClassToken {
    fn new(raw: &str, original_index: usize) -> Self {
        let sort_key = SortKey::from_class(raw);
        Self {
            raw: raw.to_string(),
            original_index,
            sort_key,
        }
    }
}

impl SortKey {
    fn from_class(raw: &str) -> Self {
        let (variants, utility) = split_variants(raw);
        let normalized = normalize_utility(utility);
        let variant_rank = variants
            .iter()
            .map(|variant| variant_rank(variant))
            .min()
            .unwrap_or(0);
        let group = classify_group(&normalized);
        let utility_rank = utility_rank(&normalized);

        Self {
            group,
            utility_rank,
            variant_rank,
            normalized: normalized.to_string(),
        }
    }
}

fn sort_tokens(mut tokens: Vec<ClassToken>) -> Vec<ClassToken> {
    tokens.sort_by(|left, right| {
        left.sort_key
            .cmp(&right.sort_key)
            .then_with(|| left.raw.cmp(&right.raw))
            .then_with(|| left.original_index.cmp(&right.original_index))
    });
    tokens
}

fn grouped_class_lines(
    tokens: &[ClassToken],
    tokens_per_line: usize,
    group_blank_lines: bool,
) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_group: Option<UtilityGroup> = None;
    let mut pending_group_tokens: Vec<&str> = Vec::new();

    for token in tokens {
        if current_group.is_some_and(|group| group != token.sort_key.group) {
            flush_group(&mut lines, &pending_group_tokens, tokens_per_line);
            pending_group_tokens.clear();
            if group_blank_lines {
                lines.push(String::new());
            }
        }

        current_group = Some(token.sort_key.group);
        pending_group_tokens.push(&token.raw);
    }

    flush_group(&mut lines, &pending_group_tokens, tokens_per_line);
    lines
}

fn flush_group(lines: &mut Vec<String>, tokens: &[&str], tokens_per_line: usize) {
    for chunk in tokens.chunks(tokens_per_line) {
        lines.push(chunk.join(" "));
    }
}

fn classify_group(utility: &str) -> UtilityGroup {
    if matches_any(
        utility,
        &[
            "container",
            "static",
            "fixed",
            "absolute",
            "relative",
            "sticky",
            "inset",
            "top",
            "right",
            "bottom",
            "left",
            "z",
            "block",
            "inline",
            "inline-block",
            "inline-flex",
            "inline-grid",
            "hidden",
            "table",
            "flow-root",
            "contents",
            "float",
            "clear",
            "isolate",
            "isolation",
            "visible",
            "invisible",
            "collapse",
            "overflow",
            "overscroll",
            "object",
        ],
    ) {
        return UtilityGroup::Layout;
    }

    if matches_any(
        utility,
        &[
            "flex",
            "grid",
            "basis",
            "grow",
            "shrink",
            "order",
            "col",
            "row",
            "auto-cols",
            "auto-rows",
            "grid-cols",
            "grid-rows",
            "gap",
            "justify",
            "content",
            "items",
            "self",
            "place",
            "space",
        ],
    ) {
        return UtilityGroup::FlexGrid;
    }

    if matches_any(
        utility,
        &[
            "m", "mx", "my", "ms", "me", "mt", "mr", "mb", "ml", "p", "px", "py", "ps", "pe", "pt",
            "pr", "pb", "pl",
        ],
    ) {
        return UtilityGroup::Spacing;
    }

    if matches_any(
        utility,
        &[
            "aspect", "size", "w", "min-w", "max-w", "h", "min-h", "max-h",
        ],
    ) {
        return UtilityGroup::Sizing;
    }

    if matches_any(
        utility,
        &[
            "font",
            "text",
            "antialiased",
            "subpixel-antialiased",
            "italic",
            "not-italic",
            "uppercase",
            "lowercase",
            "capitalize",
            "normal-case",
            "leading",
            "tracking",
            "list",
            "placeholder",
            "align",
            "whitespace",
            "break",
            "truncate",
            "decoration",
            "underline",
            "overline",
            "line-through",
        ],
    ) {
        return UtilityGroup::Typography;
    }

    if matches_any(
        utility,
        &[
            "bg", "from", "via", "to", "rounded", "border", "divide", "outline", "ring", "shadow",
            "opacity",
        ],
    ) {
        return UtilityGroup::Visual;
    }

    if matches_any(
        utility,
        &[
            "transition",
            "duration",
            "ease",
            "delay",
            "animate",
            "transform",
            "scale",
            "rotate",
            "translate",
            "skew",
            "origin",
            "filter",
            "blur",
            "brightness",
            "contrast",
            "drop-shadow",
            "grayscale",
            "hue-rotate",
            "invert",
            "saturate",
            "sepia",
            "backdrop",
            "cursor",
            "pointer-events",
            "resize",
            "select",
            "scroll",
            "snap",
            "touch",
            "accent",
            "caret",
            "appearance",
            "sr-only",
            "not-sr-only",
        ],
    ) {
        return UtilityGroup::Effects;
    }

    UtilityGroup::Unknown
}

fn utility_rank(utility: &str) -> usize {
    UTILITY_ORDER
        .iter()
        .position(|prefix| matches_prefix(utility, prefix))
        .unwrap_or(usize::MAX)
}

fn variant_rank(variant: &str) -> usize {
    VARIANT_ORDER
        .iter()
        .position(|prefix| *prefix == variant)
        .unwrap_or(usize::MAX)
}

fn matches_any(utility: &str, prefixes: &[&str]) -> bool {
    prefixes
        .iter()
        .any(|prefix| matches_prefix(utility, prefix))
}

fn matches_prefix(utility: &str, prefix: &str) -> bool {
    utility == prefix || utility.starts_with(&format!("{prefix}-"))
}

fn split_variants(raw: &str) -> (Vec<String>, &str) {
    let mut depth = 0usize;
    let mut start = 0usize;
    let mut variants = Vec::new();

    for (index, ch) in raw.char_indices() {
        match ch {
            '[' => depth += 1,
            ']' => depth = depth.saturating_sub(1),
            ':' if depth == 0 => {
                variants.push(raw[start..index].to_string());
                start = index + 1;
            }
            _ => {}
        }
    }

    (variants, &raw[start..])
}

fn normalize_utility(utility: &str) -> &str {
    utility
        .trim_start_matches('!')
        .trim_start_matches('-')
        .split_once('/')
        .map(|(base, _)| base)
        .unwrap_or(utility)
}

fn quoted_inner(text: &str) -> Option<(char, &str)> {
    if text.len() < 2 {
        return None;
    }

    let quote = text.chars().next()?;
    if !matches!(quote, '"' | '\'') || !text.ends_with(quote) {
        return None;
    }

    Some((quote, &text[1..text.len() - 1]))
}

fn indent_string(context: &HookContext<'_>, depth: usize) -> String {
    if context.use_tabs {
        "\t".repeat(depth)
    } else {
        " ".repeat(depth * context.indent_size)
    }
}

fn quote_multiline(
    quote: char,
    lines: &[String],
    token_indent: &str,
    close_indent: &str,
) -> String {
    let mut out = String::new();
    out.push(quote);
    out.push('\n');

    for line in lines {
        if line.is_empty() {
            out.push('\n');
        } else {
            out.push_str(token_indent);
            out.push_str(line);
            out.push('\n');
        }
    }

    out.push_str(close_indent);
    out.push(quote);
    out
}

const VARIANT_ORDER: &[&str] = &[
    "sm",
    "md",
    "lg",
    "xl",
    "2xl",
    "motion-safe",
    "motion-reduce",
    "dark",
    "portrait",
    "landscape",
    "first",
    "last",
    "odd",
    "even",
    "visited",
    "checked",
    "empty",
    "enabled",
    "disabled",
    "group-hover",
    "group-focus",
    "peer-hover",
    "peer-focus",
    "hover",
    "focus",
    "focus-visible",
    "active",
];

const UTILITY_ORDER: &[&str] = &[
    "container",
    "static",
    "fixed",
    "absolute",
    "relative",
    "sticky",
    "inset",
    "top",
    "right",
    "bottom",
    "left",
    "z",
    "block",
    "inline",
    "inline-block",
    "inline-flex",
    "inline-grid",
    "hidden",
    "flex",
    "grid",
    "basis",
    "grow",
    "shrink",
    "order",
    "grid-cols",
    "grid-rows",
    "col",
    "row",
    "gap",
    "justify",
    "content",
    "items",
    "self",
    "place",
    "space",
    "m",
    "mx",
    "my",
    "ms",
    "me",
    "mt",
    "mr",
    "mb",
    "ml",
    "p",
    "px",
    "py",
    "ps",
    "pe",
    "pt",
    "pr",
    "pb",
    "pl",
    "aspect",
    "size",
    "w",
    "min-w",
    "max-w",
    "h",
    "min-h",
    "max-h",
    "font",
    "text",
    "leading",
    "tracking",
    "list",
    "placeholder",
    "align",
    "whitespace",
    "break",
    "truncate",
    "bg",
    "from",
    "via",
    "to",
    "rounded",
    "border",
    "divide",
    "outline",
    "ring",
    "shadow",
    "opacity",
    "transition",
    "duration",
    "ease",
    "delay",
    "animate",
    "transform",
    "scale",
    "rotate",
    "translate",
    "skew",
    "origin",
    "filter",
    "blur",
    "backdrop",
    "cursor",
    "pointer-events",
    "select",
    "resize",
];

// ─── Generative tests ────────────────────────────────────────────────────────
//
// Instead of hard-coding expected strings, each test scenario carries the
// *sorted, grouped* token list and a formatting config.  `generate_expected`
// is a simple, independent reference implementation that builds the output
// string from those two inputs.  The assertion is then:
//
//   process_class_string(unsorted_input, options, ctx)
//     == generate_expected(sorted_groups, options, ctx)
//
// This means:
//   • changing a config value updates both sides automatically,
//   • the wrapping contract is verified against a different code-path, and
//   • we can sweep many configs in a single parameterised loop.
#[cfg(test)]
mod generative_tests {
    use super::process_class_string;
    use crate::options::TailwindOptions;
    use fua_plugin_api::HookContext;

    // ── Scenario description ─────────────────────────────────────────────────

    struct Scenario {
        description: &'static str,
        /// Unsorted class tokens as they appear in the source (no surrounding quotes).
        input: &'static str,
        /// Tokens already sorted and split into their utility groups.
        /// Each inner `Vec` is one contiguous group; order of groups matters.
        sorted_groups: Vec<Vec<&'static str>>,
        options: TailwindOptions,
        /// Values forwarded to `HookContext::with_class_wrapping`.
        context_min: Option<usize>,
        context_per_line: usize,
        current_indent: usize,
        indent_size: usize,
        use_tabs: bool,
    }

    // ── Reference implementation (the "generator") ───────────────────────────

    fn make_indent(depth: usize, indent_size: usize, use_tabs: bool) -> String {
        if use_tabs {
            "\t".repeat(depth)
        } else {
            " ".repeat(depth * indent_size)
        }
    }

    /// Build the expected multiline class attribute value from sorted groups and config.
    /// This is intentionally written differently from `process_class_string` so that
    /// a bug in either path shows up as a test failure.
    fn generate_expected(s: &Scenario) -> String {
        let tokens_per_line = s
            .options
            .class_wrap_tokens_per_line
            .unwrap_or(s.context_per_line)
            .max(1);
        let group_blank_lines = s.options.group_blank_lines;
        let token_indent = make_indent(s.current_indent + 2, s.indent_size, s.use_tabs);
        let close_indent = make_indent(s.current_indent + 1, s.indent_size, s.use_tabs);

        let mut lines: Vec<String> = Vec::new();
        for (gi, group) in s.sorted_groups.iter().enumerate() {
            if gi > 0 && group_blank_lines {
                lines.push(String::new());
            }
            for chunk in group.chunks(tokens_per_line) {
                lines.push(chunk.join(" "));
            }
        }

        let mut out = String::from('"');
        out.push('\n');
        for line in &lines {
            if line.is_empty() {
                out.push('\n');
            } else {
                out.push_str(&token_indent);
                out.push_str(line);
                out.push('\n');
            }
        }
        out.push_str(&close_indent);
        out.push('"');
        out
    }

    // ── Test runner ──────────────────────────────────────────────────────────

    fn run(s: &Scenario) {
        let input = format!("\"{}\"", s.input);
        let context =
            HookContext::new("OPEN_TAG", Some("div"), Some("class"), s.current_indent, s.indent_size, s.use_tabs)
                .with_class_wrapping(s.context_min, s.context_per_line);
        let expected = generate_expected(s);
        let actual = process_class_string(&input, &s.options, &context);
        assert_eq!(
            Some(expected),
            actual,
            "scenario '{}' failed\n  input: {:?}",
            s.description,
            s.input,
        );
    }

    // ── Shared fixtures ──────────────────────────────────────────────────────

    // "p-1 m-4 opacity-50"
    //   m-4      → Spacing  (utility_rank for "m")
    //   p-1      → Spacing  (utility_rank for "p", higher than "m")
    //   opacity-50 → Visual
    fn spacing_visual_groups() -> Vec<Vec<&'static str>> {
        vec![vec!["m-4", "p-1"], vec!["opacity-50"]]
    }

    fn options_wrap(tokens_per_line: usize, group_blank_lines: bool) -> TailwindOptions {
        TailwindOptions {
            class_wrap_tokens_min: Some(1),
            class_wrap_tokens_per_line: Some(tokens_per_line),
            group_blank_lines,
        }
    }

    // ── Named single-config tests ─────────────────────────────────────────────

    #[test]
    fn one_per_line_with_group_blank_lines() {
        run(&Scenario {
            description: "1 token/line, blank lines between groups",
            input: "p-1 m-4 opacity-50",
            sorted_groups: spacing_visual_groups(),
            options: options_wrap(1, true),
            context_min: Some(1),
            context_per_line: 1,
            current_indent: 0,
            indent_size: 2,
            use_tabs: false,
        });
    }

    #[test]
    fn two_per_line_with_group_blank_lines() {
        run(&Scenario {
            description: "2 tokens/line, blank lines between groups",
            input: "p-1 m-4 opacity-50",
            sorted_groups: spacing_visual_groups(),
            options: options_wrap(2, true),
            context_min: Some(1),
            context_per_line: 2,
            current_indent: 0,
            indent_size: 2,
            use_tabs: false,
        });
    }

    #[test]
    fn one_per_line_no_group_blank_lines() {
        run(&Scenario {
            description: "1 token/line, no blank lines between groups",
            input: "p-1 m-4 opacity-50",
            sorted_groups: spacing_visual_groups(),
            options: options_wrap(1, false),
            context_min: Some(1),
            context_per_line: 1,
            current_indent: 0,
            indent_size: 2,
            use_tabs: false,
        });
    }

    #[test]
    fn nested_indent_level() {
        run(&Scenario {
            description: "current_indent=2, 1 token/line",
            input: "p-1 m-4 opacity-50",
            sorted_groups: spacing_visual_groups(),
            options: options_wrap(1, true),
            context_min: Some(1),
            context_per_line: 1,
            current_indent: 2,
            indent_size: 2,
            use_tabs: false,
        });
    }

    #[test]
    fn tab_indentation() {
        run(&Scenario {
            description: "tab indentation, 1 token/line",
            input: "p-1 m-4 opacity-50",
            sorted_groups: spacing_visual_groups(),
            options: options_wrap(1, true),
            context_min: Some(1),
            context_per_line: 1,
            current_indent: 0,
            indent_size: 4,
            use_tabs: true,
        });
    }

    // ── Parameterised sweep ───────────────────────────────────────────────────
    //
    // The generator makes it trivial to validate every combination: we only
    // have to specify the *groups* once and let the config drive both sides.

    #[test]
    fn all_config_combinations() {
        let groups = spacing_visual_groups();
        let input = "p-1 m-4 opacity-50";

        for tokens_per_line in 1..=3_usize {
            for group_blank_lines in [true, false] {
                for current_indent in 0..=2_usize {
                    for indent_size in [2_usize, 4] {
                        run(&Scenario {
                            description: "config sweep",
                            input,
                            sorted_groups: groups.clone(),
                            options: options_wrap(tokens_per_line, group_blank_lines),
                            context_min: Some(1),
                            context_per_line: tokens_per_line,
                            current_indent,
                            indent_size,
                            use_tabs: false,
                        });
                    }
                }
            }
        }
    }
}
