use serde::{Serialize, Deserialize};

use crate::{IdPath, IdPathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identified<T> {
    id_path: IdPathBuf,
    value: T,
}

impl<T> Identified<T> {
    pub fn root(value: T) -> Self {
        Self { id_path: IdPathBuf::root(), value }
    }

    pub fn new(id_path: &IdPath, value: T) -> Self {
        Self { id_path: id_path.to_owned(), value }
    }
}
