mod depth_control;
mod lca_controls;
mod traversal_controls;
mod tree_view;
mod validation;

use leptos::prelude::*;
use leptos::task::spawn_local;
use shared::{LcaRequest, TraverseRequest};

use crate::api::{run_lca, run_traverse};
use crate::app::{AppContext, AppStage, RunMode};

use self::depth_control::DepthControl;
use self::lca_controls::{LcaInputTarget, parse_lca_index, LcaControls};
use self::traversal_controls::TraversalControls;
use self::tree_view::SvgTreeView;

#[component]
pub fn TreeSelectorStage() -> impl IntoView {
    let ctx = use_context::<AppContext>().expect("AppContext not provided");
    let tree = ctx.dom_tree.get();
    let local_error = RwSignal::new(None::<String>);
    let is_running = RwSignal::new(false);
    let lca_input_target = RwSignal::new(LcaInputTarget::NodeA);

    let Some(tree_data) = tree else {
        return view! {
            <div class="card max-w-3xl mx-auto space-y-4">
                <h2 class="accent-title">"CSS Selector"</h2>
                <p class="muted-copy">
                    "Tidak ada pohon DOM yang dimuat. Validasi HTML sebelum membuka layar ini."
                </p>
                <div class="flex gap-3">
                    <button class="btn-secondary" on:click=move |_| ctx.stage.set(AppStage::HtmlInput) type="button">
                        "Kembali"
                    </button>
                </div>
            </div>
        }
        .into_any();
    };

    let max_depth_value = tree_data.max_depth();
    let default_depth = if max_depth_value == 0 { 0 } else { usize::min(3, max_depth_value) };
    let render_depth = RwSignal::new(default_depth);
    let top_n_enabled = RwSignal::new(ctx.top_n.get().is_some());
    let top_n_text = RwSignal::new(ctx.top_n.get().unwrap_or(3).to_string());

    let pick_node = Callback::new(move |node_index: usize| {
        if ctx.run_mode.get() != RunMode::Lca {
            return;
        }

        match lca_input_target.get() {
            LcaInputTarget::NodeA => {
                ctx.lca_node_a_text.set(node_index.to_string());
                lca_input_target.set(LcaInputTarget::NodeB);
            }
            LcaInputTarget::NodeB => {
                ctx.lca_node_b_text.set(node_index.to_string());
                lca_input_target.set(LcaInputTarget::NodeA);
            }
        }
    });

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

    let selector_error = Memo::new(move |_| {
        validation::selector_error(ctx.run_mode.get(), &ctx.css_selector_text.get())
    });

    let top_n_error = Memo::new(move |_| {
        validation::top_n_error(ctx.run_mode.get(), top_n_enabled.get(), &top_n_text.get())
    });

    let lca_error = Memo::new(move |_| {
        validation::lca_error(
            ctx.run_mode.get(),
            node_count(),
            &ctx.lca_node_a_text.get(),
            &ctx.lca_node_b_text.get(),
        )
    });

    let can_run = move || {
        if is_running.get() {
            return false;
        }

        match ctx.run_mode.get() {
            RunMode::Traversal => selector_error.get().is_none() && top_n_error.get().is_none(),
            RunMode::Lca => lca_error.get().is_none(),
        }
    };

    view! {
        <section class="panel-grid items-start">
            <div class="space-y-6">
                <div class="card space-y-4">
                    <div>
                        <h2 class="accent-title">"Visualisasi Pohon DOM"</h2>
                        <p class="muted-copy mt-2">
                            "Berikut Hasil parsing HTML dan statistik pohon yang didapatkan. Kurangi kedalaman tampilan jika pohon terlalu besar untuk divisualisasikan."
                        </p>
                    </div>

                    <div class="grid gap-3 sm:grid-cols-3">
                        <div class="stat-badge items-start text-left">
                            <span class="metric-title">"Jumlah Node"</span>
                            <span class="metric-value">{node_count}</span>
                            <span class="metric-label">"total"</span>
                        </div>
                        <div class="stat-badge items-start text-left">
                            <span class="metric-title">"Jumlah Edge"</span>
                            <span class="metric-value">{edge_count}</span>
                            <span class="metric-label">"total"</span>
                        </div>
                        <div class="stat-badge items-start text-left">
                            <span class="metric-title">"Kedalaman Pohon"</span>
                            <span class="metric-value">{max_depth}</span>
                            <span class="metric-label">"maksimum"</span>
                        </div>
                    </div>
                </div>

                <SvgTreeView tree=tree_data.clone() render_depth=render_depth.get() on_pick=Some(pick_node) />
            </div>

            <div class="card space-y-5">
                <div>
                    <h2 class="accent-title">"CSS Selector"</h2>
                    <p class="muted-copy mt-2">
                        "Masukkan selector, jenis algoritma, and batasan hasil sebelum melanjutkan."
                    </p>
                </div>

                <div>
                    <label class="field-label">"Mode Run"</label>
                    <div class="toggle-row">
                        <button
                            class=move || if ctx.run_mode.get() == RunMode::Traversal {
                                "toggle-chip toggle-chip-active"
                            } else {
                                "toggle-chip"
                            }
                            on:click=move |_| {
                                ctx.run_mode.set(RunMode::Traversal);
                                local_error.set(None);
                            }
                            type="button"
                        >
                            "Traversal"
                        </button>
                        <button
                            class=move || if ctx.run_mode.get() == RunMode::Lca {
                                "toggle-chip toggle-chip-active"
                            } else {
                                "toggle-chip"
                            }
                            on:click=move |_| {
                                ctx.run_mode.set(RunMode::Lca);
                                local_error.set(None);
                            }
                            type="button"
                        >
                            "LCA"
                        </button>
                    </div>
                </div>

                <DepthControl render_depth=render_depth max_depth=max_depth_value />

                {move || if ctx.run_mode.get() == RunMode::Traversal {
                    view! {<TraversalControls
                        top_n_enabled=top_n_enabled
                        top_n_text=top_n_text
                        selector_error=selector_error
                        top_n_error=top_n_error
                    />}
                    .into_any()
                } else {
                    view! {<LcaControls lca_input_target=lca_input_target lca_error=lca_error />}
                    .into_any()
                }}

                {move || local_error.get().map(|message| view! {
                    <div class="notice-banner error">{message}</div>
                })}

                {move || if is_running.get() {
                    view! {
                        <div class="notice-banner info">"Menjalankan komputasi..."</div>
                    }
                    .into_any()
                } else {
                    ().into_any()
                }}

                <div class="flex flex-wrap gap-3 pt-2">
                    <button
                        class="btn-secondary"
                        on:click=move |_| ctx.stage.set(AppStage::HtmlInput)
                        type="button"
                    >
                        "Kembali"
                    </button>
                    <button
                        class="btn-primary"
                        disabled=move || !can_run()
                        on:click=move |_| {
                            local_error.set(None);
                            is_running.set(true);

                            let tree = match ctx.dom_tree.get() {
                                Some(tree) => tree,
                                None => {
                                    local_error.set(Some("Tidak ada pohon DOM yang didapatkn.".to_string()));
                                    is_running.set(false);
                                    return;
                                }
                            };

                            match ctx.run_mode.get() {
                                RunMode::Traversal => {
                                    let top_n = if top_n_enabled.get() {
                                        match top_n_text.get().trim().parse::<usize>() {
                                            Ok(value) if value > 0 => Some(value),
                                            _ => {
                                                local_error.set(Some("Top N harus berupa bilangan bulat.".to_string()));
                                                is_running.set(false);
                                                return;
                                            }
                                        }
                                    } else {
                                        None
                                    };

                                    let request = TraverseRequest {
                                        tree,
                                        selector: ctx.css_selector_text.get(),
                                        algorithm: ctx.algorithm_kind.get(),
                                        top_n,
                                        parallel: ctx.parallel_mode.get(),
                                    };

                                    let ctx = ctx;
                                    let local_error = local_error;
                                    let is_running = is_running;
                                    spawn_local(async move {
                                        match run_traverse(&request).await {
                                            Ok(response) => {
                                                if let Some(result) = response.result {
                                                    ctx.top_n.set(top_n);
                                                    ctx.algorithm_result.set(Some(result));
                                                    ctx.lca_result.set(None);
                                                    ctx.backend_message.set(response.message.clone());
                                                    ctx.stage.set(AppStage::Results);
                                                } else {
                                                    let message = response
                                                        .message
                                                        .unwrap_or_else(|| "Tidak ada hasil yang didapatkan.".to_string());
                                                    local_error.set(Some(message));
                                                }
                                            }
                                            Err(message) => local_error.set(Some(message)),
                                        }

                                        is_running.set(false);
                                    });
                                }
                                RunMode::Lca => {
                                    let Some(node_a) = parse_lca_index(&ctx.lca_node_a_text.get()) else {
                                        local_error.set(Some("Node A harus berupa indeks bilangan bulat valid.".to_string()));
                                        is_running.set(false);
                                        return;
                                    };
                                    let Some(node_b) = parse_lca_index(&ctx.lca_node_b_text.get()) else {
                                        local_error.set(Some("Node B harus berupa indeks bilangan bulat valid.".to_string()));
                                        is_running.set(false);
                                        return;
                                    };

                                    let request = LcaRequest {
                                        tree,
                                        node_a,
                                        node_b,
                                    };

                                    let ctx = ctx;
                                    let local_error = local_error;
                                    let is_running = is_running;
                                    spawn_local(async move {
                                        match run_lca(&request).await {
                                            Ok(response) => {
                                                ctx.lca_result.set(Some(response.clone()));
                                                ctx.algorithm_result.set(None);
                                                ctx.backend_message.set(response.message.clone());
                                                ctx.stage.set(AppStage::Results);
                                            }
                                            Err(message) => local_error.set(Some(message)),
                                        }

                                        is_running.set(false);
                                    });
                                }
                            }
                        }
                        type="button"
                    >
                        {move || if is_running.get() {
                            "Menunggu Hasil..."
                        } else if ctx.run_mode.get() == RunMode::Traversal {
                            "Jalankan Algoritma Traversal"
                        } else {
                            "Jalankan Algoritma LCA"
                        }}
                    </button>
                </div>
            </div>
        </section>
    }
    .into_any()
}
