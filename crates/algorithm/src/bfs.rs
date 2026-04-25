//! Serial breadth-first traversal with selector matching.

use crate::index::TreeIndex;
use crate::matcher::matches;
use css_selector::ComplexSelector;
use shared::{AlgorithmKind, AlgorithmResult, DomTree, TraversalStep};
use std::collections::VecDeque;
use std::time::Instant;

/// Visits nodes level-by-level from the tree root, records each step, and collects up to `top_n`
/// matches (or unlimited when `top_n` is `None`).
pub fn bfs(tree: &DomTree, sel: &ComplexSelector, top_n: Option<usize>) -> AlgorithmResult {
    let ti = TreeIndex::new(tree);
    let mut steps = Vec::new();
    let mut matched = Vec::new();
    let t0 = Instant::now();

    let Some(root) = tree.root else {
        return AlgorithmResult {
            algorithm: AlgorithmKind::Bfs,
            matched_indices: Vec::new(),
            visited_count: 0,
            steps: Vec::new(),
            duration_ms: t0.elapsed().as_secs_f64() * 1000.0,
            top_n,
        };
    };

    let mut q = VecDeque::new();
    q.push_back((root, None));

    while let Some((node, from)) = q.pop_front() {
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
        for &c in &tree.nodes[node].children {
            q.push_back((c, Some(node)));
        }
    }

    let visited = steps.len();
    AlgorithmResult {
        algorithm: AlgorithmKind::Bfs,
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
    fn bfs_order_and_from_index() {
        let t = tiny_tree();
        let sel = parse("c").unwrap();
        let res = bfs(&t, &sel, None);
        assert_eq!(res.algorithm, AlgorithmKind::Bfs);
        // Level order: root 0, then 1,2, then 3 from 1
        let order: Vec<usize> = res.steps.iter().map(|s| s.node_index).collect();
        assert_eq!(order, vec![0, 1, 2, 3]);
        assert_eq!(res.steps[0].from_index, None);
        assert_eq!(res.steps[1].from_index, Some(0));
        assert_eq!(res.steps[3].from_index, Some(1));
    }

    #[test]
    fn bfs_top_n() {
        let t = tiny_tree();
        let sel = parse("*").unwrap();
        let res = bfs(&t, &sel, Some(1));
        assert_eq!(res.matched_indices.len(), 1);
    }
}
