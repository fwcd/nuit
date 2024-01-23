use serde::{Serialize, Deserialize};

use crate::Id;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdPath(Vec<Id>);

impl IdPath {
    pub fn root() -> Self {
        Self(Vec::new())
    }

    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }

    pub fn head(&self) -> Option<Id> {
        self.0.first().cloned()
    }

    pub fn tail(&self) -> Self {
        Self((self.0[1..]).to_vec())
    }

    pub fn child(&self, id: impl Into<Id>) -> Self {
        let mut components = self.0.clone();
        components.push(id.into());
        Self(components)
    }
}
