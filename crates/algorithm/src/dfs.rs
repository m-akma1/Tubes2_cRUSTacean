//! Serial depth-first (pre-order) traversal with an explicit stack.

use crate::index::TreeIndex;
use crate::matcher::matches;
use css_selector::ComplexSelector;
use shared::{AlgorithmKind, AlgorithmResult, DomTree, TraversalStep};
use std::time::Instant;

/// Visits nodes in pre-order (parent before children, children left-to-right), recording steps
/// and collecting up to `top_n` matches.
pub fn dfs(tree: &DomTree, sel: &ComplexSelector, top_n: Option<usize>) -> AlgorithmResult {
    let ti = TreeIndex::new(tree);
    let mut steps = Vec::new();
    let mut matched = Vec::new();
    let t0 = Instant::now();

    let Some(root) = tree.root else {
        return AlgorithmResult {
            algorithm: AlgorithmKind::Dfs,
            matched_indices: Vec::new(),
            visited_count: 0,
            steps: Vec::new(),
            duration_ms: t0.elapsed().as_secs_f64() * 1000.0,
            top_n,
        };
    };

    let mut stack = vec![(root, None)];
    while let Some((node, from)) = stack.pop() {
        let is_match = matches(node, &ti, sel);
        steps.push(TraversalStep {
            step: steps.len(),
            node_index: node,
            from_index: from,
            is_match,
        });
        if is_match {
            matched.push(node);
            if Some(matched.len()) == top_n {
                break;
            }
        }
        for &c in tree.nodes[node].children.iter().rev() {
            stack.push((c, Some(node)));
        }
    }

    let visited = steps.len();
    AlgorithmResult {
        algorithm: AlgorithmKind::Dfs,
        matched_indices: matched,
        visited_count: visited,
        steps,
        duration_ms: t0.elapsed().as_secs_f64() * 1000.0,
        top_n,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bfs::bfs;
    use css_selector::parse;
    use shared::NodeData;

    fn tiny_tree() -> DomTree {
        let mut t = DomTree::new();
        let r = t.add_node(NodeData::Document, None).unwrap();
        let a = t
            .add_node(
                NodeData::Element {
                    tag_name: "a".into(),
                    id: None,
                    classes: vec![],
                    attributes: vec![],
                },
                Some(r),
            )
            .unwrap();
        let b = t
            .add_node(
                NodeData::Element {
                    tag_name: "b".into(),
                    id: None,
                    classes: vec![],
                    attributes: vec![],
                },
                Some(r),
            )
            .unwrap();
        let c = t
            .add_node(
                NodeData::Element {
                    tag_name: "c".into(),
                    id: None,
                    classes: vec![],
                    attributes: vec![],
                },
                Some(a),
            )
            .unwrap();
        let _ = (b, c);
        t
    }

    #[test]
    fn dfs_preorder_differs_from_bfs() {
        let t = tiny_tree();
        let sel = parse("*").unwrap();
        let d = dfs(&t, &sel, None);
        let b = bfs(&t, &sel, None);
        let d_order: Vec<_> = d.steps.iter().map(|s| s.node_index).collect();
        let b_order: Vec<_> = b.steps.iter().map(|s| s.node_index).collect();
        assert_ne!(d_order, b_order);
        assert_eq!(d_order, vec![0, 1, 3, 2]);
    }
}
