//! Canvas for rendering SVG-inspired graphics

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders};
use ratatui::style::{Color, Style};
use super::Resolution;

/// SVG-inspired canvas for rendering vector graphics in terminal
pub struct SvgCanvas {
    width: u16,
    height: u16,
    resolution: Resolution,
    /// Buffer for character rendering
    buffer: Vec<Vec<char>>,
    /// Color buffer
    colors: Vec<Vec<Option<Color>>>,
}

impl SvgCanvas {
    /// Create a new canvas
    pub fn new(width: u16, height: u16) -> Self {
        let buffer = vec![vec![' '; width as usize]; height as usize];
        let colors = vec![vec![None; width as usize]; height as usize];
        
        Self {
            width,
            height,
            resolution: Resolution::CharCell,
            buffer,
            colors,
        }
    }

    /// Set resolution mode
    pub fn resolution(mut self, resolution: Resolution) -> Self {
        self.resolution = resolution;
        self
    }

    /// Clear the canvas
    pub fn clear(&mut self) {
        for row in &mut self.buffer {
            for cell in row {
                *cell = ' ';
            }
        }
        for row in &mut self.colors {
            for cell in row {
                *cell = None;
            }
        }
    }

    /// Draw a character at position
    pub fn draw_char(&mut self, x: u16, y: u16, ch: char, color: Option<Color>) {
        if x < self.width && y < self.height {
            self.buffer[y as usize][x as usize] = ch;
            self.colors[y as usize][x as usize] = color;
        }
    }

    /// Draw a horizontal line using box-drawing characters
    pub fn draw_hline(&mut self, x: u16, y: u16, length: u16, color: Option<Color>) {
        for i in 0..length {
            self.draw_char(x + i, y, '─', color);
        }
    }

    /// Draw a vertical line using box-drawing characters
    pub fn draw_vline(&mut self, x: u16, y: u16, length: u16, color: Option<Color>) {
        for i in 0..length {
            self.draw_char(x, y + i, '│', color);
        }
    }

    /// Draw a rectangle using box-drawing characters
    pub fn draw_rect(&mut self, x: u16, y: u16, width: u16, height: u16, color: Option<Color>) {
        if width < 2 || height < 2 {
            return;
        }

        // Corners
        self.draw_char(x, y, '┌', color);
        self.draw_char(x + width - 1, y, '┐', color);
        self.draw_char(x, y + height - 1, '└', color);
        self.draw_char(x + width - 1, y + height - 1, '┘', color);

        // Edges
        self.draw_hline(x + 1, y, width - 2, color);
        self.draw_hline(x + 1, y + height - 1, width - 2, color);
        self.draw_vline(x, y + 1, height - 2, color);
        self.draw_vline(x + width - 1, y + 1, height - 2, color);
    }

    /// Draw a filled rectangle
    pub fn fill_rect(&mut self, x: u16, y: u16, width: u16, height: u16, fill_char: char, color: Option<Color>) {
        for dy in 0..height {
            for dx in 0..width {
                self.draw_char(x + dx, y + dy, fill_char, color);
            }
        }
    }

    /// Draw a circle using Braille patterns or block characters
    pub fn draw_circle(&mut self, cx: u16, cy: u16, radius: u16, color: Option<Color>) {
        // Simple circle using block characters
        let r = radius as i32;
        for dy in -r..=r {
            for dx in -r..=r {
                let dist_sq = dx * dx + dy * dy;
                let r_sq = r * r;
                
                if dist_sq <= r_sq && dist_sq > (r - 1) * (r - 1) {
                    let x = (cx as i32 + dx) as u16;
                    let y = (cy as i32 + dy) as u16;
                    self.draw_char(x, y, '●', color);
                }
            }
        }
    }

    /// Draw text
    pub fn draw_text(&mut self, x: u16, y: u16, text: &str, color: Option<Color>) {
        for (i, ch) in text.chars().enumerate() {
            self.draw_char(x + i as u16, y, ch, color);
        }
    }

    /// Render the canvas to a frame
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        use ratatui::text::{Line, Span};
        use ratatui::widgets::Paragraph;

        let mut lines = Vec::new();
        for row in &self.buffer {
            let line_str: String = row.iter().collect();
            lines.push(Line::from(line_str));
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_creation() {
        let canvas = SvgCanvas::new(80, 24);
        assert_eq!(canvas.width, 80);
        assert_eq!(canvas.height, 24);
    }

    #[test]
    fn test_draw_char() {
        let mut canvas = SvgCanvas::new(10, 10);
        canvas.draw_char(5, 5, 'X', None);
        assert_eq!(canvas.buffer[5][5], 'X');
    }

    #[test]
    fn test_draw_rect() {
        let mut canvas = SvgCanvas::new(20, 10);
        canvas.draw_rect(2, 2, 10, 5, None);
        assert_eq!(canvas.buffer[2][2], '┌');
        assert_eq!(canvas.buffer[2][11], '┐');
        assert_eq!(canvas.buffer[6][2], '└');
        assert_eq!(canvas.buffer[6][11], '┘');
    }
}
