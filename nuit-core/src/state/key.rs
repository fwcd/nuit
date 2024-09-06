use serde::{Deserialize, Serialize};

use crate::IdPathBuf;

/// A key that uniquely identifies a value in a storage.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateKey {
    /// The id path of the view holding the state.
    id_path: IdPathBuf,
    /// The state index within the view.
    index: usize,
}

impl StateKey {
    pub fn new(id_path: impl Into<IdPathBuf>, index: impl Into<usize>) -> Self {
        Self { id_path: id_path.into(), index: index.into() }
    }
}
