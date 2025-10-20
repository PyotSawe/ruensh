//! Transition system with keyframes for reactive components
//!
//! This module provides a comprehensive transition system that allows components
//! to smoothly animate between states using keyframe-based animations.

use super::animations::Easing;
use ratatui::style::Color;
use std::time::{Duration, Instant};

/// Keyframe for animation timeline
#[derive(Debug, Clone)]
pub struct Keyframe<T> {
    /// Time offset (0.0 to 1.0 of total duration)
    pub offset: f32,
    /// Value at this keyframe
    pub value: T,
    /// Easing function to apply from this keyframe to the next
    pub easing: Easing,
}

impl<T> Keyframe<T> {
    pub fn new(offset: f32, value: T) -> Self {
        Self {
            offset: offset.clamp(0.0, 1.0),
            value,
            easing: Easing::EaseInOut,
        }
    }

    pub fn with_easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }
}

/// Interpolatable trait for values that can be animated
pub trait Interpolate: Clone {
    fn lerp(&self, other: &Self, t: f32) -> Self;
}

impl Interpolate for f32 {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

impl Interpolate for u16 {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        (*self as f32 + ((*other as f32) - (*self as f32)) * t) as u16
    }
}

impl Interpolate for i16 {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        (*self as f32 + ((*other as f32) - (*self as f32)) * t) as i16
    }
}

impl Interpolate for Color {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        match (self, other) {
            (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
                let r = (*r1 as f32 + ((*r2 as f32) - (*r1 as f32)) * t) as u8;
                let g = (*g1 as f32 + ((*g2 as f32) - (*g1 as f32)) * t) as u8;
                let b = (*b1 as f32 + ((*b2 as f32) - (*b1 as f32)) * t) as u8;
                Color::Rgb(r, g, b)
            }
            _ => self.clone(),
        }
    }
}

impl Interpolate for (u16, u16) {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        (self.0.lerp(&other.0, t), self.1.lerp(&other.1, t))
    }
}

/// Transition state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransitionState {
    Idle,
    Running,
    Complete,
    Paused,
}

/// Keyframe-based transition animator
pub struct Transition<T: Interpolate> {
    keyframes: Vec<Keyframe<T>>,
    duration: Duration,
    start_time: Option<Instant>,
    state: TransitionState,
    pub current_value: Option<T>,
    loop_count: Option<usize>,
    current_loop: usize,
    reverse_on_complete: bool,
}

impl<T: Interpolate> Transition<T> {
    /// Create a new transition with keyframes
    pub fn new(duration: Duration, keyframes: Vec<Keyframe<T>>) -> Self {
        Self {
            keyframes,
            duration,
            start_time: None,
            state: TransitionState::Idle,
            current_value: None,
            loop_count: None,
            current_loop: 0,
            reverse_on_complete: false,
        }
    }

    /// Create a simple two-state transition
    pub fn from_to(duration: Duration, from: T, to: T, easing: Easing) -> Self {
        Self::new(
            duration,
            vec![
                Keyframe::new(0.0, from).with_easing(easing),
                Keyframe::new(1.0, to),
            ],
        )
    }

    /// Set loop count (None = infinite)
    pub fn with_loop(mut self, count: Option<usize>) -> Self {
        self.loop_count = count;
        self
    }

    /// Enable reverse playback on complete
    pub fn with_reverse(mut self, reverse: bool) -> Self {
        self.reverse_on_complete = reverse;
        self
    }

