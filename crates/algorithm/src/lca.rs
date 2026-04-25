//! Lowest common ancestor on a [`shared::DomTree`] using binary lifting.
//!
//! LCA does **not** make selector matching asymptotically faster; on a rooted tree the parent
//! pointer already gives `O(depth)` ancestry walks. We integrate it because (a) the bonus
//! calls for an LCA structure, (b) [`crate::index::TreeIndex::is_ancestor`] via LCA is a
//! semantically clean expression of the descendant combinator, and (c) the precomputed table
//! is amortised across this API and any future feature that needs ancestor queries.

use shared::DomTree;

/// Precomputed depths and `2^k`-jump pointers for LCA queries.
#[derive(Debug, Clone)]
pub struct LcaIndex {
    depth: Vec<usize>,
    up: Vec<Vec<usize>>,
    log: usize,
}

impl LcaIndex {
    /// Builds the index in `O(n log n)` time from `tree`.
    pub fn build(tree: &DomTree) -> Self {
        let n = tree.nodes.len();
        if n == 0 {
            return Self {
                depth: Vec::new(),
                up: Vec::new(),
                log: 0,
            };
        }

        let log = usize::BITS as usize - n.leading_zeros() as usize;
        let log = log.max(1);

        let mut depth = vec![0usize; n];
        for v in 0..n {
            depth[v] = tree.nodes[v].depth;
        }

        let mut up: Vec<Vec<usize>> = (0..n).map(|_| vec![0usize; log]).collect();

        for v in 0..n {
            up[v][0] = tree.nodes[v].parent.unwrap_or(v);
        }

        for k in 1..log {
            for v in 0..n {
                let mid = up[v][k - 1];
                up[v][k] = up[mid][k - 1];
            }
        }

        Self { depth, up, log }
    }

    /// Returns the LCA of `a` and `b` (inclusive). Both indices must be valid nodes in the
    /// tree this index was built from.
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        if self.depth.is_empty() {
            return 0;
        }
        if self.depth[a] < self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        // Lift the deeper node (`a`) up to the depth of `b`.
        let mut diff = self.depth[a] - self.depth[b];
        let mut bit = 0usize;
        while diff > 0 {
            if diff & 1 != 0 {
                a = self.up[a][bit];
            }
            diff >>= 1;
            bit += 1;
        }
        if a == b {
            return a;
        }
        for k in (0..self.log).rev() {
            if self.up[a][k] != self.up[b][k] {
                a = self.up[a][k];
                b = self.up[b][k];
            }
        }
        self.up[a][0]
    }
}

/// One-shot LCA without reusing a prebuilt table. Returns `None` if the tree is empty or
/// indices are out of range.
pub fn lca(tree: &DomTree, a: usize, b: usize) -> Option<usize> {
    if tree.nodes.is_empty() || a >= tree.nodes.len() || b >= tree.nodes.len() {
        return None;
    }
    Some(LcaIndex::build(tree).lca(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::{DomTree, NodeData};

    fn elem(tag: &str) -> NodeData {
        NodeData::Element {
            tag_name: tag.into(),
            id: None,
            classes: vec![],
            attributes: vec![],
        }
    }

    fn small_tree() -> DomTree {
        let mut t = DomTree::new();
        let r = t.add_node(NodeData::Document, None).expect("root");
        let div = t.add_node(elem("div"), Some(r)).expect("div");
        let p1 = t.add_node(elem("p"), Some(div)).expect("p1");
        let p2 = t.add_node(elem("p"), Some(div)).expect("p2");
        let _ = (p1, p2);
        t
    }

    /// Document -> outer div -> two inner divs -> two spans (cousins).
    fn cousins_tree() -> DomTree {
        let mut t = DomTree::new();
        let r = t.add_node(NodeData::Document, None).expect("root");
        let outer = t.add_node(elem("div"), Some(r)).expect("outer");
        let d1 = t.add_node(elem("div"), Some(outer)).expect("d1");
        let d2 = t.add_node(elem("div"), Some(outer)).expect("d2");
        let s1 = t.add_node(elem("span"), Some(d1)).expect("s1");
        let s2 = t.add_node(elem("span"), Some(d2)).expect("s2");
        let _ = (s1, s2);
        t
    }

    #[test]
    fn lca_root_and_descendant() {
        let t = small_tree();
        let idx = LcaIndex::build(&t);
        assert_eq!(idx.lca(1, 2), 1);
    }

    #[test]
    fn lca_siblings_is_parent() {
        let t = small_tree();
        let idx = LcaIndex::build(&t);
        assert_eq!(idx.lca(2, 3), 1);
    }

    #[test]
    fn lca_cousins_is_grandparent_outer() {
        let t = cousins_tree();
        let idx = LcaIndex::build(&t);
        // indices: 0 doc, 1 outer, 2 d1, 3 d2, 4 s1, 5 s2
        assert_eq!(idx.lca(4, 5), 1);
    }

    #[test]
    fn one_shot_lca() {
        let t = small_tree();
        assert_eq!(lca(&t, 2, 3), Some(1));
    }
}
