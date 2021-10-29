mod color;

pub use color::Color;

use image::{ImageResult, Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect as ProcRect;
use rusttype::{point, Font, Point, Rect, Scale};
use std::path::Path;

pub fn generate(path: &str, c: Rgb<u8>, width: u32, height: u32, text: &str) -> ImageResult<()> {
    let path = Path::new(path);
    let mut image = RgbImage::new(width, height);
    draw_filled_rect_mut(&mut image, ProcRect::at(0, 0).of_size(width, height), c);

    let font = Vec::from(include_bytes!("../../font/ArchitectsDaughter.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    // caliculate font position
    let scale = decide_font_scale(text, &font, width, height);
    let text_rect = rect_text_area(text, &font, scale);
    let center_x = (width / 2) - (text_rect.width() / 2) as u32 - text_rect.min.x as u32;
    let center_y = (height / 2) - (text_rect.height() / 2) as u32 - text_rect.min.y as u32;

    draw_text_mut(
        &mut image,
        Color::Black.rgb(),
        center_x,
        center_y,
        scale,
        &font,
        text,
    );
    image.save(path)
}

///
/// get scale as big as possible.
/// (caliculate based on scale = 1)
///
fn decide_font_scale(text: &str, font: &Font, width: u32, height: u32) -> Scale {
    let scale = Scale::uniform(1.0);
    let rect = rect_text_area(text, font, scale);
    let w = width as i32 / rect.width();
    let h = height as i32 / rect.height();
    Scale::uniform(if w < h { w as f32 } else { h as f32 })
}

fn rect_text_area(text: &str, font: &Font, scale: Scale) -> Rect<i32> {
    let point = point(0.0, font.v_metrics(scale).ascent);
    let glyphs: Vec<Rect<i32>> = font
        .layout(text, scale, point)
        .map(|g| g.pixel_bounding_box().unwrap())
        .collect();

    let min_x = glyphs.first().unwrap().min.x;
    let min_y = glyphs.iter().map(|g| g.min.y).min().unwrap();

    let max_x = glyphs.last().unwrap().max.x;
    let max_y = glyphs.iter().map(|g| g.max.y).max().unwrap();

    Rect {
        min: Point { x: min_x, y: min_y },
        max: Point { x: max_x, y: max_y },
    }
}
