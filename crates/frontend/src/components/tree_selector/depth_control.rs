use leptos::prelude::*;

#[component]
pub fn DepthControl(render_depth: RwSignal<usize>, max_depth: usize) -> impl IntoView {
    let options = if max_depth == 0 {
        vec![0]
    } else {
        (1..=max_depth).collect::<Vec<_>>()
    };

    view! {
        <div>
            <label class="field-label" for="render-depth">
                "Kedalaman Tampilan Pohon"
            </label>
            <select
                id="render-depth"
                class="input-field"
                on:change=move |ev| {
                    if let Ok(value) = event_target_value(&ev).parse::<usize>() {
                        render_depth.set(value);
                    }
                }
                prop:value=move || render_depth.get().to_string()
            >
                {options
                    .into_iter()
                    .map(|depth| {
                        let label = if depth == max_depth {
                            format!("{} (max)", depth)
                        } else {
                            depth.to_string()
                        };
                        view! {
                            <option value={depth.to_string()}>{label}</option>
                        }
                    })
                    .collect_view()}
            </select>
        </div>
    }
}