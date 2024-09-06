use adw::gtk::Align;
use nuit_core::{HorizontalAlignment, VerticalAlignment};

pub trait ToGtk {
    type GtkValue;

    fn to_gtk(self) -> Self::GtkValue;
}

impl ToGtk for HorizontalAlignment {
    type GtkValue = Align;

    fn to_gtk(self) -> Self::GtkValue {
        match self {
            HorizontalAlignment::Leading => Align::Start,
            HorizontalAlignment::Center => Align::Center,
            HorizontalAlignment::Trailing => Align::End,
        }
    }
}

impl ToGtk for VerticalAlignment {
    type GtkValue = Align;

    fn to_gtk(self) -> Self::GtkValue {
        match self {
            VerticalAlignment::Top => Align::Start,
            VerticalAlignment::Center => Align::Center,
            VerticalAlignment::Bottom => Align::End,
            VerticalAlignment::FirstTextBaseline => Align::Baseline,
            VerticalAlignment::LastTextBaseline => Align::Baseline,
        }
    }
}
