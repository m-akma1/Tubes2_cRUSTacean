mod depth_control;
mod tree_view;

use css_selector::parse;
use leptos::prelude::*;
use shared::AlgorithmKind;

use crate::app::{AppContext, AppStage};
use crate::temp::temp_algoritm;

use self::depth_control::DepthControl;
use self::tree_view::SvgTreeView;

#[component]
pub fn TreeSelectorStage() -> impl IntoView {
    let ctx = use_context::<AppContext>().expect("AppContext not provided");
    let tree = ctx.dom_tree.get();

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

    let selector_error = move || {
        let value = ctx.css_selector_text.get();
        let trimmed = value.trim();

        if trimmed.is_empty() {
            Some("Masukkan CSS selector sebelum menjalankan algoritma.".to_string())
        } else {
            parse(trimmed).err().map(|error| error.to_string())
        }
    };

    let top_n_error = move || {
        if !top_n_enabled.get() {
            return None;
        }

        match top_n_text.get().trim().parse::<usize>() {
            Ok(value) if value > 0 => None,
            _ => Some("Top N harus berupa bilangan bulat positif.".to_string()),
        }
    };

    let can_run = move || selector_error().is_none() && top_n_error().is_none();

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

                <SvgTreeView tree=tree_data.clone() render_depth=render_depth.get() />
            </div>

            <div class="card space-y-5">
                <div>
                    <h2 class="accent-title">"CSS Selector"</h2>
                    <p class="muted-copy mt-2">
                        "Masukkan selector, jenis algoritma, and batasan hasil sebelum melanjutkan."
                    </p>
                </div>

                <DepthControl render_depth=render_depth max_depth=max_depth_value />

                <div>
                    <label class="field-label" for="css-selector-input">"CSS Selector"</label>
                    <input
                        id="css-selector-input"
                        class=move || if selector_error().is_some() {
                            "input-field error"
                        } else {
                            "input-field"
                        }
                        type="text"
                        placeholder="div.container span.highlight"
                        prop:value=move || ctx.css_selector_text.get()
                        on:input=move |ev| ctx.css_selector_text.set(event_target_value(&ev))
                    />
                    <p class="field-hint">
                        "Notasi yang didukung berupa tag, .class, #id, descendant, child (>), adjacent (+), dan sibling (~)."
                    </p>
                </div>

                {move || selector_error().map(|message| view! {
                    <div class="notice-banner error">{message}</div>
                })}

                <div>
                    <label class="field-label">"Jenis Algoritma"</label>
                    <div class="toggle-row">
                        <button
                            class=move || if ctx.algorithm_kind.get() == AlgorithmKind::Bfs {
                                "toggle-chip toggle-chip-active"
                            } else {
                                "toggle-chip"
                            }
                            on:click=move |_| ctx.algorithm_kind.set(AlgorithmKind::Bfs)
                            type="button"
                        >
                            "BFS"
                        </button>
                        <button
                            class=move || if ctx.algorithm_kind.get() == AlgorithmKind::Dfs {
                                "toggle-chip toggle-chip-active"
                            } else {
                                "toggle-chip"
                            }
                            on:click=move |_| ctx.algorithm_kind.set(AlgorithmKind::Dfs)
                            type="button"
                        >
                            "DFS"
                        </button>
                    </div>
                </div>

                <div class="space-y-3">
                    <label class="field-label">"Batasan Hasil"</label>
                    <div class="toggle-row">
                        <button
                            class=move || if !top_n_enabled.get() {
                                "toggle-chip toggle-chip-active"
                            } else {
                                "toggle-chip"
                            }
                            on:click=move |_| {
                                top_n_enabled.set(false);
                                ctx.top_n.set(None);
                            }
                            type="button"
                        >
                            "Semua"
                        </button>
                        <button
                            class=move || if top_n_enabled.get() {
                                "toggle-chip toggle-chip-active"
                            } else {
                                "toggle-chip"
                            }
                            on:click=move |_| {
                                top_n_enabled.set(true);
                                if let Ok(value) = top_n_text.get().trim().parse::<usize>() {
                                    if value > 0 {
                                        ctx.top_n.set(Some(value));
                                    }
                                }
                            }
                            type="button"
                        >
                            "Top N"
                        </button>
                    </div>

                    {move || if top_n_enabled.get() {
                        view! {
                            <input
                                class=move || if top_n_error().is_some() {
                                    "input-field error"
                                } else {
                                    "input-field"
                                }
                                type="number"
                                min="1"
                                placeholder="3"
                                prop:value=move || top_n_text.get()
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    top_n_text.set(value.clone());
                                    if let Ok(parsed) = value.trim().parse::<usize>() {
                                        if parsed > 0 {
                                            ctx.top_n.set(Some(parsed));
                                        }
                                    }
                                }
                            />
                        }
                        .into_any()
                    } else {
                        ().into_any()
                    }}
                </div>

                {move || top_n_error().map(|message| view! {
                    <div class="notice-banner error">{message}</div>
                })}

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
                            let top_n = if top_n_enabled.get() {
                                match top_n_text.get().trim().parse::<usize>() {
                                    Ok(value) if value > 0 => Some(value),
                                    _ => return,
                                }
                            } else {
                                None
                            };

                            ctx.top_n.set(top_n);

                            if let Some(tree) = ctx.dom_tree.get() {
                                let result = temp_algoritm(&tree, ctx.algorithm_kind.get(), top_n);
                                ctx.algorithm_result.set(Some(result));
                                ctx.stage.set(AppStage::Results);
                            }
                        }
                        type="button"
                    >
                        "Jalankan Algoritma"
                    </button>
                </div>
            </div>
        </section>
    }
    .into_any()
}
