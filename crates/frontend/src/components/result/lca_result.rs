use leptos::prelude::*;
use shared::{DomTree, LcaResponse};

use crate::components::tree_selector::tree_view::SvgTreeView;

#[component]
pub(crate) fn LcaResultPanel(
    lca_result: Option<LcaResponse>,
    node_a: String,
    node_b: String,
    tree: Option<DomTree>,
) -> impl IntoView {
    let node_a_idx = node_a.trim().parse::<usize>().ok();
    let node_b_idx = node_b.trim().parse::<usize>().ok();

    let (lca_index, lca_value, status) = match &lca_result {
        Some(result) if result.found => (
            result.lca_index,
            result
                .lca_index
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
            "Lowest Common Ancestor ditemukan".to_string(),
        ),
        Some(result) => (
            None,
            "-".to_string(),
            result
                .message
                .clone()
                .unwrap_or_else(|| "Lowest Common Ancestor tidak ditemukan".to_string()),
        ),
        None => (None, "-".to_string(), "Tidak ada hasil LCA".to_string()),
    };

    // Build highlight sets:
    // - visited_nodes: node A and node B (yellow)
    // - active_node: LCA result (orange)
    let visited: Vec<usize> = [node_a_idx, node_b_idx]
        .into_iter()
        .flatten()
        .collect();
    let active = lca_index;

    let max_depth = tree.as_ref().map(|t| t.max_depth()).unwrap_or(0);
    let render_depth = RwSignal::new(if max_depth == 0 { 0 } else { usize::min(3, max_depth) });

    view! {
        <div class="space-y-4">
            <p class="muted-copy">
                "Didapatkan Lowest Common Ancestor (LCA) antara Node A dan Node B sebagai berikut."
            </p>

            <div class="flex flex-wrap gap-4">
                <div class="stat-badge">
                    <span class="metric-title">"Node A"</span>
                    <span class="metric-value">{node_a}</span>
                    <span class="metric-label">"input (kuning)"</span>
                </div>
                <div class="stat-badge">
                    <span class="metric-title">"Node B"</span>
                    <span class="metric-value">{node_b}</span>
                    <span class="metric-label">"input (kuning)"</span>
                </div>
                <div class="stat-badge">
                    <span class="metric-title">"LCA"</span>
                    <span class="metric-value">{lca_value}</span>
                    <span class="metric-label">"leluhur terdekat (yang jingga yak)"</span>
                </div>
            </div>

            <div class="notice-banner info">{status}</div>

            {match tree {
                Some(tree) => view! {
                    <div class="space-y-3">
                        <div class="flex items-center gap-3 flex-wrap">
                            <h3 class="accent-title text-lg">"Visualisasi Pohon DOM"</h3>
                            <div class="flex items-center gap-2">
                                <label class="field-label mb-0" for="lca-depth">"Kedalaman View"</label>
                                <select
                                    id="lca-depth"
                                    class="input-field py-1 w-28"
                                    on:change=move |ev| {
                                        if let Ok(value) = event_target_value(&ev).parse::<usize>() {
                                            render_depth.set(value);
                                        }
                                    }
                                    prop:value=move || render_depth.get().to_string()
                                >
                                    {(if max_depth == 0 { vec![0] } else { (1..=max_depth).collect::<Vec<_>>() })
                                        .into_iter()
                                        .map(|d| view! { <option value={d.to_string()}>{d.to_string()}</option> })
                                        .collect_view()}
                                </select>
                            </div>
                        </div>
                        {move || view! {
                            <SvgTreeView
                                tree=tree.clone()
                                render_depth=render_depth.get()
                                on_pick=None
                                visited_nodes=Some(visited.clone())
                                matched_nodes=None
                                active_node=active
                                highlighted_edges=None
                            />
                        }}
                    </div>
                }
                    .into_any(),
                None => ().into_any(),
            }}
        </div>
    }
}
