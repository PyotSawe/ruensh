//! Shape primitives for SVG-inspired rendering

use ratatui::style::{Color, Style};

/// 2D Point in terminal space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Base trait for all shapes
pub trait Shape {
    /// Render the shape to terminal using Unicode characters
    fn render(&self, canvas: &mut super::canvas::SvgCanvas);
    
    /// Get bounding box of the shape
    fn bounds(&self) -> (Point, Point);
    
    /// Check if point is inside shape
    fn contains(&self, point: Point) -> bool;
}

/// Rectangle shape
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: u8,
    pub corner_radius: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            fill: None,
            stroke: Some(Color::White),
            stroke_width: 1,
            corner_radius: 0.0,
        }
    }

    pub fn fill(mut self, color: Color) -> Self {
        self.fill = Some(color);
        self
    }

    pub fn stroke(mut self, color: Color) -> Self {
        self.stroke = Some(color);
        self
    }

    pub fn rounded(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }
}

impl Shape for Rectangle {
    fn render(&self, canvas: &mut super::canvas::SvgCanvas) {
        // Will be implemented in canvas module
    }

    fn bounds(&self) -> (Point, Point) {
        (
            Point::new(self.x, self.y),
            Point::new(self.x + self.width, self.y + self.height),
        )
    }

    fn contains(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
}

/// Circle shape
#[derive(Debug, Clone)]
pub struct Circle {
    pub cx: f32,
    pub cy: f32,
    pub radius: f32,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: u8,
}

impl Circle {
    pub fn new(cx: f32, cy: f32, radius: f32) -> Self {
        Self {
            cx,
            cy,
            radius,
            fill: None,
            stroke: Some(Color::White),
            stroke_width: 1,
        }
    }

    pub fn fill(mut self, color: Color) -> Self {
        self.fill = Some(color);
        self
    }

    pub fn stroke(mut self, color: Color) -> Self {
        self.stroke = Some(color);
        self
    }
}

impl Shape for Circle {
    fn render(&self, canvas: &mut super::canvas::SvgCanvas) {
        // Will be implemented in canvas module
    }

    fn bounds(&self) -> (Point, Point) {
        (
            Point::new(self.cx - self.radius, self.cy - self.radius),
            Point::new(self.cx + self.radius, self.cy + self.radius),
        )
    }

    fn contains(&self, point: Point) -> bool {
        let dx = point.x - self.cx;
        let dy = point.y - self.cy;
        (dx * dx + dy * dy) <= (self.radius * self.radius)
    }
}

/// Line shape
#[derive(Debug, Clone)]
pub struct Line {
    pub from: Point,
    pub to: Point,
    pub stroke: Color,
    pub stroke_width: u8,
}

impl Line {
    pub fn new(from: Point, to: Point) -> Self {
        Self {
            from,
            to,
            stroke: Color::White,
            stroke_width: 1,
        }
    }

    pub fn stroke(mut self, color: Color) -> Self {
        self.stroke = color;
        self
    }
}

impl Shape for Line {
    fn render(&self, canvas: &mut super::canvas::SvgCanvas) {
        // Will be implemented in canvas module
    }

    fn bounds(&self) -> (Point, Point) {
        (
            Point::new(self.from.x.min(self.to.x), self.from.y.min(self.to.y)),
            Point::new(self.from.x.max(self.to.x), self.from.y.max(self.to.y)),
        )
    }

    fn contains(&self, _point: Point) -> bool {
        // Lines have no area
        false
    }
}

/// Path shape for complex curves
#[derive(Debug, Clone)]
pub struct Path {
    pub points: Vec<Point>,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: u8,
    pub closed: bool,
}

impl Path {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            fill: None,
            stroke: Some(Color::White),
            stroke_width: 1,
            closed: false,
        }
    }

    pub fn add_point(mut self, point: Point) -> Self {
        self.points.push(point);
        self
    }

    pub fn close(mut self) -> Self {
        self.closed = true;
        self
    }

    pub fn fill(mut self, color: Color) -> Self {
        self.fill = Some(color);
        self
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

impl Shape for Path {
    fn render(&self, canvas: &mut super::canvas::SvgCanvas) {
        // Will be implemented in canvas module
    }

    fn bounds(&self) -> (Point, Point) {
        if self.points.is_empty() {
            return (Point::new(0.0, 0.0), Point::new(0.0, 0.0));
        }

        let mut min_x = self.points[0].x;
        let mut min_y = self.points[0].y;
        let mut max_x = self.points[0].x;
        let mut max_y = self.points[0].y;

        for point in &self.points {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }

        (Point::new(min_x, min_y), Point::new(max_x, max_y))
    }

    fn contains(&self, _point: Point) -> bool {
        // Complex polygon containment test would go here
        false
    }
}
