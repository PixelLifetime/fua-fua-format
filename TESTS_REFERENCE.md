# Tests Reference (Generative + Examples)

This document describes the current meaningful generator-based tests in the project, shows the real test code, and explains what each test proves with concrete input/output examples.

## How To Run

```bash
# all tests
cargo test --workspace

# per crate
cargo test -p fua-core
cargo test -p fua-plugin-tailwind
cargo test -p fua-plugin-angular
```

---

## 1) Tailwind Plugin Generative Tests

File: `crates/fua-plugin-tailwind/src/classes.rs`

### Generator + runner

```rust
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

fn run(s: &Scenario) {
    let input = format!("\"{}\"", s.input);
    let context =
        HookContext::new("OPEN_TAG", Some("div"), Some("class"), s.current_indent, s.indent_size, s.use_tabs)
            .with_class_wrapping(s.context_min, s.context_per_line);
    let expected = generate_expected(s);
    let actual = process_class_string(&input, &s.options, &context);
    assert_eq!(Some(expected), actual);
}
```

### Tests

```rust
#[test]
fn one_per_line_with_group_blank_lines() { /* ... */ }

#[test]
fn two_per_line_with_group_blank_lines() { /* ... */ }

#[test]
fn one_per_line_no_group_blank_lines() { /* ... */ }

#[test]
fn nested_indent_level() { /* ... */ }

#[test]
fn tab_indentation() { /* ... */ }

#[test]
fn all_config_combinations() {
    for tokens_per_line in 1..=3_usize {
        for group_blank_lines in [true, false] {
            for current_indent in 0..=2_usize {
                for indent_size in [2_usize, 4] {
                    // run scenario
                }
            }
        }
    }
}
```

### Example behavior

Input:

```html
class="p-1 m-4 opacity-50"
```

Expected with `tokens_per_line=1`, `group_blank_lines=true`:

```html
class="
    m-4
    p-1

    opacity-50
  "
```

---

## 2) Core Formatter Generative Tests

File: `crates/core/src/formatter.rs`

### Generator + runner

```rust
fn generate_expected_button_with_class(lines: &[String], indent_size: usize, use_tabs: bool) -> String {
    let tag_indent = indent(1, indent_size, use_tabs);
    let class_indent = indent(2, indent_size, use_tabs);
    let close_quote_indent = tag_indent.clone();

    let mut out = String::from("\n<button\n");
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
    let expected = generate_expected_button_with_class(&generated_lines, s.indent_size, s.use_tabs);
    assert_eq!(expected, actual);
}
```

### Tests

```rust
#[test]
fn generative_class_wrapping_scenarios() { /* tokens_per_line: 1..=3 */ }

#[test]
fn generative_class_wrapping_with_tabs() { /* tab indentation */ }

#[test]
fn generative_class_wrapping_with_non_divisible_chunk_size() { /* 5 tokens / 4 per line */ }

#[test]
fn generative_class_wrapping_for_deep_indent_context() { /* realistic utility set */ }

#[test]
fn generative_class_wrapping_respects_min_threshold_toggle() { /* no-wrap vs wrap */ }
```

### Example behavior

Input:

```html
<button class="flex items-center gap-2"></button>
```

- with `class_wrap_tokens_min=10` => unchanged single-line class
- with `class_wrap_tokens_min=1`, `class_wrap_tokens_per_line=2`:

```html
<button
  class="
    flex items-center
    gap-2
  "
>
</button>
```

---

## 3) Angular Condition Generative Tests

File: `crates/fua-plugin-angular/src/hooks.rs`

### Generator + runner

```rust
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
    let op_width = parts.get(1).map(|(op, _)| op.len() + 1).unwrap_or(3);
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
```

### Tests

```rust
#[test]
fn wraps_conditions_across_config_sweep() { /* min_ops, indent, size sweeps */ }

#[test]
fn wraps_conditions_with_tabs() { /* tabs */ }

#[test]
fn keeps_short_condition_single_line_when_below_threshold() { /* no wrap */ }

#[test]
fn preserves_suffix_after_wrapped_condition() { /* suffix retained */ }

#[test]
fn handles_nested_parentheses_without_splitting_inner_expression() { /* top-level split only */ }

#[test]
fn normalizes_irregular_whitespace_before_wrapping() { /* whitespace normalization */ }
```

### Example behavior

Input:

```text
(user && isAdmin || hasFeature) // trailing
```

Expected (when threshold reached):

```text
(
     user
   && isAdmin
   || hasFeature
) // trailing
```

---

## 4) Angular `ngClass` Generative Tests

File: `crates/fua-plugin-angular/src/attributes.rs`

### Generator

```rust
fn generate_ngclass_expected(
    entries: &[NgClassEntry<'_>],
    wrap_conditions_min: usize,
    wrap_conditions_in_parens: bool,
    current_indent: usize,
    indent_size: usize,
    use_tabs: bool,
) -> String {
    // builds multiline object with optional wrapped boolean expressions
    // including support for negated grouped expressions: !(a || b || c)
    // ...
}
```

### Tests

```rust
#[test]
fn generates_ngclass_object_wrapping_like_real_template_case() { /* sample.html style */ }

#[test]
fn forces_single_line_ngclass_into_multiline_entries() { /* threshold-based force wrap */ }

#[test]
fn keeps_ngclass_unchanged_when_threshold_not_reached() { /* unchanged */ }
```

### Example behavior

Input:

```html
[ngClass]="{'border-accent': this.hasImageChanges, 'border-quaternary': !this.hasImageChanges}"
```

With `ngclass_wrap_entries_min=2`, expected:

```html
"{
			'border-accent': this.hasImageChanges,
			'border-quaternary': !this.hasImageChanges
			}"
```

---

## Notes

- The generator pattern intentionally computes expected output from config and structured input rather than hardcoding formatted strings for every case.
- This makes tests resilient to config permutations and better at catching behavioral regressions.
