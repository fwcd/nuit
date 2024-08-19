use crate::{Event, Frame, Handler, Insets, Modified, Modifier, Style, Vec2, View};

pub trait ViewExt: Sized {
    fn modifier(self, modifier: Modifier) -> Modified<Self> {
        Modified::new(self, modifier)
    }

    fn fill(self, style: Style) -> Modified<Self> {
        self.modifier(Modifier::Fill { style })
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

    fn on_appear(self, action: impl Fn() + 'static) -> Handler<Self, impl Fn(Event)> {
        Handler::new(self, move |e| {
            if let Event::Appear = e {
                action();
            }
        })
    }

    fn on_disappear(self, action: impl Fn() + 'static) -> Handler<Self, impl Fn(Event)> {
        Handler::new(self, move |e| {
            if let Event::Disappear = e {
                action();
            }
        })
    }
}

impl<T> ViewExt for T where T: View {}
