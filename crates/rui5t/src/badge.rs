use moxie_dom::interfaces::{content_categories::FlowContent, node::{Child, NodeBuilder, Parent}};

mod raw {
    use moxie_dom::html_element;

    html_element! {
        <ui5-badge>

        categories { Flow }

        // TODO: Restrict to text children
        children { categories { Flow } }

        attributes {
            accessible-name
            interactive(bool)
            name
            show-tooltip
        }
    }
}

use raw::{ui5_badge, Ui5BadgeBuilder};

pub struct Badge(Ui5BadgeBuilder);

// TODO: Macro for these fns.
pub fn badge(text: impl ToString) -> Badge {
    Badge(ui5_badge().child(text.to_string()))
}

// TODO: impl Badge with icon and color_scheme methods

impl NodeBuilder for Badge {
    type Target = Self;

    fn build(self) -> Self::Target {
        self
    }
}

impl Child for Badge {
    fn to_bind(&self) -> &augdom::Node {
        self.0.to_bind()
    }
}

impl FlowContent for Badge {}
