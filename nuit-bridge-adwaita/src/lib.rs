mod node_widget;

use adw::{gtk::{Box, ListBox, Orientation, SelectionMode}, prelude::*, ActionRow, Application, ApplicationWindow, HeaderBar};
use nuit_core::{Root, View};

pub fn run_app<T>(root: Root<T>) where T: View + 'static {
    let app = Application::builder()
        .application_id("com.example.NuitApp")
        .build();

    app.connect_activate(|app| {
        let row = ActionRow::builder()
            .activatable(true)
            .title("Click me")
            .build();
        row.connect_activated(|_| {
            eprintln!("Clicked!");
        });

        let list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            .selection_mode(SelectionMode::None)
            .css_classes(vec!["boxed-list".to_owned()])
            .build();
        list.append(&row);

        let content = Box::new(Orientation::Vertical, 0);
        content.append(&HeaderBar::new());
        content.append(&list);

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
