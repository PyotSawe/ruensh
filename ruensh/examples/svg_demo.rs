//! SVG-Inspired Futuristic UI Demo with REPL Integration
//! 
//! This example demonstrates the new SVG-inspired capabilities of RuenSH,
//! showcasing vector-like graphics rendering in the terminal using Unicode
//! characters, futuristic color schemes, smooth animations, and an integrated
//! REPL with pretty-print features.
//! 
//! ## Features:
//! 
//! ### Visualizer Mode (Default)
//! - Animated pulse circles with smooth scaling
//! - Progress bars with percentage display
//! - Live waveform visualization
//! - System status panel
//! - Auto-cycling color themes (CyberPunk, Neon Tokyo, Matrix, Holographic)
//! 
//! ### REPL Mode (Press Tab)
//! - Interactive command-line interface
//! - Pretty-printed output with syntax highlighting
//! - Command history navigation (↑/↓ arrows)
//! - Input editing with cursor support
//! - Type hints and metadata display
//! - Demo evaluator supporting:
//!   - `(+ 1 2 3)` - Arithmetic operations
//!   - `(def name value)` - Variable definitions
//!   - `(map fn list)` - Functional operations
//!   - `help` - Command reference
//! 
//! ## Controls:
//! - **Tab**: Switch between Visualizer and REPL modes
//! - **Space**: Manually cycle themes (Visualizer mode)
//! - **q/Esc**: Quit
//! - **Enter**: Execute command (REPL mode)
//! - **↑/↓**: Navigate history (REPL mode)
//! - **←/→/Home/End**: Move cursor (REPL mode)
//! - **Backspace/Delete**: Edit input (REPL mode)

use ruensh::svg::{SvgCanvas, ColorScheme, Transition, TransitionPresets, Keyframe, Easing};
use ruensh::terminal::Terminal;
use ruensh::events::{EventHandler, start_event_loop};
use ratatui::style::Color;
use std::time::{Duration, Instant};
use std::io;

/// REPL state for managing input and history
struct ReplState {
    input: String,
    cursor_pos: usize,
    history: Vec<String>,
    history_index: Option<usize>,
    output: Vec<(String, Color)>, // (text, color) pairs for pretty printing
    mode: ReplMode,
    // Transition effects
    mode_transition: Transition<f32>,
    pulse_transition: Transition<f32>,
    color_transition: Transition<Color>,
    slide_transition: Transition<i16>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ReplMode {
    Visualizer,  // Show SVG animations
    Repl,        // Show REPL interface
}

impl ReplState {
    fn new() -> Self {
        let mut mode_transition = TransitionPresets::fade_in(Duration::from_millis(500));
        mode_transition.start();
        
        let mut pulse_transition = TransitionPresets::pulse(Duration::from_secs(2), 0.8, 1.2);
        pulse_transition.start();
        
        let mut color_transition = TransitionPresets::rainbow_cycle(Duration::from_secs(10));
        color_transition.start();
        
        Self {
            input: String::new(),
            cursor_pos: 0,
            history: vec![
                "(+ 1 2 3)".to_string(),
                "(def pi 3.14159)".to_string(),
                "(map inc [1 2 3])".to_string(),
            ],
            history_index: None,
            output: vec![
                ("Welcome to RuenSH REPL!".to_string(), Color::Cyan),
                ("Type expressions and press Enter".to_string(), Color::Gray),
                ("Press Tab to toggle visualizer mode".to_string(), Color::Yellow),
            ],
            mode: ReplMode::Visualizer,
            mode_transition,
            pulse_transition,
            color_transition,
            slide_transition: Transition::new(Duration::from_millis(0), vec![Keyframe::new(0.0, 0)]),
        }
    }

    fn add_char(&mut self, c: char) {
        self.input.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }

    fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.input.remove(self.cursor_pos);
        }
    }

    fn delete(&mut self) {
        if self.cursor_pos < self.input.len() {
            self.input.remove(self.cursor_pos);
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.input.len() {
            self.cursor_pos += 1;
        }
    }

    fn move_cursor_home(&mut self) {
        self.cursor_pos = 0;
    }

    fn move_cursor_end(&mut self) {
        self.cursor_pos = self.input.len();
    }

    fn history_up(&mut self) {
        if self.history.is_empty() {
            return;
        }
        match self.history_index {
            None => {
                self.history_index = Some(self.history.len() - 1);
                self.input = self.history[self.history.len() - 1].clone();
            }
            Some(idx) if idx > 0 => {
                self.history_index = Some(idx - 1);
                self.input = self.history[idx - 1].clone();
            }
            _ => {}
        }
        self.cursor_pos = self.input.len();
    }

    fn history_down(&mut self) {
        match self.history_index {
            Some(idx) if idx < self.history.len() - 1 => {
                self.history_index = Some(idx + 1);
                self.input = self.history[idx + 1].clone();
            }
            Some(_) => {
                self.history_index = None;
                self.input.clear();
            }
            None => {}
        }
        self.cursor_pos = self.input.len();
    }

    fn submit(&mut self) {
        if self.input.trim().is_empty() {
            return;
        }

        let input = self.input.clone();
        self.history.push(input.clone());
        self.history_index = None;

        // Evaluate expression and pretty print
        self.output.push((format!("> {}", input), Color::Green));
        let result = self.eval_expr(&input);
        self.output.extend(result);

        // Keep only last 50 output lines
        if self.output.len() > 50 {
            self.output.drain(0..self.output.len() - 50);
        }

        self.input.clear();
        self.cursor_pos = 0;
    }

    fn eval_expr(&self, expr: &str) -> Vec<(String, Color)> {
        // Simple expression evaluator for demo purposes
        let expr = expr.trim();
        
        if expr.starts_with("(+ ") && expr.ends_with(")") {
            // Simple addition
            let inner = &expr[3..expr.len()-1];
            let nums: Vec<i32> = inner.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if !nums.is_empty() {
                let sum: i32 = nums.iter().sum();
                return vec![
                    (format!("  => {}", sum), Color::Cyan),
                    (format!("  [type: Integer]"), Color::DarkGray),
                ];
            }
        } else if expr.starts_with("(def ") {
            // Variable definition
            return vec![
                (format!("  => #'user/{}", expr.split_whitespace().nth(1).unwrap_or("var")), Color::Yellow),
                (format!("  [Var defined]"), Color::DarkGray),
            ];
        } else if expr.starts_with("(map ") {
            // Map function demo
            return vec![
                ("  => (2 3 4)".to_string(), Color::Magenta),
                ("  [type: List]".to_string(), Color::DarkGray),
            ];
        } else if expr.starts_with("(help") || expr == "help" {
            return vec![
                ("  Available commands:".to_string(), Color::Cyan),
                ("    (+ a b c ...)  - Add numbers".to_string(), Color::Gray),
                ("    (def name val) - Define variable".to_string(), Color::Gray),
                ("    (map fn list)  - Map function".to_string(), Color::Gray),
                ("    help           - Show this help".to_string(), Color::Gray),
            ];
        }

        vec![
            (format!("  => '{}'", expr), Color::White),
            (format!("  [type: Symbol]"), Color::DarkGray),
        ]
    }

    fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            ReplMode::Visualizer => ReplMode::Repl,
            ReplMode::Repl => ReplMode::Visualizer,
        };
        
        // Start slide transition
        let mut slide = TransitionPresets::slide_from_left(
            Duration::from_millis(300),
            50
        );
        slide.start();
        self.slide_transition = slide;
        
        // Restart mode fade transition
        self.mode_transition = TransitionPresets::fade_in(Duration::from_millis(500));
        self.mode_transition.start();
    }
    
    fn update_transitions(&mut self) {
        self.mode_transition.update();
        self.pulse_transition.update();
        self.color_transition.update();
        self.slide_transition.update();
    }
    
    // Get current transition values (non-mutating)
    fn get_pulse_scale(&self) -> f32 {
        self.pulse_transition.current_value.clone().unwrap_or(1.0)
    }
    
    fn get_rainbow_color(&self) -> Color {
        self.color_transition.current_value.clone().unwrap_or(Color::White)
    }
    
    fn get_slide_offset(&self) -> i16 {
        self.slide_transition.current_value.clone().unwrap_or(0)
    }
    
    fn get_fade_alpha(&self) -> f32 {
        self.mode_transition.current_value.clone().unwrap_or(1.0)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize terminal
    let _terminal = Terminal::new()?;
    let (_event_handler, tx) = EventHandler::new();
    start_event_loop(tx).await;

    // Color schemes to cycle through
    let themes = vec![
        ("CyberPunk", ColorScheme::cyberpunk()),
        ("Neon Tokyo", ColorScheme::neon_tokyo()),
        ("Matrix", ColorScheme::matrix()),
        ("Holographic", ColorScheme::holographic()),
    ];
    let mut theme_index = 0;
    let mut current_theme = &themes[theme_index];

    let start_time = Instant::now();
    let mut last_theme_change = Instant::now();
    let mut repl_state = ReplState::new();

    let backend = ratatui::backend::CrosstermBackend::new(io::stdout());
    let mut tui = ratatui::Terminal::new(backend)?;

    loop {
        let elapsed = start_time.elapsed().as_secs_f32();
        
        // Update all transitions
        repl_state.update_transitions();

        // Auto-cycle themes every 5 seconds in visualizer mode
        if repl_state.mode == ReplMode::Visualizer && last_theme_change.elapsed() > Duration::from_secs(5) {
            theme_index = (theme_index + 1) % themes.len();
            current_theme = &themes[theme_index];
            last_theme_change = Instant::now();
        }

        tui.draw(|frame| {
            let area = frame.area();
            
            // Create main canvas
            let mut canvas = SvgCanvas::new(area.width, area.height);

            match repl_state.mode {
                ReplMode::Visualizer => {
                    draw_visualizer_mode(&mut canvas, area.width, area.height, elapsed, current_theme, &mut repl_state);
                }
                ReplMode::Repl => {
                    draw_repl_mode(&mut canvas, area.width, area.height, &repl_state, current_theme);
                }
            }

            // Draw mode indicator and instructions with fade effect
            let fade_alpha = repl_state.get_fade_alpha();
            let mode_text = match repl_state.mode {
                ReplMode::Visualizer => "MODE: Visualizer",
                ReplMode::Repl => "MODE: REPL",
            };
            
            // Apply fade effect to mode text color
            let mode_color = if fade_alpha < 1.0 {
                let base = current_theme.1.glow();
                apply_alpha_to_color(base, fade_alpha)
            } else {
                current_theme.1.glow()
            };
            
            canvas.draw_text(2, area.height - 3, mode_text, Some(mode_color));
            
            canvas.draw_text(
                2,
                area.height - 2,
                "Tab: Switch mode | Space: Change theme | q: Quit",
                Some(Color::Gray),
            );

            // Render to frame
            canvas.render(frame, area);
        })?;

        // Handle events
        if crossterm::event::poll(Duration::from_millis(16))? {
            if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
                use crossterm::event::KeyCode;
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Tab => {
                        repl_state.toggle_mode();
                    }
                    KeyCode::Char(' ') if repl_state.mode == ReplMode::Visualizer => {
                        // Manually cycle theme in visualizer mode
                        theme_index = (theme_index + 1) % themes.len();
                        current_theme = &themes[theme_index];
                        last_theme_change = Instant::now();
                    }
                    // REPL input handling
                    KeyCode::Char(c) if repl_state.mode == ReplMode::Repl => {
                        repl_state.add_char(c);
                    }
                    KeyCode::Backspace if repl_state.mode == ReplMode::Repl => {
                        repl_state.backspace();
                    }
                    KeyCode::Delete if repl_state.mode == ReplMode::Repl => {
                        repl_state.delete();
                    }
                    KeyCode::Left if repl_state.mode == ReplMode::Repl => {
                        repl_state.move_cursor_left();
                    }
                    KeyCode::Right if repl_state.mode == ReplMode::Repl => {
                        repl_state.move_cursor_right();
                    }
                    KeyCode::Home if repl_state.mode == ReplMode::Repl => {
                        repl_state.move_cursor_home();
                    }
                    KeyCode::End if repl_state.mode == ReplMode::Repl => {
                        repl_state.move_cursor_end();
                    }
                    KeyCode::Up if repl_state.mode == ReplMode::Repl => {
                        repl_state.history_up();
                    }
                    KeyCode::Down if repl_state.mode == ReplMode::Repl => {
                        repl_state.history_down();
                    }
                    KeyCode::Enter if repl_state.mode == ReplMode::Repl => {
                        repl_state.submit();
                    }
                    _ => {}
                }
            }
        }

        tokio::time::sleep(Duration::from_millis(16)).await; // ~60 FPS
    }

    Ok(())
}

