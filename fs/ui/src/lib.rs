mod app;
mod directory;

use wasm_bindgen::prelude::*;
use crate::app::App;

#[wasm_bindgen(start)]
fn start() {
    yew::Renderer::<App>::new().render();
}
