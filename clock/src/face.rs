use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Stroke};

const FACE_MARGIN: f32 = 16.0;
const FACE_STROKE_WIDTH: f32 = 2.0;

pub fn draw_background(pixmap: &mut Pixmap) {
    pixmap.fill(Color::WHITE);
}

pub fn draw_circle(pixmap: &mut Pixmap) {
    let width = pixmap.width() as f32;
    let height = pixmap.height() as f32;
    let center = (width / 2.0, height / 2.0);
    let radius = width.min(height) / 2.0 - FACE_MARGIN;

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
