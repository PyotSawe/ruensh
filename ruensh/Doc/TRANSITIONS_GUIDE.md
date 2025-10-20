# Transition Effects & Keyframe Animations

The RuenSH framework now includes a comprehensive transition system with keyframe-based animations, making components highly reactive and visually appealing.

## Overview

The transition system provides smooth, customizable animations using:
- **Keyframes**: Define animation states at specific time offsets
- **Easing Functions**: Control animation acceleration/deceleration
- **Interpolation**: Smooth transitions between values
- **Presets**: Ready-to-use common animations

## Quick Start

```rust
use ruensh::svg::{Transition, TransitionPresets, Keyframe, Easing};
use std::time::Duration;

// Create a fade-in transition
let mut fade = TransitionPresets::fade_in(Duration::from_millis(500));
fade.start();

// Update in your render loop
if let Some(alpha) = fade.update() {
    // Use alpha (0.0 to 1.0) to fade your component
}
```

## Features

### ✨ Built-in Transition Presets

#### 1. **Fade Effects**
```rust
// Fade in from transparent to opaque
let fade_in = TransitionPresets::fade_in(Duration::from_millis(500));

// Fade out from opaque to transparent
let fade_out = TransitionPresets::fade_out(Duration::from_millis(500));
```

#### 2. **Slide Effects**
```rust
// Slide from left
let slide = TransitionPresets::slide_from_left(Duration::from_millis(300), 50);
```

#### 3. **Scale Effects**
```rust
// Bounce scale (overshoots then settles)
let bounce = TransitionPresets::bounce_scale(Duration::from_millis(600));

// Elastic entrance (bouncy spring effect)
let elastic = TransitionPresets::elastic_entrance(Duration::from_millis(800));

// Zoom in
let zoom = TransitionPresets::zoom_in(Duration::from_millis(400));
```

#### 4. **Continuous Effects**
```rust
// Pulse effect (infinite loop between min/max)
let pulse = TransitionPresets::pulse(Duration::from_secs(2), 0.8, 1.2);

// Rainbow color cycle (infinite)
let rainbow = TransitionPresets::rainbow_cycle(Duration::from_secs(10));
```

#### 5. **Shake Effect**
```rust
// Horizontal shake (for errors/alerts)
let shake = TransitionPresets::shake_horizontal(Duration::from_millis(400), 5);
```

### 🎯 Custom Keyframe Animations

Create your own complex animations:

```rust
use ruensh::svg::{Transition, Keyframe, Easing};

let transition = Transition::new(
    Duration::from_secs(2),
    vec![
        Keyframe::new(0.0, 0.0).with_easing(Easing::EaseOut),
        Keyframe::new(0.3, 1.5).with_easing(Easing::Bounce),
        Keyframe::new(0.7, 0.8).with_easing(Easing::EaseInOut),
        Keyframe::new(1.0, 1.0),
    ],
);
```

### 📊 Easing Functions

Control the acceleration curve:

| Easing | Description | Best For |
|--------|-------------|----------|
| `Linear` | Constant speed | Progress bars, mechanical motion |
| `EaseIn` | Slow start, fast end | Exiting elements |
| `EaseOut` | Fast start, slow end | Entering elements |
| `EaseInOut` | Slow start/end | Smooth, natural motion |
| `Bounce` | Bounces at the end | Fun, playful effects |
| `Elastic` | Spring-like overshoot | Attention-grabbing entrances |

### 🔄 Looping & Control

```rust
// Infinite loop
let mut transition = pulse.with_loop(None);

// Loop N times
let mut transition = fade.with_loop(Some(3));

// Reverse on complete (ping-pong)
let mut transition = slide.with_reverse(true);

// Control playback
transition.start();
transition.pause();
transition.resume();
transition.reset();

// Check state
if transition.is_running() {
    // ...
}
```

### 🎨 Interpolatable Types

The system supports animating various types:

