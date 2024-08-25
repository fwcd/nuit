mod imp;

use adw::{glib::{self, Object}, gtk::{self, Button, Label, Orientation, Text}, prelude::BoxExt, subclass::prelude::*};
use nuit_core::Node;

// See https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_subclassing.html

glib::wrapper! {
    pub struct NodeWidget(ObjectSubclass<imp::NodeWidget>)
        @extends gtk::Box, gtk::Widget;
}

impl NodeWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn update(&self, node: Node) {
        // TODO: Diff node

        match &node {
            Node::Empty {} => {},
            Node::Text { content } => {
                let label = Label::new(Some(&content));
                self.append(&label);
            },
            Node::TextField { content } => {
                let text = Text::builder().text(content).build();
                self.append(&text);
            },
            Node::Button { label } => match label.value() {
                Node::Text { content } => {
                    let button = Button::builder().label(content).build();
                    self.append(&button);
                },
                _ => {}, // TODO: Handle non-text button labels
            },
            Node::HStack { wrapped } => {
                let gtk_box = gtk::Box::new(Orientation::Horizontal, 0);
                for (_, child) in wrapped.value().children() {
                    gtk_box.append(&NodeWidget::from(child.clone()))
                }
                self.append(&gtk_box);
            },
            Node::VStack { wrapped } => {
                let gtk_box = gtk::Box::new(Orientation::Vertical, 0);
                for (_, child) in wrapped.value().children() {
                    gtk_box.append(&NodeWidget::from(child.clone()))
                }
                self.append(&gtk_box);
            },
            _ => {}, // TODO: Handle other node types
        }

        let imp = imp::NodeWidget::from_obj(&self);
        imp.node.replace(node);
    }
}

impl From<Node> for NodeWidget {
    fn from(node: Node) -> Self {
        let widget = Self::new();
        widget.update(node);
        widget
    }
}
