# RuenSH - Enhanced TUI Library Documentation

## Overview

RuenSH is a powerful, ergonomic Terminal User Interface (TUI) library in Rust, inspired by Charm's Bubble Tea ecosystem. This document details the implementation of the enhanced Modal component with robust event handling.

## Recent Enhancements

### 1. Popup Modal Capability

The Modal component now features smooth popup animations:

#### Features:
- **State Management**: Four modal states (`Hidden`, `Appearing`, `Visible`, `Disappearing`)
- **Animation Frames**: 10-frame animation transitions for smooth appearance/disappearance
- **Visibility Calculation**: Opacity-based rendering during animations
- **Show/Hide Methods**: Easy-to-use API for controlling modal visibility

```rust
let mut modal = Modal::new("Are you sure?");
modal.show();  // Smooth popup animation
modal.hide();  // Smooth disappear animation
```

### 2. Robust Event Handling

The Modal component handles all terminal events with sophisticated logic:

#### Keyboard Events:
- **Navigation Keys**:
  - `Tab`: Cycle forward through buttons
  - `Shift+Tab`: Cycle backward through buttons
  - `Left/Right Arrows`: Navigate between buttons
  
- **Activation Keys**:
  - `Enter`: Confirm the currently focused button
  - `Esc`: Dismiss the modal (cancel action)
  
- **Quick Keys**:
  - `Y/y`: Immediately activate primary button
  - `N/n`: Immediately activate secondary button

#### Mouse Events:
- **Hover Detection**: Track mouse position and update button focus state
- **Click Detection**: Detect clicks on button areas and trigger appropriate actions
- **Coordinate Tracking**: Maintains last mouse position for precise interaction

### 3. Button Hover Highlighting

Buttons provide visual feedback for both keyboard and mouse interaction:

#### Styles:
- **Normal State**: Colored text `[Button]`
- **Hovered/Focused State**: Inverted colors with background highlight ` Button `
- **Pressed State**: Temporarily highlighted during click

#### Visual Changes:
```
Normal:    [ Yep! ]        [ Nope ]
Focused:    Yep!            Nope
```

### 4. Scalable Event System

The event handling is built on a flexible foundation:

#### Event Types:
```rust
pub enum Event {
    Key(KeyEvent),        // Keyboard input
    Mouse(MouseEvent),    // Mouse input
    Resize(u16, u16),    // Terminal resize
    Tick,                // Periodic update
}
```

#### Message Flow:
1. `Event` → Terminal input
2. `Component::handle_event()` → Parse and identify action
3. `ModalMessage` → Semantic message type
4. `Component::update()` → Update state and return `Action`
5. `Action` → Application-level response

### 5. Button Focus Management

#### ButtonFocus Enum:
```rust
pub enum ButtonFocus {
    Primary,      // Primary button (left)
    Secondary,    // Secondary button (right)
    None,         // No button focused
}
```

#### Focus Tracking:
- Maintained in Modal state
- Updated on keyboard navigation
- Updated on mouse hover
- Used for rendering visual feedback

## Architecture

### Modal Component Structure

```
Modal
├── Title & Content
├── Theme Configuration
├── State Management
│   ├── focused_button: ButtonFocus
│   ├── modal_state: ModalState
│   └── animation_frame: u8
├── Mouse Tracking
│   ├── last_mouse_x: u16
│   └── last_mouse_y: u16
└── Event Handling
    ├── Keyboard Navigation
    ├── Mouse Interaction
    └── Animation Updates
```

### Component Trait

All components implement the standard trait:

```rust
pub trait Component {
    type Message;
    
    fn update(&mut self, msg: Self::Message) -> Option<Action>;
    fn render(&self, frame: &mut Frame<'_>);
    fn handle_event(&mut self, event: &Event) -> Option<Self::Message>;
}
```

## Usage Examples

### Basic Modal with All Features

```rust
// Create modal
let mut modal = Modal::new("Are you sure you want to quit?")
    .title("")
    .primary_button("Yep!")
    .secondary_button("Nope")
    .theme(Theme::default());

// Show with animation
modal.show();

// Handle events
if let Some(msg) = modal.handle_event(&event) {
    if let Some(action) = modal.update(msg) {
        match action {
            Action::Confirm => { /* User pressed primary */ }
            Action::Cancel => { /* User pressed secondary */ }
            _ => {}
        }
    }
}

// Animate if needed
modal.update_animation();

// Render
modal.render(frame);
```

### Keyboard Navigation Flow

```
User presses Tab
    ↓
Event::Key(KeyCode::Tab)
    ↓
handle_event() switches focused_button
    ↓
render() highlights new button
    ↓
User presses Enter
    ↓
Event::Key(KeyCode::Enter)
    ↓
handle_event() returns ModalMessage::PrimaryButton
    ↓
update() returns Action::Confirm
```

### Mouse Interaction Flow

```
User moves mouse to button
    ↓
Event::Mouse(MouseEvent { row: y, column: x, ... })
    ↓
handle_event() checks if (x,y) is on button
    ↓
Updates focused_button to ButtonFocus::Primary
    ↓
render() highlights button
    ↓
User clicks
    ↓
MouseEventKind::Down
    ↓
handle_event() returns ModalMessage::PrimaryButton
    ↓
update() returns Action::Confirm
```

