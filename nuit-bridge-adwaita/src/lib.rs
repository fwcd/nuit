#![feature(reentrant_lock)]

mod node_widget;

use std::sync::{Arc, ReentrantLock};

use adw::{gtk::{Box, Orientation}, prelude::*, Application, ApplicationWindow, HeaderBar};
use node_widget::NodeWidget;
use nuit_core::{clone, Root, View};

/// Runs the given app root using Adwaita/GTK4.
pub fn run_app<T>(root: Root<T>) where T: View + 'static {
    let root = Arc::new(ReentrantLock::new(root));
    let app = Application::builder()
        .application_id("com.example.NuitApp")
        .build();

    app.connect_activate(move |app| {
        let node = Root::render(&root.lock());
        let node_widget = NodeWidget::root(node, clone!(root => move |id_path, event| {
            root.lock().fire_event(id_path, event);
        }));

        root.lock().set_update_callback(clone!(root, node_widget => move || {
            node_widget.update(Root::render(&mut root.lock()));
        }));

        let content = Box::new(Orientation::Vertical, 0);
        content.append(&HeaderBar::new());
        content.append(&node_widget);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Nuit App")
            .default_width(640)
            .default_height(480)
            .content(&content)
            .build();
        window.present();
    });

    app.run();
}
