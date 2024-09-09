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

    /// Creates a new color with the given RGB components and alpha 1.
    #[must_use]
    pub const fn with_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self::with_rgba(red, green, blue, 1.0)
    }

    /// Creates a new color with the given grayscale value and alpha 1.
    #[must_use]
    pub const fn with_grayscale(gray: f64) -> Self {
        Self::with_rgb(gray, gray, gray)
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