    /// Start the transition
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.state = TransitionState::Running;
        self.current_loop = 0;
    }

    /// Pause the transition
    pub fn pause(&mut self) {
        if self.state == TransitionState::Running {
            self.state = TransitionState::Paused;
        }
    }

    /// Resume the transition
    pub fn resume(&mut self) {
        if self.state == TransitionState::Paused {
            self.state = TransitionState::Running;
        }
    }

    /// Reset the transition
    pub fn reset(&mut self) {
        self.start_time = None;
        self.state = TransitionState::Idle;
        self.current_value = None;
        self.current_loop = 0;
    }

    /// Get current state
    pub fn state(&self) -> TransitionState {
        self.state
    }

    /// Check if transition is running
    pub fn is_running(&self) -> bool {
        self.state == TransitionState::Running
    }

    /// Update transition and get current value
    pub fn update(&mut self) -> Option<&T> {
        if self.state != TransitionState::Running {
            return self.current_value.as_ref();
        }

        let start = self.start_time?;
        let elapsed = start.elapsed();

        // Calculate progress (0.0 to 1.0)
        let mut progress = elapsed.as_secs_f32() / self.duration.as_secs_f32();

        // Handle looping
        if progress >= 1.0 {
            if let Some(max_loops) = self.loop_count {
                if self.current_loop >= max_loops - 1 {
                    self.state = TransitionState::Complete;
                    progress = 1.0;
                } else {
                    self.current_loop += 1;
                    self.start_time = Some(Instant::now());
                    progress = 0.0;
                }
            } else {
                // Infinite loop
                self.start_time = Some(Instant::now());
                progress = 0.0;
            }
        }

        // Reverse if enabled
        if self.reverse_on_complete && self.current_loop % 2 == 1 {
            progress = 1.0 - progress;
        }

        // Find keyframes to interpolate between
        self.current_value = Some(self.interpolate_at(progress));
        self.current_value.as_ref()
    }

    /// Interpolate value at given progress
    fn interpolate_at(&self, progress: f32) -> T {
        if self.keyframes.is_empty() {
            panic!("Transition must have at least one keyframe");
        }

        if self.keyframes.len() == 1 {
            return self.keyframes[0].value.clone();
        }

        // Find surrounding keyframes
        let mut start_idx = 0;
        let mut end_idx = 1;

        for (i, keyframe) in self.keyframes.iter().enumerate() {
            if keyframe.offset <= progress {
                start_idx = i;
            }
            if keyframe.offset >= progress {
                end_idx = i;
                break;
            }
        }

        if start_idx == end_idx {
            return self.keyframes[start_idx].value.clone();
        }

        let start_kf = &self.keyframes[start_idx];
        let end_kf = &self.keyframes[end_idx];

        // Calculate local progress between keyframes
        let segment_duration = end_kf.offset - start_kf.offset;
        let local_progress = if segment_duration > 0.0 {
            (progress - start_kf.offset) / segment_duration
        } else {
            0.0
        };

        // Apply easing
        let eased_progress = start_kf.easing.apply(local_progress);

        // Interpolate
        start_kf.value.lerp(&end_kf.value, eased_progress)
    }

    /// Get value at specific progress (0.0 to 1.0)
    pub fn value_at(&self, progress: f32) -> T {
        self.interpolate_at(progress.clamp(0.0, 1.0))
    }
}

/// Pre-defined transition presets
pub struct TransitionPresets;

impl TransitionPresets {
    /// Fade in effect
    pub fn fade_in(duration: Duration) -> Transition<f32> {
        Transition::from_to(duration, 0.0, 1.0, Easing::EaseOut)
    }

    /// Fade out effect
    pub fn fade_out(duration: Duration) -> Transition<f32> {
        Transition::from_to(duration, 1.0, 0.0, Easing::EaseIn)
    }

    /// Slide from left
    pub fn slide_from_left(duration: Duration, distance: u16) -> Transition<i16> {
        Transition::from_to(duration, 0, distance as i16, Easing::EaseInOut)
    }

    /// Bounce scale effect
    pub fn bounce_scale(duration: Duration) -> Transition<f32> {
        Transition::new(
            duration,
            vec![
                Keyframe::new(0.0, 0.0).with_easing(Easing::EaseOut),
                Keyframe::new(0.6, 1.2).with_easing(Easing::Bounce),
                Keyframe::new(1.0, 1.0),
            ],
        )
    }

    /// Elastic entrance
    pub fn elastic_entrance(duration: Duration) -> Transition<f32> {
        Transition::from_to(duration, 0.0, 1.0, Easing::Elastic)
    }

    /// Pulse effect (infinite loop)
    pub fn pulse(duration: Duration, min: f32, max: f32) -> Transition<f32> {
        Transition::new(
            duration,
            vec![
                Keyframe::new(0.0, min).with_easing(Easing::EaseInOut),
                Keyframe::new(0.5, max).with_easing(Easing::EaseInOut),
                Keyframe::new(1.0, min),
            ],
        )
        .with_loop(None) // Infinite
    }

    /// Color shift effect
    pub fn color_shift(duration: Duration, from: Color, to: Color) -> Transition<Color> {
        Transition::from_to(duration, from, to, Easing::EaseInOut)
    }

