use crate::models::range_model::Range;

/// Angle type, in degrees by default
#[derive(Clone, Copy)]
pub struct Angle(pub f64);

impl Angle {
    fn set(&mut self, value: f64) {
        self.0 = value;

        if self.is_in_range() == false {
            self.0 %= Self::MAX;
        }
    }

    /// Set the value with a value in degrees
    pub fn set_degrees(&mut self, value: f64) {
        self.set(value);
    }

    /// Set the value with a value in radian
    pub fn set_radian(&mut self, value: f64) {
        self.set(value.to_degrees());
    }

    /// Get the value with a value in degress
    pub fn degrees(&self) -> f64 {
        self.0
    }

    /// Get the value with a value in radian
    pub fn radian(&self) -> f64 {
        self.0.to_radians()
    }
}

impl Range<f64> for Angle {
    const MIN: f64 = 0.;
    const MAX: f64 = 360.;

    fn is_in_range(&self) -> bool {
        self.degrees() >= Self::MIN && self.degrees() <= Self::MAX
    }
}

impl Into<Angle> for f64 {
    fn into(self) -> Angle {
        let angle = Angle(self);

        if angle.is_in_range() == false {
            return Angle(self % Self::MAX);
        }

        angle
    }
}
