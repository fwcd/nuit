use std::fmt;

use serde::{Serialize, Deserialize};

// TODO: Investigate if we can use a `Cow<str>` to avoid unnecessary clones.
// We'd likely have to fight with lifetimes a bit, since `Id` would likely
// require a lifetime parameter now.

/// An identifier for a view.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Id {
    Index(i64),
    String(String),
}

impl Id {
    pub fn index(value: impl Into<i64>) -> Self {
        Self::Index(value.into())
    }

    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
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

macro_rules! impl_id_from_integer {
    ($($tys:ty),*) => {
        $(impl From<$tys> for Id {
            fn from(value: $tys) -> Self {
                Self::Index(value as i64)
            }
        })*
    };
}

impl_id_from_integer!(
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize
);

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
