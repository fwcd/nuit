use nuit_core::{Root, View};
use gtk::prelude::*;
use relm4::prelude::*;

struct NuitApp<T> {
    root: Root<T>,
}

#[relm4::component]
impl<T> SimpleComponent for NuitApp<T> where T: View + 'static {
    type Init = Root<T>;
    type Input = ();
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Nuit app"),
            set_default_size: (640, 480)
        }
    }

    fn init(
        nuit_root: Root<T>,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = NuitApp { root: nuit_root };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}

pub fn run_app<T>(root: Root<T>) where T: View + 'static {
    let relm_app = RelmApp::new("nuitapp"); // TODO: Provide a way to customize the app id
    relm_app.run::<NuitApp<T>>(root);
}
