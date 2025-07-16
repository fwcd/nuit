use serde::{Serialize, Deserialize};

use crate::{Alignment, Angle, EdgeSet, Font, Frame, Insets, NavigationTitleDisplayMode, Style, UnitPoint, Vec2};

/// A rendered modifier.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ModifierNode {
    Padding { insets: Insets },
    Position { position: Vec2<f64> },
    Offset { delta: Vec2<f64> },
    Opacity { opacity: f64 },
    Frame { frame: Frame, alignment: Alignment },
    Fill { style: Style },
    Font { font: Font },
    ForegroundStyle { style: Style },
    Background { style: Style, safe_area_ignoring_edges: EdgeSet },
    ScaleEffect { factor: f64, anchor: UnitPoint },
    RotationEffect { angle: Angle, anchor: UnitPoint },
    Help { text: String },
    NavigationTitle { title: String },
    NavigationSubtitle { subtitle: String },
    NavigationTitleDisplayMode { display_mode: NavigationTitleDisplayMode },
    Border { style: Style, width: f64 },
    Shadow { color: Style, radius: f64, offset: Vec2<f64> },
    Blur { radius: f64 },
    CornerRadius { radius: f64 },
    Overlay { style: Style, alignment: Alignment },
    ZIndex { z_index: f64 },
    Hidden { is_hidden: bool },
    Disabled { is_disabled: bool },
    Grayscale { intensity: f64 },
    Brightness { amount: f64 },
    Contrast { amount: f64 },
    Saturation { amount: f64 },
    HueRotation { angle: Angle },
    Clipped { },
}
