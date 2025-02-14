use yew::prelude::*;
use web_sys::window;
use crate::directory::Directory;

// Main app component
#[function_component(App)]
pub fn app() -> Html {
    let path = window()
        .and_then(|w| w.location().pathname().ok())
        .unwrap_or_else(|| "/".to_string());
    html! {
        <Directory path={path.clone()} />
    }
}
