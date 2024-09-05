use super::{IdPath, IdPathBuf};

/// A type that can be diffed in terms of id paths.
pub trait Diff: Sized {
    /// Appends the difference to "construct" this type from the given other one
    /// to the given difference.
    fn record_diff(&self, old: &Self, id_path: &IdPath, difference: &mut Difference<Self>);

    /// Computes the difference to "construct" this type from the given other one.
    fn diff(&self, old: &Self) -> Difference<Self> {
        let mut difference = Difference::new();
        self.record_diff(old, IdPath::root(), &mut difference);
        return difference;
    }
}

/// The difference between two values.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Difference<'a, T> {
    pub removed: Vec<(IdPathBuf, &'a T)>,
    pub changed: Vec<(IdPathBuf, &'a T)>,
    pub added: Vec<(IdPathBuf, &'a T)>,
}

impl<'a, T> Difference<'a, T> {
    pub fn new() -> Self {
        Self {
            removed: Vec::new(),
            changed: Vec::new(),
            added: Vec::new(),
        }
    }
}

impl<'a, T> Default for Difference<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}
