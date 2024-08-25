mod node_widget;

use std::sync::{Arc, Mutex};

use adw::{gtk::{Box, Orientation}, prelude::*, Application, ApplicationWindow, HeaderBar};
use node_widget::NodeWidget;
use nuit_core::{IdPath, Root, View};

pub fn run_app<T>(root: Root<T>) where T: View + 'static {
    let root = Arc::new(Mutex::new(root));
    let app = Application::builder()
        .application_id("com.example.NuitApp")
        .build();

    app.connect_activate(move |app| {
        let node = Root::render(&mut root.lock().unwrap());
        let node_widget = NodeWidget::from_node(node, IdPath::root());

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
