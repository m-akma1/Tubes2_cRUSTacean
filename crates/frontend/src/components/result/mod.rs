mod lca_result;
mod traversal_animation;
mod traversal_log;
mod traversal_stats;

use leptos::prelude::*;
use crate::app::{AppContext, AppStage, HtmlInputMode, RunMode};

use self::lca_result::LcaResultPanel;
use self::traversal_animation::TraversalAnimation;
use self::traversal_log::TraversalLog;
use self::traversal_stats::TraversalStats;

#[component]
pub fn ResultsStage() -> impl IntoView {
    let ctx = use_context::<AppContext>().expect("AppContext not provided");
    let current_step = RwSignal::new(0_usize);

    view! {
        <div class="card max-w-6xl mx-auto space-y-6">
            <h2 class="accent-title">"Hasil Algoritma"</h2>

            {move || if ctx.run_mode.get() == RunMode::Traversal {
                let result = ctx.algorithm_result.get();
                let tree = ctx.dom_tree.get();

                match (result, tree) {
                    (Some(result), Some(tree)) => view! {
                        <div class="space-y-4">
                            <p class="muted-copy">"Perhitungan backend selesai. Statistik komputasi sebagai berikut."</p>

                            <TraversalStats result=result.clone() />

                            {move || ctx.backend_message.get().map(|message| view! {
                                <div class="notice-banner info">{message}</div>
                            })}

                            <div class="space-y-4">
                                <TraversalAnimation result=result.clone() tree=tree.clone() current_step=current_step />
                                <TraversalLog result=result tree=tree current_step=current_step />
                            </div>
                        </div>
                    }
                        .into_any(),
                    _ => view! {
                        <div class="notice-banner warning">"Hasil algoritma belum tersedia."</div>
                    }
                        .into_any(),
                }
            } else {
                let tree = ctx.dom_tree.get();
                view! {
                    <LcaResultPanel
                        lca_result=ctx.lca_result.get()
                        node_a=ctx.lca_node_a_text.get()
                        node_b=ctx.lca_node_b_text.get()
                        tree=tree
                    />
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
