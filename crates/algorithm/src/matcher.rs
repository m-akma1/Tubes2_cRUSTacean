//! Right-to-left CSS selector matching against a [`crate::index::TreeIndex`].

use crate::index::TreeIndex;
use css_selector::{Combinator, ComplexSelector, SelectorSequence, SimpleSelector};
use shared::NodeData;

/// Returns whether `idx` matches `sel` (element targets only).
pub fn matches(idx: usize, ti: &TreeIndex, sel: &ComplexSelector) -> bool {
    let Some(node) = ti.tree.nodes.get(idx) else {
        return false;
    };
    if !matches!(node.data, NodeData::Element { .. }) {
        return false;
    }

    let n_seq = 1 + sel.rest.len();
    let last = n_seq - 1;
    if !matches_sequence(idx, ti, seq_at(sel, last)) {
        return false;
    }
    if last == 0 {
        return true;
    }
    walk_left(last - 1, idx, idx, ti, sel)
}

fn seq_at<'a>(sel: &'a ComplexSelector, i: usize) -> &'a SelectorSequence {
    if i == 0 {
        &sel.first
    } else {
        &sel.rest[i - 1].1
    }
}

/// `k` = index of the sequence we must satisfy to the left of the chain; `cur` already matches
/// `seq[k+1]..seq[last]`. `anchor` is the original subject node for descendant `is_ancestor` checks.
fn walk_left(k: usize, cur: usize, anchor: usize, ti: &TreeIndex, sel: &ComplexSelector) -> bool {
    match &sel.rest[k].0 {
        Combinator::Child => {
            let Some(p) = ti.parent(cur) else {
                return false;
            };
            if !matches_sequence(p, ti, seq_at(sel, k)) {
                return false;
            }
            if k == 0 {
                return true;
            }
            walk_left(k - 1, p, anchor, ti, sel)
        }
        Combinator::Descendant => {
            // Walk strict ancestors; each candidate must still be an ancestor of the original
            // anchor - enforced via LCA (`is_ancestor`) so the descendant combinator hits the
            // precomputed table once per candidate.
            let mut a = ti.parent(cur);
            while let Some(anc) = a {
                if matches_sequence(anc, ti, seq_at(sel, k)) && ti.is_ancestor(anc, anchor) {
                    if k == 0 {
                        return true;
                    }
                    if walk_left(k - 1, anc, anchor, ti, sel) {
                        return true;
                    }
                }
                a = ti.parent(anc);
            }
            false
        }
        Combinator::AdjacentSibling => {
            let Some(ps) = ti.prev_element_sibling(cur) else {
                return false;
            };
            if !matches_sequence(ps, ti, seq_at(sel, k)) {
                return false;
            }
            if k == 0 {
                return true;
            }
            walk_left(k - 1, ps, anchor, ti, sel)
        }
        Combinator::GeneralSibling => {
            for ps in ti.preceding_element_siblings(cur) {
                if matches_sequence(ps, ti, seq_at(sel, k)) {
                    if k == 0 {
                        return true;
                    }
                    if walk_left(k - 1, ps, anchor, ti, sel) {
                        return true;
                    }
                }
            }
            false
        }
    }
}

fn matches_sequence(idx: usize, ti: &TreeIndex, seq: &SelectorSequence) -> bool {
    seq.selectors
        .iter()
        .all(|s| matches_simple(idx, ti, s))
}

fn matches_simple(idx: usize, ti: &TreeIndex, s: &SimpleSelector) -> bool {
    let Some(node) = ti.tree.nodes.get(idx) else {
        return false;
    };
    let NodeData::Element {
        tag_name,
        id,
        classes,
        ..
    } = &node.data
    else {
        return false;
    };
    match s {
        SimpleSelector::Universal => true,
        SimpleSelector::Tag(t) => tag_name.eq_ignore_ascii_case(t),
        SimpleSelector::Class(c) => classes.iter().any(|x| x == c),
        SimpleSelector::Id(i) => id.as_deref() == Some(i.as_str()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::index::TreeIndex;
    use css_selector::parse;
    use shared::{DomTree, NodeData};

    fn elem(tag: &str, id: Option<&str>, classes: &[&str]) -> NodeData {
        NodeData::Element {
            tag_name: tag.into(),
            id: id.map(String::from),
            classes: classes.iter().map(|s| (*s).to_string()).collect(),
            attributes: vec![],
        }
    }

    fn tree_matcher_fixture() -> DomTree {
        let mut t = DomTree::new();
        let r = t.add_node(NodeData::Document, None).unwrap();
        let div = t.add_node(elem("div", None, &[]), Some(r)).unwrap();
        let p = t.add_node(elem("p", None, &[]), Some(div)).unwrap();
        let h1 = t.add_node(elem("h1", None, &[]), Some(div)).unwrap();
        let _ = t.add_node(NodeData::Text(" ".into()), Some(div)).unwrap();
        let p2 = t.add_node(elem("p", None, &[]), Some(div)).unwrap();
        let _ = (p, h1, p2);
        t
    }

    #[test]
    fn matcher_universal_tag_class_id_descendant_child_sibling() {
        let t = tree_matcher_fixture();
        let ti = TreeIndex::new(&t);

        assert!(matches(1, &ti, &parse("*").unwrap()));
        assert!(matches(1, &ti, &parse("div").unwrap()));
        assert!(!matches(2, &ti, &parse("div").unwrap()));

        let mut tc = DomTree::new();
        let r = tc.add_node(NodeData::Document, None).unwrap();
        let d = tc.add_node(elem("div", Some("a"), &["x"]), Some(r)).unwrap();
        let _ = tc.add_node(elem("span", None, &[]), Some(d)).unwrap();
        let tix = TreeIndex::new(&tc);
        assert!(matches(1, &tix, &parse(".x").unwrap()));
        assert!(matches(1, &tix, &parse("#a").unwrap()));
        assert!(matches(1, &tix, &parse("div.x#a").unwrap()));

        assert!(matches(2, &ti, &parse("div p").unwrap()));
        assert!(matches(2, &ti, &parse("div > p").unwrap()));
        assert!(matches(5, &ti, &parse("h1 + p").unwrap()));
        assert!(matches(5, &ti, &parse("h1 ~ p").unwrap()));
        assert!(!matches(2, &ti, &parse("h1 + p").unwrap()));
    }
}
