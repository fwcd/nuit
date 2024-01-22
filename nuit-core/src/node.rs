use serde::{Serialize, Deserialize};

use crate::{Identified, Modifier};

/// A UI component tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Node {
    // Primitive
    Empty,
    Text { content: String },
    TextField { content: String },
    Button { label: Box<Identified<Node>> },

    // Aggregation
    Group { children: Vec<Identified<Node>> },

    // Layout
    VStack { wrapped: Box<Identified<Node>> },
    HStack { wrapped: Box<Identified<Node>> },
    ZStack { wrapped: Box<Identified<Node>> },
    List { wrapped: Box<Identified<Node>> },

    // Modifier
    Modified { wrapped: Box<Identified<Node>>, modifier: Modifier, }
}
