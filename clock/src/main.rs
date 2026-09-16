mod canvas;
mod face;

fn main() -> Result<(), canvas::Error> {
    run()
}

#[cfg(not(target_arch = "wasm32"))]
fn run() -> Result<(), canvas::Error> {
    let mut canvas = canvas::Canvas::open("Clock", 512, 512)?;

    while canvas.is_open() {
        let pixmap = canvas.pixmap();
        face::draw_background(pixmap);
        face::draw_circle(pixmap);
        canvas.present()?;
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn run() -> Result<(), canvas::Error> {
    // Single frame for now; requestAnimationFrame loop comes in step 6.
    let mut canvas = canvas::Canvas::open("Clock", 512, 512)?;
    let pixmap = canvas.pixmap();
    face::draw_background(pixmap);
    face::draw_circle(pixmap);
    canvas.present()
}