```rust
// Numbers
Transition::<f32>::from_to(duration, 0.0, 1.0, easing);
Transition::<u16>::from_to(duration, 0, 100, easing);
Transition::<i16>::from_to(duration, -50, 50, easing);

// Colors (smooth color transitions)
Transition::<Color>::from_to(
    duration,
    Color::Rgb(255, 0, 0),    // Red
    Color::Rgb(0, 0, 255),    // Blue
    easing
);

// Positions (2D coordinates)
Transition::<(u16, u16)>::from_to(
    duration,
    (0, 0),
    (100, 50),
    easing
);
```

## Integration Examples

### Example 1: Animated Button

```rust
struct AnimatedButton {
    scale_transition: Transition<f32>,
    color_transition: Transition<Color>,
    is_hovered: bool,
}

impl AnimatedButton {
    fn on_hover(&mut self) {
        self.scale_transition = Transition::from_to(
            Duration::from_millis(200),
            1.0,
            1.1,
            Easing::EaseOut
        );
        self.scale_transition.start();
        
        self.color_transition = TransitionPresets::color_shift(
            Duration::from_millis(200),
            Color::Blue,
            Color::Cyan
        );
        self.color_transition.start();
    }
    
    fn render(&mut self, canvas: &mut SvgCanvas) {
        let scale = self.scale_transition.update().unwrap_or(&1.0);
        let color = self.color_transition.update().unwrap_or(&Color::Blue);
        
        // Draw button with animated scale and color
        draw_button(canvas, *scale, *color);
    }
}
```

### Example 2: Modal with Entrance Animation

```rust
struct Modal {
    fade_in: Transition<f32>,
    slide_in: Transition<i16>,
    is_visible: bool,
}

impl Modal {
    fn show(&mut self) {
        self.fade_in = TransitionPresets::fade_in(Duration::from_millis(300));
        self.fade_in.start();
        
        self.slide_in = TransitionPresets::slide_from_left(
            Duration::from_millis(400),
            50
        );
        self.slide_in.start();
        
        self.is_visible = true;
    }
    
    fn render(&mut self, canvas: &mut SvgCanvas) {
        if !self.is_visible {
            return;
        }
        
        let alpha = self.fade_in.update().unwrap_or(&1.0);
        let offset = self.slide_in.update().unwrap_or(&0);
        
        // Apply alpha to colors
        let bg_color = apply_alpha(Color::Rgb(0, 0, 0), *alpha * 0.8);
        
        // Draw modal with offset
        canvas.draw_rect(
            10 + *offset as u16,
            10,
            60,
            20,
            Some(bg_color)
        );
    }
}
```

### Example 3: Reactive Component State

```rust
struct ReactiveComponent {
    state: ComponentState,
    transition: Transition<f32>,
}

enum ComponentState {
    Idle,
    Loading,
    Success,
    Error,
}

impl ReactiveComponent {
    fn set_state(&mut self, new_state: ComponentState) {
        self.state = new_state;
        
        // Trigger different transitions based on state
        match new_state {
            ComponentState::Loading => {
                // Infinite pulse
                self.transition = TransitionPresets::pulse(
                    Duration::from_millis(1000),
                    0.5,
                    1.0
                );
            }
            ComponentState::Success => {
                // Bounce in
                self.transition = TransitionPresets::bounce_scale(
                    Duration::from_millis(500)
                );
            }
            ComponentState::Error => {
                // Shake
                self.transition = TransitionPresets::shake_horizontal(
                    Duration::from_millis(300),
                    3
                );
            }
            _ => {}
        }
        
        self.transition.start();
    }
}
```

## SVG Demo Implementation

The `svg_demo` example showcases transitions in action:

### Visualizer Mode Features

1. **Rainbow Title**: Uses `rainbow_cycle` for continuously shifting colors
2. **Pulse Circle**: Scales smoothly between 0.8x and 1.2x using keyframes
3. **Waveform**: Animated with rainbow color transition
4. **Transition Indicators**: Live display of active transition values

