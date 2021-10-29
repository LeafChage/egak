use image::Rgb;

pub enum Color {
    Black,
    Blue,
    Cyan,
    Gray,
    Green,
    Magenta,
    Red,
    White,
    Yellow,
    Default,
    Other(u8, u8, u8),
}

impl Color {
    pub fn rgb(&self) -> Rgb<u8> {
        match self {
            Color::Black => Rgb([0, 0, 0]),
            Color::Blue => Rgb([0, 0, 255]),
            Color::Cyan => Rgb([0, 255, 255]),
            Color::Gray => Rgb([128, 128, 128]),
            Color::Green => Rgb([0, 255, 0]),
            Color::Magenta => Rgb([255, 0, 255]),
            Color::Red => Rgb([255, 0, 0]),
            Color::White => Rgb([255, 255, 255]),
            Color::Yellow => Rgb([255, 235, 4]),
            Color::Default => Rgb([128, 128, 128]),
            Color::Other(r, g, b) => Rgb([*r, *g, *b]),
        }
    }
}
