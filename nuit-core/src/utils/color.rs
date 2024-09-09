use nuit_derive::ApproxEq;
use serde::{Serialize, Deserialize};

use super::Angle;

/// An RGBA color.
#[derive(Debug, Clone, Copy, PartialEq, ApproxEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
}

impl Color {
    pub const BLACK: Self = Self::with_grayscale(0.0);
    pub const GRAY: Self = Self::with_grayscale(0.5);
    pub const WHITE: Self = Self::with_grayscale(1.0);

    pub const RED: Self = Self::with_rgb(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::with_rgb(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::with_rgb(0.0, 0.0, 1.0);

    pub const CYAN: Self = Self::with_rgb(0.0, 1.0, 1.0);
    pub const MAGENTA: Self = Self::with_rgb(1.0, 0.0, 1.0);
    pub const YELLOW: Self = Self::with_rgb(1.0, 1.0, 0.0);

    /// Creates a new color with the given RGBA components.
    #[must_use]
    pub const fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Self { red, green, blue, alpha }
    }

    /// Creates a new color with the given RGBA components.
    #[must_use]
    pub const fn with_rgba(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Self::new(red, green, blue, alpha)
    }

    /// Creates a new color with the given RGB components and no transparency.
    #[must_use]
    pub const fn with_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self::with_rgba(red, green, blue, 1.0)
    }

    /// Creates a new color with the given grayscale value and no transparency.
    #[must_use]
    pub const fn with_grayscale(gray: f64) -> Self {
        Self::with_rgb(gray, gray, gray)
    }

    /// Creates a new color with the given RGBA u8 components as no transparency.
    #[must_use]
    pub const fn with_rgba_u8(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::with_rgba(
            red as f64 / 255.0,
            green as f64 / 255.0,
            blue as f64 / 255.0,
            alpha as f64 / 255.0,
        )
    }

    /// Creates a new color with the given RGB u8 components as no transparency.
    #[must_use]
    pub const fn with_rgb_u8(red: u8, green: u8, blue: u8) -> Self {
        Self::with_rgba_u8(red, green, blue, 255)
    }

    /// Creates a new color from the given RGBA u32 value.
    #[must_use]
    pub const fn from_rgba_u32(rgba: u32) -> Self {
        Self::with_rgba_u8(
            ((rgba >> 24) & 0xFF) as u8,
            ((rgba >> 16) & 0xFF) as u8,
            ((rgba >> 8) & 0xFF) as u8,
            (rgba & 0xFF) as u8,
        )
    }

    /// Creates a new color from the given RGB u32 value.
    #[must_use]
    pub const fn from_rgb_u32(rgba: u32) -> Self {
        Self::with_rgb_u8(
            ((rgba >> 16) & 0xFF) as u8,
            ((rgba >> 8) & 0xFF) as u8,
            (rgba & 0xFF) as u8,
        )
    }

    /// Creates a new color from the given ARGB u32 value.
    #[must_use]
    pub const fn from_argb_u32(argb: u32) -> Self {
        Self::with_rgba_u8(
            ((argb >> 16) & 0xFF) as u8,
            ((argb >> 8) & 0xFF) as u8,
            (argb & 0xFF) as u8,
            ((argb >> 24) & 0xFF) as u8,
        )
    }

    /// A random RGBA color.
    #[cfg(feature = "rand")]
    #[must_use]
    pub fn random_rgba() -> Self {
        Self::new(rand::random(), rand::random(), rand::random(), rand::random())
    }

    /// A random RGB color with alpha 1.
    #[cfg(feature = "rand")]
    #[must_use]
    pub fn random_rgb() -> Self {
        Self::with_rgb(rand::random(), rand::random(), rand::random())
    }

    /// A random grayscale color with alpha 1.
    #[cfg(feature = "rand")]
    #[must_use]
    pub fn random_grayscale() -> Self {
        Self::with_grayscale(rand::random())
    }

