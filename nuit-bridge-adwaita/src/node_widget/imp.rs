use std::cell::{Cell, RefCell};

use adw::{glib, gtk, subclass::prelude::*};
use nuit_core::{IdPathBuf, Node};

// See https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_subclassing.html

// Object holding the state
#[derive(Default)]
pub struct NodeWidget {
    pub node: Cell<Node>,
    pub id_path: RefCell<IdPathBuf>,
}

#[glib::object_subclass]
impl ObjectSubclass for NodeWidget {
    const NAME: &'static str = "NuitNodeWidget";
    type Type = super::NodeWidget;
    type ParentType = gtk::Box;
}

impl ObjectImpl for NodeWidget {}

impl WidgetImpl for NodeWidget {}

impl BoxImpl for NodeWidget {}
