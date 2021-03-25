use moxie_dom::{elements::html::div, interfaces::node::Parent, prelude::document};
use rui5t::side_navigation::{Item, SideNavigation};
use wasm_bindgen::prelude::{wasm_bindgen, *};
use web_sys::console;

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
        div().child(SideNavigation::new(vec![
            Item::text("Item 0"),
            Item::text("Item 1"),
        ]))
    });

    Ok(())
}
