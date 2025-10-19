//! A powerful, ergonomic Terminal User Interface (TUI) library in Rust
//! 
//! This library provides high-level components for building beautiful terminal applications,
//! inspired by Charm's Bubble Tea ecosystem.

pub mod terminal;
pub mod events;
pub mod components;
pub mod layout;
pub mod style;
pub mod state;
pub mod svg;

pub use components::{Component, Element};
pub use events::{Event, EventHandler};
pub use style::Theme;
pub use terminal::Terminal;
