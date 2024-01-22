use serde::{Serialize, Deserialize};

use crate::Id;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdPath(Vec<Id>);

impl IdPath {
    pub fn root() -> Self {
        Self(Vec::new())
    }

    pub fn child(&self, id: impl Into<Id>) -> Self {
        let mut components = self.0.clone();
        components.push(id.into());
        Self(components)
    }
}
