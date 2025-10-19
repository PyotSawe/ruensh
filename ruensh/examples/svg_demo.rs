//! SVG-Inspired Futuristic UI Demo
//! 
//! This example demonstrates the new SVG-inspired capabilities of RuenSH,
//! showcasing vector-like graphics rendering in the terminal using Unicode
//! characters, futuristic color schemes, and smooth animations.

use ruensh::svg::{SvgCanvas, ColorScheme};
use ruensh::terminal::Terminal;
use ruensh::events::{Event, EventHandler};
use ratatui::layout::Rect;
use ratatui::style::Color;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize terminal
    let mut terminal = Terminal::init()?;
    let mut event_handler = EventHandler::new();
    let _handler = event_handler.spawn_event_loop();

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

    loop {
        let elapsed = start_time.elapsed().as_secs_f32();

        // Auto-cycle themes every 5 seconds
        if last_theme_change.elapsed() > Duration::from_secs(5) {
            theme_index = (theme_index + 1) % themes.len();
            current_theme = &themes[theme_index];
            last_theme_change = Instant::now();
        }

        terminal.draw(|frame| {
            let area = frame.area();
            
            // Create main canvas
            let mut canvas = SvgCanvas::new(area.width, area.height);

            // Draw futuristic border
            draw_neon_border(&mut canvas, 0, 0, area.width, area.height, current_theme.1.primary());

            // Draw title
            let title = format!("╔═══ RUENSH SVG DEMO - {} ═══╗", current_theme.0);
            canvas.draw_text(
                (area.width / 2).saturating_sub(title.len() as u16 / 2),
                1,
                &title,
                Some(current_theme.1.glow()),
            );

            // Draw animated elements
            draw_pulse_circle(&mut canvas, 20, 8, 5, elapsed, current_theme.1.primary());
            draw_progress_bar(&mut canvas, 40, 8, 30, (elapsed % 3.0) / 3.0, current_theme.1.glow());
            draw_waveform(&mut canvas, 10, 15, 60, 5, elapsed, current_theme.1.glow());

            // Draw info panel
            draw_info_panel(&mut canvas, area.width - 35, 5, 32, 15, &current_theme.1);

            // Instructions
            canvas.draw_text(
                2,
                area.height - 2,
                "Press 'q' to quit | Themes auto-cycle every 5s",
                Some(Color::Gray),
            );

            // Render to frame
            canvas.render(frame, area);
        })?;

        // Handle events
        if let Some(event) = event_handler.next_event().await {
            if let Event::Key(key_event) = event {
                use crossterm::event::KeyCode;
                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char(' ') => {
                        // Manually cycle theme
                        theme_index = (theme_index + 1) % themes.len();
                        current_theme = &themes[theme_index];
                        last_theme_change = Instant::now();
                    }
                    _ => {}
                }
            }
        }

        tokio::time::sleep(Duration::from_millis(16)).await; // ~60 FPS
    }

    Terminal::restore()?;
    Ok(())
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
