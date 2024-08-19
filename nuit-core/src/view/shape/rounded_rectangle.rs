use crate::{Bind, Context, Event, IdPath, Node, Vec2, View};

#[derive(Debug, Clone, PartialEq)]
pub struct RoundedRectangle {
    corner_size: Vec2<f64>,
}

impl RoundedRectangle {
    pub fn new(corner_size: Vec2<f64>) -> Self {
        Self { corner_size }
    }

    pub fn with_corner_radius(corner_radius: f64) -> Self {
        Self::new(Vec2::new(corner_radius, corner_radius))
    }
}

impl Bind for RoundedRectangle {}

impl View for RoundedRectangle {
    fn fire(&self, _event: &Event, _id_path: &IdPath) {}

    fn render(&mut self, _context: &Context) -> Node {
        Node::RoundedRectangle {
            corner_size: self.corner_size
        }
    }
}
