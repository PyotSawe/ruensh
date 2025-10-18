//! Event handling system

use crossterm::event::{KeyEvent, MouseEvent};
use std::time::Duration;
use tokio::sync::mpsc;

/// Represents different types of events that can occur in the terminal
#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Tick,
}

/// Event handler for receiving terminal events
pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
    /// Create a new event handler with a background event listener
    pub fn new() -> (Self, mpsc::UnboundedSender<Event>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (EventHandler { rx }, tx)
    }

    /// Try to receive the next event without blocking
    pub fn try_recv(&mut self) -> Option<Event> {
        self.rx.try_recv().ok()
    }

    /// Receive the next event, blocking until one is available
    pub async fn recv(&mut self) -> Option<Event> {
        self.rx.recv().await
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new().0
    }
}

/// Start background event loop
pub async fn start_event_loop(tx: mpsc::UnboundedSender<Event>) {
    tokio::spawn(async move {
        loop {
            if crossterm::event::poll(Duration::from_millis(16)).unwrap_or(false) {
                match crossterm::event::read() {
                    Ok(crossterm::event::Event::Key(key)) => {
                        let _ = tx.send(Event::Key(key));
                    }
                    Ok(crossterm::event::Event::Mouse(mouse)) => {
                        let _ = tx.send(Event::Mouse(mouse));
                    }
                    Ok(crossterm::event::Event::Resize(width, height)) => {
                        let _ = tx.send(Event::Resize(width, height));
                    }
                    _ => {}
                }
            } else {
                let _ = tx.send(Event::Tick);
                tokio::time::sleep(Duration::from_millis(16)).await;
            }
        }
    });
}
