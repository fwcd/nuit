use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdPath(Vec<usize>);

impl IdPath {
    pub fn root() -> Self {
        Self(Vec::new())
    }

    pub fn child(&self, i: usize) -> Self {
        let mut components = self.0.clone();
        components.push(i);
        Self(components)
    }
}
