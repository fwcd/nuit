use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
}

impl Color {
    pub const BLACK: Self = Self::grayscale(0.0);
    pub const GRAY: Self = Self::grayscale(0.5);
    pub const WHITE: Self = Self::grayscale(1.0);

    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);

    pub const CYAN: Self = Self::rgb(0.0, 1.0, 1.0);
    pub const MAGENTA: Self = Self::rgb(1.0, 0.0, 1.0);
    pub const YELLOW: Self = Self::rgb(1.0, 1.0, 0.0);

    pub const fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Self { red, green, blue, alpha }
    }

    pub const fn rgb(red: f64, green: f64, blue: f64) -> Self {
        Self::new(red, green, blue, 1.0)
    }

    pub const fn grayscale(gray: f64) -> Self {
        Self::new(gray, gray, gray, 1.0)
    }
}
