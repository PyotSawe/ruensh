//! Application state management

/// Represents possible application actions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Quit,
    Cancel,
    Confirm,
    Custom(String),
}

/// Trait for managing application state
pub trait StateManager {
    type Message;
    type State;

    fn update(&mut self, msg: Self::Message);
    fn current_state(&self) -> &Self::State;
}
