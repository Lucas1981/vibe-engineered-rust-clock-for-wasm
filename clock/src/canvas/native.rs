use minifb::{Window, WindowOptions};
use tiny_skia::Pixmap;

pub struct Backend {
    window: Window,
    buffer: Vec<u32>,
    width: u32,
    height: u32,
}

impl Backend {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let window = Window::new(
            title,
            width as usize,
            height as usize,
            WindowOptions::default(),
        )?;
        let buffer = vec![0; (width * height) as usize];
        Ok(Self {
            window,
            buffer,
            width,
            height,
        })
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn present(&mut self, pixmap: &Pixmap) -> Result<(), Box<dyn std::error::Error>> {
        let pixels = pixmap.data().as_chunks::<4>().0;
        for (dst, src) in self.buffer.iter_mut().zip(pixels) {
            // minifb expects 0RGB; tiny-skia stores premultiplied RGBA8888.
            *dst = (u32::from(src[0]) << 16) | (u32::from(src[1]) << 8) | u32::from(src[2]);
        }

        self.window
            .update_with_buffer(&self.buffer, self.width as usize, self.height as usize)
            .map_err(|e| e.into())
    }
}
