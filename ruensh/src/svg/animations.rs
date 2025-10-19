//! Animation system for SVG-inspired components

use super::shapes::Point;

/// Animation types
#[derive(Debug, Clone)]
pub enum Animation {
    FadeIn {
        duration_ms: u64,
        easing: Easing,
    },
    FadeOut {
        duration_ms: u64,
        easing: Easing,
    },
    Slide {
        from: Point,
        to: Point,
        duration_ms: u64,
        easing: Easing,
    },
    Scale {
        from: f32,
        to: f32,
        duration_ms: u64,
        easing: Easing,
    },
    Rotate {
        degrees: f32,
        duration_ms: u64,
        easing: Easing,
    },
    Pulse {
        scale_factor: f32,
        frequency_hz: f32,
    },
}

/// Easing functions for smooth animations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
}

impl Easing {
    /// Apply easing function to a progress value (0.0 to 1.0)
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => t * (2.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Easing::Bounce => {
                if t < 1.0 / 2.75 {
                    7.5625 * t * t
                } else if t < 2.0 / 2.75 {
                    let t = t - 1.5 / 2.75;
                    7.5625 * t * t + 0.75
                } else if t < 2.5 / 2.75 {
                    let t = t - 2.25 / 2.75;
                    7.5625 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / 2.75;
                    7.5625 * t * t + 0.984375
                }
            }
            Easing::Elastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let p = 0.3;
                    let s = p / 4.0;
                    let t = t - 1.0;
                    -(2.0_f32.powf(10.0 * t) * ((t - s) * (2.0 * std::f32::consts::PI) / p).sin())
                }
            }
        }
    }
}

/// Animation state tracker
pub struct AnimationState {
    start_time_ms: u64,
    duration_ms: u64,
    easing: Easing,
}

impl AnimationState {
    pub fn new(duration_ms: u64, easing: Easing) -> Self {
        Self {
            start_time_ms: 0,
            duration_ms,
            easing,
        }
    }

    /// Update animation and return progress (0.0 to 1.0)
    pub fn update(&mut self, current_time_ms: u64) -> f32 {
        if self.start_time_ms == 0 {
            self.start_time_ms = current_time_ms;
        }

        let elapsed = current_time_ms.saturating_sub(self.start_time_ms);
        let progress = (elapsed as f32 / self.duration_ms as f32).min(1.0);
        self.easing.apply(progress)
    }

    /// Check if animation is complete
    pub fn is_complete(&self, current_time_ms: u64) -> bool {
        if self.start_time_ms == 0 {
            return false;
        }
        current_time_ms.saturating_sub(self.start_time_ms) >= self.duration_ms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_linear() {
        let easing = Easing::Linear;
        assert_eq!(easing.apply(0.0), 0.0);
        assert_eq!(easing.apply(0.5), 0.5);
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_in() {
        let easing = Easing::EaseIn;
        assert_eq!(easing.apply(0.0), 0.0);
        assert!(easing.apply(0.5) < 0.5);
        assert_eq!(easing.apply(1.0), 1.0);
    }
}
