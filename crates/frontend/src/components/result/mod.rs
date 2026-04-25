use leptos::prelude::*;
use crate::app::{AppContext, AppStage, HtmlInputMode, RunMode};

#[component]
pub fn ResultsStage() -> impl IntoView {
    let ctx = use_context::<AppContext>().expect("AppContext not provided");

    let alg_kind = move || {
        ctx.algorithm_result.get()
            .map(|r| format!("{:?}", r.algorithm))
            .unwrap_or_else(|| "-".to_string())
    };

    let visited = move || {
        ctx.algorithm_result.get()
            .map(|r| r.visited_count.to_string())
            .unwrap_or_else(|| "-".to_string())
    };

    let result_count = move || {
        ctx.algorithm_result.get()
            .map(|r| r.matched_indices.len().to_string())
            .unwrap_or_else(|| "-".to_string())
    };

    let duration_ms = move || {
        ctx.algorithm_result.get()
            .map(|r| format!("{:.2}", r.duration_ms))
            .unwrap_or_else(|| "-".to_string())
    };

    let lca_display = move || {
        match ctx.lca_result.get() {
            Some(result) if result.found => {
                let lca = result
                    .lca_index
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "-".to_string());
                (
                    ctx.lca_node_a_text.get(),
                    ctx.lca_node_b_text.get(),
                    lca,
                    "Lowest Common Ancestor Ditemukan".to_string(),
                )
            }
            Some(result) => (
                ctx.lca_node_a_text.get(),
                ctx.lca_node_b_text.get(),
                "-".to_string(),
                result
                    .message
                    .unwrap_or_else(|| "Lowest Common Ancestor tidak ditemukan".to_string()),
            ),
            None => (
                ctx.lca_node_a_text.get(),
                ctx.lca_node_b_text.get(),
                "-".to_string(),
                "Tidak ada hasil LCA".to_string(),
            ),
        }
    };

    view! {
        <div class="card max-w-2xl mx-auto space-y-6">
            <h2 class="accent-title">"Hasil Algoritma"</h2>

            {move || if ctx.run_mode.get() == RunMode::Traversal {
                view! {
                    <div class="space-y-4">
                        <p class="muted-copy">"Perhitungan dari backend sudah selesai. Statistik komputasi sebagai berikut."</p>

                        <div class="flex flex-wrap gap-4">
                            <div class="stat-badge">
                                <span class="metric-title">"Jenis Algoritma"</span>
                                <span class="metric-value">{alg_kind}</span>
                                <span class="metric-label">"Traversal"</span>
                            </div>
                            <div class="stat-badge">
                                <span class="metric-title">"Node Dikunjungi"</span>
                                <span class="metric-value">{visited}</span>
                                <span class="metric-label">"nodes"</span>
                            </div>
                            <div class="stat-badge">
                                <span class="metric-title">"Node Hasil"</span>
                                <span class="metric-value">{result_count}</span>
                                <span class="metric-label">"nodes"</span>
                            </div>
                            <div class="stat-badge">
                                <span class="metric-title">"Durasi"</span>
                                <span class="metric-value">{duration_ms}</span>
                                <span class="metric-label">"ms"</span>
                            </div>
                        </div>

                        {move || ctx.backend_message.get().map(|message| view! {
                            <div class="notice-banner info">{message}</div>
                        })}
                    </div>
                }
                    .into_any()
            } else {
                view! {
                    <div class="space-y-4">
                        <p class="muted-copy">"Didapatkan Lowest Common Ancestor (LCA) antara Node A dan Node B sebagai berikut."</p>

                        <div class="flex flex-wrap gap-4">
                            <div class="stat-badge">
                                <span class="metric-title">"Node A"</span>
                                <span class="metric-value">{move || lca_display().0}</span>
                                <span class="metric-label">"input"</span>
                            </div>
                            <div class="stat-badge">
                                <span class="metric-title">"Node B"</span>
                                <span class="metric-value">{move || lca_display().1}</span>
                                <span class="metric-label">"input"</span>
                            </div>
                            <div class="stat-badge">
                                <span class="metric-title">"LCA"</span>
                                <span class="metric-value">{move || lca_display().2}</span>
                                <span class="metric-label">"Leluhur Terendah"</span>
                            </div>
                        </div>

                        <div class="notice-banner info">{move || lca_display().3}</div>
                    </div>
                }
                    .into_any()
            }}

            <div class="flex gap-3 pt-2">
                <button
                    class="btn-secondary"
                    on:click=move |_| ctx.stage.set(AppStage::TreeSelector)
                >
                    "Kembali"
                </button>
                <button
                    class="btn-primary"
                    on:click=move |_| {
                        ctx.dom_tree.set(None);
                        ctx.tree_stats.set(None);
                        ctx.algorithm_result.set(None);
                        ctx.lca_result.set(None);
                        ctx.backend_message.set(None);
                        ctx.css_selector_text.set(String::new());
                        ctx.html_input_mode.set(HtmlInputMode::RawHtml);
                        ctx.html_input_value.set(String::new());
                        ctx.run_mode.set(RunMode::Traversal);
                        ctx.parallel_mode.set(false);
                        ctx.top_n.set(None);
                        ctx.lca_node_a_text.set(String::new());
                        ctx.lca_node_b_text.set(String::new());
                        ctx.stage.set(AppStage::HtmlInput);
                    }
                >
                    "Pohon HTML Baru"
                </button>
            </div>
        </div>
    }
}
