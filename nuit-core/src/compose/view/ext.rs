use crate::{Alignment, Event, Frame, Gesture, Handler, Insets, Modified, ModifierNode, TapGesture, Vec2, View};

use super::{Gestured, Overlay};

/// An extension trait with various convenience methods for views.
pub trait ViewExt: Sized {
    fn modifier(self, modifier: ModifierNode) -> Modified<Self> {
        Modified::new(self, modifier)
    }

    fn gesture<G>(self, gesture: G) -> Gestured<Self, G> where G: Gesture {
        Gestured::new(self, gesture)
    }

    fn on_taps<F>(self, count: usize, action: F) -> Gestured<Self, TapGesture<F>> where F: Fn() {
        self.gesture(TapGesture::new(count, action))
    }

    fn on_tap<F>(self, action: F) -> Gestured<Self, TapGesture<F>> where F: Fn() {
        self.on_taps(1, action)
    }

    fn overlay_at<O>(self, alignment: Alignment, overlayed: O) -> Overlay<Self, O> where O: View {
        Overlay::new(self, alignment, overlayed)
    }

    fn overlay<O>(self, overlayed: O) -> Overlay<Self, O> where O: View {
        self.overlay_at(Alignment::Center, overlayed)
    }

    fn padding(self, insets: impl Into<Insets>) -> Modified<Self> {
        self.modifier(ModifierNode::Padding { insets: insets.into() })
    }

    fn position(self, position: Vec2<f64>) -> Modified<Self> {
        self.modifier(ModifierNode::Position { position })
    }

    fn offset(self, delta: Vec2<f64>) -> Modified<Self> {
        self.modifier(ModifierNode::Offset { delta })
    }

    fn opacity(self, opacity: impl Into<f64>) -> Modified<Self> {
        self.modifier(ModifierNode::Opacity { opacity: opacity.into() })
    }

    fn frame(self, frame: impl Into<Frame>) -> Modified<Self> {
        self.modifier(ModifierNode::Frame { frame: frame.into() })
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
