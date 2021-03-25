use moxie_dom::interfaces::{
    content_categories::FlowContent,
    node::{Child, NodeBuilder, Parent},
};

// TODO: Which functions should be made topo::nested?

mod raw {
    use moxie_dom::html_element;

    html_element! {
        <ui5-side-navigation>

        categories { Flow }

        children { categories { Flow } }

        attributes {
            collapsed(bool)
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
                .fold(ui5_side_navigation(), |side_nav, i| side_nav.child(i)),
        )
    }

    pub fn collapsed(self) -> SideNavigation {
        Self(self.0.collapsed(true))
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
