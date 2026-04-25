use shared::{AlgorithmKind, AlgorithmResult, DomTree, Node, NodeData, TraversalStep};
use std::collections::VecDeque;


/// Template tree sementara buat testing aja.
/// ```
/// Document (0)
///  └─ html (1)
///      ├─ head (2)
///      │   └─ title (3)
///      │       └─ "cRUSTacean" (4)
///      └─ body (5)
///          ├─ div#app.container (6)
///          │   ├─ h1.title (7)
///          │   │   └─ "Judul Random" (8)
///          │   ├─ p.intro (9)
///          │   │   ├─ "Bla bla yapp" (10)
///          │   │   └─ span.highlight (11)
///          │   │       └─ "WOW SAYA SUKA TUBES" (12)
///          │   └─ ul (13)
///          │       ├─ li (14)  └─ "BFS" (15)
///          │       ├─ li (16)  └─ "DFS" (17)
///          │       └─ li (18)  └─ "LCA" (19)
///          └─ footer (20)
///              └─ p (21)
///                  └─ "IF2211 — Tugas Besar 2" (22)
/// ```
/// Expected targetnya di node 11 (span.highlight)


pub fn temp_html_parser(_input: &str) -> Result<DomTree, String> {
    let mut t = DomTree::new();

    macro_rules! elem {
        ($tag:expr) => {
            NodeData::Element {
                tag_name: $tag.into(),
                id: None,
                classes: vec![],
                attributes: vec![],
            }
        };
        ($tag:expr, id=$id:expr) => {
            NodeData::Element {
                tag_name: $tag.into(),
                id: Some($id.into()),
                classes: vec![],
                attributes: vec![],
            }
        };
        ($tag:expr, cls=$cls:expr) => {
            NodeData::Element {
                tag_name: $tag.into(),
                id: None,
                classes: vec![$cls.into()],
                attributes: vec![],
            }
        };
        ($tag:expr, id=$id:expr, cls=$cls:expr) => {
            NodeData::Element {
                tag_name: $tag.into(),
                id: Some($id.into()),
                classes: vec![$cls.into()],
                attributes: vec![],
            }
        };
        ($tag:expr, attr=$key:expr => $val:expr) => {
            NodeData::Element {
                tag_name: $tag.into(),
                id: None,
                classes: vec![],
                attributes: vec![($key.into(), $val.into())],
            }
        };
    }

    // 0: Root
    let doc   = t.add_node(NodeData::Document,              None)?;
    // 1: <html lang="en">
    let html  = t.add_node(elem!("html", attr="lang"=>"en"), Some(doc))?;
    // 2-4: <head><title>text
    let head  = t.add_node(elem!("head"),                   Some(html))?;
    let title = t.add_node(elem!("title"),                  Some(head))?;
    /*  4 */ t.add_node(NodeData::Text("cRUSTacean".into()), Some(title))?;
    // 5: <body>
    let body  = t.add_node(elem!("body"),                   Some(html))?;
    // 6: <div id="app" class="container">
    let app   = t.add_node(elem!("div", id="app", cls="container"), Some(body))?;
    // 7-8: <h1 class="title">text
    let h1    = t.add_node(elem!("h1", cls="title"),        Some(app))?;
    /*  8 */ t.add_node(NodeData::Text("HTML Tree Search".into()), Some(h1))?;
    // 9-12: <p class="intro"> text <span class="highlight"> text
    let p     = t.add_node(elem!("p", cls="intro"),         Some(app))?;
    /* 10 */ t.add_node(NodeData::Text("Explore the ".into()), Some(p))?;
    let span  = t.add_node(elem!("span", cls="highlight"),  Some(p))?;  // target
    /* 12 */ t.add_node(NodeData::Text("DOM structure".into()), Some(span))?;
    // 13-19: <ul><li>
    let ul    = t.add_node(elem!("ul"),                     Some(app))?;
    let li1   = t.add_node(elem!("li"),                     Some(ul))?;
    /* 15 */ t.add_node(NodeData::Text("BFS Search".into()), Some(li1))?;
    let li2   = t.add_node(elem!("li"),                     Some(ul))?;
    /* 17 */ t.add_node(NodeData::Text("DFS Search".into()), Some(li2))?;
    let li3   = t.add_node(elem!("li"),                     Some(ul))?;
    /* 19 */ t.add_node(NodeData::Text("CSS Selectors".into()), Some(li3))?;
    // 20-22: <footer><p>text
    let footer = t.add_node(elem!("footer"),                Some(body))?;
    let fp     = t.add_node(elem!("p"),                     Some(footer))?;
    /* 22 */ t.add_node(NodeData::Text("IF2211 — Tugas Besar 2".into()), Some(fp))?;

    Ok(t)
}


pub fn temp_algoritm(
    tree: &DomTree,
    kind: AlgorithmKind,
    top_n: Option<usize>,
) -> AlgorithmResult {
    let mut steps: Vec<TraversalStep> = Vec::new();
    let mut matched_indices: Vec<usize> = Vec::new();

    let is_match = |node: &Node| -> bool {
        matches!(&node.data,
            NodeData::Element { tag_name, classes, .. }
            if tag_name == "span" || classes.iter().any(|c| c == "highlight")
        )
    };

    let root = match tree.root {
        Some(r) => r,
        None => return AlgorithmResult {
            algorithm: kind,
            matched_indices: vec![],
            visited_count: 0,
            steps: vec![],
            duration_ms: 0.0,
            top_n,
        },
    };

    match kind {
        AlgorithmKind::Bfs => {
            let mut queue: VecDeque<(usize, Option<usize>)> = VecDeque::new();
            queue.push_back((root, None));

            while let Some((idx, from)) = queue.pop_front() {
                let node = &tree.nodes[idx];
                let m = is_match(node);
                steps.push(TraversalStep { step: steps.len(), node_index: idx, from_index: from, is_match: m });
                if m {
                    matched_indices.push(idx);
                    if top_n.is_some_and(|n| matched_indices.len() >= n) { break; }
                }
                for &child in &node.children {
                    queue.push_back((child, Some(idx)));
                }
            }
        }

        AlgorithmKind::Dfs => {
            let mut stack: Vec<(usize, Option<usize>)> = vec![(root, None)];

            while let Some((idx, from)) = stack.pop() {
                let node = &tree.nodes[idx];
                let m = is_match(node);
                steps.push(TraversalStep { step: steps.len(), node_index: idx, from_index: from, is_match: m });
                if m {
                    matched_indices.push(idx);
                    if top_n.is_some_and(|n| matched_indices.len() >= n) { break; }
                }
                // Push children reversed so left-child is visited first
                for &child in node.children.iter().rev() {
                    stack.push((child, Some(idx)));
                }
            }
        }
    }

    let visited_count = steps.len();
    AlgorithmResult {
        algorithm: kind,
        matched_indices,
        visited_count,
        steps,
        duration_ms: 1.23,
        top_n,
    }
}

pub async fn temp_scrape_url(_url: &str) -> Result<String, String> {
    gloo_timers::future::TimeoutFuture::new(600).await;
    Ok(r#"<!DOCTYPE html>
<html lang="en">
  <head><title>cRUSTacean</title></head>
  <body>
    <div id="app" class="container">
      <h1 class="title">HTML Tree Search</h1>
      <p class="intro">Explore the <span class="highlight">DOM structure</span></p>
      <ul>
        <li>BFS Search</li>
        <li>DFS Search</li>
        <li>CSS Selectors</li>
      </ul>
    </div>
    <footer><p>IF2211 — Tugas Besar 2</p></footer>
  </body>
</html>"#
    .to_string())
}