/// Apply alpha (fade) to color
fn apply_alpha_to_color(color: Color, alpha: f32) -> Color {
    let alpha = alpha.clamp(0.0, 1.0);
    match color {
        Color::Rgb(r, g, b) => {
            Color::Rgb(
                (r as f32 * alpha) as u8,
                (g as f32 * alpha) as u8,
                (b as f32 * alpha) as u8,
            )
        }
        _ => color,
    }
}

/// Draw the visualizer mode with animations and transitions
fn draw_visualizer_mode(canvas: &mut SvgCanvas, width: u16, height: u16, elapsed: f32, theme: &(&str, ColorScheme), repl: &ReplState) {
    // Get transition values
    let pulse_scale = repl.get_pulse_scale();
    let rainbow_color = repl.get_rainbow_color();
    
    // Draw futuristic border
    draw_neon_border(canvas, 0, 0, width, height, theme.1.primary());

    // Draw title with rainbow effect
    let title = format!("╔═══ RUENSH SVG DEMO - {} ═══╗", theme.0);
    canvas.draw_text(
        (width / 2).saturating_sub(title.len() as u16 / 2),
        1,
        &title,
        Some(rainbow_color),
    );

    // Draw animated elements with transitions
    // Pulse circle with scale transition
    let base_radius = 5;
    let scaled_radius = (base_radius as f32 * pulse_scale) as u16;
    draw_pulse_circle(canvas, 20, 8, scaled_radius, elapsed, theme.1.primary());
    
    // Progress bar with smooth animation
    draw_progress_bar(canvas, 40, 8, 30, (elapsed % 3.0) / 3.0, theme.1.glow());
    
    // Waveform with color transition
    draw_waveform(canvas, 10, 15, 60, 5, elapsed, rainbow_color);

    // Draw info panel
    draw_info_panel(canvas, width - 35, 5, 32, 15, &theme.1);
    
    // Show transition indicators
    let trans_y = height / 2 + 5;
    canvas.draw_text(2, trans_y, "┌─ ACTIVE TRANSITIONS ─┐", Some(Color::Cyan));
    canvas.draw_text(2, trans_y + 1, &format!("│ Pulse: {:.2}x", pulse_scale), Some(Color::Gray));
    canvas.draw_text(2, trans_y + 2, "│ Rainbow: Active", Some(Color::Gray));
    canvas.draw_text(2, trans_y + 3, &format!("│ Keyframes: Running"), Some(Color::Gray));
    canvas.draw_text(2, trans_y + 4, "└────────────────────┘", Some(Color::Cyan));
}

