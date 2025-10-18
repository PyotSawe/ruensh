//! Enhanced Modal demo - Shows robust event handling with mouse and keyboard navigation
//! 
//! Features:
//! - Modal popup with animations
//! - Mouse click and hover detection
//! - Button highlighting on hover
//! - Keyboard navigation (Tab/Shift+Tab, Arrow keys)
//! - Quick keys (Y/N)
//! 
//! Run with: cargo run --example modal_demo

use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use ruensh::components::{Component, Modal};
use ruensh::components::modal::ButtonFocus;
use ruensh::events::{Event, EventHandler, start_event_loop};
use ruensh::style::Theme;
use ruensh::terminal::Terminal;
use std::io;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppState {
    ShowModal,
    Confirmed,
    Cancelled,
}

struct App {
    modal: Modal,
    state: AppState,
    message: String,
    start_time: Instant,
}

impl App {
    fn new() -> Self {
        let theme = Theme::default()
            .set_primary(Color::Magenta)
            .set_secondary(Color::Blue);

        let mut modal = Modal::new("Are you sure you want to quit?")
            .title("")
            .primary_button("Yep!")
            .secondary_button("Nope")
            .theme(theme);
        
        modal.show();

        App {
            modal,
            state: AppState::ShowModal,
            message: String::from("Ready to interact... Use mouse or keyboard (Tab/Y/N)"),
            start_time: Instant::now(),
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        // Draw header
        let header = format!("Charm™ Crush {}", "/".repeat((75usize).saturating_sub(20)));
        let header_widget = Paragraph::new(header)
            .style(
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            );
        frame.render_widget(header_widget, Rect {
            y: 0,
            height: 1,
            ..area
        });

        // Draw LSPs and MCPs labels
        let labels_area = Rect {
            y: 2,
            height: 1,
            ..area
        };
        
        let labels_text = format!("{}   {}", "LSPs", "MCPs");
        let labels_widget = Paragraph::new(labels_text)
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(labels_widget, labels_area);

        // Draw focused button indicator
        let focus_text = match self.modal.focused_button() {
            ButtonFocus::Primary => "Focused: Yep! (Primary)",
            ButtonFocus::Secondary => "Focused: Nope (Secondary)",
            ButtonFocus::None => "Focused: None",
        };
        
        let focus_area = Rect {
            y: 4,
            height: 1,
            ..area
        };
        let focus_widget = Paragraph::new(focus_text)
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(focus_widget, focus_area);

        // Draw status bar
        let status_area = Rect {
            y: area.height.saturating_sub(3),
            height: 1,
            ..area
        };
        let status = Paragraph::new(format!("> {}", self.message))
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(status, status_area);

        // Draw animation indicators
        let anim_area = Rect {
            y: area.height.saturating_sub(2),
            height: 2,
            ..area
        };
        
        let elapsed = self.start_time.elapsed().as_millis();
        let spinner = match (elapsed / 100) % 4 {
            0 => "⠋",
            1 => "⠙",
            2 => "⠹",
            _ => "⠸",
        };
        
        let anim_text = format!("{} Interactive Modal", spinner);
        let anim_widget = Paragraph::new(anim_text)
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(anim_widget, anim_area);

        // Render modal with popup capability
        self.modal.render(frame);
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        if let Some(msg) = self.modal.handle_event(event) {
            if let Some(action) = self.modal.update(msg) {
                match action {
                    ruensh::state::Action::Confirm => {
                        self.state = AppState::Confirmed;
                        self.message = String::from("Confirmed! Exiting...");
                        return true;
                    }
                    ruensh::state::Action::Cancel => {
                        self.state = AppState::Cancelled;
                        self.message = String::from("Cancelled! Exiting...");
                        return true;
                    }
                    _ => {}
                }
            }
        }

        if let Event::Key(key) = event {
            if key.code == KeyCode::Esc {
                self.state = AppState::Cancelled;
                self.message = String::from("Escape pressed! Exiting...");
                return true;
            }
        }

        // Update message based on interaction
        if let Event::Mouse(_) = event {
            self.message = String::from("Mouse interaction detected!");
        } else if let Event::Key(_) = event {
            self.message = String::from("Keyboard navigation active!");
        }

        false
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let _terminal = Terminal::new()?;
    let (_event_handler, _tx) = EventHandler::new();
    start_event_loop(_tx).await;

    let mut app = App::new();
    let mut running = true;

    let backend = ratatui::backend::CrosstermBackend::new(io::stdout());
    let mut tui = ratatui::Terminal::new(backend)?;

    while running {
        tui.draw(|frame| app.render(frame))?;

        // Update modal animations
        app.modal.update_animation();

        // Handle events
        if crossterm::event::poll(std::time::Duration::from_millis(16))? {
            if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
                let event = Event::Key(key);
                if app.handle_event(&event) {
                    running = false;
                }
            } else if let Ok(crossterm::event::Event::Mouse(mouse)) = crossterm::event::read() {
                let event = Event::Mouse(mouse);
                let _ = app.handle_event(&event);
            }
        }
    }

    Ok(())
}
