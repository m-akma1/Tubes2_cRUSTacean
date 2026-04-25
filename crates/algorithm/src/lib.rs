#![warn(missing_docs)]
//! DOM traversals (BFS / DFS), serial and Rayon-parallel, plus a right-to-left CSS selector matcher
//! and binary-lifting LCA support bundled in [`TreeIndex`].
//!
//! Selector strings are parsed elsewhere; this crate matches already-parsed [`css_selector::ComplexSelector`] values.

mod bfs;
mod dfs;
mod index;
mod lca;
mod matcher;
mod parallel;
mod recorder;

pub use index::TreeIndex;
pub use lca::{lca, LcaIndex};
pub use matcher::matches;

pub use bfs::bfs;
pub use dfs::dfs;
pub use parallel::{bfs_parallel, dfs_parallel};

use css_selector::ComplexSelector;
use shared::{DomTree, TraversalBundle};

/// Runs BFS and DFS on `tree` with the same selector and `top_n` cap; when `parallel` is `true`,
/// uses the Rayon-backed traversals.
pub fn traverse(
    tree: DomTree,
    selector: &ComplexSelector,
    top_n: Option<usize>,
    parallel: bool,
) -> TraversalBundle {
    let (bfs_res, dfs_res) = if parallel {
        (
            Some(bfs_parallel(&tree, selector, top_n)),
            Some(dfs_parallel(&tree, selector, top_n)),
        )
    } else {
        (
            Some(bfs(&tree, selector, top_n)),
            Some(dfs(&tree, selector, top_n)),
        )
    };
    TraversalBundle {
        bfs: bfs_res,
        dfs: dfs_res,
        tree,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use css_selector::parse;
    use shared::NodeData;

    #[test]
    fn traverse_bundle_non_empty() {
        let mut t = DomTree::new();
        let r = t.add_node(NodeData::Document, None).unwrap();
        let _ = t
            .add_node(
                NodeData::Element {
                    tag_name: "div".into(),
                    id: None,
                    classes: vec![],
                    attributes: vec![],
                },
                Some(r),
            )
            .unwrap();
        let sel = parse("div").unwrap();
        let bundle = traverse(t, &sel, None, false);
        assert!(bundle.bfs.is_some());
        assert!(bundle.dfs.is_some());
        assert!(!bundle.bfs.as_ref().unwrap().matched_indices.is_empty());
        assert!(!bundle.dfs.as_ref().unwrap().matched_indices.is_empty());
    }
}
