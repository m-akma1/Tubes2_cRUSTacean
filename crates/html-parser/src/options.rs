//! Tunable behaviour for [`crate::parse_with_options`].

/// Controls leniency and optional nodes when building a [`shared::DomTree`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseOptions {
    /// When `true`, the first unexpected end tag or tokenizer parse error stops the build.
    pub strict: bool,
    /// When `true`, a synthetic `!doctype` element is inserted for doctype tokens.
    pub include_doctype: bool,
}

impl Default for ParseOptions {
    /// Lenient parsing with doctype nodes enabled.
    fn default() -> Self {
        Self {
            strict: false,
            include_doctype: true,
        }
    }
}
