use moxie_dom::interfaces::{
    content_categories::FlowContent,
    node::{Child, NodeBuilder},
};

mod raw {
    use moxie_dom::html_element;

    html_element! {
        <ui5-icon>

        categories { Flow }

        attributes {
            accessible-name
            interactive(bool)
            name
            show-tooltip
        }
    }
}

use raw::{ui5_icon, Ui5IconBuilder};

pub struct Icon(Ui5IconBuilder);

// TODO: Macro for these fns.
pub fn accept() -> Icon {
    named_icon("accept")
}

fn named_icon(name: &str) -> Icon {
    Icon(ui5_icon().name(name))
}

impl Icon {
    pub fn accessible_name(self, name: impl ToString) -> Self {
        Self(self.0.accessible_name(name))
    }

    pub fn show_tooltip(self) -> Self {
        Self(self.0.show_tooltip(true))
    }

    pub fn interactive(self) -> Self {
        Self(self.0.interactive(true))
    }
}

impl NodeBuilder for Icon {
    type Target = Self;

    fn build(self) -> Self::Target {
        self
    }
}

impl Child for Icon {
    fn to_bind(&self) -> &augdom::Node {
        self.0.to_bind()
    }
}

impl FlowContent for Icon {}
