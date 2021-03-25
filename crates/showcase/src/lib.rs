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

    rui5t::run();
    
    Ok(())
}