    /// Rainbow cycle (through spectrum)
    pub fn rainbow_cycle(duration: Duration) -> Transition<Color> {
        Transition::new(
            duration,
            vec![
                Keyframe::new(0.0, Color::Rgb(255, 0, 0)).with_easing(Easing::Linear),
                Keyframe::new(0.16, Color::Rgb(255, 127, 0)).with_easing(Easing::Linear),
                Keyframe::new(0.33, Color::Rgb(255, 255, 0)).with_easing(Easing::Linear),
                Keyframe::new(0.5, Color::Rgb(0, 255, 0)).with_easing(Easing::Linear),
                Keyframe::new(0.66, Color::Rgb(0, 0, 255)).with_easing(Easing::Linear),
                Keyframe::new(0.83, Color::Rgb(75, 0, 130)).with_easing(Easing::Linear),
                Keyframe::new(1.0, Color::Rgb(148, 0, 211)),
            ],
        )
        .with_loop(None)
    }

    /// Shake effect (horizontal)
    pub fn shake_horizontal(duration: Duration, amplitude: u16) -> Transition<i16> {
        Transition::new(
            duration,
            vec![
                Keyframe::new(0.0, 0).with_easing(Easing::Linear),
                Keyframe::new(0.1, amplitude as i16).with_easing(Easing::Linear),
                Keyframe::new(0.2, -(amplitude as i16)).with_easing(Easing::Linear),
                Keyframe::new(0.3, amplitude as i16).with_easing(Easing::Linear),
                Keyframe::new(0.4, -(amplitude as i16)).with_easing(Easing::Linear),
                Keyframe::new(0.5, amplitude as i16 / 2).with_easing(Easing::Linear),
                Keyframe::new(0.6, -(amplitude as i16 / 2)).with_easing(Easing::Linear),
                Keyframe::new(1.0, 0),
            ],
        )
    }

    /// Zoom in effect
    pub fn zoom_in(duration: Duration) -> Transition<f32> {
        Transition::new(
            duration,
            vec![
                Keyframe::new(0.0, 0.5).with_easing(Easing::EaseOut),
                Keyframe::new(1.0, 1.0),
            ],
        )
    }
}

/// Transition manager for multiple transitions
pub struct TransitionManager {
    transitions: Vec<(String, Box<dyn TransitionTrait>)>,
}

trait TransitionTrait {
    fn update(&mut self) -> bool; // Returns true if still running
    fn reset(&mut self);
}

impl<T: Interpolate + 'static> TransitionTrait for Transition<T> {
    fn update(&mut self) -> bool {
        self.update();
        self.is_running()
    }

    fn reset(&mut self) {
        self.reset();
    }
}

impl TransitionManager {
    pub fn new() -> Self {
        Self {
            transitions: Vec::new(),
        }
    }

    pub fn add<T: Interpolate + 'static>(&mut self, name: &str, mut transition: Transition<T>) {
        transition.start();
        self.transitions
            .push((name.to_string(), Box::new(transition)));
    }

    pub fn update_all(&mut self) {
        self.transitions.retain_mut(|(_, transition)| transition.update());
    }

    pub fn reset(&mut self, name: &str) {
        if let Some((_, transition)) = self.transitions.iter_mut().find(|(n, _)| n == name) {
            transition.reset();
        }
    }

    pub fn clear(&mut self) {
        self.transitions.clear();
    }
}

impl Default for TransitionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyframe_interpolation() {
        let transition = Transition::new(
            Duration::from_secs(1),
            vec![
                Keyframe::new(0.0, 0.0_f32),
                Keyframe::new(0.5, 50.0),
                Keyframe::new(1.0, 100.0),
            ],
        );

        assert_eq!(transition.value_at(0.0), 0.0);
        assert_eq!(transition.value_at(0.5), 50.0);
        assert_eq!(transition.value_at(1.0), 100.0);
    }

    #[test]
    fn test_color_interpolation() {
        let c1 = Color::Rgb(0, 0, 0);
        let c2 = Color::Rgb(255, 255, 255);
        let mid = c1.lerp(&c2, 0.5);
        
        if let Color::Rgb(r, g, b) = mid {
            assert!(r > 100 && r < 150);
            assert!(g > 100 && g < 150);
            assert!(b > 100 && b < 150);
        }
    }
}
