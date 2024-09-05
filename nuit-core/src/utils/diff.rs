use super::IdPathBuf;

/// A type that can be diffed in terms of id paths.
pub trait Diff: Sized {
    /// Appends the difference to "construct" this type from the given other one
    /// to the given difference.
    fn record_diff(&self, old: &Self, difference: &mut Difference<Self>);

    /// Computes the difference to "construct" this type from the given other one.
    fn diff(&self, old: &Self) -> Difference<Self> {
        let mut difference = Difference::new();
        self.record_diff(old, &mut difference);
        return difference;
    }
}

/// The difference between two values.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Difference<T> {
    pub removed: Vec<(IdPathBuf, T)>,
    pub changed: Vec<(IdPathBuf, T)>,
    pub added: Vec<(IdPathBuf, T)>,
}

impl<T> Difference<T> {
    pub fn new() -> Self {
        Self {
            removed: Vec::new(),
            changed: Vec::new(),
            added: Vec::new(),
        }
    }
}

impl<T> Default for Difference<T> {
    fn default() -> Self {
        Self::new()
    }
}
