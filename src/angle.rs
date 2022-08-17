use crate::models::range_model::Range;

/// Angle type
pub struct Angle {
    /// In degrees by default
    pub value: f64
}

impl Angle {
    /// Set `value` with a value in degrees
    pub fn set_degrees(&mut self, value: f64) {
        self.value = value;
    }

    /// Set `value` with a value in radian
    pub fn set_radian(&mut self, value: f64) {
        self.value = value.to_degrees();
    }

    /// Get `value` with a value in degress
    pub fn degrees(&self) -> f64 {
        self.value
    }

    /// Get `value` with a value in radian
    pub fn radian(&self) -> f64 {
        self.value.to_radians()
    }
}

impl Range<f64> for Angle {
    const MIN: f64 = 0.;
    const MAX: f64 = 360.;

    fn is_in_range(&self) -> bool {
        self.degrees() >= Self::MIN && self.degrees() <= Self::MAX
    }
}
