//! [`TreeIndex`] bundles a [`shared::DomTree`] reference with a precomputed [`crate::lca::LcaIndex`].

use crate::lca::LcaIndex;
use shared::{DomTree, NodeData};

/// Cached LCA table plus cheap navigation helpers for selector matching and traversals.
#[derive(Debug)]
pub struct TreeIndex<'a> {
    /// Borrowed DOM tree this index describes.
    pub tree: &'a DomTree,
    /// Binary-lifting table built from `tree`.
    pub lca: LcaIndex,
}

impl<'a> TreeIndex<'a> {
    /// Builds a new index (including LCA preprocessing) for `tree`.
    pub fn new(tree: &'a DomTree) -> Self {
        let lca = LcaIndex::build(tree);
        Self { tree, lca }
    }

    /// Returns the parent index of `i`, or `None` at the document root.
    pub fn parent(&self, i: usize) -> Option<usize> {
        self.tree.nodes.get(i)?.parent
    }

    /// Element children of `i` in tree order (skips text and comment nodes).
    pub fn element_children(&self, i: usize) -> impl Iterator<Item = usize> + '_ {
        self.tree
            .nodes
            .get(i)
            .into_iter()
            .flat_map(|n| n.children.iter().copied())
            .filter(move |&c| is_element(&self.tree.nodes[c].data))
    }

    /// The nearest preceding **element** sibling of `i`, if any.
    pub fn prev_element_sibling(&self, i: usize) -> Option<usize> {
        let p = self.tree.nodes.get(i)?.parent?;
        let ch = &self.tree.nodes[p].children;
        let pos = ch.iter().position(|&x| x == i)?;
        ch[..pos]
            .iter()
            .rev()
            .copied()
            .find(|&s| is_element(&self.tree.nodes[s].data))
    }

    /// All preceding element siblings of `i` from immediate to farthest.
    pub fn preceding_element_siblings(&self, i: usize) -> impl Iterator<Item = usize> + '_ {
        let mut cur = self.prev_element_sibling(i);
        std::iter::from_fn(move || {
            let out = cur?;
            cur = self.prev_element_sibling(out);
            Some(out)
        })
    }

    /// `O(log n)` ancestry test using the LCA table: `true` iff `a` is `b` or a strict ancestor
    /// of `b` in the DOM tree (same as `a` being on the path from the root to `b`).
    pub fn is_ancestor(&self, a: usize, b: usize) -> bool {
        self.lca.lca(a, b) == a
    }
}

fn is_element(data: &NodeData) -> bool {
    matches!(data, NodeData::Element { .. })
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::NodeData;

    fn elem(tag: &str) -> NodeData {
        NodeData::Element {
            tag_name: tag.into(),
            id: None,
            classes: vec![],
            attributes: vec![],
        }
    }

    fn tree_with_siblings() -> DomTree {
        let mut t = DomTree::new();
        let r = t.add_node(NodeData::Document, None).unwrap();
        let div = t.add_node(elem("div"), Some(r)).unwrap();
        let h1 = t.add_node(elem("h1"), Some(div)).unwrap();
        let _text = t.add_node(NodeData::Text(" ".into()), Some(div)).unwrap();
        let p = t.add_node(elem("p"), Some(div)).unwrap();
        let _ = (h1, p);
        t
    }

    #[test]
    fn is_ancestor_reflexive() {
        let t = tree_with_siblings();
        let ti = TreeIndex::new(&t);
        let p = 4usize;
        assert!(ti.is_ancestor(p, p));
    }

    #[test]
    fn is_ancestor_root_covers_all() {
        let t = tree_with_siblings();
        let ti = TreeIndex::new(&t);
        let root = 0usize;
        for i in 0..t.nodes.len() {
            assert!(ti.is_ancestor(root, i), "root should be ancestor of {i}");
        }
    }

    #[test]
    fn is_ancestor_sibling_false() {
        let t = tree_with_siblings();
        let ti = TreeIndex::new(&t);
        let h1 = 2usize;
        let p = 4usize;
        assert!(!ti.is_ancestor(h1, p));
        assert!(!ti.is_ancestor(p, h1));
    }

    #[test]
    fn prev_element_sibling_skips_text() {
        let t = tree_with_siblings();
        let ti = TreeIndex::new(&t);
        assert_eq!(ti.prev_element_sibling(4), Some(2));
    }
}
