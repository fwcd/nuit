use crate::{Insets, Padding, View};

pub trait ViewExt: Sized {
    fn padding(self, insets: Insets) -> Padding<Self> {
        Padding::new(self, insets)
    }
}

impl<T> ViewExt for T where T: View {}
