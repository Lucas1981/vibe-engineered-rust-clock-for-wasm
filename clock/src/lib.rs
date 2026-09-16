pub mod canvas;
pub mod face;
pub mod text;

use canvas::Canvas;
use text::ClockText;

pub type Error = canvas::Error;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_start() {
    if let Err(err) = run() {
        panic!("{err}");
    }
}

pub fn run() -> Result<(), Error> {
    let numerals = ClockText::new();
    let mut canvas = Canvas::open("Clock", 512, 512)?;

    #[cfg(not(target_arch = "wasm32"))]
    run_native_loop(&mut canvas, &numerals)?;

    #[cfg(target_arch = "wasm32")]
    {
        draw_frame(&mut canvas, &numerals)?;
        start_animation_loop(canvas, numerals)?;
    }

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
const NATIVE_TARGET_FPS: u32 = 60;

#[cfg(not(target_arch = "wasm32"))]
fn run_native_loop(canvas: &mut Canvas, numerals: &ClockText) -> Result<(), Error> {
    use std::time::{Duration, Instant};

    let frame_interval = Duration::from_secs(1) / NATIVE_TARGET_FPS;

    while canvas.is_open() {
        let frame_start = Instant::now();
        draw_frame(canvas, numerals)?;

        let elapsed = frame_start.elapsed();
        if elapsed < frame_interval {
            std::thread::sleep(frame_interval - elapsed);
        }
    }

    Ok(())
}

fn draw_frame(canvas: &mut Canvas, numerals: &ClockText) -> Result<(), Error> {
    let now = chrono::Local::now();
    let pixmap = canvas.pixmap();
    face::draw_background(pixmap);
    face::draw_circle(pixmap);
    numerals.draw_numerals(pixmap);
    face::draw_hands(pixmap, now);
    canvas.present()
}

#[cfg(target_arch = "wasm32")]
fn start_animation_loop(canvas: Canvas, numerals: ClockText) -> Result<(), Error> {
    use std::cell::RefCell;
    use std::rc::Rc;

    use wasm_bindgen::JsCast;
    use wasm_bindgen::closure::Closure;

    let numerals = Rc::new(numerals);
    let canvas = Rc::new(RefCell::new(canvas));
    let callback = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let callback_clone = Rc::clone(&callback);
    let canvas_clone = Rc::clone(&canvas);
    let numerals_clone = Rc::clone(&numerals);

    *callback.borrow_mut() = Some(Closure::new(move || {
        if let Ok(mut canvas) = canvas_clone.try_borrow_mut() {
            let _ = draw_frame(&mut canvas, &numerals_clone);
        }

        let window = web_sys::window().expect("no global window");
        let closure = callback_clone.borrow();
        let closure = closure.as_ref().expect("animation closure");
        let _ = window.request_animation_frame(closure.as_ref().unchecked_ref());
    }));

    let window = web_sys::window().expect("no global window");
    let closure = callback.borrow();
    let closure = closure.as_ref().expect("animation closure");
    window
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .map_err(|err| format!("requestAnimationFrame failed: {err:?}"))?;

    Ok(())
}
