use wasm_bindgen::prelude::*;
use web_sys::console;
use moxie_dom::{
    elements::html::{div, p},
    html_element,
    interfaces::node::Parent,
    prelude::{state, document},
};
use wasm_bindgen::prelude::wasm_bindgen;

html_element! {
    <ui5-button>
    categories { Flow }

    children {
        categories {
            Flow
        }
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    {
        console_error_panic_hook::set_once();
        console::log_1(&JsValue::from_str("Debug"));
    }

    #[cfg(not(debug_assertions))]
    console::log_1(&JsValue::from_str("Release"));

    moxie_dom::boot(document().body(), || {
        let (expand_count, inc_on_expand) = state(|| 0);
        let (selected_count, inc_on_select) = state(|| 0);

        div().child(ui5_button().child("Hello, world!"))
    });

    Ok(())
}
