use std::marker::PhantomData;

use nuit_derive::Bind;

use crate::{Context, Event, EventResponse, Geometry, Id, IdPath, IdentifyExt, Node, View};

#[derive(Debug, Clone, PartialEq, Eq, Bind)]
pub struct GeometryReader<F, T> {
    view_func: F,
    phantom_view: PhantomData<T>,
}

impl<F, T> GeometryReader<F, T>
where
    F: Fn(Geometry) -> T,
    T: View,
{
    #[must_use]
    pub const fn new(view_func: F) -> Self {
        Self {
            view_func,
            phantom_view: PhantomData,
        }
    }
}

impl<F, T> View for GeometryReader<F, T>
where
    F: Fn(Geometry) -> T,
    T: View,
{
    fn fire(&self, event: &Event, event_path: &IdPath, context: &Context) -> EventResponse {
        if let Some(head) = event_path.head() {
            match head {
                Id::Index(i) => panic!("Cannot fire event for child id {i} on GeometryReader"),
                Id::String(geometry_json) => {
                    let geometry = serde_json::from_str(&geometry_json).expect("Could not deserialize geometry reader id");
                    let view = (self.view_func)(geometry);
                    view.fire(event, event_path.tail(), &context.child(geometry_json))
                },
            }
        } else if let Event::GetGeometryReaderView { geometry } = event {
            let geometry_json = serde_json::to_string(geometry).expect("Could not serialize geometry reader id");
            let id = Id::string(geometry_json);

            let view = (self.view_func)(*geometry);
            let node = view.render(&context.child(id.clone())).identify(id);
            EventResponse::Node { node }
        } else {
            EventResponse::default()
        }
    }

    fn render(&self, _context: &Context) -> Node {
        Node::GeometryReader {}
    }
}
