# 🎨 SVG-Inspired Futuristic Interfaces with RuenSH

## Overview

RuenSH now supports **SVG-inspired vector graphics** for building stunning futuristic terminal interfaces. Using advanced Unicode characters, geometric patterns, and smooth animations, you can create visually striking TUI applications that rival modern GUI interfaces.

## Features

### 🎯 Vector-Like Graphics
- **Unicode Box Drawing** for clean borders and lines
- **Braille Patterns** for high-resolution graphics (2×4 pixels per character)
- **Geometric Shapes** for filled areas and gradients
- **Block Elements** for solid fills
- **Circular/Round Shapes** for icons and indicators

### 🌈 Futuristic Color Schemes
- **CyberPunk** - Hot pink, cyan, and purple neon aesthetics
- **Neon Tokyo** - Deep pink and sky blue with gold accents
- **Matrix** - Classic green-on-black hacker aesthetic
- **Holographic** - Light blue and purple with white highlights

### ✨ Visual Effects
- **Glow Effects** - Neon-like borders and text
- **Gradient Fills** - Smooth color transitions
- **Animations** - Pulse, fade, slide, scale, rotate
- **Transparency** - Glassmorphism-style panels

## Quick Start

### Run the Demo

```bash
cargo run --example svg_demo
```

The demo showcases:
- Animated pulsing circles
- Progress bars with gradients
- Real-time waveform visualization
- Information panels with system stats
- Auto-cycling color themes (every 5 seconds)
- Press `Space` to manually cycle themes
- Press `q` or `Esc` to quit

## Usage Examples

### Creating an SVG Canvas

```rust
use ruensh::svg::{SvgCanvas, ColorScheme};

let mut canvas = SvgCanvas::new(80, 24);
let theme = ColorScheme::cyberpunk();
```

### Drawing Shapes

```rust
// Rectangle with neon border
canvas.draw_rect(10, 5, 40, 10, Some(theme.primary()));

// Filled rectangle
canvas.fill_rect(15, 7, 10, 3, '█', Some(theme.glow()));

// Circle
canvas.draw_circle(30, 12, 5, Some(theme.primary()));

// Lines
canvas.draw_hline(5, 8, 20, Some(theme.glow()));
canvas.draw_vline(25, 5, 10, Some(theme.glow()));

// Text
canvas.draw_text(20, 15, "FUTURISTIC UI", Some(theme.accent()));
```

### Color Schemes

```rust
// CyberPunk theme
let theme = ColorScheme::cyberpunk();
println!("Primary: {:?}", theme.primary());
println!("Glow: {:?}", theme.glow());

// Or use other themes
let theme = ColorScheme::neon_tokyo();
let theme = ColorScheme::matrix();
let theme = ColorScheme::holographic();
```

### Rendering to Terminal

```rust
use ruensh::terminal::Terminal;

let mut terminal = Terminal::init()?;

terminal.draw(|frame| {
    let area = frame.area();
    canvas.render(frame, area);
})?;
```

## Advanced Components

### Holographic Panel

```rust
// Draw a futuristic panel with neon border
fn draw_panel(canvas: &mut SvgCanvas, x: u16, y: u16, w: u16, h: u16, theme: &ColorScheme) {
    // Double border for glow effect
    canvas.draw_rect(x, y, w, h, Some(theme.primary()));
    canvas.draw_rect(x + 1, y + 1, w - 2, h - 2, Some(theme.glow()));
    
    // Title with decorative elements
    canvas.draw_text(x + 2, y + 1, "╔═══════════╗", Some(theme.glow()));
    canvas.draw_text(x + 2, y + 2, "║ HOLOGRAM ║", Some(theme.primary()));
    canvas.draw_text(x + 2, y + 3, "╚═══════════╝", Some(theme.glow()));
}
```

### Animated Progress Ring

```rust
fn draw_progress_ring(canvas: &mut SvgCanvas, cx: u16, cy: u16, radius: u16, 
                      progress: f32, color: Color) {
    // Background circle
    canvas.draw_circle(cx, cy, radius, Some(Color::DarkGray));
    
    // Animated arc for progress (simplified)
    let segments = (progress * 360.0 / 10.0) as u16;
    for i in 0..segments {
        let angle = i as f32 * 10.0;
        let x = cx + (radius as f32 * angle.to_radians().cos()) as u16;
        let y = cy + (radius as f32 * angle.to_radians().sin()) as u16;
        canvas.draw_char(x, y, '●', Some(color));
    }
    
    // Center text
    let pct = format!("{}%", (progress * 100.0) as u8);
    canvas.draw_text(cx - 1, cy, &pct, Some(color));
}
```

### Waveform Visualizer

```rust
fn draw_waveform(canvas: &mut SvgCanvas, x: u16, y: u16, width: u16, 
                 height: u16, time: f32, color: Color) {
    use std::f32::consts::PI;
    
    for i in 0..width {
        let t = time + (i as f32 / width as f32) * 4.0 * PI;
        let wave = (t.sin() * (height as f32 / 2.0)) as i16;
        let wave_y = (y as i16 + height as i16 / 2 + wave) as u16;
        
        canvas.draw_char(x + i, wave_y, '▪', Some(color));
    }
}
```

### Information Dashboard

