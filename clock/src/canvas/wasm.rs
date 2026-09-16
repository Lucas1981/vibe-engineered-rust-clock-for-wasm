use tiny_skia::Pixmap;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

const CANVAS_ID: &str = "clock";

pub struct Backend {
    context: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}

impl Backend {
    pub fn new(_title: &str, width: u32, height: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let window = web_sys::window().ok_or("no global window")?;
        let document = window.document().ok_or("no document")?;
        let canvas = document
            .get_element_by_id(CANVAS_ID)
            .ok_or("canvas element #clock not found")?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "element #clock is not a canvas")?;
        canvas.set_width(width);
        canvas.set_height(height);

        let context = canvas
            .get_context("2d")
            .map_err(|err| js_error("canvas getContext failed", err))?
            .ok_or("canvas 2d context unavailable")?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| "canvas 2d context has wrong type")?;

        Ok(Self {
            context,
            width,
            height,
        })
    }

    pub fn is_open(&self) -> bool {
        true
    }

    pub fn present(&mut self, pixmap: &Pixmap) -> Result<(), Box<dyn std::error::Error>> {
        let image = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(pixmap.data()),
            self.width,
            self.height,
        )
        .map_err(|err| js_error("ImageData creation failed", err))?;
        self.context
            .put_image_data(&image, 0.0, 0.0)
            .map_err(|err| js_error("putImageData failed", err))?;
        Ok(())
    }
}

fn js_error(context: &str, err: wasm_bindgen::JsValue) -> Box<dyn std::error::Error> {
    format!("{context}: {err:?}").into()
}
