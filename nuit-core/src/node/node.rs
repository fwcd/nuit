use nuit_derive::Diff;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::{Alignment, Axis, HorizontalAlignment, Id, IdPath, IdPathBuf, Identified, VerticalAlignment};

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
    Slider { value: f64, lower_bound: f64, upper_bound: f64, step: Option<f64> },

    // Aggregation
    Child { wrapped: Box<Identified<Node>> },
    Group { children: Vec<Identified<Node>> },

    // Layout
    GeometryReader {},
    VStack { alignment: HorizontalAlignment, spacing: f64, wrapped: Box<Identified<Node>> },
    HStack { alignment: VerticalAlignment, spacing: f64, wrapped: Box<Identified<Node>> },
    ZStack { alignment: Alignment, spacing: f64, wrapped: Box<Identified<Node>> },
    List { wrapped: Box<Identified<Node>> },
    ScrollView { axes: Axis, show_indicators: bool, wrapped: Box<Identified<Node>> },
    Overlay { wrapped: Box<Identified<Node>>, alignment: Alignment, overlayed: Box<Identified<Node>> },

    // Navigation
    NavigationStack { path: Option<Vec<Value>>, wrapped: Box<Identified<Node>> },
    NavigationSplitView { sidebar: Box<Identified<Node>>, content: Box<Identified<Node>>, detail: Box<Identified<Node>> },
    NavigationLink { label: Box<Identified<Node>>, value: Value },
    NavigationDestination { wrapped: Box<Identified<Node>> },

    // Wrapper
    Shape { shape: ShapeNode },
    Gestured { wrapped: Box<Identified<Node>>, gesture: Identified<GestureNode>, },
    Modified { wrapped: Box<Identified<Node>>, modifier: ModifierNode, }
}

impl Node {
    #[must_use]
    pub fn children(&self) -> Vec<(IdPathBuf, &Self)> {
        self.children_from(IdPath::root())
    }

    #[must_use]
    pub fn children_from(&self, path: &IdPath) -> Vec<(IdPathBuf, &Self)> {
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
        Self::Empty {}
    }
}
