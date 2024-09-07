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
            Self::Leading => Align::Start,
            Self::Center => Align::Center,
            Self::Trailing => Align::End,
        }
    }
}

impl ToGtk for VerticalAlignment {
    type GtkValue = Align;

    fn to_gtk(self) -> Self::GtkValue {
        match self {
            Self::Top => Align::Start,
            Self::Center => Align::Center,
            Self::Bottom => Align::End,
            Self::FirstTextBaseline
            | Self::LastTextBaseline => Align::Baseline,
        }
    }
}
