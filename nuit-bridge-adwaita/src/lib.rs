use adw::{prelude::*, Application, ApplicationWindow};
use nuit_core::{Root, View};

pub fn run_app<T>(root: Root<T>) where T: View + 'static {
    let app = Application::builder()
        .application_id("com.example.NuitApp")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Nuit App")
            .default_width(640)
            .default_height(480)
            .build();
        window.present();
    });

    app.run();
}
