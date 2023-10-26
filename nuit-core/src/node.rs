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
    Group { children: Vec<Id<Node>> },
}
