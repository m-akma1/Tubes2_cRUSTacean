//! Rayon-parallel BFS and DFS variants.
//!
//! Step order in parallel mode follows task completion order, not strict BFS/DFS order - this
//! matches the tolerance described for early-exit in large traversals.

use crate::index::TreeIndex;
use crate::matcher::matches;
use crate::recorder::Recorder;
use css_selector::ComplexSelector;
use rayon::prelude::*;
use shared::{AlgorithmKind, AlgorithmResult, DomTree};
use std::time::Instant;

/// Level-synchronous parallel BFS: each frontier is processed with `par_iter`.
pub fn bfs_parallel(tree: &DomTree, sel: &ComplexSelector, top_n: Option<usize>) -> AlgorithmResult {
    let ti = TreeIndex::new(tree);
    let recorder = Recorder::new(top_n);
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

    let mut level = vec![(root, None)];
    while !level.is_empty() && !recorder.is_done() {
        level.par_iter().for_each(|&(node, from)| {
            if recorder.is_done() {
                return;
            }
            let is_match = matches(node, &ti, sel);
            recorder.record_visit(node, from, is_match);
        });

        if recorder.is_done() {
            break;
        }

        let mut next = Vec::new();
        for &(node, _) in &level {
            if recorder.is_done() {
                break;
            }
            for &c in &tree.nodes[node].children {
                next.push((c, Some(node)));
            }
        }
        level = next;
    }

    recorder.into_algorithm_result(AlgorithmKind::Bfs, top_n, t0)
}

/// Parallel depth-first traversal: each node's children are split and processed with
/// [`rayon::join`], respecting the same `top_n` early-stop flag as [`crate::bfs_parallel`].
pub fn dfs_parallel(tree: &DomTree, sel: &ComplexSelector, top_n: Option<usize>) -> AlgorithmResult {
    let ti = TreeIndex::new(tree);
    let recorder = Recorder::new(top_n);
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

    dfs_par_visit(root, None, tree, &ti, sel, &recorder);

    recorder.into_algorithm_result(AlgorithmKind::Dfs, top_n, t0)
}

fn dfs_par_visit(
    node: usize,
    from: Option<usize>,
    tree: &DomTree,
    ti: &TreeIndex<'_>,
    sel: &ComplexSelector,
    rec: &Recorder,
) {
    if rec.is_done() {
        return;
    }
    let is_match = matches(node, ti, sel);
    rec.record_visit(node, from, is_match);
    if rec.is_done() {
        return;
    }

    let children: Vec<usize> = tree.nodes[node].children.clone();
    if children.is_empty() {
        return;
    }
    if children.len() == 1 {
        dfs_par_visit(children[0], Some(node), tree, ti, sel, rec);
        return;
    }

    let mid = children.len() / 2;
    let (left, right) = children.split_at(mid);
    rayon::join(
        || {
            for &c in left {
                if rec.is_done() {
                    break;
                }
                dfs_par_visit(c, Some(node), tree, ti, sel, rec);
            }
        },
        || {
            for &c in right {
                if rec.is_done() {
                    break;
                }
                dfs_par_visit(c, Some(node), tree, ti, sel, rec);
            }
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bfs::bfs;
    use crate::dfs::dfs;
    use css_selector::parse;
    use shared::NodeData;
    use std::collections::HashSet;

    fn wide_tree() -> DomTree {
        let mut t = DomTree::new();
        let r = t.add_node(NodeData::Document, None).unwrap();
        for _ in 0..6 {
            t.add_node(
                NodeData::Element {
                    tag_name: "span".into(),
                    id: None,
                    classes: vec![],
                    attributes: vec![],
                },
                Some(r),
            )
            .unwrap();
        }
        t
    }

    #[test]
    fn parallel_sets_match_serial_for_bfs_dfs() {
        let tree = wide_tree();
        let sel = parse("span").unwrap();
        let b = bfs(&tree, &sel, None);
        let bp = bfs_parallel(&tree, &sel, None);
        let d = dfs(&tree, &sel, None);
        let dp = dfs_parallel(&tree, &sel, None);
        assert_eq!(
            b.matched_indices.iter().copied().collect::<HashSet<_>>(),
            bp.matched_indices.iter().copied().collect::<HashSet<_>>()
        );
        assert_eq!(
            d.matched_indices.iter().copied().collect::<HashSet<_>>(),
            dp.matched_indices.iter().copied().collect::<HashSet<_>>()
        );
    }

    #[test]
    fn parallel_top_n_is_capped() {
        let tree = wide_tree();
        let sel = parse("span").unwrap();

        let bfs_res = bfs_parallel(&tree, &sel, Some(2));
        let dfs_res = dfs_parallel(&tree, &sel, Some(2));

        assert!(bfs_res.matched_indices.len() <= 2);
        assert!(dfs_res.matched_indices.len() <= 2);
    }
}
