use std::collections::HashMap;
use std::collections::HashSet;

use leptos::prelude::*;
use shared::{DomTree, NodeData};

const H_GAP: f32 = 164.0;
const V_GAP: f32 = 112.0;
const PADDING_X: f32 = 84.0;
const PADDING_Y: f32 = 68.0;

#[derive(Clone, Debug)]
struct RawNodeLayout {
    index: usize,
    parent: Option<usize>,
    depth: usize,
    x_units: f32,
}

#[derive(Clone, Debug)]
struct PositionedNode {
    index: usize,
    parent: Option<usize>,
    depth: usize,
    x: f32,
    y: f32,
}

fn push_layout(
    tree: &DomTree,
    node_index: usize,
    render_depth: usize,
    cursor: &mut f32,
    nodes: &mut Vec<RawNodeLayout>,
) -> f32 {
    let node = &tree.nodes[node_index];
    let child_indices = node
        .children
        .iter()
        .copied()
        .filter(|child| tree.nodes[*child].depth <= render_depth)
        .collect::<Vec<_>>();

    let x_units = if child_indices.is_empty() {
        let current = *cursor;
        *cursor += 1.0;
        current
    } else {
        let mut child_positions = Vec::with_capacity(child_indices.len());
        for child in child_indices {
            child_positions.push(push_layout(tree, child, render_depth, cursor, nodes));
        }

        let first = *child_positions.first().unwrap_or(cursor);
        let last = *child_positions.last().unwrap_or(cursor);
        (first + last) / 2.0
    };

    nodes.push(RawNodeLayout {
        index: node_index,
        parent: node.parent,
        depth: node.depth,
        x_units,
    });

    x_units
}

fn build_layout(tree: &DomTree, render_depth: usize) -> Option<(Vec<PositionedNode>, f32, f32)> {
    let root = tree.root?;
    let mut cursor = 0.0;
    let mut raw_nodes = Vec::new();
    push_layout(tree, root, render_depth, &mut cursor, &mut raw_nodes);

    raw_nodes.sort_by_key(|node| (node.depth, node.index));

    let positioned = raw_nodes
        .into_iter()
        .map(|node| PositionedNode {
            index: node.index,
            parent: node.parent,
            depth: node.depth,
            x: PADDING_X + node.x_units * H_GAP,
            y: PADDING_Y + node.depth as f32 * V_GAP,
        })
        .collect::<Vec<_>>();

    let max_depth = positioned.iter().map(|node| node.depth).max().unwrap_or(0);
    let width = PADDING_X * 2.0 + ((cursor.max(1.0) - 1.0) * H_GAP) + 140.0;
    let height = PADDING_Y * 2.0 + (max_depth as f32 * V_GAP) + 96.0;

    Some((positioned, width, height))
}

fn trim_label(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let trimmed = chars.by_ref().take(max_chars).collect::<String>();
    if chars.next().is_some() {
        format!("{}...", trimmed)
    } else {
        trimmed
    }
}

fn node_title(data: &NodeData) -> String {
    match data {
        NodeData::Document => "DOC".to_string(),
        NodeData::Element { tag_name, .. } => tag_name.clone(),
        NodeData::Text(value) => trim_label(value.trim(), 18),
        NodeData::Comment(value) => trim_label(value.trim(), 18),
    }
}

fn node_subtitle(data: &NodeData) -> String {
    match data {
        NodeData::Document => "root".to_string(),
        NodeData::Element { id, classes, .. } => {
            let mut parts = Vec::new();
            if let Some(id) = id {
                parts.push(format!("#{}", id));
            }
            if !classes.is_empty() {
                parts.push(format!(".{}", classes.join(".")));
            }

            if parts.is_empty() {
                "element".to_string()
            } else {
                trim_label(&parts.join(" "), 22)
            }
        }
        NodeData::Text(_) => "text node".to_string(),
        NodeData::Comment(_) => "comment".to_string(),
    }
}

