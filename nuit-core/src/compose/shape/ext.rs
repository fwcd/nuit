use crate::Style;

use super::{Fill, Shape, Stroke};

pub trait ShapeExt: Sized {
    fn fill(self, style: impl Into<Style>) -> Fill<Self> {
        Fill::new(self, style.into())
    }

    fn stroke(self, style: impl Into<Style>) -> Stroke<Self> {
        Stroke::new(self, style.into())
    }
}

impl<T> ShapeExt for T where T: Shape {}
