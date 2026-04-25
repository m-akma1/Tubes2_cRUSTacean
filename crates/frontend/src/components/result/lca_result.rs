use leptos::prelude::*;
use shared::LcaResponse;

#[component]
pub(crate) fn LcaResultPanel(
    lca_result: Option<LcaResponse>,
    node_a: String,
    node_b: String,
) -> impl IntoView {
    let (lca_value, status) = match lca_result {
        Some(result) if result.found => (
            result
                .lca_index
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
            "Lowest Common Ancestor ditemukan".to_string(),
        ),
        Some(result) => (
            "-".to_string(),
            result
                .message
                .unwrap_or_else(|| "Lowest Common Ancestor tidak ditemukan".to_string()),
        ),
        None => ("-".to_string(), "Tidak ada hasil LCA".to_string()),
    };

    view! {
        <div class="space-y-4">
            <p class="muted-copy">"Didapatkan Lowest Common Ancestor (LCA) antara Node A dan Node B sebagai berikut."</p>

            <div class="flex flex-wrap gap-4">
                <div class="stat-badge">
                    <span class="metric-title">"Node A"</span>
                    <span class="metric-value">{node_a}</span>
                    <span class="metric-label">"input"</span>
                </div>
                <div class="stat-badge">
                    <span class="metric-title">"Node B"</span>
                    <span class="metric-value">{node_b}</span>
                    <span class="metric-label">"input"</span>
                </div>
                <div class="stat-badge">
                    <span class="metric-title">"LCA"</span>
                    <span class="metric-value">{lca_value}</span>
                    <span class="metric-label">"Leluhur Terdekat"</span>
                </div>
            </div>

            <div class="notice-banner info">{status}</div>
        </div>
    }
}
