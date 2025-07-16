use serde::{Deserialize, Serialize};

use super::Edge;

/// A set of edges in a 2D rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EdgeSet {
    raw_value: u8,
}

impl EdgeSet {
    pub const EMPTY: Self = Self::new();

    pub const TOP: Self = Self::from_raw_value(1 << (Edge::Top as u8));
    pub const BOTTOM: Self = Self::from_raw_value(1 << (Edge::Bottom as u8));
    pub const LEADING: Self = Self::from_raw_value(1 << (Edge::Leading as u8));
    pub const TRAILING: Self = Self::from_raw_value(1 << (Edge::Trailing as u8));

    // TODO: Figure out if we can construct these in a const context without hardcoding the bits
    pub const HORIZONTAL: Self = Self::from_raw_value(0b1100);
    pub const VERTICAL: Self = Self::from_raw_value(0b0011);
    pub const ALL: Self = Self::from_raw_value(0b1111);

    /// Creates a set containing no edges.
    #[must_use]
    pub const fn new() -> Self {
        Self { raw_value: 0 }
    }

    /// Creates a set from the given raw value.
    #[must_use]
    pub const fn from_raw_value(raw_value: u8) -> Self {
        Self { raw_value }
    }

    /// The raw value of this set.
    #[must_use]
    pub const fn raw_value(self) -> u8 {
        self.raw_value
    }

    /// Inserts the given edge into the set.
    pub fn insert(&mut self, edge: Edge) {
        self.raw_value |= 1 << u8::from(edge);
    }

    /// The union with the given set.
    #[must_use]
    pub fn union(self, rhs: impl Into<Self>) -> Self {
        Self { raw_value: self.raw_value | rhs.into().raw_value }
    }

    /// Whether the set contains the given value.
    #[must_use]
    pub fn contains(self, edge: Edge) -> bool {
        (self.raw_value & (1 << u8::from(edge))) != 0
    }

    /// Converts the edge set to a [`Vec`].
    #[must_use]
    pub fn to_vec(self) -> Vec<Edge> {
        self.into_iter().collect()
    }
}

impl Default for EdgeSet {
    fn default() -> Self {
        Self::new()
    }
}

impl From<u8> for EdgeSet {
    fn from(raw_value: u8) -> Self {
        Self { raw_value }
    }
}

impl From<Edge> for EdgeSet {
    fn from(edge: Edge) -> Self {
        Self::from(1 << u8::from(edge))
    }
}

impl<const N: usize> From<[Edge; N]> for EdgeSet {
    fn from(value: [Edge; N]) -> Self {
        Self::from_iter(value)
    }
}

impl FromIterator<Edge> for EdgeSet {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = Edge> {
        iter.into_iter().fold(Self::new(), Self::union)
    }
}

impl From<EdgeSet> for Vec<Edge> {
    fn from(set: EdgeSet) -> Self {
        set.to_vec()
    }
}

pub struct EdgeSetIterator {
    set: EdgeSet,
    i: u8,
}

impl Iterator for EdgeSetIterator {
    type Item = Edge;

    #[allow(clippy::cast_possible_truncation)]
    fn next(&mut self) -> Option<Edge> {
        while self.i < Edge::COUNT as u8 {
            let edge = Edge::try_from(self.i).unwrap();
            self.i += 1;
            if self.set.contains(edge) {
                return Some(edge);
            }
        }
        None
    }
}

impl IntoIterator for EdgeSet {
    type Item = Edge;
    type IntoIter = EdgeSetIterator;

    fn into_iter(self) -> EdgeSetIterator {
        EdgeSetIterator { set: self, i: 0 }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Edge, EdgeSet};

    #[test]
    fn empty() {
        assert_eq!(EdgeSet::new().to_vec(), vec![]);
        assert!(!EdgeSet::new().contains(Edge::Top));
        assert!(!EdgeSet::new().contains(Edge::Bottom));
        assert!(!EdgeSet::new().contains(Edge::Leading));
        assert!(!EdgeSet::new().contains(Edge::Trailing));
    }

    #[test]
    fn constants() {
        assert_eq!(EdgeSet::EMPTY, EdgeSet::new());
        assert_eq!(EdgeSet::TOP, Edge::Top.into());
        assert_eq!(EdgeSet::BOTTOM, Edge::Bottom.into());
        assert_eq!(EdgeSet::LEADING, Edge::Leading.into());
        assert_eq!(EdgeSet::TRAILING, Edge::Trailing.into());
        assert_eq!(EdgeSet::HORIZONTAL, Edge::HORIZONTAL.into());
        assert_eq!(EdgeSet::VERTICAL, Edge::VERTICAL.into());
        assert_eq!(EdgeSet::ALL, Edge::ALL.into());
    }

    #[test]
    fn conversions() {
        assert_eq!(EdgeSet::from(Edge::Leading).to_vec(), vec![Edge::Leading]);
        assert_eq!(EdgeSet::from(Edge::Trailing).to_vec(), vec![Edge::Trailing]);
        assert_eq!(EdgeSet::from([Edge::Leading, Edge::Trailing]).to_vec(), vec![Edge::Leading, Edge::Trailing]);

        assert!(EdgeSet::from(Edge::Leading).contains(Edge::Leading));
        assert!(!EdgeSet::from(Edge::Leading).contains(Edge::Trailing));
        assert!(!EdgeSet::from(Edge::Leading).contains(Edge::Top));
        assert!(!EdgeSet::from(Edge::Leading).contains(Edge::Bottom));
    }

    #[test]
    fn union() {
        assert_eq!(EdgeSet::new().union(Edge::Leading).to_vec(), vec![Edge::Leading]);
        assert_eq!(EdgeSet::new().union([Edge::Leading, Edge::Trailing]).to_vec(), vec![Edge::Leading, Edge::Trailing]);
        assert_eq!(EdgeSet::from([Edge::Trailing]).union([Edge::Leading, Edge::Top]).to_vec(), vec![Edge::Top, Edge::Leading, Edge::Trailing]);
    }
}
