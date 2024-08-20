use std::{ops::Deref, borrow::Borrow};

use ref_cast::RefCast;
use serde::{Serialize, Deserialize};

use crate::Id;

/// A borrowed path of ids.
#[derive(Debug, Hash, PartialEq, Eq, RefCast)]
#[repr(transparent)]
pub struct IdPath([Id]);

/// An owned path of ids.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdPathBuf(Vec<Id>);

impl IdPath {
    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }

    pub fn head(&self) -> Option<Id> {
        self.0.first().cloned()
    }

    pub fn tail(&self) -> &Self {
        Self::ref_cast(&self.0[1..])
    }

    pub fn child(&self, id: impl Into<Id>) -> IdPathBuf {
        self.to_owned().child(id)
    }
}

impl IdPathBuf {
    pub fn root() -> Self {
        Self(Vec::new())
    }

    pub fn child(&self, id: impl Into<Id>) -> Self {
        let mut components = self.0.clone();
        components.push(id.into());
        Self(components)
    }
}

impl ToOwned for IdPath {
    type Owned = IdPathBuf;

    fn to_owned(&self) -> IdPathBuf {
        IdPathBuf(self.0.to_vec())
    }
}

impl Deref for IdPathBuf {
    type Target = IdPath;

    fn deref(&self) -> &IdPath {
        // Unfortunately, casting `&[Id]` to our (newtype-style) DST `IdPath`
        // cannot be done easily and safely using the language only, see
        // https://internals.rust-lang.org/t/brainstorming-newtypes-for-dsts-without-needing-unsafe/8515/7
        // We therefore use the ref-cell library providing a safe abstraction.
        IdPath::ref_cast(&self.0[..])
    }
}

impl AsRef<IdPath> for IdPathBuf {
    fn as_ref(&self) -> &IdPath {
        self.deref()
    }
}

impl Borrow<IdPath> for IdPathBuf {
    fn borrow(&self) -> &IdPath {
        self.deref()
    }
}
