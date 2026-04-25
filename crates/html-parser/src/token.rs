//! Internal token representation decoupled from `html5ever` (for tests and the tree builder).

/// One high-level token after the tokenizer stage, before DOM insertion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token {
    /// `<!DOCTYPE ...>` declaration.
    Doctype {
        /// Doctype name, if any.
        name: Option<String>,
        /// Public identifier string.
        public_id: Option<String>,
        /// System identifier string.
        system_id: Option<String>,
    },
    /// Start tag (normal or self-closing).
    Start {
        /// Local tag name (ASCII lowercased for HTML elements).
        name: String,
        /// Raw `(name, value)` attribute pairs in tokenizer order.
        attrs: Vec<(String, String)>,
        /// `true` if the token ended with `/>`.
        self_closing: bool,
    },
    /// End tag `</name>`.
    End {
        /// Tag name (ASCII lowercased).
        name: String,
    },
    /// Character data (already decoded from character references).
    Text(
        /// Decoded text chunk.
        String,
    ),
    /// `<!-- ... -->` contents (without delimiters).
    Comment(
        /// Comment body.
        String,
    ),
    /// Tokenizer-level parse error description.
    ParseError(
        /// Error message from `html5ever`.
        String,
    ),
}
