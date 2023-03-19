use serde::{Serialize, Deserialize};

/// A UI component tree composed of primitive components.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Primitive {
    Text(String),
    VStack(Box<Primitive>),
    HStack(Box<Primitive>),
    ZStack(Box<Primitive>),
    Group(Vec<Primitive>),
}
