mod imp;

use std::rc::Rc;

use adw::{glib::{self, Object}, gtk::{self, Align, Button, Label, Orientation, Text}, prelude::{BoxExt, ButtonExt, EditableExt, WidgetExt}, subclass::prelude::*};
use nuit_core::{clone, Event, Id, IdPath, IdPathBuf, Identified, Node};

// See https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_subclassing.html

glib::wrapper! {
    pub struct NodeWidget(ObjectSubclass<imp::NodeWidget>)
        @extends gtk::Box, gtk::Widget;
}

impl NodeWidget {
    fn new(
        node: Node,
        id_path: IdPathBuf,
        fire_event: Option<Rc<Box<dyn Fn(&IdPath, &Event)>>>,
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

    pub fn root(node: Node, fire_event: impl Fn(&IdPath, &Event) + 'static) -> Self {
        Self::new(node, IdPathBuf::root(), Some(Rc::new(Box::new(fire_event))))
    }

    fn child_with_id(&self, node: Node, id: &Id) -> Self {
        let imp = imp::NodeWidget::from_obj(self);
        let id_path = imp.id_path.borrow().child(id.clone());
        let fire_event = imp.fire_event.borrow().clone();

        Self::new(node, id_path, fire_event)
    }

    fn child_from_identified(&self, node: &Identified<Node>) -> Self {
        self.child_with_id(node.value().clone(), node.id())
    }

    fn child_with_path(&self, node: Node, child_path: &IdPath) -> Self {
        let imp = imp::NodeWidget::from_obj(self);
        let id_path = imp.id_path.borrow().join(child_path);
        let fire_event = imp.fire_event.borrow().clone();

        Self::new(node, id_path, fire_event)
    }

    pub fn update(&self, node: Node) {
        let imp = imp::NodeWidget::from_obj(&self);
        imp.node.replace(node.clone());

        let id_path = imp.id_path.borrow();
        let fire_event = imp.fire_event.borrow();

        // TODO: Diff nodes instead of replacing the entire tree

        while let Some(child) = self.first_child() {
            self.remove(&child);
        }

        match &node {
            Node::Empty {} => {},
            Node::Text { content } => {
                let label = Label::new(Some(&content));
                self.append(&label);
            },
            Node::TextField { content } => {
                let text = Text::builder().text(content).build();
                if let Some(ref fire_event) = *fire_event {
                    text.connect_changed(clone!(fire_event, id_path => move |text| {
                        fire_event(&id_path, &Event::UpdateText { content: text.text().into() });
                    }));
                }
                self.append(&text);
            },
            Node::Button { label } => {
                let button = Button::new();
                match label.value() {
                    Node::Text { content } => button.set_label(content),
                    _ => button.set_child(Some(&self.child_from_identified(&label))),
                }
                if let Some(ref fire_event) = *fire_event {
                    button.connect_clicked(clone!(fire_event, id_path => move |_button| {
                        fire_event(&id_path, &Event::Click {});
                    }));
                }
                self.append(&button);
            },
            Node::HStack { spacing, wrapped } => {
                let gtk_box = gtk::Box::new(Orientation::Horizontal, *spacing as i32);
                for (child_path, child) in wrapped.value().children_from(&IdPathBuf::from(wrapped.id().clone())) {
                    gtk_box.append(&self.child_with_path(child.clone(), &child_path))
                }
                self.append(&gtk_box);
            },
            Node::VStack { spacing, wrapped } => {
                let gtk_box = gtk::Box::new(Orientation::Vertical, *spacing as i32);
                for (child_path, child) in wrapped.value().children_from(&IdPathBuf::from(wrapped.id().clone())) {
                    gtk_box.append(&self.child_with_path(child.clone(), &child_path))
                }
                self.append(&gtk_box);
            },
            Node::Modified { wrapped, modifier: _ } => {
                // TODO: Implement modifiers
                eprintln!("Warning: Modifiers are not supported yet and ignored");
                self.append(&self.child_from_identified(&wrapped))
            },
            // TODO: Add remaining node types
            _ => {
                eprintln!("Warning: Unimplemented node type {node:?}");
            },
        }
    }
}
