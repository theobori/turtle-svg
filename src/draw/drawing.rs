use svg::{node::element::{Rectangle, Line}, Document};

use crate::{
    color::{Color, ColorSvg},
    size::Size,
    draw::pen::Pen,
    position::{Distance}
};

/// The drawing contains
pub struct Drawing {
    size: Size,
    bg: Color,
    composer: Vec<Line>,
}

impl Default for Drawing {
    fn default() -> Self {
        Self {
            size: (800., 600.).into(),
            bg: Color::default(),
            composer: Vec::new()
        }
    }
}

impl Drawing {
    pub fn new() -> Self {
        Self::default()
    }

    /// Used to move the pen in a direction
    fn go(
        &mut self,
        pen: &mut Pen,
        distance: Distance,
        f: &dyn Fn(&mut Pen, Distance)
    ) {
        let pen_color: ColorSvg = pen.color.into();
        let (x1, y1) = pen.position.into();

        f(pen, distance);
        
        let (x2, y2) = pen.position.into();

        // Create the line with pen informations
        let line = Line::new()
            .set("stroke", pen_color)
            .set("stroke-width", pen.thickness)
            .set("x1", x1)
            .set("y1", y1)
            .set("x2", x2)
            .set("y2", y2);

        // Add the line into the composer
        self.composer.push(line);
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
    pub fn set_size(&mut self, size: Size) {
        self.size = size;
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

        for node in &self.composer {
            document = document.add(node.clone());
        }

        // Save the document as a SVG file
        svg::save(path, &document).unwrap();
    }
}
