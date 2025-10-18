//! List component for displaying selectable items

use crate::events::Event;
use crate::state::Action;
use crate::style::Theme;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List as RatatuiList, ListItem, Paragraph};

/// Message types for list
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListMessage {
    Select(usize),
    Up,
    Down,
    Confirm,
}

/// List component for displaying and selecting from items
pub struct List {
    items: Vec<String>,
    selected: usize,
    title: String,
    theme: Theme,
}

impl List {
    /// Create a new list with items
    pub fn new(items: Vec<String>) -> Self {
        List {
            items,
            selected: 0,
            title: String::from("List"),
            theme: Theme::default(),
        }
    }

    /// Set list title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set theme
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Get currently selected item index
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// Get currently selected item
    pub fn selected_item(&self) -> Option<&str> {
        self.items.get(self.selected).map(|s| s.as_str())
    }

    /// Move selection up
    fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Move selection down
    fn move_down(&mut self) {
        if self.selected < self.items.len().saturating_sub(1) {
            self.selected += 1;
        }
    }
}

impl super::Component for List {
    type Message = ListMessage;

    fn update(&mut self, msg: Self::Message) -> Option<Action> {
        match msg {
            ListMessage::Up => {
                self.move_up();
                None
            }
            ListMessage::Down => {
                self.move_down();
                None
            }
            ListMessage::Select(idx) => {
                if idx < self.items.len() {
                    self.selected = idx;
                }
                None
            }
            ListMessage::Confirm => Some(Action::Confirm),
        }
    }

    fn render(&self, frame: &mut Frame<'_>) {
        let area = frame.area();

        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(idx, item)| {
                let _content = if idx == self.selected {
                    Paragraph::new(format!("▸ {}", item))
                        .style(Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD))
                } else {
                    Paragraph::new(format!("  {}", item))
                        .style(Style::default().fg(Color::White))
                };
                ListItem::new("")
            })
            .collect();

        let list = RatatuiList::new(items)
            .block(
                Block::default()
                    .title(self.title.as_str())
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Style::default().fg(self.theme.secondary))
                    .style(Style::default().bg(Color::Black).fg(Color::White)),
            )
            .highlight_style(
                Style::default()
                    .fg(self.theme.primary)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▸ ");

        frame.render_widget(list, area);
    }

    fn handle_event(&mut self, event: &Event) -> Option<Self::Message> {
        match event {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Up | KeyCode::Char('k') => Some(ListMessage::Up),
                KeyCode::Down | KeyCode::Char('j') => Some(ListMessage::Down),
                KeyCode::Enter => Some(ListMessage::Confirm),
                _ => None,
            },
            _ => None,
        }
    }
}
