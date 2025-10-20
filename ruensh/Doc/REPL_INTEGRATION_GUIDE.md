# Integrating RuenSH with Your Language's REPL

This guide explains how to use RuenSH to build a production-ready REPL CLI for your language (like Clojure, Python, Lisp, etc.).

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Basic Integration](#basic-integration)
3. [Complete REPL Implementation](#complete-repl-implementation)
4. [Advanced Features](#advanced-features)
5. [Testing Your REPL](#testing-your-repl)

## Architecture Overview

Your REPL architecture should follow this pattern:

```
┌─────────────────────────────────────────┐
│         Your Language Interpreter       │
│   (AST evaluation, error handling)      │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│         REPL Application Layer          │
│   (Command processing, state mgmt)      │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│         RuenSH UI Components            │
│   (Modal, Input, List, etc)             │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│    Terminal (ratatui + crossterm)       │
│   (Low-level rendering and input)       │
└─────────────────────────────────────────┘
```

## Basic Integration

### Step 1: Define Your Language's REPL State

```rust
use ruensh::components::{Component, Modal};
use ruensh::events::{Event, EventHandler};

pub struct LanguageRepl {
    // Your language's interpreter/context
    interpreter: Interpreter,
    
    // REPL state
    input_history: Vec<String>,
    current_input: String,
    output: String,
    
    // UI components
    modal: Option<Modal>,
    focus: ReplFocus,
}

#[derive(Debug, Clone, Copy)]
enum ReplFocus {
    Input,
    History,
    Modal,
}
```

### Step 2: Implement Component Messages

```rust
pub enum ReplMessage {
    // Input events
    InputChar(char),
    InputBackspace,
    InputSubmit,
    InputClear,
    
    // Navigation
    HistoryUp,
    HistoryDown,
    
    // Modal responses
    ConfirmExit,
    CancelExit,
    
    // Output
    DisplayResult(String),
    DisplayError(String),
}
```

### Step 3: Process Commands in Your Language

```rust
impl LanguageRepl {
    fn execute_command(&mut self, code: &str) -> Result<String, String> {
        // Parse code using your language's parser
        match self.interpreter.parse(code) {
            Ok(ast) => {
                // Evaluate AST
                match self.interpreter.eval(ast) {
                    Ok(result) => {
                        // Format result for display
                        Ok(self.format_result(result))
                    }
                    Err(e) => Err(format!("Runtime error: {}", e))
                }
            }
            Err(e) => Err(format!("Parse error: {}", e))
        }
    }

    fn format_result(&self, value: Value) -> String {
        // Format your language's values for display
        format!("{}", value)
    }
}
```

## Complete REPL Implementation

Here's a complete, minimal REPL implementation:

### File: `src/main.rs`

```rust
mod repl;
use repl::LanguageRepl;
use ruensh::events::EventHandler;
use ruensh::terminal::Terminal;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Initialize terminal
    let mut terminal = Terminal::init()?;
    
    // Create REPL instance
    let mut repl = LanguageRepl::new();
    
    // Setup event handler
    let (mut event_handler, _tx) = EventHandler::new();
    
    // Main REPL loop
    loop {
        // Render
        terminal.draw(|f| {
            repl.render(f.area(), f);
        })?;

        // Handle events
        if let Some(event) = event_handler.next_event() {
            if let Some(msg) = repl.handle_event(&event) {
                if !repl.update(msg) {
                    // Exit requested
                    break;
                }
            }
        }
    }

    // Cleanup
    Terminal::restore()?;
    println!("Goodbye!");
    Ok(())
}
```

### File: `src/repl.rs`

```rust
use ruensh::components::{Component, Modal};
use ruensh::events::Event;
use ruensh::state::Action;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

pub struct LanguageRepl {
    input_buffer: String,
    cursor_pos: usize,
    history: Vec<String>,
    output: String,
    status: String,
}

pub enum ReplMessage {
    InputChar(char),
    InputBackspace,
    InputSubmit,
    HistoryUp,
    HistoryDown,
    ClearScreen,
}

impl LanguageRepl {
    pub fn new() -> Self {
        Self {
            input_buffer: String::new(),
            cursor_pos: 0,
            history: Vec::new(),
            output: "Welcome to Your Language REPL\nType :help for commands\n\n".to_string(),
            status: "Ready".to_string(),
        }
    }

    pub fn execute(&mut self, code: &str) -> Result<String, String> {
        // TODO: Call your language's interpreter
        // For now, echo the input
        Ok(code.to_string())
    }
}

impl Component for LanguageRepl {
    type Message = ReplMessage;

    fn update(&mut self, msg: Self::Message) -> Option<Action> {
        match msg {
            ReplMessage::InputChar(ch) => {
                self.input_buffer.insert(self.cursor_pos, ch);
                self.cursor_pos += 1;
            }
            ReplMessage::InputBackspace => {
                if self.cursor_pos > 0 {
                    self.input_buffer.remove(self.cursor_pos - 1);
                    self.cursor_pos -= 1;
                }
            }
            ReplMessage::InputSubmit => {
                let code = self.input_buffer.trim().to_string();
                if !code.is_empty() {
                    self.history.push(code.clone());
                    match self.execute(&code) {
                        Ok(result) => {
                            self.output.push_str(&format!("> {}\n{}\n\n", code, result));
                        }
                        Err(err) => {
                            self.output.push_str(&format!("> {}\nError: {}\n\n", code, err));
                        }
                    }
                    self.input_buffer.clear();
                    self.cursor_pos = 0;
                    self.status = "Executed".to_string();
                }
            }
            ReplMessage::HistoryUp => {
                if !self.history.is_empty() && self.history.len() > 0 {
                    if self.input_buffer.is_empty() {
                        self.input_buffer = self.history.last().unwrap().clone();
                        self.cursor_pos = self.input_buffer.len();
                    }
                }
            }
            ReplMessage::ClearScreen => {
                self.output.clear();
            }
        }
        None
    }

    fn render(&self, _area: Rect, f: &mut Frame) {
        // Render your REPL UI
        let output_para = Paragraph::new(self.output.clone())
            .block(Block::default().borders(Borders::ALL).title("Output"));
        f.render_widget(output_para, _area);
    }

    fn handle_event(&mut self, event: &Event) -> Option<Self::Message> {
        use crossterm::event::{KeyCode, KeyEvent};

        match event {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char(ch) => {
                    return Some(ReplMessage::InputChar(*ch));
                }
                KeyCode::Backspace => {
                    return Some(ReplMessage::InputBackspace);
                }
                KeyCode::Enter => {
                    return Some(ReplMessage::InputSubmit);
                }
                KeyCode::Up => {
                    return Some(ReplMessage::HistoryUp);
                }
                _ => {}
            },
            _ => {}
        }
        None
    }
}
```

## Advanced Features

### Feature 1: Command History with Completion

```rust
impl LanguageRepl {
    fn get_history_matches(&self, prefix: &str) -> Vec<String> {
        self.history
            .iter()
            .filter(|cmd| cmd.starts_with(prefix))
            .cloned()
            .collect()
    }

    fn autocomplete(&mut self) {
        let matches = self.get_history_matches(&self.input_buffer);
        if matches.len() == 1 {
            self.input_buffer = matches[0].clone();
            self.cursor_pos = self.input_buffer.len();
        }
    }
}
```

### Feature 2: Error Display with Modal

```rust
impl LanguageRepl {
    fn show_error(&mut self, error: String) {
        self.modal = Some(
            Modal::new("Error")
                .content(&error)
                .primary_button("OK")
        );
    }
}
```

### Feature 3: Multi-line Input

```rust
pub struct MultilineInput {
    lines: Vec<String>,
    current_line: usize,
}

impl MultilineInput {
    fn is_complete(&self) -> bool {
        // Check if input forms complete expression in your language
        true
    }
}
```

### Feature 4: Syntax Highlighting

```rust
use ratatui::text::{Line, Span};
use ratatui::style::{Color, Style};

impl LanguageRepl {
    fn highlight_code(&self, code: &str) -> Vec<Span> {
        let mut spans = vec![];
        for token in self.tokenize(code) {
            let style = match token.kind {
                TokenKind::Keyword => Style::default().fg(Color::Blue),
                TokenKind::String => Style::default().fg(Color::Green),
                TokenKind::Number => Style::default().fg(Color::Cyan),
                _ => Style::default(),
            };
            spans.push(Span::styled(token.text, style));
        }
        spans
    }
}
```

## Testing Your REPL

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_evaluation() {
        let mut repl = LanguageRepl::new();
        let result = repl.execute("(+ 1 2)").unwrap();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_history_tracking() {
        let mut repl = LanguageRepl::new();
        repl.execute("(+ 1 2)").unwrap();
        assert_eq!(repl.history.len(), 1);
    }

    #[test]
    fn test_error_handling() {
        let mut repl = LanguageRepl::new();
        let result = repl.execute("(undefined-function)");
        assert!(result.is_err());
    }
}
```

### Integration Tests

```bash
# Test REPL with interactive input
cargo run

# Test examples
cargo run --example modal_demo
```

### Performance Testing

```rust
use std::time::Instant;

#[test]
fn test_performance() {
    let mut repl = LanguageRepl::new();
    let start = Instant::now();
    
    for i in 0..1000 {
        repl.execute(&format!("(+ {} 1)", i)).unwrap();
    }
    
    let elapsed = start.elapsed();
    println!("Evaluated 1000 expressions in {:?}", elapsed);
    assert!(elapsed.as_millis() < 1000, "Performance regression!");
}
```

## REPL Best Practices

### 1. **Error Handling**
- Catch panics from interpreter
- Show user-friendly error messages
- Suggest corrections when possible

### 2. **Performance**
- Cache compiled code when appropriate
- Lazy-load heavy features
- Monitor memory usage

### 3. **User Experience**
- Clear prompts and status messages
- Consistent keyboard shortcuts
- Responsive UI (< 100ms input latency)

### 4. **Documentation**
- Built-in `:help` command
- Contextual hints
- Example commands on startup

### 5. **Configuration**
- Loadable init files
- Customizable themes
- Settable preferences

## Common Patterns

### Pattern 1: Command Routing

```rust
fn handle_command(&mut self, cmd: &str) -> Result<String, String> {
    match cmd.trim() {
        ":help" => Ok(self.get_help()),
        ":history" => Ok(self.show_history()),
        ":clear" => { self.output.clear(); Ok(String::new()) }
        _ => self.execute_code(cmd),
    }
}
```

### Pattern 2: Settings Management

```rust
#[derive(Serialize, Deserialize)]
pub struct ReplSettings {
    pub prompt: String,
    pub theme: String,
    pub syntax_highlighting: bool,
}

impl ReplSettings {
    fn load_from_file() -> Self { /* ... */ }
    fn save_to_file(&self) { /* ... */ }
}
```

### Pattern 3: Plugin System

```rust
pub trait ReplPlugin {
    fn name(&self) -> &str;
    fn execute(&self, code: &str) -> Result<String, String>;
}
```

---

## Next Steps

1. **Clone the template** from this guide
2. **Integrate your language's interpreter**
3. **Add language-specific features**
4. **Test thoroughly with real workloads**
5. **Publish and share!**

For more examples, see:
- `COMPONENT_GUIDE.md` - Building custom components
- `examples/modal_demo.rs` - Interactive components
- `examples/repl_cli.rs` - REPL scaffolding
- GitHub issues - Community templates and examples

Happy REPL building! 🚀
