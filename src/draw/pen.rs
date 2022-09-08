use crate::{
    position::{Pos, Distance},
    color::Color, angle::Angle
};

/// Position type for Pen
pub type PenPos = Pos<f64>;

/// Pen state (down or up)
#[derive(PartialEq, Clone, Copy)]
pub enum PenState {
    Up,
    Down
}

/// This Pen will be applied on a 2D plan
/// It represents the cursor, almost every operations will pass throught the pen
#[derive(Clone, Copy)]
pub struct Pen {
    /// Size of the pen line
    pub thickness: f64,
    /// Location on the plan
    pub position: PenPos,
    /// Pen line Color
    pub color: Color,
    /// Describes the pen state
    state: PenState,
    /// Angle (for vector graphics)
    pub angle: Angle
}

impl Default for Pen {
    fn default() -> Self {
        Self {
            thickness: 1.,
            position: PenPos::default(),
            color: Color::default(),
            state: PenState::Down,
            angle: Angle(0.)
        }
    }
}

impl Pen {
    pub fn new<P, C>(thickness: f64, position: P, color: C) -> Self 
    where
        P: Into<PenPos>,
        C: Into<Color>
    {
        Self {
            thickness,
            position: position.into(),
            color: color.into(),
            state: PenState::Down,
            angle: Angle(0.)
        }
    }

    /// Move the pen by `distance` in the same direction
    pub fn forward(&mut self, distance: Distance) {
        let next_pos = self.position.next_pos_turn(
            self.angle,
            (0.).into(),
            distance
        );
    
        self.position = next_pos;
    }

    /// Move the pen by `distance` in the opposit direction
    pub fn backward(&mut self, distance: Distance) {
        let next_pos = self.position.next_pos_turn(
            self.angle,
            (180.).into(),
            distance
        );

        self.position = next_pos;
    }

    /// Remove the pen from the drawing
    pub fn up(&mut self) {
        self.state = PenState::Up;
    }

    /// Put the pen on the drawing
    pub fn down(&mut self) {
        self.state = PenState::Down;
    }

    /// Indicate if the pen is on the drawing
    pub fn is_down(&self) -> bool {
        self.state == PenState::Down
    }
}
