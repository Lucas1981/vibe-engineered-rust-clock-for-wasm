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
            .dyn_into::<HtmlCanvasElement>()?;
        canvas.set_width(width);
        canvas.set_height(height);

        let context = canvas
            .get_context("2d")?
            .ok_or("canvas 2d context unavailable")?
            .dyn_into::<CanvasRenderingContext2d>()?;

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
        )?;
        self.context.put_image_data(&image, 0.0, 0.0)?;
        Ok(())
    }
}