fn element_fill(tag_name: &str) -> &'static str {
    match tag_name {
        "html" | "body" => "#84BCDA",
        "div" | "section" | "article" => "#067BC2",
        "h1" | "h2" | "h3"| "h4" | "h5" | "h6" | "p" | "title" => "#ECC30B",
        "span" | "a" => "#F37748",
        "ul" | "li" | "nav" => "#B8C073",
        _ => "#D56062",
    }
}

fn node_palette(data: &NodeData) -> (&'static str, &'static str, &'static str) {
    match data {
        NodeData::Document => ("#067BC2", "#055A8E", "#FFFFFF"),
        NodeData::Element { tag_name, .. } => {
            let fill = element_fill(tag_name);
            (fill, "#0F172A", "#0F172A")
        }
        NodeData::Text(_) => ("#FFFFFF", "#84BCDA", "#0F172A"),
        NodeData::Comment(_) => ("#D56062", "#0F172A", "#FFFFFF"),
    }
}

#[component]
pub fn SvgTreeView(
    tree: DomTree,
    render_depth: usize,
    on_pick: Option<Callback<usize>>,
    visited_nodes: Option<Vec<usize>>,
    matched_nodes: Option<Vec<usize>>,
    active_node: Option<usize>,
    highlighted_edges: Option<Vec<(usize, usize)>>,
) -> impl IntoView {
    let Some((nodes, width, height)) = build_layout(&tree, render_depth) else {
        return view! {
            <div class="notice-banner info">"No tree available yet."</div>
        }
        .into_any();
    };

    let nodes_by_index = nodes
        .iter()
        .map(|node| (node.index, node.clone()))
        .collect::<HashMap<_, _>>();

    let visited_set = visited_nodes
        .unwrap_or_default()
        .into_iter()
        .collect::<HashSet<_>>();
    let matched_set = matched_nodes
        .unwrap_or_default()
        .into_iter()
        .collect::<HashSet<_>>();
    let edge_set = highlighted_edges
        .unwrap_or_default()
        .into_iter()
        .collect::<HashSet<_>>();

    let visible_nodes = nodes.len();
    let visible_edges = nodes.iter().filter(|node| node.parent.is_some()).count();
    let view_box = format!("0 0 {} {}", width, height);

    view! {
        <div class="tree-shell">
            <div class="tree-scroll">
                <svg class="min-w-full" width={width} height={height} viewBox={view_box}>
                    {nodes
                        .iter()
                        .filter_map(|node| {
                            let parent = node.parent.and_then(|index| nodes_by_index.get(&index)).cloned()?;
                            let is_highlight = edge_set.contains(&(parent.index, node.index))
                                || edge_set.contains(&(node.index, parent.index));
                            Some(view! {
                                <line
                                    x1={parent.x}
                                    y1={parent.y + 22.0}
                                    x2={node.x}
                                    y2={node.y - 22.0}
                                    stroke={if is_highlight { "#F37748" } else { "#84BCDA" }}
                                    stroke-width={if is_highlight { "5" } else { "3" }}
                                    stroke-linecap="round"
                                    opacity={if is_highlight { "1.0" } else { "0.78" }}
                                />
                            })
                        })
                        .collect_view()}

                    {nodes
                        .iter()
                        .map(|layout| {
                            let node = &tree.nodes[layout.index];
                            let node_index = layout.index;
                            let title = node_title(&node.data);
                            let subtitle = node_subtitle(&node.data);
                            let (base_fill, base_stroke, base_text_fill) = node_palette(&node.data);
                            let is_visited = visited_set.contains(&layout.index);
                            let is_matched = matched_set.contains(&layout.index);
                            let is_active = active_node.is_some_and(|index| index == layout.index);

                            let fill = if is_active {
                                "#F37748"
                            } else if is_matched {
                                "#D56062"
                            } else if is_visited {
                                "#ECC30B"
                            } else {
                                base_fill
                            };

                            let stroke = if is_active || is_matched || is_visited {
                                "#0F172A"
                            } else {
                                base_stroke
                            };

                            let text_fill = if is_active || is_matched {
                                "#FFFFFF"
                            } else {
                                base_text_fill
                            };

                            let on_pick = on_pick.clone();

                            let shape = match &node.data {
                                NodeData::Document => view! {
                                    <circle
                                        cx={layout.x}
                                        cy={layout.y}
                                        r="26"
                                        fill={fill}
                                        stroke={stroke}
                                        stroke-width="2.5"
                                    />
                                }
                                    .into_any(),
                                NodeData::Element { .. } => view! {
                                    <rect
                                        x={layout.x - 52.0}
                                        y={layout.y - 24.0}
                                        width="104"
                                        height="48"
                                        rx="16"
                                        fill={fill}
                                        stroke={stroke}
                                        stroke-width="2.5"
                                    />
                                }
                                    .into_any(),
                                NodeData::Text(_) => view! {
                                    <ellipse
                                        cx={layout.x}
                                        cy={layout.y}
                                        rx="58"
                                        ry="24"
                                        fill={fill}
                                        stroke={stroke}
                                        stroke-width="2.5"
                                    />
                                }
                                    .into_any(),
                                NodeData::Comment(_) => view! {
                                    <polygon
                                        points={format!(
                                            "{},{} {},{} {},{} {},{}",
                                            layout.x,
                                            layout.y - 28.0,
                                            layout.x + 52.0,
                                            layout.y,
                                            layout.x,
                                            layout.y + 28.0,
                                            layout.x - 52.0,
                                            layout.y
                                        )}
                                        fill={fill}
                                        stroke={stroke}
                                        stroke-width="2.5"
                                    />
                                }
                                    .into_any(),
                            };

                            view! {
                                <g
                                    class=move || if on_pick.is_some() { "cursor-pointer" } else { "" }
                                    on:click=move |_| {
                                        if let Some(callback) = on_pick {
                                            callback.run(node_index);
                                        }
                                    }
                                >
                                    {shape}
                                    <text
                                        x={layout.x}
                                        y={layout.y - 2.0}
                                        text-anchor="middle"
                                        font-size="14"
                                        font-weight="700"
                                        fill={text_fill}
                                    >
                                        {title}
                                    </text>
                                    <text
                                        x={layout.x}
                                        y={layout.y + 16.0}
                                        text-anchor="middle"
                                        font-size="11"
                                        fill={text_fill}
                                        opacity="0.82"
                                    >
                                        {subtitle}
                                    </text>
                                </g>
                            }
                        })
                        .collect_view()}
                </svg>
            </div>

            <div class="legend-row">
                <span class="legend-chip">
                    <span class="legend-dot" style="background:#067BC2"></span>
                    "Document"
                </span>
                <span class="legend-chip">
                    <span class="legend-dot" style="background:#F37748"></span>
                    "Inline element"
                </span>
                <span class="legend-chip">
                    <span class="legend-dot" style="background:#ECC30B"></span>
                    "Content element"
                </span>
                <span class="legend-chip">
                    <span class="legend-dot" style="background:#FFFFFF;border:1px solid #84BCDA"></span>
                    "Text node"
                </span>
                <span class="legend-chip">
                    <span class="legend-dot" style="background:#ECC30B"></span>
                    "Visited"
                </span>
                <span class="legend-chip">
                    <span class="legend-dot" style="background:#D56062"></span>
                    "Matched"
                </span>
                <span class="legend-chip">
                    <span class="legend-dot" style="background:#F37748"></span>
                    "Current"
                </span>
            </div>

            <div class="tree-meta-grid">
                <div class="stat-badge items-start text-left">
                    <span class="metric-title">"Total Node Terlihat"</span>
                    <span class="metric-value">{visible_nodes}</span>
                    <span class="metric-label">"dari kedalaman saat ini"</span>
                </div>
                <div class="stat-badge items-start text-left">
                    <span class="metric-title">"Total Edge Terlihat"</span>
                    <span class="metric-value">{visible_edges}</span>
                    <span class="metric-label">"dari kedalaman saat ini"</span>
                </div>
                <div class="stat-badge items-start text-left">
                    <span class="metric-title">"Kedalaman Tampilan"</span>
                    <span class="metric-value">{render_depth}</span>
                    <span class="metric-label">"batas saat ini"</span>
                </div>
            </div>
        </div>
    }
    .into_any()
}