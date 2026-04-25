use leptos::prelude::*;
use crate::app::{AppContext, AppStage};

#[component]
pub fn TreeSelectorStage() -> impl IntoView {
    let ctx = use_context::<AppContext>().expect("AppContext not provided");

    let node_count = move || {
        ctx.dom_tree.get()
            .map(|t| t.node_count())
            .unwrap_or(0)
    };
    let max_depth = move || {
        ctx.dom_tree.get()
            .map(|t| t.max_depth())
            .unwrap_or(0)
    };
    let edge_count = move || {
        ctx.dom_tree.get()
            .map(|t| t.nodes.iter().map(|n| n.children.len()).sum::<usize>())
            .unwrap_or(0)
    };

    view! {
        <div class="card max-w-2xl mx-auto space-y-6">
            <h2 class="accent-title">"HTML DOM Tree"</h2>
            <p class="muted-copy">"Test. masih stub yak. Sabar."</p>

            <div class="flex gap-4">
                <div class="stat-badge">
                    <span class="metric-value">{node_count}</span>
                    <span class="metric-label">"Nodes"</span>
                </div>
                <div class="stat-badge">
                    <span class="metric-value">{edge_count}</span>
                    <span class="metric-label">"Edges"</span>
                </div>
                <div class="stat-badge">
                    <span class="metric-value">{max_depth}</span>
                    <span class="metric-label">"Max depth"</span>
                </div>
            </div>

            <div class="flex gap-3 pt-2">
                <button
                    class="btn-secondary"
                    on:click=move |_| ctx.stage.set(AppStage::HtmlInput)
                >
                    "Kembali"
                </button>
                <button
                    class="btn-primary"
                    on:click=move |_| {
                        use crate::temp::temp_algoritm;
                        if let Some(tree) = ctx.dom_tree.get() {
                            let result = temp_algoritm(
                                &tree,
                                ctx.algorithm_kind.get(),
                                ctx.top_n.get(),
                            );
                            ctx.algorithm_result.set(Some(result));
                            ctx.stage.set(AppStage::Results);
                        }
                    }
                >
                    "Jalankan Algoritma"
                </button>
            </div>
        </div>
    }
}
