//! Component system for building TUI applications

pub mod modal;
pub mod list;

use ratatui::Frame;

/// Type alias for rendered elements
pub type Element = ratatui::widgets::Block<'static>;

/// Base component trait
pub trait Component {
    type Message;

    /// Update component with a message
    fn update(&mut self, msg: Self::Message) -> Option<crate::state::Action>;

    /// Render the component
    fn render(&self, frame: &mut Frame<'_>);

    /// Handle keyboard/mouse events
    fn handle_event(&mut self, event: &crate::events::Event) -> Option<Self::Message>;
}

pub use modal::Modal;
pub use list::List;
