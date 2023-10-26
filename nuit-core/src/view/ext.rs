use crate::{Insets, Modified, Modifier, View, Frame};

pub trait ViewExt: Sized {
    fn modifier(self, modifier: Modifier) -> Modified<Self> {
        Modified::new(self, modifier)
    }

    fn padding(self, insets: Insets) -> Modified<Self> {
        self.modifier(Modifier::Padding { insets })
    }

    fn frame(self, frame: Frame) -> Modified<Self> {
        self.modifier(Modifier::Frame { frame })
    }
}

impl<T> ViewExt for T where T: View {}
