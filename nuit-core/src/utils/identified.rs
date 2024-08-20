use serde::{Serialize, Deserialize};

use crate::Id;

/// A value with an id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identified<T> {
    id: Id,
    value: T,
}

impl<T> Identified<T> {
    pub fn new(id: &Id, value: T) -> Self {
        Self { id: id.clone(), value }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

pub trait IdentifyExt where Self: Sized {
    fn identify(self, id: impl Into<Id>) -> Identified<Self>;
}

impl<T> IdentifyExt for T {
    fn identify(self, id: impl Into<Id>) -> Identified<Self> {
        Identified { id: id.into(), value: self }
    }
}
