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
    #[must_use]
    pub fn root() -> &'static Self {
        Self::ref_cast(&[])
    }

    #[must_use]
    pub const fn is_root(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn head(&self) -> Option<Id> {
        self.0.first().cloned()
    }

    #[must_use]
    pub fn tail(&self) -> &Self {
        Self::ref_cast(&self.0[1..])
    }

    #[must_use]
    pub fn child(&self, id: impl Into<Id>) -> IdPathBuf {
        self.to_owned().child(id)
    }

    #[must_use]
    pub fn join(&self, path: &Self) -> IdPathBuf {
        self.to_owned().join(path)
    }
}

impl IdPathBuf {
    #[must_use]
    pub const fn root() -> Self {
        Self(Vec::new())
    }

    #[must_use]
    pub fn child(&self, id: impl Into<Id>) -> Self {
        let mut components = self.0.clone();
        components.push(id.into());
        Self(components)
    }

    #[must_use]
    pub fn join(&self, path: &IdPath) -> Self {
        let mut components = self.0.clone();
        components.extend(path.0.iter().cloned());
        Self(components)
    }
}

impl ToOwned for IdPath {
    type Owned = IdPathBuf;

    fn to_owned(&self) -> IdPathBuf {
        IdPathBuf(self.0.to_vec())
    }
}

impl Default for IdPathBuf {
    fn default() -> Self {
        Self::root()
    }
}

impl From<Id> for IdPathBuf {
    fn from(id: Id) -> Self {
        Self(vec![id])
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
        self
    }
}

impl Borrow<IdPath> for IdPathBuf {
    fn borrow(&self) -> &IdPath {
        self
    }
}

impl From<&IdPath> for IdPathBuf {
    fn from(id_path: &IdPath) -> Self {
        id_path.to_owned()
    }
}
