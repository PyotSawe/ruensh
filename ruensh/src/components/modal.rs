//! Modal/Dialog component

use crate::events::Event;
use crate::state::Action;
use crate::style::Theme;
use crossterm::event::{KeyCode, KeyEvent, MouseEventKind};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

/// Message types for modal
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModalMessage {
    PrimaryButton,
    SecondaryButton,
    Dismiss,
    HoverPrimary,
    HoverSecondary,
    NoHover,
}

/// Button state tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonFocus {
    Primary,
    Secondary,
    None,
}

/// Modal state for animations and interactions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalState {
    Hidden,
    Appearing,
    Visible,
    Disappearing,
}

/// Modal dialog component with advanced event handling
pub struct Modal {
    title: String,
    content: String,
    primary_label: String,
    secondary_label: String,
    theme: Theme,
    focused_button: ButtonFocus,
    modal_state: ModalState,
    animation_frame: u8,
    last_mouse_x: u16,
    last_mouse_y: u16,
}

impl Modal {
    /// Create a new modal with the given message
    pub fn new(content: impl Into<String>) -> Self {
        Modal {
            title: String::from("Confirm"),
            content: content.into(),
            primary_label: String::from("Confirm"),
            secondary_label: String::from("Cancel"),
            theme: Theme::default(),
            focused_button: ButtonFocus::Primary,
            modal_state: ModalState::Hidden,
            animation_frame: 0,
            last_mouse_x: 0,
            last_mouse_y: 0,
        }
    }

    /// Set primary button label and action
    pub fn primary_button(mut self, label: impl Into<String>) -> Self {
        self.primary_label = label.into();
        self
    }

    /// Set secondary button label
    pub fn secondary_button(mut self, label: impl Into<String>) -> Self {
        self.secondary_label = label.into();
        self
    }

    /// Set modal title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set theme
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Show the modal with popup animation
    pub fn show(&mut self) {
        self.modal_state = ModalState::Appearing;
        self.animation_frame = 0;
    }

    /// Hide the modal with disappear animation
    pub fn hide(&mut self) {
        self.modal_state = ModalState::Disappearing;
        self.animation_frame = 0;
    }

    /// Get current modal state
    pub fn is_visible(&self) -> bool {
        self.modal_state == ModalState::Visible || self.modal_state == ModalState::Appearing
    }

    /// Get focused button
    pub fn focused_button(&self) -> ButtonFocus {
        self.focused_button
    }

    /// Update animation state
    pub fn update_animation(&mut self) {
        match self.modal_state {
            ModalState::Appearing => {
                self.animation_frame = self.animation_frame.saturating_add(1);
                if self.animation_frame >= 10 {
                    self.modal_state = ModalState::Visible;
                }
            }
            ModalState::Disappearing => {
                self.animation_frame = self.animation_frame.saturating_add(1);
                if self.animation_frame >= 10 {
                    self.modal_state = ModalState::Hidden;
                }
            }
            _ => {}
        }
    }

    /// Check if mouse is over primary button
    #[allow(dead_code)]
    fn is_mouse_over_primary(&self, button_area: Rect) -> bool {
        self.last_mouse_x >= button_area.x
            && self.last_mouse_x < button_area.x + button_area.width
            && self.last_mouse_y == button_area.y
    }

    /// Check if mouse is over secondary button
    #[allow(dead_code)]
    fn is_mouse_over_secondary(&self, button_area: Rect) -> bool {
        let secondary_start = button_area.x + (self.primary_label.len() as u16) + 4;
        self.last_mouse_x >= secondary_start
            && self.last_mouse_x < secondary_start + (self.secondary_label.len() as u16) + 2
            && self.last_mouse_y == button_area.y
    }

