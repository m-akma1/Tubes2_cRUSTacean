mod app;
pub(crate) mod api;
pub(crate) mod components;

use wasm_bindgen::prelude::wasm_bindgen;
use leptos::mount::mount_to_body;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(app::App);
}
