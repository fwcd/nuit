use nui_shared::{Text, Group, VStack};

fn main() {
    nui::run_app(VStack::new(Group::new([
        Text::new("Hello"),
        Text::new("World"),
    ])));
}
