use leptos::prelude::*;
use shared::AlgorithmKind;

use crate::app::AppContext;

#[component]
pub(crate) fn TraversalControls(
    top_n_enabled: RwSignal<bool>,
    top_n_text: RwSignal<String>,
    selector_error: Memo<Option<String>>,
    top_n_error: Memo<Option<String>>,
) -> impl IntoView {
    let ctx = use_context::<AppContext>().expect("AppContext not provided");

    view! {
        <div class="space-y-5">
            <div>
                <label class="field-label" for="css-selector-input">"CSS Selector"</label>
                <input
                    id="css-selector-input"
                    class=move || if selector_error.get().is_some() {
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
                    "Notasi yang didukung: tag, .class, #id, descendant, child (>), adjacent (+), sibling (~)."
                </p>
            </div>

            {move || selector_error.get().map(|message| view! {
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

            <div>
                <label class="field-label">"Paralellisme"</label>
                <div class="toggle-row">
                    <button
                        class=move || if !ctx.parallel_mode.get() {
                            "toggle-chip toggle-chip-active"
                        } else {
                            "toggle-chip"
                        }
                        on:click=move |_| ctx.parallel_mode.set(false)
                        type="button"
                    >
                        "Single Thread"
                    </button>
                    <button
                        class=move || if ctx.parallel_mode.get() {
                            "toggle-chip toggle-chip-active"
                        } else {
                            "toggle-chip"
                        }
                        on:click=move |_| ctx.parallel_mode.set(true)
                        type="button"
                    >
                        "Multi Thread"
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
                            class=move || if top_n_error.get().is_some() {
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

            {move || top_n_error.get().map(|message| view! {
                <div class="notice-banner error">{message}</div>
            })}
        </div>
    }
}
