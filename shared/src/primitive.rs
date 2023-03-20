use serde::{Serialize, Deserialize};

/// A UI component tree composed of primitive components.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Primitive {
    Empty,
    Button { label: Box<Primitive> },
    Text { content: String },
    VStack { wrapped: Box<Primitive> },
    HStack { wrapped: Box<Primitive> },
    ZStack { wrapped: Box<Primitive> },
    Tuple2 { child1: Box<Primitive>, child2: Box<Primitive> },
    Tuple3 { child1: Box<Primitive>, child2: Box<Primitive>, child3: Box<Primitive> },
    Tuple4 { child1: Box<Primitive>, child2: Box<Primitive>, child3: Box<Primitive>, child4: Box<Primitive> },
    Tuple5 { child1: Box<Primitive>, child2: Box<Primitive>, child3: Box<Primitive>, child4: Box<Primitive>, child5: Box<Primitive> },
}
