mod imp;

use adw::{glib::{self, Object}, gtk::{self, Align, Button, Label, Orientation, Text}, prelude::{BoxExt, WidgetExt}, subclass::prelude::*};
use nuit_core::{IdPath, Node};

// See https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_subclassing.html

const DEFAULT_SPACING: i32 = 10;

glib::wrapper! {
    pub struct NodeWidget(ObjectSubclass<imp::NodeWidget>)
        @extends gtk::Box, gtk::Widget;
}

impl NodeWidget {
    pub fn new(id_path: &IdPath) -> Self {
        let widget: Self = Object::builder().build();

        widget.set_halign(Align::Center);
        widget.set_valign(Align::Center);
        widget.set_vexpand(true);

        let imp = imp::NodeWidget::from_obj(&widget);
        imp.id_path.replace(id_path.to_owned());

        widget
    }

    pub fn from_node(node: Node, id_path: &IdPath) -> Self {
        let widget = Self::new(id_path);
        widget.update(node);
        widget
    }

    pub fn update(&self, node: Node) {
        // TODO: Diff node

        let imp = imp::NodeWidget::from_obj(&self);
        imp.node.replace(node.clone());

        let id_path = imp.id_path.borrow();

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
                let gtk_box = gtk::Box::new(Orientation::Horizontal, DEFAULT_SPACING);
                for (child_path, child) in wrapped.value().children() {
                    gtk_box.append(&NodeWidget::from_node(child.clone(), &id_path.join(&child_path)))
                }
                self.append(&gtk_box);
            },
            Node::VStack { wrapped } => {
                let gtk_box = gtk::Box::new(Orientation::Vertical, DEFAULT_SPACING);
                for (child_path, child) in wrapped.value().children() {
                    gtk_box.append(&NodeWidget::from_node(child.clone(), &id_path.join(&child_path)))
                }
                self.append(&gtk_box);
            },
            _ => {}, // TODO: Handle other node types
        }
    }
}
