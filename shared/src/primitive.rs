use serde::{Serialize, Deserialize};

use crate::Id;

/// A UI component tree composed of primitive components.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Primitive {
    Empty,
    Text { content: String },
    Button { label: Box<Id<Primitive>> },
    VStack { wrapped: Box<Id<Primitive>> },
    HStack { wrapped: Box<Id<Primitive>> },
    ZStack { wrapped: Box<Id<Primitive>> },
    Tuple2 { child1: Box<Id<Primitive>>, child2: Box<Id<Primitive>> },
    Tuple3 { child1: Box<Id<Primitive>>, child2: Box<Id<Primitive>>, child3: Box<Id<Primitive>> },
    Tuple4 { child1: Box<Id<Primitive>>, child2: Box<Id<Primitive>>, child3: Box<Id<Primitive>>, child4: Box<Id<Primitive>> },
    Tuple5 { child1: Box<Id<Primitive>>, child2: Box<Id<Primitive>>, child3: Box<Id<Primitive>>, child4: Box<Id<Primitive>>, child5: Box<Id<Primitive>> },
}
