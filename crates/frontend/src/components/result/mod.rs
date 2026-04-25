use leptos::prelude::*;
use crate::app::{AppContext, AppStage};

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

    view! {
        <div class="card max-w-2xl mx-auto space-y-6">
            <h2 class="accent-title">"Hasil Algoritma"</h2>
            <p class="muted-copy">"nanti dilanjut"</p>

            <div class="flex gap-4">
                <div class="stat-badge">
                    <span class="metric-title">"Jenis Algoritma"</span>
                    <span class="metric-value">{alg_kind}</span>
                    <span class="metric-label">"Traversal"</span>
                </div>
                <div class="stat-badge">
                    <span class="metric-title">"Node Dikunjungi"</span>
                    <span class="metric-value">{visited}</span>
                    <span class="metric-label">"node"</span>
                </div>
                <div class="stat-badge">
                    <span class="metric-title">"Hasil Ditemukan"</span>
                    <span class="metric-value">{result_count}</span>
                    <span class="metric-label">"node"</span>
                </div>
                <div class="stat-badge">
                    <span class="metric-title">"Waktu Komputasi"</span>
                    <span class="metric-value">{duration_ms}</span>
                    <span class="metric-label">"ms"</span>
                </div>
            </div>

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
                        use crate::app::HtmlInputMode;

                        ctx.dom_tree.set(None);
                        ctx.algorithm_result.set(None);
                        ctx.css_selector_text.set(String::new());
                        ctx.html_input_mode.set(HtmlInputMode::RawHtml);
                        ctx.html_input_value.set(String::new());
                        ctx.top_n.set(None);
                        ctx.stage.set(AppStage::HtmlInput);
                    }
                >
                    "HTML Baru"
                </button>
            </div>
        </div>
    }
}
