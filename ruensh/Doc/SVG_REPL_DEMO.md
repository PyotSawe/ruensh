# SVG Demo with REPL Integration

The `svg_demo` example now includes a fully integrated REPL (Read-Eval-Print Loop) with pretty-print features, demonstrating how to combine RuenSH's SVG-inspired graphics with interactive command-line interfaces.

## Quick Start

```bash
cargo run --example svg_demo
```

## Two Modes

### 1. Visualizer Mode (Default)

A futuristic dashboard showcasing SVG-inspired graphics:

- **Animated Pulse Circle**: Smooth breathing effect with scaling
- **Progress Bar**: Animated loading bar with percentage display
- **Waveform Visualization**: Real-time sine wave animation
- **System Status Panel**: Live metrics display
- **Auto-Cycling Themes**: Changes every 5 seconds
  - CyberPunk (purple/magenta)
  - Neon Tokyo (pink/cyan)
  - Matrix (green)
  - Holographic (blue/cyan)

### 2. REPL Mode (Press Tab)

An interactive command-line interface with:

- **Pretty-Printed Output**: Syntax-highlighted results with type annotations
- **Command History**: Navigate with ↑/↓ arrows
- **Live Input Editing**: Full cursor support (←/→, Home/End)
- **Scrolling Output**: Automatically shows last 50 lines
- **Real-time Stats**: History count and output line tracking

## REPL Features

### Supported Commands

```lisp
(+ 1 2 3)              ; => 6 [type: Integer]
(def pi 3.14159)       ; => #'user/pi [Var defined]
(map inc [1 2 3])      ; => (2 3 4) [type: List]
help                   ; Show command reference
```

### Pretty Printing

The REPL uses color-coded output:

- **Green**: Input prompt `λ>`
- **White**: User input
- **Cyan**: Numeric results
- **Yellow**: Variable definitions
- **Magenta**: List/collection results
- **Dark Gray**: Type annotations and metadata
- **Red**: Error messages (future feature)

### Input Editing

| Key | Action |
|-----|--------|
| **Character keys** | Insert character at cursor |
| **Backspace** | Delete character before cursor |
| **Delete** | Delete character at cursor |
| **←/→** | Move cursor left/right |
| **Home** | Move to start of line |
| **End** | Move to end of line |
| **↑** | Previous command in history |
| **↓** | Next command in history |
| **Enter** | Execute command |

## Architecture

The demo showcases best practices for building TUI REPLs:

```
┌─────────────────────────────┐
│      ReplState              │
│  - input: String            │
│  - cursor_pos: usize        │
│  - history: Vec<String>     │
│  - output: Vec<(String,     │
│              Color)>        │
│  - mode: ReplMode           │
└──────────┬──────────────────┘
           │
┌──────────▼──────────────────┐
│  Event Handling             │
│  - Keyboard input           │
│  - Mode switching           │
│  - Theme cycling            │
└──────────┬──────────────────┘
           │
┌──────────▼──────────────────┐
│  Rendering                  │
│  - SvgCanvas for graphics   │
│  - Pretty-printed output    │
│  - Input with cursor        │
└─────────────────────────────┘
```

## Code Structure

### Core Components

1. **ReplState**: Manages input, history, and output
   - Tracks cursor position for editing
   - Maintains command history
   - Stores colored output for pretty printing

2. **Expression Evaluator**: Simple demo evaluator
   - Pattern matches on Lisp-like syntax
   - Returns colored results with type info
   - Extensible for real language implementations

3. **Dual-Mode Rendering**:
   - `draw_visualizer_mode()`: SVG animations
   - `draw_repl_mode()`: REPL interface

### Key Functions

```rust
// State management
impl ReplState {
    fn add_char(&mut self, c: char)
    fn backspace(&mut self)
    fn move_cursor_left/right(&mut self)
    fn history_up/down(&mut self)
    fn submit(&mut self)
    fn eval_expr(&self, expr: &str) -> Vec<(String, Color)>
}

// Rendering
fn draw_repl_mode(canvas: &mut SvgCanvas, ...)
fn draw_visualizer_mode(canvas: &mut SvgCanvas, ...)
```

## Extending the REPL

To integrate your own language:

1. **Replace the evaluator**:
   ```rust
   fn eval_expr(&self, expr: &str) -> Vec<(String, Color)> {
       match your_parser::parse(expr) {
           Ok(ast) => {
               let result = your_eval::eval(ast);
               pretty_print(result)
           }
           Err(e) => vec![(format!("Error: {}", e), Color::Red)]
       }
   }
   ```

2. **Add syntax highlighting**:
   ```rust
   fn highlight_input(&self) -> Vec<(String, Color)> {
       // Tokenize and color-code input
   }
   ```

3. **Implement completion**:
   ```rust
   fn complete(&self, partial: &str) -> Vec<String> {
       // Return completion suggestions
   }
   ```

4. **Add multi-line support**:
   ```rust
   fn is_complete(&self) -> bool {
       // Check if expression is complete
   }
   ```

## Performance Notes

- Runs at ~60 FPS (16ms frame time)
- Output scrolling shows only visible lines
- History limited to prevent memory growth
- Efficient redrawing using ratatui's buffer system

## Future Enhancements

- [ ] Multi-line input support
- [ ] Tab completion
- [ ] Syntax highlighting in input
- [ ] Parentheses matching
- [ ] Export/import history
- [ ] Configurable key bindings
- [ ] Custom color themes via config file
- [ ] Error highlighting and suggestions
- [ ] Documentation tooltips
- [ ] Session saving/loading

## Integration with Real Languages

This demo serves as a template for integrating RuenSH with real language REPLs:

- **Clojure**: Use `clojure-rs` or shell out to JVM
- **Python**: Integrate with `rustpython` or `pyo3`
- **Lisp**: Use `risp` or custom Lisp interpreter
- **JavaScript**: Integrate with `boa` or `rusty_v8`
- **Scheme**: Use `oxischeme` or custom interpreter

See `REPL_INTEGRATION_GUIDE.md` for detailed language integration examples.

## Troubleshooting

**Issue**: Cursor not visible
- **Solution**: Ensure terminal supports Unicode block characters (▮)

**Issue**: Colors not showing
- **Solution**: Use a terminal with true color support

**Issue**: Input lag
- **Solution**: Reduce polling interval or optimize evaluator

**Issue**: Themes not cycling
- **Solution**: Only works in Visualizer mode; press Space manually or wait 5s

## Related Files

- `examples/svg_demo.rs` - Full implementation
- `src/svg/canvas.rs` - SVG canvas implementation
- `src/svg/color_schemes.rs` - Color theme definitions
- `REPL_INTEGRATION_GUIDE.md` - Language integration guide
- `SVG_USAGE_GUIDE.md` - SVG graphics guide
