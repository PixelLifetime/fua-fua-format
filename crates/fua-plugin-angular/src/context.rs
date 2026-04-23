pub(crate) fn is_content_context(parent_kind: &str) -> bool {
    parent_kind == "ELEMENT" || parent_kind == "ROOT"
}

pub(crate) fn is_block_opener(text: &str) -> bool {
    matches!(
        text,
        "@if" | "@for" | "@switch" | "@case" | "@default" | "@empty"
    )
}

pub(crate) fn is_else_like(text: &str) -> bool {
    text == "@else" || text.starts_with("@else ")
}

pub(crate) fn is_angular_binding(attr_name: &str) -> bool {
    let name = attr_name.trim();
    (name.starts_with('[') && name.ends_with(']'))
        || (name.starts_with('(') && name.ends_with(')'))
        || name.starts_with("*ng")
        || name.starts_with("*cdk")
}

pub(crate) fn is_ngclass_attr(attr_name: &str) -> bool {
    let name = attr_name.trim();
    name.eq_ignore_ascii_case("[ngclass]") || name.eq_ignore_ascii_case("ngclass")
}

pub(crate) fn is_class_attr(attr_name: &str) -> bool {
    attr_name.trim().eq_ignore_ascii_case("class")
}
