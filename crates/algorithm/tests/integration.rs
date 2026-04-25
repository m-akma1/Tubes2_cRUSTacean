use algorithm::{bfs, bfs_parallel, dfs, dfs_parallel, lca, traverse, TreeIndex};
use css_selector::parse;
use shared::{DomTree, NodeData};

fn sample_tree() -> DomTree {
    let mut t = DomTree::new();
    let r = t.add_node(NodeData::Document, None).unwrap();
    let div = t
        .add_node(
            NodeData::Element {
                tag_name: "section".into(),
                id: None,
                classes: vec![],
                attributes: vec![],
            },
            Some(r),
        )
        .unwrap();
    let p = t
        .add_node(
            NodeData::Element {
                tag_name: "p".into(),
                id: None,
                classes: vec![],
                attributes: vec![],
            },
            Some(div),
        )
        .unwrap();
    let _ = p;
    t
}

#[test]
fn traverse_serial_and_parallel_bundle() {
    let tree = sample_tree();
    let sel = parse("section p").unwrap();
    let serial = traverse(tree.clone(), &sel, None, false);
    assert!(serial.bfs.is_some());
    assert!(serial.dfs.is_some());
    let parallel = traverse(tree, &sel, None, true);
    assert_eq!(
        serial.bfs.as_ref().unwrap().matched_indices,
        parallel.bfs.as_ref().unwrap().matched_indices
    );
}

#[test]
fn lca_on_sample_tree() {
    let t = sample_tree();
    assert_eq!(lca(&t, 1, 2), Some(1));
    let ti = TreeIndex::new(&t);
    assert!(ti.is_ancestor(1, 2));
}

#[test]
fn serial_parallel_traversal_counts_match() {
    let t = sample_tree();
    let sel = parse("p").unwrap();
    let b = bfs(&t, &sel, None);
    let bp = bfs_parallel(&t, &sel, None);
    let d = dfs(&t, &sel, None);
    let dp = dfs_parallel(&t, &sel, None);
    assert_eq!(b.matched_indices, bp.matched_indices);
    assert_eq!(d.matched_indices, dp.matched_indices);
}
