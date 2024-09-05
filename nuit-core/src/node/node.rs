use nuit_derive::Diff;
use serde::{Serialize, Deserialize};

use crate::{Alignment, Id, IdPath, IdPathBuf, Identified};

use super::{GestureNode, ModifierNode, ShapeNode};

/// A rendered UI component tree.
#[derive(Debug, Clone, PartialEq, Diff, Serialize, Deserialize)]
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
    VStack { spacing: f64, wrapped: Box<Identified<Node>> },
    HStack { spacing: f64, wrapped: Box<Identified<Node>> },
    ZStack { spacing: f64, wrapped: Box<Identified<Node>> },
    List { wrapped: Box<Identified<Node>> },
    Overlay { wrapped: Box<Identified<Node>>, alignment: Alignment, overlayed: Box<Identified<Node>> },

    // Wrapper
    Shape { shape: ShapeNode },
    Gesture { wrapped: Box<Identified<Node>>, gesture: GestureNode, },
    Modified { wrapped: Box<Identified<Node>>, modifier: ModifierNode, }
}

impl Node {
    pub fn children(&self) -> Vec<(IdPathBuf, &Node)> {
        self.children_from(IdPath::root())
    }

    pub fn children_from(&self, path: &IdPath) -> Vec<(IdPathBuf, &Node)> {
        match self {
            Self::Group { children } => children.iter()
                .flat_map(|c| c.value().children_from(&path.child(c.id().clone())).into_iter())
                .collect(),
            _ => vec![(path.to_owned(), self)]
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node::Empty {}
    }
}
