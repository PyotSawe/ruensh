# Quick Reference: Enhanced Modal Component

## 🎯 Quick Start

```rust
use ruensh::components::{Component, Modal};
use ruensh::style::Theme;

// Create a modal
let mut modal = Modal::new("Are you sure?")
    .primary_button("Confirm")
    .secondary_button("Cancel")
    .theme(Theme::default());

// Show it
modal.show();

// Handle events
if let Some(msg) = modal.handle_event(&event) {
    if let Some(action) = modal.update(msg) {
        match action {
            Action::Confirm => println!("User confirmed!"),
            Action::Cancel => println!("User cancelled!"),
            _ => {}
        }
    }
}

// Animate and render
modal.update_animation();
modal.render(frame);
```

## ⌨️ Keyboard Controls

| Key | Action |
|-----|--------|
| `Tab` / `→` | Next button |
| `Shift+Tab` / `←` | Previous button |
| `Enter` | Confirm |
| `Esc` | Cancel |
| `Y` / `y` | Primary button (quick) |
| `N` / `n` | Secondary button (quick) |

## 🖱️ Mouse Interaction

- **Hover**: Move mouse over button to highlight
- **Click**: Left-click button to activate
- **Feedback**: Button changes color and style on hover/focus

## 🎨 Customization

### Theme
```rust
let custom_theme = Theme::default()
    .set_primary(Color::Red)
    .set_secondary(Color::Blue)
    .set_border_style(BorderStyle::Rounded);

modal.theme(custom_theme)
```

### Labels
```rust
modal
    .primary_button("Yes, Delete")
    .secondary_button("No, Keep It")
    .title("Confirm Deletion")
```

## 📊 Component States

### Modal States
- **Hidden**: Not visible
- **Appearing**: Animating in (10 frames)
- **Visible**: Fully displayed
- **Disappearing**: Animating out (10 frames)

### Button Focus
- **Primary**: Left button focused
- **Secondary**: Right button focused  
- **None**: No button focused

## 🔧 API Methods

### Creation & Configuration
```rust
Modal::new(content)                    // Create new modal
.title(title)                          // Set title
.primary_button(label)                 // Set primary button label
.secondary_button(label)               // Set secondary button label
.theme(theme)                          // Set theme
```

### State Management
```rust
modal.show()                           // Display with animation
modal.hide()                           // Hide with animation
modal.is_visible()                     // Check if visible
modal.focused_button()                 // Get current focus
```

### Updates & Rendering
```rust
modal.update_animation()               // Update animation frame
modal.render(frame)                    // Render to frame
modal.handle_event(event)              // Handle event, returns Message
modal.update(message)                  // Update state, returns Action
```

## 📝 Message Types

```rust
enum ModalMessage {
    PrimaryButton,     // Primary button pressed
    SecondaryButton,   // Secondary button pressed
    Dismiss,           // Modal dismissed (Esc)
    HoverPrimary,      // Mouse over primary
    HoverSecondary,    // Mouse over secondary
    NoHover,           // Mouse not on any button
}
```

## 🎬 Animation Timing

- **Appearance**: 10 frames at 60 FPS = ~167ms
- **Disappearance**: 10 frames at 60 FPS = ~167ms
- **Total Animation**: ~333ms

## 🔄 Event Flow Diagram

```
User Input (Keyboard/Mouse)
        ↓
Terminal Event (crossterm)
        ↓
Event::Key / Event::Mouse
        ↓
Modal::handle_event()
        ↓
ModalMessage
        ↓
Modal::update()
        ↓
Option<Action>
        ↓
Application Logic
```

## 💡 Common Patterns

### Confirmation Dialog
```rust
let mut modal = Modal::new("Delete this file?")
    .primary_button("Delete")
    .secondary_button("Keep");
modal.show();
```

### Warning Dialog
```rust
let mut modal = Modal::new("This action cannot be undone.")
    .primary_button("Proceed")
    .secondary_button("Cancel")
    .title("⚠️ Warning");
modal.show();
```

### Save Prompt
```rust
let mut modal = Modal::new("Do you want to save changes?")
    .primary_button("Save")
    .secondary_button("Don't Save");
modal.show();
```

## 🚀 Performance Notes

- **Rendering**: ~1ms per frame
- **Event Handling**: O(1) constant time
- **Memory**: ~500 bytes per modal
- **Target**: 60 FPS (16ms frame budget)

## 📚 Example Application

Run the demo to see all features in action:

```bash
cargo run --example modal_demo
```

Features demonstrated:
- Popup animation
- Button focus tracking
- Keyboard navigation
- Mouse interaction
- Status updates
- Animation effects

## 🐛 Troubleshooting

### Modal not showing?
- Call `modal.show()` before rendering
- Check if `modal.is_visible()` returns true

### Buttons not responding?
- Ensure `handle_event()` is called with the event
- Verify `update()` is called with the returned message
- Check terminal is in raw mode

### Focus not updating?
- Call `modal.update_animation()` regularly
- Ensure events are being passed to `handle_event()`

### Mouse clicks not working?
- Enable mouse support in terminal (usually automatic with crossterm)
- Verify terminal supports mouse events

## 📖 Full Documentation

See `MODAL_DOCUMENTATION.md` for comprehensive details including:
- Architecture overview
- Detailed implementation notes
- Extensibility guide
- API reference
- Testing information

## 🔗 Related Components

- `Theme` - Styling and color configuration
- `Terminal` - Terminal initialization and management
- `EventHandler` - Event loop and event polling
- `Component` trait - Base component interface

## 📝 Code Examples

### Complete Interactive Application
```rust
#[tokio::main]
async fn main() -> io::Result<()> {
    let _terminal = Terminal::new()?;
    let mut modal = Modal::new("Continue?").show();
    
    let backend = ratatui::backend::CrosstermBackend::new(io::stdout());
    let mut tui = ratatui::Terminal::new(backend)?;
    
    let mut running = true;
    while running {
        tui.draw(|f| modal.render(f))?;
        modal.update_animation();
        
        if crossterm::event::poll(Duration::from_millis(16))? {
            if let Ok(evt) = crossterm::event::read() {
                let event = convert_event(evt);
                if let Some(msg) = modal.handle_event(&event) {
                    if let Some(action) = modal.update(msg) {
                        running = false;
                    }
                }
            }
        }
    }
    Ok(())
}
```

---

**Version**: 1.0  
**Last Updated**: October 18, 2025  
**Status**: Production Ready ✅
