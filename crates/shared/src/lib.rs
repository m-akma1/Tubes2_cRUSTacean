use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeData {
    Document,
    Element {
        tag_name: String,
        id: Option<String>,
        classes: Vec<String>,
        attributes: Vec<(String, String)>,
    },
    Text(String),
    Comment(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub index: usize,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub data: NodeData,
    pub depth: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DomTree {
    pub nodes: Vec<Node>,
    pub root: Option<usize>,
}

impl DomTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, data: NodeData, parent: Option<usize>) -> Result<usize, String> {
        let depth = if let Some(p) = parent {
            self.nodes.get(p)
                    .map(|n| n.depth + 1)
                    .ok_or_else(|| format!("Parent index {} does not exist!", p))?
        } else {0};

        let index = self.nodes.len();
        if self.nodes.is_empty() {
            self.root = Some(index);
        }

        self.nodes.push(Node {
            index,
            parent,
            children: Vec::new(),
            data,
            depth,
        });        
        if let Some(p) = parent {
            self.nodes[p].children.push(index);
        }

        return Ok(index)
    }

    pub fn max_depth(&self) -> usize {
        self.nodes.iter().map(|n| n.depth).max().unwrap_or(0)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn validate_integrity(&self) -> Result<(), String> {
        let n = self.nodes.len();
        if n == 0 {
            if self.root.is_some() {
                return Err("root must be None when tree is empty".to_string());
            }
            return Ok(());
        }

        let root = self
            .root
            .ok_or_else(|| "root must be set when tree is non-empty".to_string())?;
        if root >= n {
            return Err(format!("root index {} is out of bounds", root));
        }

        for (i, node) in self.nodes.iter().enumerate() {
            if node.index != i {
                return Err(format!(
                    "node at position {} has inconsistent index field {}",
                    i, node.index
                ));
            }

            if let Some(parent) = node.parent {
                if parent >= n {
                    return Err(format!("node {} has out-of-bounds parent {}", i, parent));
                }
            }
        }

        let root_node = &self.nodes[root];
        if root_node.parent.is_some() {
            return Err("root node must not have a parent".to_string());
        }
        if root_node.depth != 0 {
            return Err("root node depth must be 0".to_string());
        }

        let mut seen = vec![false; n];
        let mut stack = vec![root];

        while let Some(node_index) = stack.pop() {
            if seen[node_index] {
                return Err(format!(
                    "cycle or duplicate reachability detected at node {}",
                    node_index
                ));
            }
            seen[node_index] = true;

            let node = &self.nodes[node_index];
            if let Some(parent) = node.parent {
                let parent_node = &self.nodes[parent];
                if !parent_node.children.contains(&node_index) {
                    return Err(format!(
                        "node {} parent {} does not reference it as a child",
                        node_index, parent
                    ));
                }
                if node.depth != parent_node.depth + 1 {
                    return Err(format!(
                        "node {} depth {} is inconsistent with parent depth {}",
                        node_index, node.depth, parent_node.depth
                    ));
                }
            }

            let mut unique_children = std::collections::HashSet::new();
            for &child in &node.children {
                if child >= n {
                    return Err(format!(
                        "node {} references out-of-bounds child {}",
                        node_index, child
                    ));
                }
                if !unique_children.insert(child) {
                    return Err(format!(
                        "node {} has duplicated child reference {}",
                        node_index, child
                    ));
                }

                let child_node = &self.nodes[child];
                if child_node.parent != Some(node_index) {
                    return Err(format!(
                        "child {} parent {:?} does not point back to node {}",
                        child, child_node.parent, node_index
                    ));
                }
                if child_node.depth != node.depth + 1 {
                    return Err(format!(
                        "child {} depth {} is inconsistent with parent {} depth {}",
                        child, child_node.depth, node_index, node.depth
                    ));
                }
                stack.push(child);
            }
        }

        if let Some(disconnected) = seen.iter().position(|&ok| !ok) {
            return Err(format!("node {} is disconnected from root", disconnected));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlgorithmKind {
    Bfs,
    Dfs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalStep {
    pub step: usize,
    pub node_index: usize,
    pub from_index: Option<usize>,
    pub is_match: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmResult {
    pub algorithm: AlgorithmKind,
    pub matched_indices: Vec<usize>,
    pub visited_count: usize,
    pub steps: Vec<TraversalStep>,
    pub duration_ms: f64,
    pub top_n: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalBundle {
    pub bfs: Option<AlgorithmResult>,
    pub dfs: Option<AlgorithmResult>,
    pub tree: DomTree,                 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeStats {
    pub node_count: usize,
    pub edge_count: usize,
    pub max_depth: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOptionsDto {
    pub strict: bool,
    pub include_doctype: bool,
}

impl Default for ParseOptionsDto {
    fn default() -> Self {
        Self {
            strict: false,
            include_doctype: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapeTreeResponse {
    pub requested_url: String,
    pub final_url: String,
    pub status_code: u16,
    pub content_type: Option<String>,
    pub html: Option<String>,
    pub tree: DomTree,
    pub stats: TreeStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseHtmlRequest {
    pub html: String,
    pub options: Option<ParseOptionsDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseHtmlResponse {
    pub tree: DomTree,
    pub stats: TreeStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraverseRequest {
    pub tree: DomTree,
    pub selector: String,
    pub algorithm: AlgorithmKind,
    pub top_n: Option<usize>,
    #[serde(default)]
    pub parallel: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraverseResponse {
    pub result: Option<AlgorithmResult>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LcaRequest {
    pub tree: DomTree,
    pub node_a: usize,
    pub node_b: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LcaResponse {
    pub found: bool,
    pub lca_index: Option<usize>,
    pub message: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_integrity_accepts_valid_tree() {
        let mut tree = DomTree::new();
        let root = tree.add_node(NodeData::Document, None).unwrap();
        let _ = tree
            .add_node(
                NodeData::Element {
                    tag_name: "div".to_string(),
                    id: None,
                    classes: vec![],
                    attributes: vec![],
                },
                Some(root),
            )
            .unwrap();

        assert!(tree.validate_integrity().is_ok());
    }

    #[test]
    fn validate_integrity_rejects_out_of_bounds_child() {
        let mut tree = DomTree::new();
        let root = tree.add_node(NodeData::Document, None).unwrap();
        tree.nodes[root].children.push(999);

        assert!(tree.validate_integrity().is_err());
    }
}
