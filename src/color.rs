/// The unique Color model for this crate
#[derive(Clone, Copy)]
pub struct Color {
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
    /// Alpha
    pub a: u8
}

/// Pre-defined colors
pub enum ColorPre {
    Red,
    Green,
    Blue,
    Black,
    White,
    Grey,
    Default,
    None
}

/// Convert a pre-defined color into Color
impl Into<Color> for ColorPre {
    fn into(self) -> Color {
        match self {
            ColorPre::Red => Color::from((255, 0, 0, 255)),
            ColorPre::Green => Color::from((0, 255, 0, 255)),
            ColorPre::Blue => Color::from((0, 0, 255, 255)),
            ColorPre::Black => Color::from((0, 0, 0, 255)),
            ColorPre::White => Color::from((255, 255, 255, 255)),
            ColorPre::Grey => Color::from((127, 127, 127, 255)),
            ColorPre::Default => Color::from((0, 0, 0, 255)),
            ColorPre::None => Color::from((0, 0, 0, 0)),
        }
    }
}

/// SVG color format
pub type ColorSvg = String;

impl Default for Color {
    fn default() -> Self {
        ColorPre::Default.into()
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(color: (u8, u8, u8, u8)) -> Self {
        let (r, g, b, a) = color;

        Self { r, g, b, a }
    }
}

impl Into<ColorSvg> for Color {
    fn into(self) -> ColorSvg {
        let alpha = (self.a / 255) as f64;

        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, alpha)
    }
}
