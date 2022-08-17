use crate::{
    position::Pos,
    color::Color
};

/// This Pen will be applied on a 2D plan
/// It represents the cursor, almost every operations will pass throught the pen
pub struct Pen {
    /// Size of the pen line
    pub thickness: f64,
    /// Location on the plan
    pub pos: Pos<i32>,
    /// Pen line Color
    pub color: Color,
    /// Describes if the pen is on the drawing
    pub on: bool
}

impl Pen {
    pub fn new<P, C>(thickness: f64, pos: P, color: C) -> Self 
    where
        P: Into<Pos<i32>>,
        C: Into<Color>
    {
        Self {
            thickness,
            pos: pos.into(),
            color: color.into(),
            on: true
        }
    }
}
