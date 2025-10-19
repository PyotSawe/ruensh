//! Visual effects for SVG-inspired rendering

use ratatui::style::Color;

/// Glow effect intensity
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GlowIntensity {
    Low,
    Medium,
    High,
}

/// Glow effect for neon-like borders and text
#[derive(Debug, Clone)]
pub struct GlowEffect {
    pub color: Color,
    pub intensity: GlowIntensity,
    pub radius: u8,
}

impl GlowEffect {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            intensity: GlowIntensity::Medium,
            radius: 1,
        }
    }

    pub fn intensity(mut self, intensity: GlowIntensity) -> Self {
        self.intensity = intensity;
        self
    }

    pub fn radius(mut self, radius: u8) -> Self {
        self.radius = radius;
        self
    }
}

/// Gradient fill for shapes
#[derive(Debug, Clone)]
pub struct GradientFill {
    pub colors: Vec<Color>,
    pub direction: GradientDirection,
}

/// Direction of gradient
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GradientDirection {
    Horizontal,
    Vertical,
    Diagonal,
    Radial,
}

impl GradientFill {
    pub fn new(colors: Vec<Color>) -> Self {
        Self {
            colors,
            direction: GradientDirection::Horizontal,
        }
    }

    pub fn direction(mut self, direction: GradientDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Get interpolated color at position (0.0 to 1.0)
    pub fn color_at(&self, position: f32) -> Color {
        if self.colors.is_empty() {
            return Color::White;
        }
        if self.colors.len() == 1 {
            return self.colors[0];
        }

        let position = position.clamp(0.0, 1.0);
        let segment = position * (self.colors.len() - 1) as f32;
        let index = segment.floor() as usize;
        let t = segment - index as f32;

        if index >= self.colors.len() - 1 {
            return self.colors[self.colors.len() - 1];
        }

        // Linear interpolation between colors
        self.interpolate(self.colors[index], self.colors[index + 1], t)
    }

    fn interpolate(&self, c1: Color, c2: Color, t: f32) -> Color {
        match (c1, c2) {
            (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
                let r = (r1 as f32 + (r2 as f32 - r1 as f32) * t) as u8;
                let g = (g1 as f32 + (g2 as f32 - g1 as f32) * t) as u8;
                let b = (b1 as f32 + (b2 as f32 - b1 as f32) * t) as u8;
                Color::Rgb(r, g, b)
            }
            _ => c1, // Fallback for non-RGB colors
        }
    }
}

/// Filter effects
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Filter {
    Blur,
    Glow,
    Shadow,
    Invert,
}

/// Blur level for glassmorphism
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlurLevel {
    Low,
    Medium,
    High,
}
