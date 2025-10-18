//! Styling system for components

use ratatui::style::{Color, Modifier, Style};

/// Border style options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
    Rounded,
    Single,
    Double,
    Thick,
    None,
}

impl BorderStyle {
    pub fn to_ratatui_border(&self) -> ratatui::widgets::BorderType {
        match self {
            BorderStyle::Rounded => ratatui::widgets::BorderType::Rounded,
            BorderStyle::Single => ratatui::widgets::BorderType::Plain,
            BorderStyle::Double => ratatui::widgets::BorderType::Double,
            BorderStyle::Thick => ratatui::widgets::BorderType::Thick,
            BorderStyle::None => ratatui::widgets::BorderType::Plain,
        }
    }
}

/// Theme configuration
#[derive(Debug, Clone)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub background: Color,
    pub text: Color,
    pub border_style: BorderStyle,
    pub accent: Color,
}

impl Theme {
    /// Create a dark theme (default)
    pub fn dark() -> Self {
        Theme {
            primary: Color::Magenta,
            secondary: Color::Blue,
            background: Color::Black,
            text: Color::White,
            border_style: BorderStyle::Rounded,
            accent: Color::Cyan,
        }
    }

    /// Create a light theme
    pub fn light() -> Self {
        Theme {
            primary: Color::Magenta,
            secondary: Color::Blue,
            background: Color::White,
            text: Color::Black,
            border_style: BorderStyle::Rounded,
            accent: Color::Cyan,
        }
    }

    pub fn set_primary(mut self, color: Color) -> Self {
        self.primary = color;
        self
    }

    pub fn set_secondary(mut self, color: Color) -> Self {
        self.secondary = color;
        self
    }

    pub fn set_border_style(mut self, style: BorderStyle) -> Self {
        self.border_style = style;
        self
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::dark()
    }
}

/// Helper to create styled text
pub fn highlight_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.primary)
        .add_modifier(Modifier::BOLD)
}

pub fn normal_style(theme: &Theme) -> Style {
    Style::default().fg(theme.text)
}
