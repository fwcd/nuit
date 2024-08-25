use serde::{Serialize, Deserialize};

use crate::{Id, Identified};

use super::{ModifierNode, ShapeNode};

/// A rendered UI component tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Node {
    Empty {}, // Intentionally not a unit variant for uniform serialization

    // Widget
    Text { content: String },
    TextField { content: String },
    Button { label: Box<Identified<Node>> },
    Picker { title: String, selection: Id, content: Box<Identified<Node>> },

    // Aggregation
    Child { wrapped: Box<Identified<Node>> },
    Group { children: Vec<Identified<Node>> },

    // Layout
    VStack { wrapped: Box<Identified<Node>> },
    HStack { wrapped: Box<Identified<Node>> },
    ZStack { wrapped: Box<Identified<Node>> },
    List { wrapped: Box<Identified<Node>> },

    // Wrapper
    Shape { shape: ShapeNode },
    Modified { wrapped: Box<Identified<Node>>, modifier: ModifierNode, }
}

impl Default for Node {
    fn default() -> Self {
        Node::Empty {}
    }
}
