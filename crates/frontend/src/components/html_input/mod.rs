use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::{parse_html_tree, scrape_tree};
use crate::app::{AppContext, AppStage, HtmlInputMode};

fn looks_like_url(value: &str) -> bool {
    value.starts_with("http://") || value.starts_with("https://")
}

#[component]
pub fn HtmlInputStage() -> impl IntoView {
    let ctx = use_context::<AppContext>().expect("AppContext not provided");
    let error_message = RwSignal::new(None::<String>);
    let is_loading = RwSignal::new(false);

    let input_label = move || match ctx.html_input_mode.get() {
        HtmlInputMode::RawHtml => "Masukkan teks HTML kasar",
        HtmlInputMode::Url => "Masukkan URL valid",
    };

    let input_hint = move || match ctx.html_input_mode.get() {
        HtmlInputMode::RawHtml => {
            "Pastikan struktur HTML valid agar DOM tree dapat dibuat."
        }
        HtmlInputMode::Url => {
            "URL akan diteruskan ke backend untuk di-scrape. Pastikan URL benar dan dapat diakses."
        }
    };

    let can_submit = move || !is_loading.get() && !ctx.html_input_value.get().trim().is_empty();

    view! {
        <section class="panel-grid items-start">
            <div class="card space-y-5">
                <h1 class="title">"Tugas Besar 2"</h1>
                <h2 class="accent-title">"IF2211 Strategi Algoritma"</h2>
                <h3 class="accent-h3">
                    "Pemanfaatan Algoritma BFS dan DFS dalam Mekanisme Penelusuran CSS pada Pohon Document Object Model"
                </h3>
                <p class="muted-copy leading-7">
                    "Dibuat dengan RUST, FULL STACK! Terima kasih kepada WebAssembly. Fronted dibuat menggunakan Leptos via Trunk dengan TaiwindCSS. Backend dibuat native dengan Axum dan Tokio. Web sudah dideploy lewat Microsoft Azure. Luar biasa ngidenya emang :)"
                </p>
                <div class="grid gap-3 sm:grid-cols-3">
                    <div class="stat-badge items-start text-left">
                        <span class="metric-label ">"13524009"</span>
                        <span class="metric-title ">"Mikhael Benrael Tampubolon"</span>
                    </div>
                    <div class="stat-badge items-start text-left">
                        <span class="metric-label ">"13524011"</span>
                        <span class="metric-title ">"Muhammad Iqbal Raihan"</span>
                    </div>
                    <div class="stat-badge items-start text-left">
                        <span class="metric-label ">"13524099"</span>
                        <span class="metric-title ">"Muhammad Akmal"</span>
                    </div>
                </div>
            </div>

            <div class="card space-y-5">
                <h2 class="accent-title">"HTML Input"</h2>
                <p class="muted-copy leading-7">
                    "Masukkan file HTML kasar atau URL untuk di-scrape. Aplikasi akan memproses input dan membangun DOM tree untuk tahap berikutnya."
                </p>
                <div>
                    <label class="field-label">"Jenis Input"</label>
                    <div class="toggle-row">
                        <button
                            class=move || if ctx.html_input_mode.get() == HtmlInputMode::RawHtml {
                                "toggle-chip toggle-chip-active"
                            } else {
                                "toggle-chip"
                            }
                            on:click=move |_| {
                                ctx.html_input_mode.set(HtmlInputMode::RawHtml);
                                error_message.set(None);
                            }
                            type="button"
                        >
                            "Raw HTML"
                        </button>
                        <button
                            class=move || if ctx.html_input_mode.get() == HtmlInputMode::Url {
                                "toggle-chip toggle-chip-active"
                            } else {
                                "toggle-chip"
                            }
                            on:click=move |_| {
                                ctx.html_input_mode.set(HtmlInputMode::Url);
                                error_message.set(None);
                            }
                            type="button"
                        >
                            "URL"
                        </button>
                    </div>
                </div>

                <div>
                    <label class="field-label" for="html-source">{input_label}</label>

                    {move || match ctx.html_input_mode.get() {
                        HtmlInputMode::RawHtml => view! {
                            <textarea
                                id="html-source"
                                class="input-field h-96 resize-y"
                                placeholder="<html>\n  <body>\n    <div class=\"card\">Hello</div>\n  </body>\n</html>"
                                prop:value=move || ctx.html_input_value.get()
                                on:input=move |ev| ctx.html_input_value.set(event_target_value(&ev))
                            ></textarea>
                        }
                            .into_any(),
                        HtmlInputMode::Url => view! {
                            <input
                                id="html-source"
                                class="input-field"
                                type="url"
                                placeholder="https://example.com"
                                prop:value=move || ctx.html_input_value.get()
                                on:input=move |ev| ctx.html_input_value.set(event_target_value(&ev))
                            />
                        }
                            .into_any(),
                    }}

                    <p class="field-hint">{input_hint}</p>
                </div>

                {move || error_message.get().map(|message| view! {
                    <div class="notice-banner error">{message}</div>
                })}

                {move || if is_loading.get() {
                    view! {
                        <div class="notice-banner info">"Memeriksa HTML dan memvalidasi..."</div>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }}

                <div class="flex flex-wrap gap-3 pt-1">
                    <button
                        class="btn-primary"
                        disabled=move || !can_submit()
                        on:click=move |_| {
                            let value = ctx.html_input_value.get();
                            let trimmed = value.trim().to_string();

                            if trimmed.is_empty() {
                                error_message.set(Some("Input tidak bisa kosong.".to_string()));
                                return;
                            }

                            error_message.set(None);

                            match ctx.html_input_mode.get() {
                                HtmlInputMode::RawHtml => {
                                    is_loading.set(true);
                                    let ctx = ctx;
                                    let error_message = error_message;
                                    let is_loading = is_loading;

                                    spawn_local(async move {
                                        match parse_html_tree(&trimmed).await {
                                            Ok(response) => {
                                                ctx.dom_tree.set(Some(response.tree));
                                                ctx.tree_stats.set(Some(response.stats));
                                                ctx.algorithm_result.set(None);
                                                ctx.lca_result.set(None);
                                                ctx.backend_message.set(None);
                                                ctx.stage.set(AppStage::TreeSelector);
                                            }
                                            Err(message) => error_message.set(Some(message)),
                                        }

                                        is_loading.set(false);
                                    });
                                }
                                HtmlInputMode::Url => {
                                    if !looks_like_url(&trimmed) {
                                        error_message.set(Some("URL tidak valid. URL harus diawali http:// or https://".to_string()));
                                        return;
                                    }

                                    is_loading.set(true);
                                    let ctx = ctx;
                                    let error_message = error_message;
                                    let is_loading = is_loading;

                                    spawn_local(async move {
                                        match scrape_tree(&trimmed).await {
                                            Ok(response) => {
                                                ctx.dom_tree.set(Some(response.tree));
                                                ctx.tree_stats.set(Some(response.stats));
                                                ctx.algorithm_result.set(None);
                                                ctx.lca_result.set(None);
                                                ctx.backend_message.set(None);
                                                ctx.stage.set(AppStage::TreeSelector);
                                            }
                                            Err(message) => error_message.set(Some(message)),
                                        }

                                        is_loading.set(false);
                                    });
                                }
                            }
                        }
                        type="button"
                    >
                        {move || if is_loading.get() { "Memvalidasi..." } else { "Lanjutkan" }}
                    </button>
                </div>
            </div>
        </section>
    }
}
