use crate::{Insets, Modified, Modifier, View, Frame, Vec2};

pub trait ViewExt: Sized {
    fn modifier(self, modifier: Modifier) -> Modified<Self> {
        Modified::new(self, modifier)
    }

    fn padding(self, insets: Insets) -> Modified<Self> {
        self.modifier(Modifier::Padding { insets })
    }

    fn position(self, position: Vec2<f64>) -> Modified<Self> {
        self.modifier(Modifier::Position { position })
    }

    fn frame(self, frame: Frame) -> Modified<Self> {
        self.modifier(Modifier::Frame { frame })
    }
}

impl<T> ViewExt for T where T: View {}
