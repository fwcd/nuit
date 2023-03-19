use serde::{Serialize, Deserialize};

/// A UI component tree composed of primitive components.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Primitive {
    Text { content: String },
    VStack { wrapped: Box<Primitive> },
    HStack { wrapped: Box<Primitive> },
    ZStack { wrapped: Box<Primitive> },
    Group { children: Vec<Primitive> },
}
