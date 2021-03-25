use moxie_dom::{elements::html::div, html_element, interfaces::node::Parent, prelude::document};

html_element! {
    <ui5-button>
    categories { Flow }

    children {
        categories {
            Flow
        }
    }
}

pub fn run() {
    moxie_dom::boot(document().body(), || {
        div().child(ui5_button().child("Hello, world!"))
    });
}
