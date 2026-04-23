use super::FormatSession;

impl FormatSession {
    pub(super) fn finish(self) -> String {
        self.output
    }

    pub(super) fn current_line_width(&self) -> usize {
        self.output
            .rsplit('\n')
            .next()
            .unwrap_or_default()
            .chars()
            .count()
    }

    pub(super) fn adjust_indent(&mut self, delta: i32) {
        if delta < 0 {
            self.current_indent = self.current_indent.saturating_sub((-delta) as usize);
        } else {
            self.current_indent += delta as usize;
        }
    }

    pub(super) fn push_current_indent(&mut self) {
        self.push_indent(self.current_indent);
    }

    pub(super) fn push_indent_at(&mut self, depth: usize) {
        self.push_indent(depth);
    }

    fn push_indent(&mut self, depth: usize) {
        if self.config.use_tabs {
            self.output.push_str(&"\t".repeat(depth));
        } else {
            self.output
                .push_str(&" ".repeat(depth * self.config.indent_size));
        }
    }

    pub(super) fn ensure_space(&mut self) {
        if self
            .output
            .chars()
            .next_back()
            .is_some_and(|ch| ch == ' ' || ch == '\t' || ch == '\n')
        {
            return;
        }

        self.output.push(' ');
    }

    pub(super) fn push_single_indent_unit(&mut self) {
        self.push_indent(1);
    }

    pub(super) fn push_newlines_with_indent(&mut self, count: usize) {
        trim_trailing_horizontal_whitespace(&mut self.output);

        let trailing_newlines = self
            .output
            .chars()
            .rev()
            .take_while(|ch| *ch == '\n')
            .count();
        for _ in trailing_newlines..count {
            self.output.push('\n');
        }

        self.push_current_indent();
    }
}

fn trim_trailing_horizontal_whitespace(output: &mut String) {
    let mut trim_len = output.len();
    for ch in output.chars().rev() {
        if ch == ' ' || ch == '\t' {
            trim_len -= ch.len_utf8();
        } else {
            break;
        }
    }
    output.truncate(trim_len);
}
