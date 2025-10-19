//! SVG-inspired futuristic UI components for terminal interfaces
//! 
//! This module provides vector-like graphics capabilities for creating
//! stunning, modern terminal interfaces using Unicode characters.

mod canvas;
mod shapes;
mod effects;
mod animations;

pub use canvas::SvgCanvas;
pub use shapes::{Shape, Rectangle, Circle, Line, Path, Point};
pub use effects::{GlowEffect, GradientFill, Filter};
pub use animations::{Animation, Easing};

/// Resolution modes for rendering
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Resolution {
    /// Standard character cell rendering
    CharCell,
    /// High-resolution Braille patterns (2×4 pixels per cell)
    Braille,
    /// Block elements for filled shapes
    Block,
}

/// Color scheme presets for futuristic UIs
#[derive(Debug, Clone)]
pub enum ColorScheme {
    CyberPunk {
        primary: ratatui::style::Color,
        secondary: ratatui::style::Color,
        accent: ratatui::style::Color,
        background: ratatui::style::Color,
        glow: ratatui::style::Color,
    },
    NeonTokyo {
        primary: ratatui::style::Color,
        secondary: ratatui::style::Color,
        accent: ratatui::style::Color,
        background: ratatui::style::Color,
        glow: ratatui::style::Color,
    },
    Matrix {
        primary: ratatui::style::Color,
        secondary: ratatui::style::Color,
        accent: ratatui::style::Color,
        background: ratatui::style::Color,
        glow: ratatui::style::Color,
    },
    Holographic {
        primary: ratatui::style::Color,
        secondary: ratatui::style::Color,
        accent: ratatui::style::Color,
        background: ratatui::style::Color,
        glow: ratatui::style::Color,
    },
}

impl ColorScheme {
    /// Get the CyberPunk color scheme
    pub fn cyberpunk() -> Self {
        use ratatui::style::Color;
        Self::CyberPunk {
            primary: Color::Rgb(255, 0, 128),      // Hot Pink
            secondary: Color::Rgb(0, 255, 255),    // Cyan
            accent: Color::Rgb(255, 255, 0),       // Yellow
            background: Color::Rgb(10, 0, 20),     // Dark Purple
            glow: Color::Rgb(180, 0, 255),         // Purple
        }
    }

    /// Get the Neon Tokyo color scheme
    pub fn neon_tokyo() -> Self {
        use ratatui::style::Color;
        Self::NeonTokyo {
            primary: Color::Rgb(255, 20, 147),     // Deep Pink
            secondary: Color::Rgb(0, 191, 255),    // Deep Sky Blue
            accent: Color::Rgb(255, 215, 0),       // Gold
            background: Color::Rgb(15, 15, 30),    // Navy
            glow: Color::Rgb(138, 43, 226),        // Blue Violet
        }
    }

    /// Get the Matrix color scheme
    pub fn matrix() -> Self {
        use ratatui::style::Color;
        Self::Matrix {
            primary: Color::Rgb(0, 255, 65),       // Bright Green
            secondary: Color::Rgb(0, 200, 50),     // Green
            accent: Color::Rgb(150, 255, 150),     // Light Green
            background: Color::Rgb(0, 0, 0),       // Black
            glow: Color::Rgb(0, 255, 100),         // Neon Green
        }
    }

    /// Get the Holographic color scheme
    pub fn holographic() -> Self {
        use ratatui::style::Color;
        Self::Holographic {
            primary: Color::Rgb(100, 200, 255),    // Light Blue
            secondary: Color::Rgb(200, 100, 255),  // Light Purple
            accent: Color::Rgb(255, 255, 255),     // White
            background: Color::Rgb(5, 10, 25),     // Deep Blue
            glow: Color::Rgb(150, 200, 255),       // Sky Blue
        }
    }

    /// Get primary color
    pub fn primary(&self) -> ratatui::style::Color {
        match self {
            Self::CyberPunk { primary, .. } => *primary,
            Self::NeonTokyo { primary, .. } => *primary,
            Self::Matrix { primary, .. } => *primary,
            Self::Holographic { primary, .. } => *primary,
        }
    }

    /// Get glow color
    pub fn glow(&self) -> ratatui::style::Color {
        match self {
            Self::CyberPunk { glow, .. } => *glow,
            Self::NeonTokyo { glow, .. } => *glow,
            Self::Matrix { glow, .. } => *glow,
            Self::Holographic { glow, .. } => *glow,
        }
    }
}
