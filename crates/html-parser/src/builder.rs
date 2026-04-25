//! Stack-based DOM construction from [`super::token::Token`]s.

use crate::error::ParseError;
use crate::options::ParseOptions;
use crate::token::Token;
use crate::void::is_void;
use shared::{DomTree, NodeData};

/// Incrementally builds a [`DomTree`] from a token stream.
pub(crate) struct Builder {
    /// Flat node arena produced by [`shared::DomTree::add_node`].
    tree: DomTree,
    /// Open element stack; index `0` is always the synthetic document node.
    open: Vec<usize>,
    /// Lenient vs strict behaviour (see [`ParseOptions`]).
    opts: ParseOptions,
    /// First fatal error, if any, after which [`Self::apply`] becomes a no-op.
    fatal: Option<ParseError>,
}

impl Builder {
    /// Creates an empty builder with a single document root and default open stack `[root]`.
    pub(crate) fn new(opts: ParseOptions) -> Self {
        let mut tree = DomTree::new();
        let root = tree
            .add_node(NodeData::Document, None)
            .expect("document root");
        Self {
            tree,
            open: vec![root],
            opts,
            fatal: None,
        }
    }

    /// Applies one token; records [`ParseError`] in strict mode instead of silently recovering.
    pub(crate) fn apply(&mut self, token: Token) {
        if self.fatal.is_some() {
            return;
        }
        match token {
            Token::Doctype {
                name,
                public_id,
                system_id,
            } => self.on_doctype(name, public_id, system_id),
            Token::Start {
                name,
                attrs,
                self_closing,
            } => self.on_start(name, attrs, self_closing),
            Token::End { name } => self.on_end(name),
            Token::Text(s) => self.on_text(&s),
            Token::Comment(s) => self.on_comment(s),
            Token::ParseError(msg) => self.on_parse_error(msg),
        }
    }

    /// Finalizes the tree: returns stored fatal errors, then closes stray open elements.
    pub(crate) fn finish(mut self) -> Result<DomTree, ParseError> {
        if let Some(e) = self.fatal.take() {
            return Err(e);
        }
        while self.open.len() > 1 {
            self.open.pop();
        }
        Ok(self.tree)
    }

    /// Inserts a synthetic `!doctype` element when [`ParseOptions::include_doctype`] is set.
    fn on_doctype(
        &mut self,
        name: Option<String>,
        public_id: Option<String>,
        system_id: Option<String>,
    ) {
        if !self.opts.include_doctype {
            return;
        }
        let parent = *self.open.last().expect("open stack non-empty");
        let attrs = vec![
            ("name".into(), name.unwrap_or_default()),
            ("public_id".into(), public_id.unwrap_or_default()),
            ("system_id".into(), system_id.unwrap_or_default()),
        ];
        let _ = self.tree.add_node(
            NodeData::Element {
                tag_name: "!doctype".into(),
                id: None,
                classes: Vec::new(),
                attributes: attrs,
            },
            Some(parent),
        );
    }

    /// Opens an element under the current node, applying void/self-closing and `<p>`-nesting rules.
    fn on_start(&mut self, name: String, attrs: Vec<(String, String)>, self_closing: bool) {
        if self.fatal.is_some() {
            return;
        }
        let tag_lc = name.to_ascii_lowercase();
        if tag_lc == "p" {
            self.close_open_p_for_new_p();
        }
        let parent = *self.open.last().expect("open stack");
        let (id, classes) = extract_id_classes(&attrs);
        let idx = match self.tree.add_node(
            NodeData::Element {
                tag_name: tag_lc.clone(),
                id,
                classes,
                attributes: attrs,
            },
            Some(parent),
        ) {
            Ok(i) => i,
            Err(e) => {
                self.set_fatal(ParseError::InvalidStructure(e));
                return;
            }
        };
        let void_el = is_void(&tag_lc);
        // Void and self-closing tags must not stay on the stack (no implicit children).
        if void_el || self_closing {
            return;
        }
        self.open.push(idx);
    }

