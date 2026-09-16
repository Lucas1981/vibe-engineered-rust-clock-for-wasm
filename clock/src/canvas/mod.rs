#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
use native as backend;
#[cfg(target_arch = "wasm32")]
use wasm as backend;

use tiny_skia::Pixmap;

pub type Error = Box<dyn std::error::Error>;

pub struct Canvas {
    pixmap: Pixmap,
    backend: backend::Backend,
}

impl Canvas {
    /// Open a drawable surface. `title` is used for the native window; on WASM the
    /// element `#clock` must exist in the page.
    pub fn open(title: &str, width: u32, height: u32) -> Result<Self, Error> {
        let pixmap = Pixmap::new(width, height).ok_or("failed to allocate pixmap")?;
        let backend = backend::Backend::new(title, width, height)?;
        Ok(Self { pixmap, backend })
    }

    pub fn pixmap(&mut self) -> &mut Pixmap {
        &mut self.pixmap
    }

    pub fn is_open(&self) -> bool {
        self.backend.is_open()
    }

    pub fn present(&mut self) -> Result<(), Error> {
        self.backend.present(&self.pixmap)
    }
}
