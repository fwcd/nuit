use super::IdPathBuf;

/// A type that can be diffed in terms of id paths.
pub trait Diff: Sized {
    /// Computes the difference to "construct" this type from the given other one.
    fn diff(&self, old: &Self) -> Difference<Self>;
}

/// The difference between two values.
pub struct Difference<T> {
    pub removed: Vec<(IdPathBuf, T)>,
    pub changed: Vec<(IdPathBuf, T)>,
    pub added: Vec<(IdPathBuf, T)>,
}
