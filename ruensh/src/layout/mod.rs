//! Layout system for composing components

use ratatui::prelude::*;

/// Constraint for layout sizing
#[derive(Debug, Clone, Copy)]
pub enum Constraint {
    Length(u16),
    Percentage(u16),
    Ratio(u32, u32),
    Min(u16),
    Max(u16),
}

impl Constraint {
    pub fn to_ratatui_constraint(&self) -> ratatui::layout::Constraint {
        match self {
            Constraint::Length(len) => ratatui::layout::Constraint::Length(*len),
            Constraint::Percentage(pct) => ratatui::layout::Constraint::Percentage(*pct),
            Constraint::Ratio(num, den) => ratatui::layout::Constraint::Ratio(*num, *den),
            Constraint::Min(min) => ratatui::layout::Constraint::Min(*min),
            Constraint::Max(max) => ratatui::layout::Constraint::Max(*max),
        }
    }
}

/// Layout builder for creating flexible layouts
pub struct Layout {
    direction: ratatui::layout::Direction,
    constraints: Vec<Constraint>,
    margin: (u16, u16),
}

impl Layout {
    /// Create a vertical layout
    pub fn vertical(constraints: impl Into<Vec<Constraint>>) -> Self {
        Layout {
            direction: ratatui::layout::Direction::Vertical,
            constraints: constraints.into(),
            margin: (0, 0),
        }
    }

    /// Create a horizontal layout
    pub fn horizontal(constraints: impl Into<Vec<Constraint>>) -> Self {
        Layout {
            direction: ratatui::layout::Direction::Horizontal,
            constraints: constraints.into(),
            margin: (0, 0),
        }
    }

    /// Set margins (vertical, horizontal)
    pub fn with_margin(mut self, v: u16, h: u16) -> Self {
        self.margin = (v, h);
        self
    }

    /// Get layout areas for rendering
    pub fn split(&self, area: Rect) -> Vec<Rect> {
        let constraints: Vec<_> = self
            .constraints
            .iter()
            .map(|c| c.to_ratatui_constraint())
            .collect();

        ratatui::layout::Layout::default()
            .direction(self.direction)
            .constraints(constraints)
            .split(area)
            .to_vec()
    }
}
