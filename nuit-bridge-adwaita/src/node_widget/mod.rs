mod imp;

use std::rc::Rc;

use adw::{glib::{self, Object}, gtk::{self, Align, Button, Label, Orientation, Scale, Text}, prelude::{BoxExt, ButtonExt, EditableExt, RangeExt, WidgetExt}, subclass::prelude::*};
use nuit_core::{clone, Event, Id, IdPath, IdPathBuf, Identified, Node};

use crate::convert::ToGtk;

// See https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_subclassing.html

glib::wrapper! {
    pub struct NodeWidget(ObjectSubclass<imp::NodeWidget>)
        @extends gtk::Box, gtk::Widget;
}

impl NodeWidget {
    #[allow(clippy::type_complexity)]
    fn new(
        node: Node,
        id_path: IdPathBuf,
        fire_event: Option<Rc<dyn Fn(&IdPath, &Event)>>,
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
        Self::new(node, IdPathBuf::root(), Some(Rc::new(fire_event)))
    }

    fn create_child_with_id(&self, node: Node, id: &Id) -> Self {
        let imp = imp::NodeWidget::from_obj(self);
        let id_path = imp.id_path.borrow().child(id.clone());
        let fire_event = imp.fire_event.borrow().clone();

        Self::new(node, id_path, fire_event)
    }

    fn create_child_from_identified(&self, node: &Identified<Node>) -> Self {
        self.create_child_with_id(node.value().clone(), node.id())
    }

    fn create_child_with_path(&self, node: Node, child_path: &IdPath) -> Self {
        let imp = imp::NodeWidget::from_obj(self);
        let id_path = imp.id_path.borrow().join(child_path);
        let fire_event = imp.fire_event.borrow().clone();

        Self::new(node, id_path, fire_event)
    }

    // TODO: Address the pass-by-value lint once we figure out a proper solution
    // to the diffing problem

    #[allow(clippy::cast_possible_truncation, clippy::needless_pass_by_value)]
    pub fn update(&self, node: Node) {
        let imp = imp::NodeWidget::from_obj(self);
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
                let label = Label::new(Some(content));
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
                    _ => button.set_child(Some(&self.create_child_from_identified(label))),
                }
                if let Some(ref fire_event) = *fire_event {
                    button.connect_clicked(clone!(fire_event, id_path => move |_button| {
                        fire_event(&id_path, &Event::ButtonTap {});
                    }));
                }
                self.append(&button);
            },
            Node::Slider { value, lower_bound, upper_bound, step } => {
                let scale = Scale::with_range(Orientation::Horizontal, *lower_bound, *upper_bound, step.unwrap_or(1e-32));
                scale.set_value(*value);
                scale.set_width_request(150);
                if let Some(ref fire_event) = *fire_event {
                    scale.connect_value_changed(clone!(fire_event, id_path => move |scale| {
                        let value = scale.value();
                        fire_event(&id_path, &Event::UpdateSliderValue { value });
                    }));
                }
                self.append(&scale);
            },
            Node::HStack { spacing, alignment, wrapped } => {
                let gtk_box = gtk::Box::new(Orientation::Horizontal, *spacing as i32);
                gtk_box.set_valign(alignment.to_gtk());
                for (child_path, child) in wrapped.value().children_from(&IdPathBuf::from(wrapped.id().clone())) {
                    gtk_box.append(&self.create_child_with_path(child.clone(), &child_path));
                }
                self.append(&gtk_box);
            },
            Node::VStack { spacing, alignment, wrapped } => {
                let gtk_box = gtk::Box::new(Orientation::Vertical, *spacing as i32);
                gtk_box.set_halign(alignment.to_gtk());
                for (child_path, child) in wrapped.value().children_from(&IdPathBuf::from(wrapped.id().clone())) {
                    gtk_box.append(&self.create_child_with_path(child.clone(), &child_path));
                }
                self.append(&gtk_box);
            },
            Node::Modified { wrapped, modifier: _ } => {
                // TODO: Implement modifiers
                eprintln!("Warning: Modifiers are not supported yet and ignored");
                self.append(&self.create_child_from_identified(wrapped));
            },
            // TODO: Add remaining node types
            _ => {
                eprintln!("Warning: Unimplemented node type {node:?}");
            },
        }
    }
}
