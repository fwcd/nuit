use super::{IdPath, IdPathBuf};

/// A type that can be diffed in terms of id paths.
pub trait Diff: Sized {
    /// Appends the difference to "construct" this type from the given other one
    /// to the given difference.
    fn record_diff<'a>(&'a self, old: &'a Self, id_path: &IdPath, difference: &mut Difference<&'a Self>);

    /// Computes the difference to "construct" this type from the given other one.
    fn diff<'a>(&'a self, old: &'a Self) -> Difference<&'a Self> {
        let mut difference = Difference::new();
        self.record_diff(old, IdPath::root(), &mut difference);
        difference
    }
}

/// The difference between two values.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Difference<T> {
    pub removed: Vec<(IdPathBuf, T)>,
    pub changed: Vec<(IdPathBuf, T, T)>,
    pub added: Vec<(IdPathBuf, T)>,
}

impl<T> Difference<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            removed: Vec::new(),
            changed: Vec::new(),
            added: Vec::new(),
        }
    }

    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Difference<U> {
        Difference {
            removed: self.removed.into_iter().map(|(p, x)| (p, f(x))).collect(),
            changed: self.changed.into_iter().map(|(p, x, y)| (p, f(x), f(y))).collect(),
            added: self.added.into_iter().map(|(p, x)| (p, f(x))).collect(),
        }
    }
}

impl<T> Default for Difference<T> {
    fn default() -> Self {
        Self::new()
    }
}
