use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Id {
    Index { value: usize },
    String { value: String },
}

impl Id {
    pub fn index(value: usize) -> Self {
        Self::Index { value }
    }

    pub fn string(value: String) -> Self {
        Self::String { value }
    }
}

impl From<usize> for Id {
    fn from(value: usize) -> Self {
        Self::Index { value }
    }
}

impl From<String> for Id {
    fn from(value: String) -> Self {
        Self::String { value }
    }
}
