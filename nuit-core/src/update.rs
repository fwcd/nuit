use serde::{Deserialize, Serialize};

use crate::Animation;

#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Update {
    animation: Option<Animation>,
}

impl Update {
    #[must_use]
    pub fn new(animation: Option<Animation>) -> Self {
        Self { animation }
    }

    #[must_use]
    pub fn with_animation(animation: Animation) -> Self {
        Self { animation: Some(animation) }
    }
}
