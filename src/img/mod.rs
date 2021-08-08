use image::{ImageResult, Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect as ProcRect;
use rusttype::{point, Font, Rect, Scale};
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
    draw_filled_rect_mut(&mut image, ProcRect::at(0, 0).of_size(width, height), c);

    let font = Vec::from(include_bytes!("../../font/ArchitectsDaughter.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    // caliculate font position
    let scale = Scale::uniform(
        if width > height {
            height as f32
        } else {
            width as f32
        } / 4.0,
    );
    let point = point(0.0, font.v_metrics(scale).ascent);
    let glyphs: Vec<Rect<i32>> = font
        .layout(text, scale, point)
        .map(|g| g.pixel_bounding_box().unwrap())
        .collect();

    let first = glyphs.first().unwrap().min;
    let last = glyphs.last().unwrap().max;
    let min_y = glyphs.iter().map(|g| g.min.y).min().unwrap();
    let max_y = glyphs.iter().map(|g| g.max.y).max().unwrap();
    let h = max_y - min_y;
    let w = last.x - first.x;
    let center_x = (width / 2) - (w / 2) as u32 - first.x as u32;
    let center_y = (height / 2) - (h / 2) as u32 - min_y as u32;

    draw_text_mut(
        &mut image,
        color(Color::Black),
        center_x,
        center_y,
        scale,
        &font,
        text,
    );
    image.save(path)
}
