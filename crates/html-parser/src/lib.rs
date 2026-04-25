#![warn(missing_docs)]
//! HTML string to [`shared::DomTree`] conversion.
//!
//! Tokenization is delegated to `html5ever`'s low-level [`html5ever::tokenizer::Tokenizer`];
//! the DOM tree itself is built here with an explicit open-element stack rather than
//! `html5ever`'s own tree builder, giving full control over node identity, ordering, and
//! attribute preservation.
//!
//! Public entry points are [`parse`] and [`parse_with_options`].

mod builder;
mod error;
mod options;
mod sink;
mod token;
mod void;

pub use error::ParseError;
pub use options::ParseOptions;

use builder::Builder;
use html5ever::tendril::StrTendril;
use html5ever::tokenizer::{BufferQueue, Tokenizer, TokenizerOpts, TokenizerResult};
use shared::DomTree;
use sink::Html5Sink;

/// Parses `html` into a [`DomTree`] using [`ParseOptions::default`].
///
/// # Errors
///
/// Returns [`ParseError::EmptyInput`] if `html` is blank after trimming, or any error
/// from [`parse_with_options`].
pub fn parse(html: &str) -> Result<DomTree, ParseError> {
    parse_with_options(html, &ParseOptions::default())
}

/// Parses `html` with explicit [`ParseOptions`].
///
/// # Errors
///
/// - [`ParseError::EmptyInput`] for whitespace-only input.
/// - [`ParseError::UnmatchedEnd`] / [`ParseError::InvalidStructure`] when `strict` is enabled.
pub fn parse_with_options(html: &str, opts: &ParseOptions) -> Result<DomTree, ParseError> {
    if html.trim().is_empty() {
        return Err(ParseError::EmptyInput);
    }
    let sink = Html5Sink::new(Builder::new(opts.clone()));
    let mut tokenizer = Tokenizer::new(sink, TokenizerOpts::default());
    let mut input = BufferQueue::default();
    input.push_back(StrTendril::from(html));
    loop {
        match tokenizer.feed(&mut input) {
            TokenizerResult::Done => {
                if input.is_empty() {
                    break;
                }
            }
            TokenizerResult::Script(_) => continue,
        }
    }
    tokenizer.end();
    let Html5Sink { builder } = tokenizer.sink;
    builder.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::NodeData;

    #[test]
    fn empty_input_errors() {
        assert!(matches!(
            parse("  \n\t"),
            Err(ParseError::EmptyInput)
        ));
    }

    #[test]
    fn simple_div() {
        let tree = parse("<div></div>").unwrap();
        assert_eq!(tree.node_count(), 2);
        let root = tree.root.expect("root");
        assert!(matches!(tree.nodes[root].data, NodeData::Document));
        let div_i = tree.nodes[root].children[0];
        assert!(
            matches!(&tree.nodes[div_i].data, NodeData::Element { tag_name, .. } if tag_name == "div")
        );
    }

    #[test]
    fn void_img_br() {
        let tree = parse(r#"<img src="x"><br>"#).unwrap();
        let root = tree.root.unwrap();
        let kids: Vec<_> = tree.nodes[root]
            .children
            .iter()
            .map(|&i| &tree.nodes[i].data)
            .collect();
        assert_eq!(kids.len(), 2);
        assert!(matches!(kids[0], NodeData::Element { tag_name, .. } if tag_name == "img"));
        assert!(matches!(kids[1], NodeData::Element { tag_name, .. } if tag_name == "br"));
    }

    #[test]
    fn id_and_class_attributes() {
        let tree = parse(r#"<div id="a" class="x y"></div>"#).unwrap();
        let root = tree.root.unwrap();
        let div_i = tree.nodes[root].children[0];
        let NodeData::Element {
            id,
            classes,
            attributes,
            ..
        } = &tree.nodes[div_i].data
        else {
            panic!("expected element");
        };
        assert_eq!(id.as_deref(), Some("a"));
        assert_eq!(classes, &["x", "y"]);
        assert!(attributes.iter().any(|(k, v)| k == "id" && v == "a"));
        assert!(attributes.iter().any(|(k, v)| k == "class" && v == "x y"));
    }

    #[test]
    fn p_auto_close_siblings() {
        let tree = parse("<p>hi<p>there</p>").unwrap();
        let root = tree.root.unwrap();
        let c = &tree.nodes[root].children;
        assert_eq!(c.len(), 2, "expected two sibling <p> elements");
        let p0 = c[0];
        let p1 = c[1];
        assert!(matches!(&tree.nodes[p0].data, NodeData::Element { tag_name, .. } if tag_name == "p"));
        assert!(matches!(&tree.nodes[p1].data, NodeData::Element { tag_name, .. } if tag_name == "p"));
        let p0_text = tree.nodes[p0].children[0];
        let p1_text = tree.nodes[p1].children[0];
        assert!(matches!(&tree.nodes[p0_text].data, NodeData::Text(s) if s == "hi"));
        assert!(matches!(&tree.nodes[p1_text].data, NodeData::Text(s) if s == "there"));
    }

    #[test]
    fn comment_node() {
        let tree = parse("<!-- comment --><div></div>").unwrap();
        let root = tree.root.unwrap();
        assert!(matches!(
            &tree.nodes[tree.nodes[root].children[0]].data,
            NodeData::Comment(s) if s == " comment "
        ));
    }

    #[test]
    fn script_rawtext() {
        let tree = parse("<script>if (a<b) {}</script>").unwrap();
        let root = tree.root.unwrap();
        let script = tree.nodes[root].children[0];
        assert!(matches!(&tree.nodes[script].data, NodeData::Element { tag_name, .. } if tag_name == "script"));
        assert_eq!(tree.nodes[script].children.len(), 1);
        let t = tree.nodes[script].children[0];
        assert!(matches!(
            &tree.nodes[t].data,
            NodeData::Text(s) if s == "if (a<b) {}"
        ));
    }

    #[test]
    fn entity_merges_to_single_text() {
        let tree = parse("<span>a&amp;b</span>").unwrap();
        let root = tree.root.unwrap();
        let span = tree.nodes[root].children[0];
        let t = tree.nodes[span].children[0];
        assert!(matches!(&tree.nodes[t].data, NodeData::Text(s) if s == "a&b"));
    }

    #[test]
    fn doctype_default_options() {
        let tree = parse("<!DOCTYPE html><html></html>").unwrap();
        let root = tree.root.unwrap();
        let dt = tree.nodes[root].children[0];
        assert!(matches!(
            &tree.nodes[dt].data,
            NodeData::Element { tag_name, .. } if tag_name == "!doctype"
        ));
    }

    #[test]
    fn strict_unmatched_end() {
        let opts = ParseOptions {
            strict: true,
            include_doctype: true,
        };
        let r = parse_with_options("</div>", &opts);
        assert!(matches!(r, Err(ParseError::UnmatchedEnd { .. })));
    }
}
