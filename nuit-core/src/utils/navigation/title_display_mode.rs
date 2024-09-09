use serde::{Deserialize, Serialize};

/// A display style for a navigation bar title.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NavigationTitleDisplayMode {
    #[default]
    Automatic,
    Inline,
    Large,
}
