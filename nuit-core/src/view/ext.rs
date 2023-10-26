use crate::{Insets, Modified, Modifier, View};

pub trait ViewExt: Sized {
    fn modifier(self, modifier: Modifier) -> Modified<Self> {
        Modified::new(self, modifier)
    }

    fn padding(self, insets: Insets) -> Modified<Self> {
        self.modifier(Modifier::Padding { insets })
    }
}

impl<T> ViewExt for T where T: View {}
