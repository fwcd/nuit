use nui_shared::{Text, VStack};

fn main() {
    nui::run_app(VStack::new((
        Text::new("Hello"),
        Text::new("World"),
    )));
}
