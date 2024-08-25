mod imp;

use adw::{glib::{self, Object}, gtk, subclass::prelude::*};
use nuit_core::Node;

// See https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_subclassing.html

glib::wrapper! {
    pub struct NodeWidget(ObjectSubclass<imp::NodeWidget>)
        @extends gtk::Widget;
}

impl NodeWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl From<Node> for NodeWidget {
    fn from(node: Node) -> Self {
        let obj = Self::new();
        let imp = imp::NodeWidget::from_obj(&obj);
        imp.node.replace(node);
        obj
    }
}