    /// Render the modal in the center of the given area
    pub fn render_centered(&self, frame: &mut Frame, area: Rect) {
        // Calculate animation opacity/scale
        let visibility = match self.modal_state {
            ModalState::Hidden => return,
            ModalState::Appearing => (self.animation_frame as f32 / 10.0).min(1.0),
            ModalState::Visible => 1.0,
            ModalState::Disappearing => ((10 - self.animation_frame) as f32 / 10.0).max(0.0),
        };

        // Create centered area for modal
        let modal_width = 60.min(area.width.saturating_sub(4));
        let modal_height = 14.min(area.height.saturating_sub(2));

        let x = (area.width.saturating_sub(modal_width)) / 2;
        let y = (area.height.saturating_sub(modal_height)) / 2;

        let modal_area = Rect {
            x: area.x + x,
            y: area.y + y,
            width: modal_width,
            height: modal_height,
        };

        // Draw semi-transparent backdrop
        let backdrop_color = if visibility > 0.5 {
            Color::Black
        } else {
            Color::Reset
        };

        frame.render_widget(
            Block::default().style(Style::default().bg(backdrop_color).fg(Color::DarkGray)),
            area,
        );

        // Draw modal box with border
        let block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD),
            )
            .style(Style::default().bg(Color::Black).fg(Color::White));

        let inner_area = block.inner(modal_area);
        frame.render_widget(block, modal_area);

        // Calculate content and button areas
        let mut inner_y = inner_area.y;

        // Render message with word wrap
        let message = Paragraph::new(self.content.as_str())
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White));

        let message_area = Rect {
            y: inner_y,
            height: (inner_area.height / 2).min(6),
            ..inner_area
        };
        frame.render_widget(message, message_area);

        inner_y += message_area.height + 1;

        // Render buttons with hover state
        let button_area = Rect {
            y: inner_y,
            height: 1,
            ..inner_area
        };

        self.render_buttons(frame, button_area);
    }

    /// Render buttons with hover and focus states
    fn render_buttons(&self, frame: &mut Frame, area: Rect) {
        let primary_focused = self.focused_button == ButtonFocus::Primary;
        let secondary_focused = self.focused_button == ButtonFocus::Secondary;

        // Primary button style
        let primary_style = if primary_focused {
            Style::default()
                .fg(Color::Black)
                .bg(self.theme.primary)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(self.theme.primary)
                .add_modifier(Modifier::BOLD)
        };

        let primary_text = if primary_focused {
            format!(" {} ", self.primary_label)
        } else {
            format!("[ {} ]", self.primary_label)
        };

        let primary_widget = Paragraph::new(primary_text).style(primary_style);

        let primary_button_area = Rect {
            x: area.x + 2,
            y: area.y,
            width: (self.primary_label.len() as u16) + 4,
            height: 1,
        };

        frame.render_widget(primary_widget, primary_button_area);

        // Secondary button style
        let secondary_style = if secondary_focused {
            Style::default()
                .fg(Color::Black)
                .bg(self.theme.secondary)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(self.theme.secondary)
                .add_modifier(Modifier::BOLD)
        };

        let secondary_text = if secondary_focused {
            format!(" {} ", self.secondary_label)
        } else {
            format!("[ {} ]", self.secondary_label)
        };

        let secondary_widget = Paragraph::new(secondary_text).style(secondary_style);

        let secondary_button_area = Rect {
            x: primary_button_area.x + primary_button_area.width + 3,
            y: area.y,
            width: (self.secondary_label.len() as u16) + 4,
            height: 1,
        };

        frame.render_widget(secondary_widget, secondary_button_area);
    }
}

impl super::Component for Modal {
    type Message = ModalMessage;

    fn update(&mut self, msg: Self::Message) -> Option<Action> {
        match msg {
            ModalMessage::PrimaryButton => {
                self.hide();
                Some(Action::Confirm)
            }
            ModalMessage::SecondaryButton => {
                self.hide();
                Some(Action::Cancel)
            }
            ModalMessage::Dismiss => {
                self.hide();
                Some(Action::Cancel)
            }
            ModalMessage::HoverPrimary => {
                self.focused_button = ButtonFocus::Primary;
                None
            }
            ModalMessage::HoverSecondary => {
                self.focused_button = ButtonFocus::Secondary;
                None
            }
            ModalMessage::NoHover => {
                self.focused_button = ButtonFocus::Primary;
                None
            }
        }
    }

