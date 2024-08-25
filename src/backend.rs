use std::str::FromStr;

/// A UI backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Adwaita,
    SwiftUI,
}

impl FromStr for Backend {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "adwaita" => Ok(Self::Adwaita),
            "swiftui" => Ok(Self::SwiftUI),
            _ => Err(format!("Unsupported backend: {s}")),
        }
    }
}
