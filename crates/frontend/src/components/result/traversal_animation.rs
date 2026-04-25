use std::collections::HashSet;

use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::task::spawn_local;
use shared::{AlgorithmResult, DomTree};

use crate::components::tree_selector::tree_view::SvgTreeView;

fn derive_highlight_state(
    result: &AlgorithmResult,
    current_step: usize,
) -> (Vec<usize>, Vec<usize>, Vec<(usize, usize)>, Option<usize>) {
    if result.steps.is_empty() {
        return (Vec::new(), Vec::new(), Vec::new(), None);
    }

    let capped = usize::min(current_step, result.steps.len() - 1);
    let mut visited = HashSet::new();
    let mut matched = HashSet::new();
    let mut edges = Vec::new();

    for step in result.steps.iter().take(capped + 1) {
        visited.insert(step.node_index);
        if step.is_match {
            matched.insert(step.node_index);
        }
        if let Some(from) = step.from_index {
            edges.push((from, step.node_index));
        }
    }

    (
        visited.into_iter().collect::<Vec<_>>(),
        matched.into_iter().collect::<Vec<_>>(),
        edges,
        result.steps.get(capped).map(|s| s.node_index),
    )
}

#[component]
pub(crate) fn TraversalAnimation(
    result: AlgorithmResult,
    tree: DomTree,
    current_step: RwSignal<usize>,
) -> impl IntoView {
    let speed_ms = RwSignal::new(500_u32);
    let is_playing = RwSignal::new(false);
    let play_token = RwSignal::new(0_u64);
    let max_depth = tree.max_depth();
    let render_depth = RwSignal::new(if max_depth == 0 { 0 } else { usize::min(3, max_depth) });

    let total_steps = result.steps.len();

    let result_for_state = result.clone();
    let step_state = Memo::new(move |_| derive_highlight_state(&result_for_state, current_step.get()));

    let start_playback = {
        let result = result.clone();
        move || {
            if result.steps.is_empty() {
                return;
            }

            is_playing.set(true);
            let token = play_token.get().wrapping_add(1);
            play_token.set(token);
            let current_step = current_step;
            let is_playing = is_playing;
            let play_token = play_token;
            let speed_ms = speed_ms;
            let total_steps = result.steps.len();

            spawn_local(async move {
                loop {
                    if !is_playing.get_untracked() || play_token.get_untracked() != token {
                        break;
                    }

                    let now = current_step.get_untracked();
                    if now + 1 >= total_steps {
                        is_playing.set(false);
                        break;
                    }

                    TimeoutFuture::new(speed_ms.get_untracked()).await;

                    if !is_playing.get_untracked() || play_token.get_untracked() != token {
                        break;
                    }

                    current_step.update(|step| {
                        if *step + 1 < total_steps {
                            *step += 1;
                        }
                    });
                }
            });
        }
    };

    view! {
        <div class="card space-y-4">
            <div class="flex items-center justify-between gap-3 flex-wrap">
                <h3 class="accent-title text-lg">"Animasi Penelusuran"</h3>
                <span class="metric-label">{move || format!("Step {} / {}", current_step.get(), total_steps.saturating_sub(1))}</span>
            </div>

            <div class="grid gap-3 sm:grid-cols-[1fr_auto] items-end">
                <div>
                    <label class="field-label" for="playback-speed">"Kecepatan (ms)"</label>
                    <input
                        id="playback-speed"
                        class="w-full"
                        type="range"
                        min="100"
                        max="2000"
                        step="50"
                        prop:value=move || speed_ms.get().to_string()
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse::<u32>() {
                                speed_ms.set(value);
                            }
                        }
                    />
                    <p class="metric-label mt-1">{move || format!("{} ms", speed_ms.get())}</p>
                </div>

                <div>
                    <label class="field-label" for="result-depth">"Kedalaman View"</label>
                    <select
                        id="result-depth"
                        class="input-field"
                        on:change=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse::<usize>() {
                                render_depth.set(value);
                            }
                        }
                        prop:value=move || render_depth.get().to_string()
                    >
                        {(if max_depth == 0 { vec![0] } else { (1..=max_depth).collect::<Vec<_>>() })
                            .into_iter()
                            .map(|depth| view! { <option value={depth.to_string()}>{depth.to_string()}</option> })
                            .collect_view()}
                    </select>
                </div>
            </div>

            <div class="toggle-row">
                <button
                    class="btn-primary"
                    type="button"
                    on:click=move |_| {
                        if is_playing.get() {
                            is_playing.set(false);
                        } else {
                            start_playback();
                        }
                    }
                >
                    {move || if is_playing.get() { "Pause" } else { "Play" }}
                </button>
                <button
                    class="btn-secondary"
                    type="button"
                    on:click=move |_| {
                        is_playing.set(false);
                        current_step.update(|step| {
                            if *step > 0 {
                                *step -= 1;
                            }
                        });
                    }
                >
                    "Prev"
                </button>
                <button
                    class="btn-secondary"
                    type="button"
                    on:click=move |_| {
                        is_playing.set(false);
                        current_step.update(|step| {
                            if *step + 1 < total_steps {
                                *step += 1;
                            }
                        });
                    }
                >
                    "Next"
                </button>
                <button
                    class="btn-secondary"
                    type="button"
                    on:click=move |_| {
                        is_playing.set(false);
                        current_step.set(0);
                    }
                >
                    "Reset"
                </button>
            </div>

            <SvgTreeView
                tree=tree
                render_depth=render_depth.get()
                on_pick=None
                visited_nodes=Some(step_state.get().0)
                matched_nodes=Some(step_state.get().1)
                active_node=step_state.get().3
                highlighted_edges=Some(step_state.get().2)
            />
        </div>
    }
}
