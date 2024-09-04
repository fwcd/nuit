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
    /// Creates a new `Identified<T>` identifying the given value with the given id.
    pub fn new(id: &Id, value: T) -> Self {
        Self { id: id.clone(), value }
    }

    /// The id.
    pub fn id(&self) -> &Id {
        &self.id
    }

    /// The wrapped value.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Consumes and returns the value.
    pub fn into_value(self) -> T {
        self.value
    }

    /// Converts from an `&Identified<T>` to an `Identified<&T>`. Note that
    /// this involves cloning the id, which may be undesirable if expensive.
    pub fn as_ref(&self) -> Identified<&T> {
        Identified { id: self.id.clone(), value: &self.value }
    }

    /// Maps the given function over the value.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Identified<U> {
        Identified { id: self.id, value: f(self.value) }
    }

    /// Maps the given function over the value, providing the id.
    pub fn map_with_id<U>(self, f: impl FnOnce(&Id, T) -> U) -> Identified<U> {
        let value = f(&self.id, self.value);
        Identified { id: self.id, value }
    }
}

/// An extension trait for conveniently creating `Identified` values.
pub trait IdentifyExt where Self: Sized {
    fn identify(self, id: impl Into<Id>) -> Identified<Self>;
}

impl<T> IdentifyExt for T {
    fn identify(self, id: impl Into<Id>) -> Identified<Self> {
        Identified { id: id.into(), value: self }
    }
}
