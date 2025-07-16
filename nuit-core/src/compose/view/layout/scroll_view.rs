use nuit_derive::Bind;
use serde::{Serialize, Deserialize};

use crate::{Context, Event, EventResponse, Id, IdPath, IdentifyExt, Node, View};

/// The axis along which a scroll view scrolls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Axis {
    /// Horizontal scrolling only
    Horizontal,
    /// Vertical scrolling only  
    Vertical,
    /// Both horizontal and vertical scrolling
    Both,
}

impl Default for Axis {
    fn default() -> Self {
        Self::Vertical
    }
}

/// A scrollable view that can contain content larger than the visible area.
#[derive(Debug, Clone, PartialEq, Bind)]
pub struct ScrollView<T> {
    axes: Axis,
    show_indicators: bool,
    wrapped: T,
}

impl<T> ScrollView<T> {
    /// Creates a new ScrollView with the given axes and wrapped content.
    #[must_use]
    pub fn new(axes: Axis, wrapped: T) -> Self {
        Self {
            axes,
            show_indicators: true,
            wrapped,
        }
    }

    /// Creates a new vertical ScrollView with the wrapped content.
    #[must_use]
    pub fn vertical(wrapped: T) -> Self {
        Self::new(Axis::Vertical, wrapped)
    }

    /// Creates a new horizontal ScrollView with the wrapped content.
    #[must_use]
    pub fn horizontal(wrapped: T) -> Self {
        Self::new(Axis::Horizontal, wrapped)
    }

    /// Sets whether to show scroll indicators.
    #[must_use]
    pub fn show_indicators(mut self, show: bool) -> Self {
        self.show_indicators = show;
        self
    }
}

impl<T> From<T> for ScrollView<T> {
    fn from(wrapped: T) -> Self {
        Self::vertical(wrapped)
    }
}

impl<T> View for ScrollView<T> where T: View {
    fn fire(&self, event: &Event, event_path: &IdPath, context: &Context) -> EventResponse {
        if let Some(head) = event_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, event_path.tail(), &context.child(0)),
                i => panic!("Cannot fire event for child id {i} on ScrollView which only has one child"),
            }
        } else {
            EventResponse::default()
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::ScrollView { 
            axes: self.axes, 
            show_indicators: self.show_indicators,
            wrapped: Box::new(self.wrapped.render(&context.child(0)).identify(0))
        }
    }
}