## Event Handling Details

### Mouse Event Processing

```rust
match mouse_event.kind {
    MouseEventKind::Down(_) | MouseEventKind::Up(_) => {
        // Detect which button was clicked
        // Return appropriate ModalMessage
    }
    MouseEventKind::Moved => {
        // Update button focus based on mouse position
        // Return HoverPrimary, HoverSecondary, or NoHover
    }
    _ => {}
}
```

### Button Area Calculation

The modal calculates button positions dynamically:
- **Modal Width**: 60 characters (clamped to terminal)
- **Modal Height**: 14 characters (clamped to terminal)
- **Button Y Position**: `modal_y + modal_height - 4`
- **Button X Positions**: Calculated based on label lengths

## Animation System

### Modal Appearance Animation

```
Frame 0-9 (Appearing)
  ↓
visibility = frame / 10.0  (0.0 → 1.0)
  ↓
Frame 10+
  ↓
modal_state = ModalState::Visible
```

### Modal Disappearance Animation

```
Frame 0-9 (Disappearing)
  ↓
visibility = (10 - frame) / 10.0  (1.0 → 0.0)
  ↓
Frame 10+
  ↓
modal_state = ModalState::Hidden
```

## Performance Considerations

1. **Event Dispatch**: O(1) complexity for event routing
2. **Mouse Hit Testing**: O(1) button area checks
3. **Rendering**: Only re-renders on state changes
4. **Animation**: Smooth 60 FPS with 16ms frame budget
5. **Memory**: Minimal allocations in hot paths

## Extensibility

The Modal component is designed for extension:

### Adding New Buttons:
```rust
// Currently supports 2 buttons, can be extended to support:
// - Three or more buttons
// - Button icons
// - Disabled states
```

### Custom Themes:
```rust
let custom_theme = Theme::default()
    .set_primary(Color::Red)
    .set_secondary(Color::Blue)
    .set_border_style(BorderStyle::Double);
```

### Message Types:
Additional `ModalMessage` variants can be added for new interactions:
- Double-click detection
- Long-press detection
- Scroll wheel handling

## Example Application

See `examples/modal_demo.rs` for a complete working example demonstrating:
- Modal popup animation
- Button focus indication
- Keyboard navigation (Tab, Y/N keys)
- Mouse hover and click detection
- Status message updates
- Spinning animation indicator

### Run the Example:
```bash
cargo run --example modal_demo
```

### Interact:
- **Keyboard**: Tab to navigate, Enter to confirm, Esc to cancel, Y/N for quick selection
- **Mouse**: Hover over buttons to highlight, click to select
- Watch the button focus indicator update with your interactions

## Future Enhancements

1. **Multiple Button Support**: Extend to support 3+ buttons
2. **Input Validation**: Add form validation helpers
3. **Async Operations**: Support long-running operations during modal display
4. **Custom Layouts**: Allow custom button arrangements
5. **Accessibility**: Improve screen reader support

## API Reference

### Modal Methods

```rust
impl Modal {
    pub fn new(content: impl Into<String>) -> Self
    pub fn primary_button(self, label: impl Into<String>) -> Self
    pub fn secondary_button(self, label: impl Into<String>) -> Self
    pub fn title(self, title: impl Into<String>) -> Self
    pub fn theme(self, theme: Theme) -> Self
    pub fn show(&mut self)
    pub fn hide(&mut self)
    pub fn is_visible(&self) -> bool
    pub fn focused_button(&self) -> ButtonFocus
    pub fn update_animation(&mut self)
    pub fn render_centered(&self, frame: &mut Frame, area: Rect)
    pub fn render_buttons(&self, frame: &mut Frame, area: Rect)
}

impl Component for Modal {
    fn update(&mut self, msg: ModalMessage) -> Option<Action>
    fn render(&self, frame: &mut Frame<'_>)
    fn handle_event(&mut self, event: &Event) -> Option<ModalMessage>
}
```

### Enums

```rust
pub enum ModalMessage {
    PrimaryButton,
    SecondaryButton,
    Dismiss,
    HoverPrimary,
    HoverSecondary,
    NoHover,
}

pub enum ButtonFocus {
    Primary,
    Secondary,
    None,
}

pub enum ModalState {
    Hidden,
    Appearing,
    Visible,
    Disappearing,
}
```

## Testing

The modal component includes comprehensive event handling:

```rust
#[cfg(test)]
mod tests {
    // Keyboard navigation tests
    // Mouse interaction tests
    // State transition tests
    // Animation frame tests
}
```

## Conclusion

The RuenSH Modal component provides a production-ready, user-friendly interface for confirmation dialogs with:
- Smooth popup animations
- Robust keyboard and mouse event handling
- Visual feedback for all interactions
- Scalable and extensible architecture
- Minimal performance overhead

This implementation demonstrates the power of Rust's type system and event-driven architecture for building responsive terminal applications.