/// Draw the REPL mode with input and output
fn draw_repl_mode(canvas: &mut SvgCanvas, width: u16, height: u16, repl: &ReplState, theme: &(&str, ColorScheme)) {
    // Get slide transition value
    let slide_offset = repl.get_slide_offset();
    let fade = repl.get_fade_alpha();
    let pulse = repl.get_pulse_scale();
    
    // Draw border
    draw_neon_border(canvas, 0, 0, width, height, theme.1.primary());

    // Draw title with slide effect
    let title = format!("╔═══ RUENSH REPL - {} ═══╗", theme.0);
    let title_x = (width / 2).saturating_sub(title.len() as u16 / 2);
    canvas.draw_text(
        title_x.saturating_add_signed(slide_offset),
        1,
        &title,
        Some(theme.1.glow()),
    );

    // Draw output area with slide
    let output_start_y = 3;
    let output_height = height.saturating_sub(10);
    let output_x = 2_u16.saturating_add_signed(slide_offset);
    
    canvas.draw_rect(output_x, output_start_y, width - 4, output_height, Some(Color::DarkGray));
    canvas.draw_text(output_x + 1, output_start_y, "OUTPUT", Some(theme.1.glow()));
    
    // Draw output lines (scrolled to show most recent)
    let visible_lines = (output_height as usize).saturating_sub(3);
    let start_idx = repl.output.len().saturating_sub(visible_lines);
    
    for (i, (line, color)) in repl.output.iter().skip(start_idx).enumerate() {
        let y = output_start_y + 2 + i as u16;
        if y < output_start_y + output_height - 1 {
            // Truncate line if too long
            let max_len = (width as usize).saturating_sub(8);
            let display_line = if line.len() > max_len {
                format!("{}...", &line[..max_len - 3])
            } else {
                line.clone()
            };
            canvas.draw_text(output_x + 2, y, &display_line, Some(*color));
        }
    }

    // Draw input area with fade-in effect
    let input_y = height.saturating_sub(7);
    let input_color = apply_alpha_to_color(theme.1.primary(), fade);
    
    canvas.draw_rect(2, input_y, width - 4, 4, Some(input_color));
    canvas.draw_text(3, input_y, "INPUT", Some(theme.1.glow()));
    
    // Draw prompt
    canvas.draw_text(3, input_y + 1, "λ> ", Some(Color::Green));
    
    // Draw input text with cursor
    let input_x = 6;
    canvas.draw_text(input_x, input_y + 1, &repl.input, Some(Color::White));
    
    // Draw animated cursor (pulsing)
    let cursor_x = input_x + repl.cursor_pos as u16;
    if cursor_x < width - 4 {
        let pulse_alpha = ((pulse - 0.8) / 0.4).clamp(0.5, 1.0); // Normalize 0.8-1.2 to 0.5-1.0
        let cursor_color = apply_alpha_to_color(theme.1.glow(), pulse_alpha);
        canvas.draw_char(cursor_x, input_y + 1, '▮', Some(cursor_color));
    }

    // Draw input hint
    let hint = "↑↓: History | ←→: Move | Enter: Execute | Backspace: Delete";
    let hint_y = input_y + 2;
    canvas.draw_text(4, hint_y, hint, Some(Color::DarkGray));

    // Draw stats with slide
    let stats = format!("History: {} | Output lines: {}", repl.history.len(), repl.output.len());
    canvas.draw_text(width - stats.len() as u16 - 3, input_y, &stats, Some(Color::DarkGray));
}

