#![recursion_limit = "1024"]

pub mod app;
pub mod components;
pub mod routes;
pub mod utils;

use wasm_bindgen::prelude::*;

use app::App;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}
