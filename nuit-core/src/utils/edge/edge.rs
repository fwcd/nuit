use serde::{Deserialize, Serialize};

/// An edge of a 2D rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Edge {
    Top = 0,
    Bottom = 1,
    Leading = 2,
    Trailing = 3,
}

impl Edge {
    pub const COUNT: usize = 4;
    pub const HORIZONTAL: [Self; 2] = [Self::Leading, Self::Trailing];
    pub const VERTICAL: [Self; 2] = [Self::Top, Self::Bottom];
    pub const ALL: [Self; Self::COUNT] = [Self::Top, Self::Bottom, Self::Leading, Self::Trailing];
}

impl TryFrom<u8> for Edge {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Top),
            1 => Ok(Self::Bottom),
            2 => Ok(Self::Leading),
            3 => Ok(Self::Trailing),
            _ => Err(format!("Value is not a valid edge: {value}")),
        }
    }
}

impl From<Edge> for u8 {
    fn from(edge: Edge) -> Self {
        edge as Self
    }
}
