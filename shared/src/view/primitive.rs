/// A UI component tree composed of primitive components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primitive {
    Text(String),
    VStack(Box<Primitive>),
    HStack(Box<Primitive>),
    ZStack(Box<Primitive>),
    Group(Vec<Primitive>),
}