    /// Creates a new color with the given HSV value and alpha.
    #[must_use]
    pub fn with_hsva(hue: Angle, saturation: f64, value: f64, alpha: f64) -> Self {
        let hue_degrees = hue.degrees().rem_euclid(360.0);

        let c = value * saturation;
        let x = c * (1.0 - ((hue_degrees / 60.0) % 2.0 - 1.0).abs());
        let m = value - c;
        
        let (r_prime, g_prime, b_prime) = match hue_degrees {
            0.0..=60.0 => (c, x, 0.0),
            60.0..=120.0 => (x, c, 0.0),
            120.0..=180.0 => (0.0, c, x),
            180.0..=240.0 => (0.0, x, c),
            240.0..=300.0 => (x, 0.0, c),
            300.0..=360.0 => (c, 0.0, x),
            _ => unreachable!("Hue is taken modulo 360 degrees"),
        };

        let red = r_prime + m;
        let green = g_prime + m;
        let blue = b_prime + m;

        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Creates a new color with the given HSV value.
    #[must_use]
    pub fn with_hsv(hue: Angle, saturation: f64, value: f64) -> Self {
        Self::with_hsva(hue, saturation, value, 1.0)
    }

    /// The inverted color with the same alpha.
    #[must_use]
    pub fn invert_rgb(self) -> Self {
        Self::with_rgba(1.0 - self.red, 1.0 - self.green, 1.0 - self.blue, self.alpha)
    }

    /// The red component.
    #[must_use]
    pub const fn red(self) -> f64 {
        self.red
    }

    /// The green component.
    #[must_use]
    pub const fn green(self) -> f64 {
        self.green
    }

    /// The blue component.
    #[must_use]
    pub const fn blue(self) -> f64 {
        self.blue
    }

    /// The alpha component.
    #[must_use]
    pub const fn alpha(self) -> f64 {
        self.alpha
    }

    /// The red component as a byte.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub const fn red_u8(self) -> u8 {
        (self.red * 255.0) as u8
    }

    /// The green component as a byte.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub const fn green_u8(self) -> u8 {
        (self.green * 255.0) as u8
    }

    /// The blue component as a byte.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub const fn blue_u8(self) -> u8 {
        (self.blue * 255.0) as u8
    }

    /// The alpha component as a byte.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub const fn alpha_u8(self) -> u8 {
        (self.alpha * 255.0) as u8
    }

    /// The color as u32 RGB value.
    #[must_use]
    pub const fn to_rgb_u32(self) -> u32 {
        ((self.red_u8() as u32) << 16) | ((self.green_u8() as u32) << 8) | self.blue_u8() as u32
    }

    /// The color as u32 RGBA value.
    #[must_use]
    pub const fn to_rgba_u32(self) -> u32 {
        ((self.red_u8() as u32) << 24) | ((self.green_u8() as u32) << 16) | (self.blue_u8() as u32) << 8 | self.alpha_u8() as u32
    }

    /// The color as u32 ARGB value.
    #[must_use]
    pub const fn to_argb_u32(self) -> u32 {
        ((self.alpha_u8() as u32) << 24) | ((self.red_u8() as u32) << 16) | (self.green_u8() as u32) << 8 | self.blue_u8() as u32
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_approx_eq, Angle, Color};

    #[test]
    fn hsv_to_rgb() {
        assert_approx_eq!(Color::with_hsv(Angle::ZERO, 1.0, 1.0), Color::RED);
        assert_approx_eq!(Color::with_hsv(Angle::HALF, 1.0, 1.0), Color::CYAN);
        assert_approx_eq!(Color::with_hsv(Angle::HALF, 0.0, 1.0), Color::WHITE);
        assert_approx_eq!(Color::with_hsv(Angle::HALF, 0.0, 0.0), Color::BLACK);

        // Hue should be taken modulo the full angle
        assert_approx_eq!(
            Color::with_hsv(Angle::QUARTER, 1.0, 1.0),
            Color::with_hsv(Angle::QUARTER + Angle::FULL, 1.0, 1.0)
        );
        assert_approx_eq!(
            Color::with_hsv(Angle::QUARTER, 1.0, 1.0),
            Color::with_hsv(Angle::QUARTER - Angle::FULL, 1.0, 1.0)
        );
    }

    #[test]
    fn invert() {
        assert_approx_eq!(Color::BLACK.invert_rgb(), Color::WHITE);
        assert_approx_eq!(Color::CYAN.invert_rgb(), Color::RED);
        assert_approx_eq!(Color::RED.invert_rgb(), Color::CYAN);
        assert_approx_eq!(Color::YELLOW.invert_rgb(), Color::BLUE);
    }
}
