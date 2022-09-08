use svg::{node::element::{Rectangle, Line}, Document};

use crate::{
    color::{Color, ColorSvg},
    size::Size,
    draw::pen::Pen,
    position::{Distance, Pos}
};

struct DrawingLine {
    color: Color,
    thickness: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64
}

/// Drawing position
pub type DrawingPos = Pos<f64>;

/// The drawing contains
pub struct Drawing {
    size: Size,
    bg: Color,
    lines: Vec<DrawingLine>,
    center: DrawingPos
}

impl Default for Drawing {
    fn default() -> Self {
        Self {
            size: (800., 600.).into(),
            bg: Color::default(),
            lines: Vec::new(),
            center: (0., 0.).into()
        }
    }
}

impl Drawing {
    pub fn new() -> Self {
        Self::default()
    }

    /// Move the pen in a direction
    fn go(
        &mut self,
        pen: &mut Pen,
        distance: Distance,
        f: &dyn Fn(&mut Pen, Distance)
    ) {
        let (x1, y1) = pen.position.into();

        f(pen, distance);
        
        let (x2, y2) = pen.position.into();
        let line = DrawingLine {
            color: pen.color,
            thickness: pen.thickness,
            x1,
            y1,
            x2,
            y2,
        };
        
        // Add the line to the 'history'
        self.lines.push(line);
    }

    /// Move by `distance` forward from `pen` with the same angle
    pub fn forward(&mut self, pen: &mut Pen, distance: Distance) {
        self.go(pen, distance, &Pen::forward)
    }

    /// Move by `distance` backward from `pen` with the same angle
    pub fn backward(&mut self, pen: &mut Pen, distance: Distance) {
        self.go(pen, distance, &Pen::backward)
    }

    /// Change the background color
    pub fn set_background_color<C: Into<Color>>(&mut self, color: C) {
        self.bg = color.into();
    }

    /// Change the drawing size
    pub fn set_size<S: Into<Size>>(&mut self, size: S) {
        self.size = size.into();
    }

    /// Centering the drawing
    pub fn set_center<P: Into<DrawingPos>>(&mut self, positon: P) {
        self.center = positon.into();
    }

    /// Parse `self.composer` and write lines into a file at `path`
    pub fn save_svg(&mut self, path: &str) {
        // Background
        let background_color: ColorSvg = self.bg.into(); 
        let background = Rectangle::new()
            .set("fill", background_color)
            .set("height", "100%")
            .set("width", "100%");
        
        // Document
        let viewbox = (0, 0, self.size.w, self.size.h);

        let mut document = Document::new()
            .set("viewBox", viewbox)
            .add(background);

        // Distance to reach the center
        let dx = (self.size.w / 2.) - self.center.0;
        let dy = (self.size.h / 2.) - self.center.1;

        for line in &self.lines {
            let color: ColorSvg = line.color.into();
            let node = Line::new()
                .set("stroke", color)
                .set("stroke-width", line.thickness)
                .set("x1", line.x1 + dx)
                .set("y1", line.y1 + dy)
                .set("x2", line.x2 + dx)
                .set("y2", line.y2 + dy);

            document = document.add(node);
        }

        // Save the document as a SVG file
        svg::save(path, &document).unwrap();
    }
}