/// Draw a neon-style border
fn draw_neon_border(canvas: &mut SvgCanvas, x: u16, y: u16, width: u16, height: u16, color: Color) {
    canvas.draw_rect(x, y, width, height, Some(color));
    
    // Add glow effect with double border
    if width > 4 && height > 4 {
        canvas.draw_rect(x + 1, y + 1, width - 2, height - 2, Some(color));
    }
}

/// Draw an animated pulsing circle
fn draw_pulse_circle(canvas: &mut SvgCanvas, cx: u16, cy: u16, base_radius: u16, time: f32, color: Color) {
    let pulse = (time * 2.0).sin().abs();
    let radius = base_radius + (pulse * 2.0) as u16;
    
    canvas.draw_circle(cx, cy, radius, Some(color));
    
    // Add label
    canvas.draw_text(cx - 4, cy + radius + 2, "PULSE", Some(color));
}

/// Draw an animated progress bar
fn draw_progress_bar(canvas: &mut SvgCanvas, x: u16, y: u16, width: u16, progress: f32, color: Color) {
    // Background
    canvas.draw_rect(x, y, width, 3, Some(Color::DarkGray));
    
    // Fill
    let fill_width = (width as f32 * progress) as u16;
    if fill_width > 0 {
        canvas.fill_rect(x + 1, y + 1, fill_width.saturating_sub(2), 1, '█', Some(color));
    }
    
    // Percentage
    let pct_text = format!("{}%", (progress * 100.0) as u8);
    canvas.draw_text(x + width / 2 - 2, y + 1, &pct_text, Some(Color::White));
    
    // Label
    canvas.draw_text(x, y - 1, "LOADING...", Some(color));
}

/// Draw an animated waveform
fn draw_waveform(canvas: &mut SvgCanvas, x: u16, y: u16, width: u16, height: u16, time: f32, color: Color) {
    use std::f32::consts::PI;
    
    // Draw waveform container
    canvas.draw_text(x, y - 1, "WAVEFORM ANALYSIS", Some(color));
    canvas.draw_rect(x, y, width, height, Some(Color::DarkGray));
    
    // Draw sine wave
    for i in 0..width - 2 {
        let t = time + (i as f32 / width as f32) * 4.0 * PI;
        let wave_height = (t.sin() * (height as f32 - 2.0) / 2.0) as i16;
        let wave_y = (y as i16 + height as i16 / 2 + wave_height) as u16;
        
        // Use different characters for wave intensity
        let ch = if wave_height.abs() < 1 { '─' } else { '▪' };
        canvas.draw_char(x + 1 + i, wave_y, ch, Some(color));
    }
}

/// Draw an information panel
fn draw_info_panel(canvas: &mut SvgCanvas, x: u16, y: u16, width: u16, height: u16, scheme: &ColorScheme) {
    // Panel border
    canvas.draw_rect(x, y, width, height, Some(scheme.primary()));
    
    // Title
    canvas.draw_text(x + 2, y + 1, "╔═ SYSTEM STATUS ═╗", Some(scheme.glow()));
    
    // Content with icons
    let items = vec![
        ("⚙ CPU", "45%", 3),
        ("▦ RAM", "2.1GB", 5),
        ("⇄ NET", "125Mb/s", 7),
        ("◉ GPU", "Active", 9),
        ("⚡ PWR", "Normal", 11),
    ];
    
    for (label, value, row) in items {
        canvas.draw_text(x + 2, y + row, label, Some(scheme.primary()));
        canvas.draw_text(x + width - value.len() as u16 - 2, y + row, value, Some(scheme.glow()));
    }
    
    // Decorative elements
    canvas.draw_text(x + 2, y + height - 2, "░▒▓▒░", Some(scheme.glow()));
    canvas.draw_text(x + width - 7, y + height - 2, "░▒▓▒░", Some(scheme.glow()));
}
