mod imp;

use std::rc::Rc;

use adw::{glib::{self, Object}, gtk::{self, Align, Button, Label, Orientation, Text}, prelude::{BoxExt, ButtonExt, WidgetExt}, subclass::prelude::*};
use nuit_core::{Event, IdPath, IdPathBuf, Node};

// See https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_subclassing.html

const DEFAULT_SPACING: i32 = 10;

glib::wrapper! {
    pub struct NodeWidget(ObjectSubclass<imp::NodeWidget>)
        @extends gtk::Box, gtk::Widget;
}

impl NodeWidget {
    fn new(
        node: Node,
        id_path: IdPathBuf,
        fire_event: Option<Rc<Box<dyn Fn(&IdPath, Event)>>>,
    ) -> Self {
        let widget: Self = Object::builder().build();

        let imp = imp::NodeWidget::from_obj(&widget);
        imp.id_path.replace(id_path);
        imp.fire_event.replace(fire_event);

        widget.set_halign(Align::Center);
        widget.set_valign(Align::Center);
        widget.set_vexpand(true);

        widget.update(node);

        widget
    }

    pub fn root(node: Node, fire_event: impl Fn(&IdPath, Event) + 'static) -> Self {
        Self::new(node, IdPathBuf::root(), Some(Rc::new(Box::new(fire_event))))
    }

    fn child(&self, node: Node, child_path: &IdPath) -> Self {
        let imp = imp::NodeWidget::from_obj(self);
        let id_path = imp.id_path.borrow().join(child_path);
        let fire_event = imp.fire_event.borrow().clone();

        Self::new(node, id_path, fire_event)
    }

    fn update(&self, node: Node) {
        // TODO: Diff node

        let imp = imp::NodeWidget::from_obj(&self);
        imp.node.replace(node.clone());

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
            Node::Button { label } => {
                let button = Button::new();
                match label.value() {
                    Node::Text { content } => button.set_label(content),
                    _ => button.set_child(Some(&self.child(label.value().clone(), &IdPathBuf::from(label.id().clone())))),
                }
                self.append(&button);
            },
            Node::HStack { wrapped } => {
                let gtk_box = gtk::Box::new(Orientation::Horizontal, DEFAULT_SPACING);
                for (child_path, child) in wrapped.value().children() {
                    gtk_box.append(&self.child(child.clone(), &child_path))
                }
                self.append(&gtk_box);
            },
            Node::VStack { wrapped } => {
                let gtk_box = gtk::Box::new(Orientation::Vertical, DEFAULT_SPACING);
                for (child_path, child) in wrapped.value().children() {
                    gtk_box.append(&self.child(child.clone(), &child_path))
                }
                self.append(&gtk_box);
            },
            _ => {}, // TODO: Handle other node types
        }
    }
}
