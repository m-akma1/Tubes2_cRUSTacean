use leptos::prelude::*;

use crate::app::AppContext;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum LcaInputTarget {
    NodeA,
    NodeB,
}

pub(crate) fn parse_lca_index(text: &str) -> Option<usize> {
    text.trim().parse::<usize>().ok()
}


#[component]
pub(crate) fn LcaControls(
    lca_input_target: RwSignal<LcaInputTarget>,
    lca_error: Memo<Option<String>>,
) -> impl IntoView {
    let ctx = use_context::<AppContext>().expect("AppContext not provided");

    view! {
        <div class="space-y-5">
            <div>
                <label class="field-label">"LCA Input"</label>
                <p class="field-hint">
                    "Masukkan indeks secara manual atau klik node yang dipilih pada tampila pohon."
                </p>
            </div>

            <div class="toggle-row">
                <button
                    class=move || if lca_input_target.get() == LcaInputTarget::NodeA {
                        "toggle-chip toggle-chip-active"
                    } else {
                        "toggle-chip"
                    }
                    on:click=move |_| lca_input_target.set(LcaInputTarget::NodeA)
                    type="button"
                >
                    "Pilih Node A"
                </button>
                <button
                    class=move || if lca_input_target.get() == LcaInputTarget::NodeB {
                        "toggle-chip toggle-chip-active"
                    } else {
                        "toggle-chip"
                    }
                    on:click=move |_| lca_input_target.set(LcaInputTarget::NodeB)
                    type="button"
                >
                    "Pilih Node B"
                </button>
            </div>

            <div class="grid gap-3 sm:grid-cols-2">
                <div>
                    <label class="field-label" for="lca-node-a">"Node A Index"</label>
                    <input
                        id="lca-node-a"
                        class="input-field"
                        type="number"
                        min="0"
                        placeholder="0"
                        prop:value=move || ctx.lca_node_a_text.get()
                        on:input=move |ev| ctx.lca_node_a_text.set(event_target_value(&ev))
                    />
                </div>
                <div>
                    <label class="field-label" for="lca-node-b">"Node B Index"</label>
                    <input
                        id="lca-node-b"
                        class="input-field"
                        type="number"
                        min="0"
                        placeholder="0"
                        prop:value=move || ctx.lca_node_b_text.get()
                        on:input=move |ev| ctx.lca_node_b_text.set(event_target_value(&ev))
                    />
                </div>
            </div>

            <div class="notice-banner info">
                {move || format!(
                    "Pilihan saat ini: {}",
                    if lca_input_target.get() == LcaInputTarget::NodeA {
                        "Node A"
                    } else {
                        "Node B"
                    }
                )}
            </div>

            {move || lca_error.get().map(|message| view! {
                <div class="notice-banner error">{message}</div>
            })}
        </div>
    }
}
