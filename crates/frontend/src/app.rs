use leptos::prelude::*;
use shared::{AlgorithmKind, AlgorithmResult, DomTree};

use crate::components::html_input::HtmlInputStage;
use crate::components::tree_selector::TreeSelectorStage;
use crate::components::result::ResultsStage;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HtmlInputMode {
    RawHtml,
    Url,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AppStage {
    HtmlInput,
    TreeSelector,
    Results,
}

#[derive(Clone, Copy, Debug)]
pub struct AppContext {
    pub stage: RwSignal<AppStage>,
    pub html_input_mode: RwSignal<HtmlInputMode>,
    pub html_input_value: RwSignal<String>,
    pub dom_tree: RwSignal<Option<DomTree>>,
    pub css_selector_text: RwSignal<String>,
    pub algorithm_kind: RwSignal<AlgorithmKind>,
    pub top_n: RwSignal<Option<usize>>,
    pub algorithm_result: RwSignal<Option<AlgorithmResult>>,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            stage:             RwSignal::new(AppStage::HtmlInput),
            html_input_mode:   RwSignal::new(HtmlInputMode::Url),
            html_input_value:  RwSignal::new(String::new()),
            dom_tree:          RwSignal::new(None),
            css_selector_text: RwSignal::new(String::new()),
            algorithm_kind:    RwSignal::new(AlgorithmKind::Bfs),
            top_n:             RwSignal::new(None),
            algorithm_result:  RwSignal::new(None),
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let ctx = AppContext::new();
    provide_context(ctx);

    view! {
        <div class="min-h-screen text-slate-900 font-sans">
            <header class="border-b border-brand-sky/30 bg-white/65 backdrop-blur
                           sticky top-0 z-50 px-6 py-3 flex items-center gap-4">
                <span class="text-xl font-bold tracking-tight">
                    <span class="text-slate-900">"c"</span>
                    <span class="text-brand-lobster">"RUST"</span>
                    <span class="text-slate-900">"acean"</span>
                </span>
                <span class="text-slate-600 text-sm hidden sm:block">
                    "HTML Tree Search Visualizer"
                </span>
                <div class="ml-auto flex items-center gap-2 text-xs text-slate-600">
                    <StepBadge label="1. HTML Input"    active=move || ctx.stage.get() == AppStage::HtmlInput />
                    <span class="text-brand-sky/40">">"</span>
                    <StepBadge label="2. CSS Selector"   active=move || ctx.stage.get() == AppStage::TreeSelector />
                    <span class="text-brand-sky/40">">"</span>
                    <StepBadge label="3. Hasil Traversal" active=move || ctx.stage.get() == AppStage::Results />
                </div>
            </header>

            <main class="container mx-auto px-4 py-8 max-w-7xl">
                {move || match ctx.stage.get() {
                    AppStage::HtmlInput    => view! { <HtmlInputStage /> }.into_any(),
                    AppStage::TreeSelector => view! { <TreeSelectorStage /> }.into_any(),
                    AppStage::Results      => view! { <ResultsStage /> }.into_any(),
                }}
            </main>
        </div>
    }
}

#[component]
fn StepBadge(label: &'static str, active: impl Fn() -> bool + Send + 'static) -> impl IntoView {
    view! {
        <span class=move || if active() {
            "px-2 py-0.5 rounded bg-brand-teal text-white font-medium shadow-md shadow-brand-teal/25"
        } else {
            "px-2 py-0.5 rounded text-slate-600"
        }>
            {label}
        </span>
    }
}
