pub struct Size {
    pub w: f64,
    pub h: f64
}

impl Size {
    pub fn new(w: f64, h: f64) -> Self {
        Self { w, h }
    }
}

impl Into<Size> for (f64, f64) {
    fn into(self) -> Size {
        Size {
            w: self.0,
            h: self.1
        }
    }
}
