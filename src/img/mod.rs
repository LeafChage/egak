use image::{ImageResult, Rgb, RgbImage};
use imageproc::drawing::Canvas;
use imageproc::drawing::{
    draw_cross_mut, draw_filled_circle_mut, draw_filled_rect_mut, draw_hollow_circle_mut,
    draw_hollow_rect_mut, draw_line_segment_mut, draw_text_mut,
};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};
use std::path::Path;

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

pub fn color(c: Color) -> Rgb<u8> {
    match c {
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
        Color::Other(r, g, b) => Rgb([r, g, b]),
    }
}

pub fn generate(path: &str, c: Rgb<u8>, width: u32, height: u32, text: &str) -> ImageResult<()> {
    let path = Path::new(path);
    let mut image = RgbImage::new(width, height);
    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(width, height), c);

    let font =
        Vec::from(include_bytes!("/usr/share/fonts/opentype/noto/NotoSansCJK-Bold.ttc") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();
    draw_text_mut(
        &mut image,
        color(Color::Black),
        0,
        0,
        Scale {
            x: 5.0 * 2.0,
            y: 5.0,
        },
        &font,
        text,
    );
    image.save(path)
}
