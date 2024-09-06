use std::{fmt, time::Duration};

use serde::{Deserialize, Serialize};

/// An animation to use for smooth view transitions.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Animation {
    Default {},
    Curve { curve: AnimationCurve, duration_seconds: Option<f64> },
    // TODO: Springs
}

/// The curve of a simple animation.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum AnimationCurve {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl Animation {
    pub const LINEAR: Self = Self::Curve { curve: AnimationCurve::Linear, duration_seconds: None };
    pub const EASE_IN: Self = Self::Curve { curve: AnimationCurve::EaseIn, duration_seconds: None };
    pub const EASE_OUT: Self = Self::Curve { curve: AnimationCurve::EaseOut, duration_seconds: None };
    pub const EASE_IN_OUT: Self = Self::Curve { curve: AnimationCurve::EaseInOut, duration_seconds: None };

    /// Creates an animation with the given curve and the given duration (or default if none).
    pub fn curve(curve: AnimationCurve, duration: Option<Duration>) -> Self {
        Self::Curve { curve, duration_seconds: duration.map(|d| d.as_secs_f64()) }
    }

    /// Creates an animation with a linear curve and the given duration (or default if none).
    pub fn linear(duration: Option<Duration>) -> Self {
        Self::curve(AnimationCurve::Linear, duration)
    }

    /// Creates an animation with an ease-in curve and the given duration (or default if none).
    pub fn ease_in(duration: Option<Duration>) -> Self {
        Self::curve(AnimationCurve::EaseIn, duration)
    }

    /// Creates an animation with an ease-out curve and the given duration (or default if none).
    pub fn ease_out(duration: Option<Duration>) -> Self {
        Self::curve(AnimationCurve::EaseOut, duration)
    }

    /// Creates an animation with an ease-in/out curve and the given duration (or default if none).
    pub fn ease_in_out(duration: Option<Duration>) -> Self {
        Self::curve(AnimationCurve::EaseInOut, duration)
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self::Default {}
    }
}

impl fmt::Display for Animation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default {} => write!(f, "Default")?,
            Self::Curve { curve, duration_seconds } => {
                write!(f, "{}", curve)?;
                if let Some(duration_seconds) = duration_seconds {
                    write!(f, " ({}s)", duration_seconds)?;
                }
            },
        }
        Ok(())
    }
}

impl fmt::Display for AnimationCurve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnimationCurve::Linear => write!(f, "Linear"),
            AnimationCurve::EaseIn => write!(f, "Ease in"),
            AnimationCurve::EaseOut => write!(f, "Ease out"),
            AnimationCurve::EaseInOut => write!(f, "Ease in/out"),
        }
    }
}
