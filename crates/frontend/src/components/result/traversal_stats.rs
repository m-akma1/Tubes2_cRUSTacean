use leptos::prelude::*;
use shared::AlgorithmResult;

#[component]
pub(crate) fn TraversalStats(result: AlgorithmResult) -> impl IntoView {
    let algo = format!("{:?}", result.algorithm);
    let visited = result.visited_count;
    let matches = result.matched_indices.len();
    let duration = format!("{:.2}", result.duration_ms);

    view! {
        <div class="flex flex-wrap gap-4">
            <div class="stat-badge">
                <span class="metric-title">"Jenis Algoritma"</span>
                <span class="metric-value">{algo}</span>
                <span class="metric-label">"Traversal"</span>
            </div>
            <div class="stat-badge">
                <span class="metric-title">"Node Dikunjungi"</span>
                <span class="metric-value">{visited}</span>
                <span class="metric-label">"nodes"</span>
            </div>
            <div class="stat-badge">
                <span class="metric-title">"Node Hasil"</span>
                <span class="metric-value">{matches}</span>
                <span class="metric-label">"nodes"</span>
            </div>
            <div class="stat-badge">
                <span class="metric-title">"Durasi"</span>
                <span class="metric-value">{duration}</span>
                <span class="metric-label">"ms"</span>
            </div>
        </div>
    }
}
