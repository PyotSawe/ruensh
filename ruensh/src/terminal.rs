//! Terminal abstraction and initialization

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout};

/// Terminal wrapper managing raw mode and alternate screen
pub struct Terminal {
    stdout: Stdout,
}

impl Terminal {
    /// Create and initialize a new terminal instance
    pub fn new() -> io::Result<Self> {
        let mut stdout = io::stdout();
        
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;
        
        Ok(Terminal { stdout })
    }

    /// Get mutable reference to stdout
    pub fn stdout_mut(&mut self) -> &mut Stdout {
        &mut self.stdout
    }

    /// Get reference to stdout
    pub fn stdout(&self) -> &Stdout {
        &self.stdout
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.stdout, LeaveAlternateScreen);
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new().expect("Failed to initialize terminal")
    }
}