    /// Closes the nearest matching open element, popping mismatched tags above it (HTML-style).
    fn on_end(&mut self, name: String) {
        if self.fatal.is_some() {
            return;
        }
        let tag_lc = name.to_ascii_lowercase();
        // Walk from the top of the open-element stack toward the document root.
        let mut pos: Option<usize> = None;
        for i in (1..self.open.len()).rev() {
            let idx = self.open[i];
            if let NodeData::Element { tag_name, .. } = &self.tree.nodes[idx].data {
                if tag_name.eq_ignore_ascii_case(&tag_lc) {
                    pos = Some(i);
                    break;
                }
            }
        }
        let Some(pos) = pos else {
            if self.opts.strict {
                self.set_fatal(ParseError::UnmatchedEnd { tag: tag_lc });
            }
            return;
        };
        // Pop every node *above* the matched frame, then pop the match (HTML auto-close).
        while self.open.len() > pos + 1 {
            self.open.pop();
        }
        self.open.pop();
    }

    /// Appends character data, merging with the previous [`NodeData::Text`] when possible.
    fn on_text(&mut self, chunk: &str) {
        if self.fatal.is_some() || chunk.is_empty() {
            return;
        }
        let parent = *self.open.last().expect("open stack");
        let last_child = self.tree.nodes[parent].children.last().copied();
        // Merge into the previous text node when possible so entity splits stay one DOM node.
        if let Some(lc) = last_child {
            if let NodeData::Text(ref mut prev) = self.tree.nodes[lc].data {
                prev.push_str(chunk);
                return;
            }
        }
        let _ = self.tree.add_node(NodeData::Text(chunk.into()), Some(parent));
    }

    /// Attaches a [`NodeData::Comment`] under the current open element.
    fn on_comment(&mut self, text: String) {
        if self.fatal.is_some() {
            return;
        }
        let parent = *self.open.last().expect("open stack");
        let _ = self.tree.add_node(NodeData::Comment(text), Some(parent));
    }

    /// Records tokenizer diagnostics; becomes fatal only when [`ParseOptions::strict`] is enabled.
    fn on_parse_error(&mut self, msg: String) {
        if self.opts.strict {
            self.set_fatal(ParseError::InvalidStructure(msg));
        }
    }

    /// Stores the first fatal [`ParseError`] so later tokens are ignored until [`Self::finish`].
    fn set_fatal(&mut self, e: ParseError) {
        self.fatal = Some(e);
    }

    /// HTML: a `<p>` start tag must close an open `<p>` so paragraphs do not nest.
    fn close_open_p_for_new_p(&mut self) {
        while self.open.len() > 1 {
            let idx = *self.open.last().expect("non-empty");
            let is_p = matches!(
                &self.tree.nodes[idx].data,
                NodeData::Element { tag_name, .. } if tag_name.eq_ignore_ascii_case("p")
            );
            if !is_p {
                break;
            }
            self.open.pop();
        }
    }
}

/// Pulls `id` and `class` out of the flat attribute list; `class` is split on HTML ASCII whitespace.
///
/// Other attributes are left untouched on the element; this helper only fills the typed fields.
fn extract_id_classes(attrs: &[(String, String)]) -> (Option<String>, Vec<String>) {
    let mut id = None;
    let mut classes = Vec::new();
    for (k, v) in attrs {
        if k.eq_ignore_ascii_case("id") {
            id = Some(v.clone());
        } else if k.eq_ignore_ascii_case("class") {
            // HTML `class` is a space-separated list; split without allocating a new string per token.
            classes.extend(
                v.split_ascii_whitespace()
                    .filter(|s| !s.is_empty())
                    .map(String::from),
            );
        }
    }
    (id, classes)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensures the builder can be driven directly from [`Token`] without the tokenizer.
    #[test]
    fn builder_token_path_round_trip() {
        let mut b = Builder::new(ParseOptions::default());
        b.apply(Token::Start {
            name: "div".into(),
            attrs: vec![("id".into(), "x".into())],
            self_closing: false,
        });
        b.apply(Token::Text("hello".into()));
        b.apply(Token::End {
            name: "div".into(),
        });
        let tree = b.finish().unwrap();
        assert_eq!(tree.node_count(), 3);
    }
}
