use serde::{Serialize, Deserialize};

/// An RGBA color.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

    pub const fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Self { red, green, blue, alpha }
    }

    pub const fn with_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self::new(red, green, blue, 1.0)
    }

    pub const fn with_grayscale(gray: f64) -> Self {
        Self::new(gray, gray, gray, 1.0)
    }

    pub fn with_hsva(hue: f64, saturation: f64, value: f64, alpha: f64) -> Self {
        let c = value * saturation;
        let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
        let m = value - c;
        
        let (r_prime, g_prime, b_prime) = match hue {
            0.0..=60.0 => (c, x, 0.0),
            60.0..=120.0 => (x, c, 0.0),
            120.0..=180.0 => (0.0, c, x),
            180.0..=240.0 => (0.0, x, c),
            240.0..=300.0 => (x, 0.0, c),
            300.0..=360.0 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0), // In case the hue is outside [0, 360], which shouldn't happen
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

    pub fn with_hsv(hue: f64, saturation: f64, value: f64) -> Self {
        Self::with_hsva(hue, saturation, value, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn hsv_to_rgb() {
        assert_eq!(Color::with_hsv(0.0, 1.0, 1.0), Color::RED);
        assert_eq!(Color::with_hsv(180.0, 1.0, 1.0), Color::CYAN);
        assert_eq!(Color::with_hsv(180.0, 0.0, 1.0), Color::WHITE);
        assert_eq!(Color::with_hsv(180.0, 0.0, 0.0), Color::BLACK);
    }
}
