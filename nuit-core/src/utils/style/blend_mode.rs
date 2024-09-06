use serde::{Deserialize, Serialize};

/// A compositing mode.
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum BlendMode {
    #[default]
    Normal,
    Darken,
    Multiply,
    ColorBurn,
    PlusDarker,
    Lighten,
    Screen,
    ColorDodge,
    PlusLighter,
    Overlay,
    SoftLight,
    HardLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
    SourceAtop,
    DestinationOver,
    DestinationOut,
}
