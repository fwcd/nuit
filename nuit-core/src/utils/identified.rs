use serde::{Serialize, Deserialize};

use crate::IdPath;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identified<T> {
    id_path: IdPath,
    value: T,
}

impl<T> Identified<T> {
    pub fn new(id_path: IdPath, value: T) -> Self {
        Self { id_path, value }
    }
}
