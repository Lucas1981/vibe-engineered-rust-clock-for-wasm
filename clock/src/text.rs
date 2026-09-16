use std::f32::consts::TAU;

use fontdue::{Font, FontSettings};
use tiny_skia::{Pixmap, PremultipliedColorU8};

use crate::face;

const FONT_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/LiberationSans-Bold.ttf"
));
const FONT_SIZE: f32 = 36.0;
const NUMERAL_RING_PADDING: f32 = 20.0;
const NUMERAL_RADIUS_INSET: f32 = 0.02;

pub struct ClockText {
    font: Font,
}

impl Default for ClockText {
    fn default() -> Self {
        Self::new()
    }
}

impl ClockText {
    pub fn new() -> Self {
        let font = Font::from_bytes(FONT_BYTES, FontSettings::default()).expect("embedded font");
        Self { font }
    }

    pub fn draw_numerals(&self, pixmap: &mut Pixmap) {
        let (center_x, center_y, face_radius) = face::face_geometry(pixmap);
        let label_radius = face_radius - NUMERAL_RING_PADDING - face_radius * NUMERAL_RADIUS_INSET;

        for hour in 1..=12u32 {
            let angle = hour_angle(hour);
            let anchor_x = center_x + label_radius * angle.cos();
            let anchor_y = center_y - label_radius * angle.sin();
            draw_label(pixmap, &self.font, &hour.to_string(), anchor_x, anchor_y);
        }
    }
}

fn hour_angle(hour: u32) -> f32 {
    // 12 at the top, advancing clockwise in standard math coordinates (y up).
    -TAU * hour as f32 / 12.0 + std::f32::consts::FRAC_PI_2
}

fn draw_label(pixmap: &mut Pixmap, font: &Font, label: &str, anchor_x: f32, anchor_y: f32) {
    let Some((min_x, min_y, max_x, max_y)) = label_bounds(font, label, FONT_SIZE) else {
        return;
    };

    let center_offset_x = (min_x + max_x) / 2.0;
    let center_offset_y = (min_y + max_y) / 2.0;
    let mut cursor_x = 0.0;

    for ch in label.chars() {
        let (metrics, bitmap) = font.rasterize(ch, FONT_SIZE);
        let x = (anchor_x - center_offset_x + cursor_x + metrics.xmin as f32).round() as i32;
        let y = (anchor_y - center_offset_y + metrics.ymin as f32).round() as i32;
        blend_glyph(pixmap, x, y, metrics.width, metrics.height, &bitmap);
        cursor_x += metrics.advance_width;
    }
}

fn label_bounds(font: &Font, label: &str, size: f32) -> Option<(f32, f32, f32, f32)> {
    font.horizontal_line_metrics(size)?;

    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut cursor_x = 0.0;

    for ch in label.chars() {
        let (metrics, _) = font.rasterize(ch, size);
        let left = cursor_x + metrics.xmin as f32;
        let top = metrics.ymin as f32;
        let right = left + metrics.width as f32;
        let bottom = top + metrics.height as f32;

        min_x = min_x.min(left);
        min_y = min_y.min(top);
        max_x = max_x.max(right);
        max_y = max_y.max(bottom);
        cursor_x += metrics.advance_width;
    }

    Some((min_x, min_y, max_x, max_y))
}

fn blend_glyph(pixmap: &mut Pixmap, x: i32, y: i32, width: usize, height: usize, coverage: &[u8]) {
    let width_px = pixmap.width() as i32;
    let height_px = pixmap.height() as i32;
    let stride = pixmap.width() as usize;
    let pixels = pixmap.pixels_mut();

    for row in 0..height {
        let py = y + row as i32;
        if !(0..height_px).contains(&py) {
            continue;
        }

        for col in 0..width {
            let px = x + col as i32;
            if !(0..width_px).contains(&px) {
                continue;
            }

            let alpha = coverage[row * width + col];
            if alpha == 0 {
                continue;
            }

            let idx = py as usize * stride + px as usize;
            let dst = pixels[idx];
            let inv_sa = 255 - alpha;
            let new_r = (u16::from(dst.red()) * u16::from(inv_sa) / 255) as u8;
            let new_g = (u16::from(dst.green()) * u16::from(inv_sa) / 255) as u8;
            let new_b = (u16::from(dst.blue()) * u16::from(inv_sa) / 255) as u8;
            let new_a = alpha + (u16::from(dst.alpha()) * u16::from(inv_sa) / 255) as u8;
            if let Some(blended) = PremultipliedColorU8::from_rgba(new_r, new_g, new_b, new_a) {
                pixels[idx] = blended;
            }
        }
    }
}
