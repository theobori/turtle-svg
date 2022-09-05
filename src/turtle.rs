use crate::{
    draw::{
        pen::{
            Pen,
            PenPos
        },
        drawing::Drawing
    },
    color::Color,
    angle::Angle,
    position::Distance, models::range_model::Range
};

/// Composing SVG with turtle graphics operations with a cursor
pub struct TurtleSvg {
    drawing: Drawing,
    pen: Pen,
}

impl Default for TurtleSvg {
    fn default() -> Self {
        Self {
            drawing: Drawing::default(),
            pen: Pen::default(),
        }
    }
}

impl TurtleSvg {
    pub fn new() -> Self {
        Self::default()
    }

    /// Reutnr a mutable referece of Drawing
    pub fn drawing_mut(&mut self) -> &mut Drawing {
        &mut self.drawing
    }

    /// Return a mutable reference of Pen
    pub fn pen_mut(&mut self) -> &mut Pen {
        &mut self.pen
    }

    /// Move forward by `distance`
    pub fn forward(&mut self, distance: Distance) {
        let mut pen = self.pen;

        self.drawing.forward(&mut pen, distance);

        self.pen = pen;
    }

    /// Move backward by `distance`
    pub fn backward(&mut self, distance: Distance) {
        let mut pen = self.pen;

        self.drawing.backward(&mut pen, distance);

        self.pen = pen;
    }

    /// Turn on the left by `angle`
    pub fn left<A: Into<Angle>>(&mut self, angle: A) {
        let angle = self.pen.angle.degrees() - angle.into().degrees();

        self.pen_mut().angle.set_degrees(angle);
    }

    /// Turn on the right by `angle`
    pub fn right<A: Into<Angle>>(&mut self, angle: A) {
        let angle = self.pen.angle.degrees() + angle.into().degrees();

        self.pen_mut().angle.set_degrees(angle);
    }

    /// Disable pen
    pub fn pen_up(&mut self) {
        self.pen_mut().up();
    }

    /// Enable pen
    pub fn pen_down(&mut self) {
        self.pen_mut().down();
    }

    /// Change pen line color
    pub fn set_pen_color<C: Into<Color>>(&mut self, color: C) {
        self.pen_mut().color = color.into();
    }

    /// Change the pen line size
    pub fn set_pen_size(&mut self, thickness: f64) {
        self.pen_mut().thickness = thickness;
    }

    /// Return a copy of the current angle
    pub fn heading(&self) -> Angle {
        self.pen.angle
    }

    /// Change the current angle
    pub fn set_heading<A: Into<Angle>>(&mut self, angle: A) {
        self.pen_mut().angle = angle.into();
    }

    /// Change the background color
    pub fn set_background_color<C: Into<Color>>(&mut self, color: C) {
        self.drawing_mut().set_background_color(color);
    }

    /// Replacing the struct
    pub fn reset(&mut self) {
        *self = Self::default()
    }

    /// Move the pen to `position`
    pub fn go_to<P: Into<PenPos>>(&mut self, position: P) {
        self.pen_mut().position = position.into();
    }
}
