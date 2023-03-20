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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Id<T> {
    id_path: IdPath,
    value: T,
}

impl<T> Id<T> {
    pub fn new(id_path: IdPath, value: T) -> Self {
        Self { id_path, value }
    }
}
