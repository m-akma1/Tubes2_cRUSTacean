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