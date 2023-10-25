use serde::{Serialize, Deserialize};

use crate::Id;

/// A UI component tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Node {
    Empty,
    Text { content: String },
    TextField { content: String },
    Button { label: Box<Id<Node>> },
    VStack { wrapped: Box<Id<Node>> },
    HStack { wrapped: Box<Id<Node>> },
    ZStack { wrapped: Box<Id<Node>> },
    Tuple2 { child1: Box<Id<Node>>, child2: Box<Id<Node>> },
    Tuple3 { child1: Box<Id<Node>>, child2: Box<Id<Node>>, child3: Box<Id<Node>> },
    Tuple4 { child1: Box<Id<Node>>, child2: Box<Id<Node>>, child3: Box<Id<Node>>, child4: Box<Id<Node>> },
    Tuple5 { child1: Box<Id<Node>>, child2: Box<Id<Node>>, child3: Box<Id<Node>>, child4: Box<Id<Node>>, child5: Box<Id<Node>> },
}