### REPL Mode Features

1. **Slide-in Effect**: Content slides from left when switching modes
2. **Fade-in**: Smooth fade when entering REPL mode
3. **Pulsing Cursor**: Cursor opacity pulses for better visibility
4. **Reactive Feedback**: Visual feedback on state changes

## Performance Considerations

- **Efficient Updates**: Transitions only update when running
- **Minimal Allocations**: Reuses keyframe data
- **60 FPS Ready**: Designed for smooth 60Hz rendering
- **Lazy Evaluation**: Only calculates values when requested

## Advanced Usage

### Custom Interpolation

Implement `Interpolate` trait for custom types:

```rust
use ruensh::svg::Interpolate;

#[derive(Clone)]
struct Transform {
    x: f32,
    y: f32,
    rotation: f32,
    scale: f32,
}

impl Interpolate for Transform {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            x: self.x.lerp(&other.x, t),
            y: self.y.lerp(&other.y, t),
            rotation: self.rotation.lerp(&other.rotation, t),
            scale: self.scale.lerp(&other.scale, t),
        }
    }
}

// Now you can animate transforms!
let transform_transition = Transition::from_to(
    Duration::from_millis(500),
    Transform { x: 0.0, y: 0.0, rotation: 0.0, scale: 1.0 },
    Transform { x: 100.0, y: 50.0, rotation: 45.0, scale: 1.5 },
    Easing::EaseInOut
);
```

### Chaining Transitions

```rust
struct TransitionChain {
    transitions: Vec<Transition<f32>>,
    current_index: usize,
}

impl TransitionChain {
    fn update(&mut self) -> Option<f32> {
        if self.current_index >= self.transitions.len() {
            return None;
        }
        
        let transition = &mut self.transitions[self.current_index];
        let value = transition.update();
        
        if transition.state() == TransitionState::Complete {
            self.current_index += 1;
            if self.current_index < self.transitions.len() {
                self.transitions[self.current_index].start();
            }
        }
        
        value.cloned()
    }
}
```

### Synchronized Transitions

```rust
use ruensh::svg::TransitionManager;

let mut manager = TransitionManager::new();

// Add multiple transitions
manager.add("fade", fade_transition);
manager.add("slide", slide_transition);
manager.add("scale", scale_transition);

// Update all at once
manager.update_all();
```

## Testing Transitions

```bash
# Run the demo to see transitions in action
cargo run --example svg_demo

# Controls:
# - Tab: Switch modes (watch slide & fade transitions)
# - Observe: Pulsing circle, rainbow colors, animated cursor
```

## API Reference

### Core Types

- `Transition<T>`: Main animation controller
- `Keyframe<T>`: Animation state at specific time offset
- `TransitionState`: Idle, Running, Complete, Paused
- `TransitionPresets`: Pre-built common animations
- `Easing`: Animation timing functions
- `Interpolate`: Trait for animatable types

### Methods

```rust
// Creation
Transition::new(duration, keyframes)
Transition::from_to(duration, from, to, easing)

// Control
.start()
.pause()
.resume()
.reset()

// Configuration
.with_loop(count)
.with_reverse(bool)

// State
.update() -> Option<&T>
.value_at(progress) -> T
.state() -> TransitionState
.is_running() -> bool
```

## Future Enhancements

- [ ] Transition callbacks (on_start, on_complete)
- [ ] Timeline-based animation sequences
- [ ] Bezier curve easing
- [ ] Path-based motion
- [ ] Physics-based springs
- [ ] Gesture-driven transitions
- [ ] Transition presets library expansion

## Related Files

- `src/svg/transitions.rs` - Core implementation
- `src/svg/animations.rs` - Easing functions
- `examples/svg_demo.rs` - Live demonstration
- `SVG_REPL_DEMO.md` - REPL integration guide
