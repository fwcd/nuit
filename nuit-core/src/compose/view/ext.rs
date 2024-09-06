use crate::{Alignment, Angle, DragEvent, DragGesture, Event, Font, Frame, Gesture, Handler, Insets, Modified, ModifierNode, Style, TapGesture, UnitPoint, Vec2, View};

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
        self.gesture(TapGesture::new_single(action))
    }

    fn on_drag_by<F>(self, minimum_distance: impl Into<f64>, action: F) -> Gestured<Self, DragGesture<F>> where F: Fn(&DragEvent) {
        self.gesture(DragGesture::new(minimum_distance.into(), action))
    }

    fn on_drag<F>(self, action: F) -> Gestured<Self, DragGesture<F>> where F: Fn(&DragEvent) {
        self.gesture(DragGesture::new_default(action))
    }

    fn overlay_at<O>(self, alignment: Alignment, overlayed: O) -> Overlay<Self, O> where O: View {
        Overlay::new(self, alignment, overlayed)
    }

    fn overlay<O>(self, overlayed: O) -> Overlay<Self, O> where O: View {
        self.overlay_at(Alignment::default(), overlayed)
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

    fn frame_with(self, alignment: Alignment, frame: impl Into<Frame>) -> Modified<Self> {
        self.modifier(ModifierNode::Frame { frame: frame.into(), alignment })
    }

    fn frame(self, frame: impl Into<Frame>) -> Modified<Self> {
        self.frame_with(Alignment::default(), frame)
    }

    fn font(self, font: impl Into<Font>) -> Modified<Self> {
        self.modifier(ModifierNode::Font { font: font.into() })
    }

    fn foreground_style(self, style: impl Into<Style>) -> Modified<Self> {
        self.modifier(ModifierNode::ForegroundStyle { style: style.into() })
    }

    fn scale_effect_around(self, anchor: UnitPoint, factor: impl Into<f64>) -> Modified<Self> {
        self.modifier(ModifierNode::ScaleEffect { factor: factor.into(), anchor })
    }

    fn scale_effect(self, factor: impl Into<f64>) -> Modified<Self> {
        self.scale_effect_around(UnitPoint::CENTER, factor)
    }

    fn rotation_effect_around(self, anchor: UnitPoint, angle: impl Into<Angle>) -> Modified<Self> {
        self.modifier(ModifierNode::RotationEffect { angle: angle.into(), anchor })
    }

    fn rotation_effect(self, angle: impl Into<Angle>) -> Modified<Self> {
        self.rotation_effect_around(UnitPoint::CENTER, angle)
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
