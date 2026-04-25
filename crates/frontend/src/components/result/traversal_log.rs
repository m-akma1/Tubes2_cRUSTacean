use leptos::prelude::*;
use shared::{AlgorithmResult, NodeData};

fn node_label(data: &NodeData) -> String {
    match data {
        NodeData::Document => "document".to_string(),
        NodeData::Element { tag_name, .. } => tag_name.clone(),
        NodeData::Text(_) => "text".to_string(),
        NodeData::Comment(_) => "comment".to_string(),
    }
}

#[component]
pub(crate) fn TraversalLog(
    result: AlgorithmResult,
    tree: shared::DomTree,
    current_step: RwSignal<usize>,
) -> impl IntoView {
    let steps = result.steps.clone();

    view! {
        <div class="card space-y-3">
            <div class="flex items-center justify-between">
                <h3 class="accent-title text-lg">"Log Urutan Penelusuran"</h3>
                <span class="metric-label">"Step-by-step visit order"</span>
            </div>
            <div class="log-scroll">
                <table class="log-table">
                    <thead>
                        <tr>
                            <th>"Step"</th>
                            <th>"Node"</th>
                            <th>"From"</th>
                            <th>"Tag"</th>
                            <th>"Match"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {steps
                            .into_iter()
                            .map(|step| {
                                let step_no = step.step;
                                let node_idx = step.node_index;
                                let is_match = step.is_match;
                                let is_current = move || step_no == current_step.get();
                                let tag = tree
                                    .nodes
                                    .get(node_idx)
                                    .map(|node| node_label(&node.data))
                                    .unwrap_or_else(|| "unknown".to_string());
                                let from = step
                                    .from_index
                                    .map(|i| i.to_string())
                                    .unwrap_or_else(|| "-".to_string());

                                view! {
                                    <tr class=move || if is_current() { "log-row current" } else { "log-row" }>
                                        <td>{step_no}</td>
                                        <td>{node_idx}</td>
                                        <td>{from}</td>
                                        <td>{tag}</td>
                                        <td>{if is_match { "yes" } else { "no" }}</td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
