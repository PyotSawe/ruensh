# Building Custom Components for RuenSH

This guide shows how to create custom components for your language's REPL or CLI.

## Table of Contents

1. [Component Basics](#component-basics)
2. [InputField Component](#inputfield-component)
3. [List Component](#list-component)
4. [Table Component](#table-component)
5. [Building REPL Components](#building-repl-components)
6. [Best Practices](#best-practices)

## Component Basics

All components in RuenSH implement the `Component` trait:

```rust
pub trait Component {
    /// The message type this component emits
    type Message;

    /// Update the component's state based on a message
    fn update(&mut self, message: Self::Message);

    /// Render the component to the frame
    fn render(&self, area: Rect, f: &mut Frame);

    /// Handle input events and return a message
    fn handle_event(&mut self, event: &Event) -> Option<Self::Message>;
}
```

### Message Enum Pattern

Define a message enum for all possible events your component can emit:

```rust
pub enum MyComponentMessage {
    InputChanged(String),
    Submitted,
    Cancelled,
    SelectionChanged(usize),
}
```

## InputField Component

Perfect for REPL input. Here's a template:

```rust
use ruensh::components::Component;
use ruensh::events::Event;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub enum InputFieldMessage {
    CharReceived(char),
    Backspace,
    Submit,
    Clear,
}

pub struct InputField {
    content: String,
    cursor_pos: usize,
    is_focused: bool,
}

impl InputField {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor_pos: 0,
            is_focused: true,
        }
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn clear(&mut self) {
        self.content.clear();
        self.cursor_pos = 0;
    }
}

impl Component for InputField {
    type Message = InputFieldMessage;

    fn update(&mut self, message: Self::Message) {
        match message {
            InputFieldMessage::CharReceived(ch) => {
                self.content.insert(self.cursor_pos, ch);
                self.cursor_pos += 1;
            }
            InputFieldMessage::Backspace => {
                if self.cursor_pos > 0 {
                    self.content.remove(self.cursor_pos - 1);
                    self.cursor_pos -= 1;
                }
            }
            InputFieldMessage::Submit => {
                // Handle submit
            }
            InputFieldMessage::Clear => {
                self.clear();
            }
        }
    }

    fn render(&self, area: Rect, f: &mut Frame) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Input ");

        let paragraph = Paragraph::new(self.content.clone())
            .block(block);

        f.render_widget(paragraph, area);

        // Draw cursor
        if self.is_focused {
            f.set_cursor(
                area.x + self.cursor_pos as u16 + 1,
                area.y + 1,
            );
        }
    }

    fn handle_event(&mut self, event: &Event) -> Option<Self::Message> {
        use ruensh::events::Event;
        use crossterm::event::{KeyCode, KeyEvent};

        match event {
            Event::Key(KeyEvent { code, .. }) => {
                match code {
                    KeyCode::Char(ch) => {
                        return Some(InputFieldMessage::CharReceived(*ch));
                    }
                    KeyCode::Backspace => {
                        return Some(InputFieldMessage::Backspace);
                    }
                    KeyCode::Enter => {
                        return Some(InputFieldMessage::Submit);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        None
    }
}
```

## List Component

For displaying history, commands, or results:

```rust
pub enum ListMessage {
    SelectNext,
    SelectPrevious,
    Confirm,
}

pub struct List<T> {
    items: Vec<T>,
    selected: Option<usize>,
}

impl<T: Clone> List<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected: None,
        }
    }

    pub fn add_item(&mut self, item: T) {
        self.items.push(item);
        if self.selected.is_none() {
            self.selected = Some(0);
        }
    }

    pub fn get_selected(&self) -> Option<&T> {
        self.selected.and_then(|idx| self.items.get(idx))
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.selected
    }
}

impl<T: Clone + ToString> Component for List<T> {
    type Message = ListMessage;

    fn update(&mut self, message: Self::Message) {
        match message {
            ListMessage::SelectNext => {
                if let Some(sel) = self.selected {
                    if sel + 1 < self.items.len() {
                        self.selected = Some(sel + 1);
                    }
                }
            }
            ListMessage::SelectPrevious => {
                if let Some(sel) = self.selected {
                    if sel > 0 {
                        self.selected = Some(sel - 1);
                    }
                }
            }
            ListMessage::Confirm => {
                // Handle confirm
            }
        }
    }

    fn render(&self, area: Rect, f: &mut Frame) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" History ");

        let mut y = area.y + 1;
        for (idx, item) in self.items.iter().enumerate() {
            let is_selected = self.selected == Some(idx);
            let style = if is_selected {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else {
                Style::default()
            };

            let text = item.to_string();
            let line = format!("  {}", text);

            f.render_widget(
                Paragraph::new(line).style(style),
                Rect {
                    x: area.x,
                    y,
                    width: area.width,
                    height: 1,
                },
            );
            y += 1;
        }

        f.render_widget(block, area);
    }

    fn handle_event(&mut self, event: &Event) -> Option<Self::Message> {
        use ruensh::events::Event;
        use crossterm::event::{KeyCode, KeyEvent};

        match event {
            Event::Key(KeyEvent { code, .. }) => {
                match code {
                    KeyCode::Down => return Some(ListMessage::SelectNext),
                    KeyCode::Up => return Some(ListMessage::SelectPrevious),
                    KeyCode::Enter => return Some(ListMessage::Confirm),
                    _ => {}
                }
            }
            _ => {}
        }
        None
    }
}
```

## Table Component

For displaying structured data:

```rust
pub struct Table {
    rows: Vec<Vec<String>>,
    headers: Vec<String>,
    selected_row: Option<usize>,
    column_widths: Vec<u16>,
}

impl Table {
    pub fn new(headers: Vec<String>) -> Self {
        let column_widths = headers.iter().map(|h| h.len() as u16 + 2).collect();
        Self {
            rows: Vec::new(),
            headers,
            selected_row: None,
            column_widths,
        }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
        if self.selected_row.is_none() {
            self.selected_row = Some(0);
        }
    }

    fn calculate_column_widths(&mut self) {
        for (col_idx, header) in self.headers.iter().enumerate() {
            let mut max_width = header.len() as u16;
            for row in &self.rows {
                if let Some(cell) = row.get(col_idx) {
                    max_width = max_width.max(cell.len() as u16);
                }
            }
            if col_idx < self.column_widths.len() {
                self.column_widths[col_idx] = max_width + 2;
            }
        }
    }
}

impl Component for Table {
    type Message = ListMessage;

    fn update(&mut self, message: Self::Message) {
        match message {
            ListMessage::SelectNext => {
                if let Some(sel) = self.selected_row {
                    if sel + 1 < self.rows.len() {
                        self.selected_row = Some(sel + 1);
                    }
                }
            }
            ListMessage::SelectPrevious => {
                if let Some(sel) = self.selected_row {
                    if sel > 0 {
                        self.selected_row = Some(sel - 1);
                    }
                }
            }
            ListMessage::Confirm => {}
        }
    }

    fn render(&self, area: Rect, f: &mut Frame) {
        self.calculate_column_widths();
        // Render table with headers and rows
        // Implementation similar to List
    }

    fn handle_event(&mut self, event: &Event) -> Option<Self::Message> {
        // Similar to List component
        None
    }
}
```

## Building REPL Components

Combine components to build a complete REPL interface:

```rust
pub enum ReplMessage {
    Input(InputFieldMessage),
    History(ListMessage),
    Output(String),
    Execute,
}

pub struct Repl {
    input: InputField,
    history: List<String>,
    output: String,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            input: InputField::new(),
            history: List::new(),
            output: String::new(),
        }
    }

    pub fn execute_command(&mut self, cmd: &str) -> String {
        // Execute command in your language
        // Return result or error
        format!("Result: {}", cmd)
    }
}

impl Component for Repl {
    type Message = ReplMessage;

    fn update(&mut self, message: Self::Message) {
        match message {
            ReplMessage::Input(msg) => {
                self.input.update(msg);
            }
            ReplMessage::History(msg) => {
                self.history.update(msg);
            }
            ReplMessage::Execute => {
                let cmd = self.input.get_content().to_string();
                let result = self.execute_command(&cmd);
                self.output = result;
                self.input.clear();
                self.history.add_item(cmd);
            }
            ReplMessage::Output(msg) => {
                self.output = msg;
            }
        }
    }

    fn render(&self, area: Rect, f: &mut Frame) {
        // Divide area into sections
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10),  // History
                Constraint::Length(3),   // Input
                Constraint::Min(0),      // Output
            ])
            .split(area);

        // Render each component
        self.history.render(chunks[0], f);
        self.input.render(chunks[1], f);
        // Render output
    }

    fn handle_event(&mut self, event: &Event) -> Option<Self::Message> {
        // Route events to appropriate component
        if let Some(msg) = self.input.handle_event(event) {
            return Some(ReplMessage::Input(msg));
        }
        if let Some(msg) = self.history.handle_event(event) {
            return Some(ReplMessage::History(msg));
        }
        None
    }
}
```

## Best Practices

### 1. **Separate Logic from Rendering**

```rust
// Good: Logic separate from rendering
impl Component for MyComponent {
    fn update(&mut self, message: Self::Message) {
        // Business logic here
        self.process_message(message);
    }

    fn render(&self, area: Rect, f: &mut Frame) {
        // Rendering only
        self.draw_to_frame(area, f);
    }
}
```

### 2. **Use Message Enums for Type Safety**

```rust
// Good: Type-safe messages
pub enum ButtonMessage {
    Clicked,
    Hovered,
    Unhovered,
}

// Avoid: Using strings or untyped events
// let msg = "clicked"; // Bad!
```

### 3. **Handle Focus and Input Properly**

```rust
impl Component for InputField {
    fn handle_event(&mut self, event: &Event) -> Option<Self::Message> {
        if !self.is_focused {
            return None;  // Don't handle if not focused
        }
        // Handle event
        None
    }
}
```

### 4. **Implement Builder Pattern for Configuration**

```rust
impl InputField {
    pub fn new() -> Self { /* ... */ }

    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn max_length(mut self, length: usize) -> Self {
        self.max_length = length;
        self
    }
}
```

### 5. **Test Components in Isolation**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_field_char_insertion() {
        let mut input = InputField::new();
        let msg = InputFieldMessage::CharReceived('a');
        input.update(msg);
        assert_eq!(input.get_content(), "a");
    }
}
```

### 6. **Document Your Components**

```rust
/// A text input field component for user input
///
/// # Example
///
/// ```rust
/// let mut input = InputField::new()
///     .placeholder("Enter command")
///     .max_length(100);
/// ```
pub struct InputField {
    // ...
}
```

## Integration with Your Language

To integrate RuenSH with your language's REPL:

1. **Define message types** for your language's specific events
2. **Implement Component trait** for language-specific logic
3. **Route events** through your language's interpreter
4. **Render results** using RuenSH components

Example:

```rust
impl Component for MyLanguageRepl {
    type Message = ReplEvent;

    fn update(&mut self, message: ReplEvent) {
        match message {
            ReplEvent::Execute(code) => {
                // Execute in your language interpreter
                let result = self.interpreter.eval(&code);
                self.display_result(result);
            }
            _ => {}
        }
    }
}
```

---

For more examples, see the `examples/` directory or check the [full documentation](../MODAL_DOCUMENTATION.md).
