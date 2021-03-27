use moxie_dom::{
    elements::{html::div, text_content::DivBuilder},
    interfaces::node::Parent,
    prelude::document,
};
use rui5t::{
    badge::badge,
    side_navigation::{Item, SideNavigation},
};
use wasm_bindgen::prelude::{wasm_bindgen, *};
use web_sys::console;

fn log(message: &str) {
    console::log_1(&JsValue::from_str(message));
}

fn page() -> DivBuilder {
    div()
        .child(
            SideNavigation::new(vec![Item::text("Item 0"), Item::text("Item 1")])
                .on_selection(|event| console::log_1(&event)),
        )
        .child(badge("A Badge"))
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    {
        log("Debug");
        console_error_panic_hook::set_once();
    }

    moxie_dom::boot(document().body(), page);

    Ok(())
}
