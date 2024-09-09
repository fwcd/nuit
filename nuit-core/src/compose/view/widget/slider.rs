use std::ops::RangeInclusive;

use nuit_derive::Bind;

use crate::{Access, Binding, Context, Event, EventResponse, IdPath, Node, View};

/// A control for selecting numeric values from a bounded range.
#[derive(Debug, Clone, Bind)]
pub struct Slider {
    value: Binding<f64>,
    range: RangeInclusive<f64>,
    step: Option<f64>,
}

impl Slider {
    #[must_use]
    pub const fn new(value: Binding<f64>, range: RangeInclusive<f64>, step: Option<f64>) -> Self {
        Self { value, range, step }
    }

    #[must_use]
    pub const fn with_default_step(value: Binding<f64>, range: RangeInclusive<f64>) -> Self {
        Self { value, range, step: None }
    }
}

impl View for Slider {
    fn fire(&self, event: &Event, event_path: &IdPath, _context: &Context) -> EventResponse {
        assert!(event_path.is_root());
        if let Event::UpdateSliderValue { value } = event {
            self.value.set(*value);
        }
        EventResponse::default()
    }

    fn render(&self, _context: &Context) -> Node {
        Node::Slider {
            value: self.value.get(),
            lower_bound: *self.range.start(),
            upper_bound: *self.range.end(),
            step: self.step,
        }
    }
}
