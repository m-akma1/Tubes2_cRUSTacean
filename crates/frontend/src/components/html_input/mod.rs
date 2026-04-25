use leptos::prelude::*;
use crate::app::{AppContext, AppStage};

#[component]
pub fn HtmlInputStage() -> impl IntoView {
    let ctx = use_context::<AppContext>().expect("AppContext not provided");

    view! {
        <div class="card max-w-2xl mx-auto text-center space-y-4">
            <h2 class="accent-title">"HTML Input"</h2>
            <p class="muted-copy">"Masih stub bro. Testing aja"</p>
            <button
                class="btn-primary"
                on:click=move |_| {
                    use crate::temp::temp_html_parser;
                    let tree = temp_html_parser("").expect("template tree");
                    ctx.dom_tree.set(Some(tree));
                    ctx.stage.set(AppStage::TreeSelector);
                }
            >
                "Lanjutkan"
            </button>
        </div>
    }
}
