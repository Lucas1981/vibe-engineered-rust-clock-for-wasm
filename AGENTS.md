# Analog clock

Build a live analog clock as an isolated project under `./clock` only — ignore sibling Rust folders. Keep the code small and readable; do not add unused APIs “for later.” Put each part of the renderer in its own function (and modules where it helps: canvas, text, face).

1. [x] Project scaffold: create `scaffold.sh` and `build-and-run.sh` recording the setup/`cargo run` commands, with brief comments on terse flags. Create a `./check.sh` file as well, where we can run fmt and clippy.
2. [x] Canvas: a thin type to set up, present, and tear down a drawable screen (`tiny-skia` pixmap + a platform backend). On native, use `minifb` for the window and pixel buffer. On `wasm32`, use `wasm-bindgen` + `web-sys` to bind a `<canvas>` and blit the pixmap each frame. Hide the split behind one API so the draw loop stays the same.
3. [x] Draw a black circle on a white background.
4. [x] Draw numerals 1..12 in clock positions. Render with `fontdue` (TrueType glyphs into the pixmap) — no hand-drawn digit bitmaps or custom stroke fonts.
5. [x] Draw hour / minute / second hands: hour 50% radius × 3px, minute 75% × 2px, second 90% × 1px. We must set this up so that with every frame the time displayed is updated and correct. Use the `chrono` dependencies for this.
6. [x] Continuous frame loop: each refresh clear → white background → circle → numbers → hands → present (via the dedicated draw functions).
7. [x] Drive hand angles from local wall-clock time (`chrono`).
8. [x] WebAssembly target: add a `wasm32-unknown-unknown` build (e.g. `build-wasm.sh` with `wasm-pack` or `trunk`), a minimal `index.html` that loads the module, and a serve step (local static server or `trunk serve`). The WASM backend from step 2 should drive the frame loop with `requestAnimationFrame`.
9. [x] Perform a review on the codebase and act on the suggestions.

Stack:

- `tiny-skia` — all 2D paths, strokes, and fills into a pixmap. Pure Rust; works on native and WASM.
- `minifb` (native only, `cfg(not(target_arch = "wasm32"))`) — OS window lifecycle and presenting the pixel buffer on desktop. Convert pixmap RGBA → `0RGB` on present. `minifb` does not support WASM; do not use it in the WASM build.
- `wasm-bindgen` + `web-sys` (WASM only, `cfg(target_arch = "wasm32")`) — canvas element, `ImageData` / pixel upload, and `requestAnimationFrame` for the frame loop.
- `fontdue` — load/embed a `.ttf`, rasterize glyphs, blend coverage into the pixmap. Pure Rust; works on native and WASM.
- `chrono` (with `clock`; enable `wasmbind` on WASM) — local hour, minute, and second. On native, `Local::now()` is fine. On WASM, enable `chrono`’s `wasmbind` feature so local time comes from the browser’s `Date`.
