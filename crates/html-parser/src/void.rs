//! HTML void elements (no closing tag, never stay on the open-element stack).

/// Returns `true` if `name` is a [void element](https://html.spec.whatwg.org/multipage/syntax.html#void-elements) (ASCII case-insensitive).
///
/// Void elements never receive a closing `</...>` in valid HTML and are modeled here
/// as nodes that are **not** pushed onto the open-element stack.
pub fn is_void(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}