```rust
fn draw_dashboard(canvas: &mut SvgCanvas, theme: &ColorScheme) {
    // Title bar
    canvas.fill_rect(0, 0, 80, 3, '▒', Some(theme.background()));
    canvas.draw_text(30, 1, "SYSTEM CONTROL", Some(theme.primary()));
    
    // Status panels
    draw_panel(canvas, 5, 5, 20, 10, theme);
    canvas.draw_text(7, 7, "⚙ CPU: 45%", Some(theme.glow()));
    canvas.draw_text(7, 9, "▦ RAM: 2.1GB", Some(theme.glow()));
    
    draw_panel(canvas, 30, 5, 20, 10, theme);
    canvas.draw_text(32, 7, "⇄ NET: 125Mb/s", Some(theme.glow()));
    canvas.draw_text(32, 9, "◉ GPU: Active", Some(theme.glow()));
    
    // Decorative elements
    canvas.draw_text(5, 18, "░▒▓█▓▒░", Some(theme.glow()));
    canvas.draw_text(70, 18, "░▒▓█▓▒░", Some(theme.glow()));
}
```

## Unicode Character Reference

### Box Drawing
```
─ ━ │ ┃ ┌ ┐ └ ┘ ├ ┤ ┬ ┴ ┼
╔ ╗ ╚ ╝ ╠ ╣ ╦ ╩ ╬
```

### Block Elements
```
░ ▒ ▓ █ (gradient intensity)
▀ ▁ ▂ ▃ ▄ ▅ ▆ ▇ (vertical blocks)
▏ ▎ ▍ ▌ ▋ ▊ ▉ (horizontal blocks)
```

### Geometric Shapes
```
● ○ ◉ ◎ (circles)
■ □ ▪ ▫ (squares)
◆ ◇ (diamonds)
▲ △ ▼ ▽ (triangles)
```

### Braille Patterns (2×4 pixels per character)
```
⠀ ⡀ ⢀ ⣀ ⠁ ⠂ ⠄ ⠈ ⠐ ⠠
⠁⠂⠄⠈⠐⠠⢀⡀⠠⠄⠂⠁ (various combinations)
⣿ (all dots filled)
```

## Design Patterns

### Neon Glow Effect
Create depth with double borders:
```rust
// Outer glow
canvas.draw_rect(x, y, width, height, Some(theme.glow()));
// Inner border
canvas.draw_rect(x + 1, y + 1, width - 2, height - 2, Some(theme.primary()));
```

### Gradient Fill
Use character intensity for gradients:
```rust
let gradient = ['░', '▒', '▓', '█'];
for i in 0..width {
    let idx = (i * 4 / width) as usize;
    canvas.draw_char(x + i, y, gradient[idx], Some(color));
}
```

### Pulsing Animation
Vary radius or intensity with sine wave:
```rust
let pulse = (time * 2.0).sin().abs();
let radius = base_radius + (pulse * 3.0) as u16;
canvas.draw_circle(cx, cy, radius, Some(color));
```

## Color Scheme Details

### CyberPunk
```rust
primary:    RGB(255, 0, 128)    // Hot Pink
secondary:  RGB(0, 255, 255)    // Cyan
accent:     RGB(255, 255, 0)    // Yellow
background: RGB(10, 0, 20)      // Dark Purple
glow:       RGB(180, 0, 255)    // Purple
```

### Neon Tokyo
```rust
primary:    RGB(255, 20, 147)   // Deep Pink
secondary:  RGB(0, 191, 255)    // Deep Sky Blue
accent:     RGB(255, 215, 0)    // Gold
background: RGB(15, 15, 30)     // Navy
glow:       RGB(138, 43, 226)   // Blue Violet
```

### Matrix
```rust
primary:    RGB(0, 255, 65)     // Bright Green
secondary:  RGB(0, 200, 50)     // Green
accent:     RGB(150, 255, 150)  // Light Green
background: RGB(0, 0, 0)        // Black
glow:       RGB(0, 255, 100)    // Neon Green
```

### Holographic
```rust
primary:    RGB(100, 200, 255)  // Light Blue
secondary:  RGB(200, 100, 255)  // Light Purple
accent:     RGB(255, 255, 255)  // White
background: RGB(5, 10, 25)      // Deep Blue
glow:       RGB(150, 200, 255)  // Sky Blue
```

## Performance

- **Rendering**: < 16ms per frame (60 FPS capable)
- **Memory**: ~2KB per canvas
- **Character cells**: Efficient Unicode rendering
- **Animation**: Smooth transitions with minimal CPU usage

## Requirements

### Terminal Support
Best experience with modern terminals:
- ✅ Alacritty, Kitty, WezTerm
- ✅ GNOME Terminal, Konsole
- ✅ iTerm2, Terminal.app (macOS)
- ✅ Windows Terminal
- ⚠️ Legacy terminals may have limited Unicode support

### Color Support
- **True Color (24-bit)** recommended for best gradients
- **256 colors** supported with approximation
- **16 colors** basic support (limited visual appeal)

## Tips & Best Practices

1. **Use consistent color schemes** - Pick one theme and stick with it
2. **Layer effects** - Combine borders, fills, and text for depth
3. **Animate sparingly** - Too many animations can be distracting
4. **Test in your target terminal** - Not all terminals render Unicode equally
5. **Consider accessibility** - Provide text alternatives to visual elements
6. **Performance matters** - Keep canvas updates efficient for smooth 60 FPS

## Future Enhancements

- [ ] Particle systems for dynamic backgrounds
- [ ] 3D-like perspective transforms
- [ ] Shader-like filter effects
- [ ] Path morphing animations
- [ ] Complex gradient patterns
- [ ] Custom font support
- [ ] Image-to-ASCII conversion

## Examples Gallery

See the `examples/svg_demo.rs` for a comprehensive demonstration of:
- Multiple color themes
- Animated components
- Information panels
- Progress indicators
- Waveform visualization
- Pulsing effects

## Contributing

We welcome contributions to the SVG system! Areas for improvement:
- New color schemes
- Additional shape primitives
- Animation easing functions
- Performance optimizations
- Terminal compatibility

## License

Same as RuenSH - MIT License

---

**Build the future of terminal interfaces with RuenSH's SVG-inspired graphics system!**