    fn render(&self, frame: &mut Frame<'_>) {
        let area = frame.area();
        self.render_centered(frame, area);
    }

    fn handle_event(&mut self, event: &Event) -> Option<Self::Message> {
        match event {
            // Keyboard event handling with robust navigation
            Event::Key(KeyEvent { code, .. }) => match code {
                // Confirm actions
                KeyCode::Enter => {
                    if self.focused_button == ButtonFocus::Primary {
                        return Some(ModalMessage::PrimaryButton);
                    } else if self.focused_button == ButtonFocus::Secondary {
                        return Some(ModalMessage::SecondaryButton);
                    }
                }
                // Dismiss on Escape
                KeyCode::Esc => return Some(ModalMessage::Dismiss),
                // Navigation between buttons
                KeyCode::Tab | KeyCode::Right => {
                    self.focused_button = match self.focused_button {
                        ButtonFocus::Primary => ButtonFocus::Secondary,
                        ButtonFocus::Secondary => ButtonFocus::Primary,
                        ButtonFocus::None => ButtonFocus::Primary,
                    };
                    None?;
                }
                KeyCode::BackTab | KeyCode::Left => {
                    self.focused_button = match self.focused_button {
                        ButtonFocus::Primary => ButtonFocus::Secondary,
                        ButtonFocus::Secondary => ButtonFocus::Primary,
                        ButtonFocus::None => ButtonFocus::Secondary,
                    };
                    None?;
                }
                // Quick keys
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    return Some(ModalMessage::PrimaryButton)
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    return Some(ModalMessage::SecondaryButton)
                }
                _ => {}
            },
            // Mouse event handling
            Event::Mouse(mouse_event) => {
                self.last_mouse_x = mouse_event.column;
                self.last_mouse_y = mouse_event.row;

                // Calculate button positions (approximate based on modal rendering)
                let area = Rect {
                    x: 0,
                    y: 0,
                    width: 80,
                    height: 24,
                };

                let modal_width = 60.min(area.width.saturating_sub(4));
                let modal_height = 14.min(area.height.saturating_sub(2));
                let modal_x = (area.width.saturating_sub(modal_width)) / 2;
                let modal_y = (area.height.saturating_sub(modal_height)) / 2;

                let button_y = modal_y + modal_height - 4;

                // Primary button area
                let primary_start_x = modal_x + 4;
                let primary_end_x = primary_start_x + (self.primary_label.len() as u16) + 4;

                // Secondary button area
                let secondary_start_x = primary_end_x + 4;
                let secondary_end_x = secondary_start_x + (self.secondary_label.len() as u16) + 4;

                match mouse_event.kind {
                    MouseEventKind::Down(_) | MouseEventKind::Up(_) => {
                        // Check if click is on primary button
                        if mouse_event.column >= primary_start_x
                            && mouse_event.column < primary_end_x
                            && mouse_event.row == button_y
                        {
                            return Some(ModalMessage::PrimaryButton);
                        }
                        // Check if click is on secondary button
                        if mouse_event.column >= secondary_start_x
                            && mouse_event.column < secondary_end_x
                            && mouse_event.row == button_y
                        {
                            return Some(ModalMessage::SecondaryButton);
                        }
                    }
                    MouseEventKind::Moved => {
                        // Update hover state
                        if mouse_event.column >= primary_start_x && mouse_event.column < primary_end_x && mouse_event.row == button_y {
                            return Some(ModalMessage::HoverPrimary);
                        } else if mouse_event.column >= secondary_start_x && mouse_event.column < secondary_end_x && mouse_event.row == button_y
                        {
                            return Some(ModalMessage::HoverSecondary);
                        } else {
                            return Some(ModalMessage::NoHover);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        None
    }
}
