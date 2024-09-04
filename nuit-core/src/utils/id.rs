use std::fmt;

use serde::{Serialize, Deserialize};

// TODO: Investigate if we can use a `Cow<str>` to avoid unnecessary clones.
// We'd likely have to fight with lifetimes a bit, since `Id` would likely
// require a lifetime parameter now.

/// An identifier for a view.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Id {
    Index(usize),
    String(String),
}

impl Id {
    pub const fn index(value: usize) -> Self {
        Self::Index(value)
    }

    pub const fn string(value: String) -> Self {
        Self::String(value)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Index(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, "{}", value),
        }
    }
}

impl From<usize> for Id {
    fn from(value: usize) -> Self {
        Self::Index(value)
    }
}

impl From<String> for Id {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Id {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}
