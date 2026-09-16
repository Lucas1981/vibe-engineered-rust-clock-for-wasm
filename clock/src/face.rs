use std::f32::consts::{FRAC_PI_2, TAU};

use chrono::{DateTime, Local, Timelike};
use tiny_skia::{Color, LineCap, Paint, PathBuilder, Pixmap, Stroke};

const FACE_MARGIN: f32 = 16.0;
const FACE_STROKE_WIDTH: f32 = 2.0;

const HOUR_HAND_LENGTH: f32 = 0.50;
const HOUR_HAND_WIDTH: f32 = 3.0;
const MINUTE_HAND_LENGTH: f32 = 0.75;
const MINUTE_HAND_WIDTH: f32 = 2.0;
const SECOND_HAND_LENGTH: f32 = 0.90;
const SECOND_HAND_WIDTH: f32 = 1.0;

pub fn face_geometry(pixmap: &Pixmap) -> (f32, f32, f32) {
    let width = pixmap.width() as f32;
    let height = pixmap.height() as f32;
    (
        width / 2.0,
        height / 2.0,
        width.min(height) / 2.0 - FACE_MARGIN,
    )
}

pub fn draw_background(pixmap: &mut Pixmap) {
    pixmap.fill(Color::WHITE);
}

pub fn draw_circle(pixmap: &mut Pixmap) {
    let (center_x, center_y, radius) = face_geometry(pixmap);
    let center = (center_x, center_y);

    let Some(path) = PathBuilder::from_circle(center.0, center.1, radius) else {
        return;
    };

    let mut paint = Paint::default();
    paint.set_color(Color::BLACK);
    paint.anti_alias = true;

    let stroke = Stroke {
        width: FACE_STROKE_WIDTH,
        ..Stroke::default()
    };

    pixmap.stroke_path(
        &path,
        &paint,
        &stroke,
        tiny_skia::Transform::identity(),
        None,
    );
}

pub fn draw_hands(pixmap: &mut Pixmap, time: DateTime<Local>) {
    let (center_x, center_y, radius) = face_geometry(pixmap);

    let second = time.second() as f64 + f64::from(time.nanosecond()) / 1_000_000_000.0;
    let minute = f64::from(time.minute()) + second / 60.0;
    let hour = f64::from(time.hour() % 12) + minute / 60.0;

    draw_hand(
        pixmap,
        center_x,
        center_y,
        radius,
        HOUR_HAND_LENGTH,
        HOUR_HAND_WIDTH,
        hand_angle(hour, 12.0),
    );
    draw_hand(
        pixmap,
        center_x,
        center_y,
        radius,
        MINUTE_HAND_LENGTH,
        MINUTE_HAND_WIDTH,
        hand_angle(minute, 60.0),
    );
    draw_hand(
        pixmap,
        center_x,
        center_y,
        radius,
        SECOND_HAND_LENGTH,
        SECOND_HAND_WIDTH,
        hand_angle(second, 60.0),
    );
}

fn hand_angle(value: f64, units: f64) -> f32 {
    -TAU * (value as f32 / units as f32) + FRAC_PI_2
}

fn draw_hand(
    pixmap: &mut Pixmap,
    center_x: f32,
    center_y: f32,
    radius: f32,
    length_factor: f32,
    width: f32,
    angle: f32,
) {
    let length = radius * length_factor;
    let end_x = center_x + length * angle.cos();
    let end_y = center_y - length * angle.sin();

    let mut path_builder = PathBuilder::new();
    path_builder.move_to(center_x, center_y);
    path_builder.line_to(end_x, end_y);
    let Some(path) = path_builder.finish() else {
        return;
    };

    let mut paint = Paint::default();
    paint.set_color(Color::BLACK);
    paint.anti_alias = true;

    let stroke = Stroke {
        width,
        line_cap: LineCap::Round,
        ..Stroke::default()
    };

    pixmap.stroke_path(
        &path,
        &paint,
        &stroke,
        tiny_skia::Transform::identity(),
        None,
    );
}
