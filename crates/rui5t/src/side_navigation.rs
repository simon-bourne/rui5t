use moxie_dom::{
    interfaces::{
        content_categories::FlowContent,
        node::{Child, NodeBuilder, Parent},
    },
    prelude::ElementBuilder,
};

// TODO: Which functions should be made topo::nested?

mod raw {
    // TODO: This is required to get `custom_events` to compile.
    // Need to fix in `html_elements!`.
    use moxie_dom::html_element;
    use wasm_bindgen::prelude::wasm_bindgen;

    html_element! {
        <ui5-side-navigation>

        categories { Flow }

        children { categories { Flow } }

        attributes {
            collapsed(bool)
        }

        custom_events {
            selection-change
        }
    }

    impl CustomSelectionChange {
        pub fn index(&self) -> usize {
            todo!("Lookup the id on self.0.detail() and convert to int")
        }
    }

    html_element! {
        <ui5-side-navigation-item>

        categories { Flow }

        children { categories { Flow } }

        attributes {
            // TODO: icon::Image enum
            expanded(bool)
            selected(bool)
            text
            whole-item-toggleable(bool)
        }
    }
}

use raw::{
    ui5_side_navigation,
    ui5_side_navigation_item,
    CustomSelectionChange,
    Ui5SideNavigationBuilder,
    Ui5SideNavigationItemBuilder,
};

// TODO: Sub items

pub struct SideNavigation(Ui5SideNavigationBuilder);

impl SideNavigation {
    pub fn new(items: impl IntoIterator<Item = Item>) -> Self {
        Self(
            items
                .into_iter()
                .enumerate()
                .fold(ui5_side_navigation(), |side_nav, (i, child_item)| {
                    side_nav.child(child_item.0.id(i))
                }),
        )
    }

    pub fn collapsed(self) -> Self {
        Self(self.0.collapsed(true))
    }

    // TODO: Specify an id when adding item (generic param on SideNavigation).
    // This should extract that info from the event and send it to the callback.
    pub fn on_selection(self, mut callback: impl FnMut(CustomSelectionChange) + 'static) -> Self {
        Self(self.0.on_selection_change(move |event| {
            // TODO: Figure out which item was clicked, rather than sending raw js object
            callback(event);
        }))
    }
}

pub struct Item(Ui5SideNavigationItemBuilder);

impl Item {
    // TODO: icon constructor, and new(icon, text).
    // TODO: Other slots
    pub fn text(value: impl ToString) -> Self {
        Self(ui5_side_navigation_item().text(value))
    }

    pub fn expanded(self) -> Self {
        Self(self.0.expanded(true))
    }

    pub fn selected(self) -> Self {
        Self(self.0.selected(true))
    }

    // TODO: Think of a better name
    pub fn whole_item_toggleable(self) -> Self {
        Self(self.0.whole_item_toggleable(true))
    }
}

impl NodeBuilder for SideNavigation {
    type Target = Self;

    // TODO: Take a good look at the internals of moxie, and understand how this
    // works.
    #[topo::nested]
    fn build(self) -> Self::Target {
        self
    }
}

impl Child for SideNavigation {
    fn to_bind(&self) -> &augdom::Node {
        self.0.to_bind()
    }
}

impl FlowContent for SideNavigation {}

impl NodeBuilder for Item {
    type Target = Self;

    fn build(self) -> Self::Target {
        self
    }
}

impl Child for Item {
    fn to_bind(&self) -> &augdom::Node {
        self.0.to_bind()
    }
}

impl FlowContent for Item {}
