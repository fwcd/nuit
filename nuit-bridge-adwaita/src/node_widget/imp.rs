use std::cell::Cell;

use adw::{glib, gtk, subclass::prelude::*};
use nuit_core::Node;

// See https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_subclassing.html

// Object holding the state
#[derive(Default)]
pub struct NodeWidget {
    node: Cell<Node>,
}

impl NodeWidget {
    pub fn update(&self, node: Node) {
        self.node.replace(node);
        // TODO: Update widget
    }
}

#[glib::object_subclass]
impl ObjectSubclass for NodeWidget {
    const NAME: &'static str = "NuitNodeWidget";
    type Type = super::NodeWidget;
    type ParentType = gtk::Widget;
}

impl ObjectImpl for NodeWidget {}

impl WidgetImpl for NodeWidget {}
