# Codebase review

Review of the analog clock project (`./clock`) against [AGENTS.md](./AGENTS.md).  
Scope: architecture, rendering correctness, native/WASM parity, scripts, and maintainability.

## Summary

The project meets the AGENTS checklist: a unified canvas API, modular draw functions (`face`, `text`, `canvas`), live time via `chrono`, and a working WASM path (`build-wasm.sh`, `index.html`, `serve-wasm.sh`). The codebase is small (~450 lines of Rust across eight source files) and readable.

No blocking bugs were found for the stated goals. The items below are improvements around performance, robustness, and developer experience—not functional gaps for a local demo clock.

---

## Strengths

- **Clear module boundaries.** Canvas platform code is isolated behind `Canvas::open` / `present`; drawing logic lives in dedicated `face` and `text` functions as specified.
- **Correct hand geometry.** Hour, minute, and second angles include fractional sub-units; numerals and hands share the same “12 at top, clockwise” convention via equivalent angle math.
- **WASM setup is minimal and works.** `cdylib` + `#[wasm_bindgen(start)]`, `requestAnimationFrame` loop, and `chrono` `wasmbind` are wired correctly.
- **Font licensing is handled.** Liberation Sans Bold ships with `assets/FONT-LICENSE.txt` (SIL OFL 1.1).
- **Scripts are documented.** `scaffold.sh`, `build-and-run.sh`, `build-wasm.sh`, `check.sh`, and `serve-wasm.sh` include brief flag comments and sensible defaults.

---

## Findings

### Medium

#### 1. Native loop spins at full CPU

`lib.rs` runs an unbounded `while canvas.is_open()` loop with no pacing. Unlike WASM (capped by display refresh via `requestAnimationFrame`), the desktop build redraws as fast as possible and can peg a CPU core.

**Suggestion:** throttle native frames—e.g. sleep to ~60 Hz, or poll minifb with a target frame interval.

#### 2. WASM animation loop silently ignores errors

In `start_animation_loop`, both `draw_frame` failures and `request_animation_frame` scheduling failures are discarded:

```rust
let _ = draw_frame(&mut canvas, &numerals_clone);
let _ = window.request_animation_frame(...);
```

A present/upload failure would fail quietly and the clock would appear frozen with no console feedback.

**Suggestion:** log errors to `web_sys::console::error_1` (or propagate once on startup and log per-frame thereafter).

#### 3. Hard-coded 512×512 size in two places

Canvas dimensions are fixed in `lib.rs` (`Canvas::open("Clock", 512, 512)`) and `index.html` (`width="512" height="512"`). Changing one without the other causes a mismatch; the WASM backend also overwrites canvas dimensions at runtime, which masks but does not fix HTML/CSS drift.

**Suggestion:** define a single `const CLOCK_SIZE: u32 = 512` (e.g. in `lib.rs`) and document it for `index.html`, or read size from the canvas element on WASM.

#### 4. Duplicated clock-angle math

`text.rs` (`hour_angle`) and `face.rs` (`hand_angle`) encode the same “12 at top, clockwise” mapping with slightly different signatures. They can drift if one is changed.

**Suggestion:** extract a shared helper (e.g. `face::clock_angle(units: f64, max_units: f64) -> f32`) used by numerals and hands.

#### 5. `scaffold.sh` is out of date

Fresh runs of `scaffold.sh` would not reproduce the current crate layout:

- Missing `[lib] crate-type = ["cdylib", "rlib"]` (required for `wasm-pack`)
- Missing `js-sys` (currently listed in `Cargo.toml` but unused—see below)
- Does not create `assets/` or document font setup

**Suggestion:** update `scaffold.sh` to match the working `Cargo.toml` and note the Liberation font + license step.

#### 6. `check.sh` only validates the native target

Clippy and fmt run for the host triple only. WASM-only code paths (`canvas/wasm.rs`, `start_animation_loop`) are not compiled during `./check.sh`, so regressions can slip in until `./build-wasm.sh` is run.

**Suggestion:** add an optional (or second) step:  
`cargo clippy --target wasm32-unknown-unknown ...` or invoke `./build-wasm.sh` in CI.

---

### Low

#### 7. Unused `js-sys` dependency

`js-sys` is declared under `[target.'cfg(target_arch = "wasm32")'.dependencies]` but never imported. It may have been added in anticipation of `Closure`/`Function` use; those come from `wasm-bindgen` today.

**Suggestion:** remove `js-sys` from `Cargo.toml` unless a direct use is added.

#### 8. Numerals re-rasterized every frame

`draw_numerals` calls `font.rasterize` for every glyph on every frame. For twelve single-character labels this is acceptable, but it is unnecessary work once font size is fixed.

**Suggestion (optional):** cache glyph bitmaps in `ClockText` at startup if profiling shows measurable cost.

#### 9. Native pixel upload ignores alpha

`native.rs` maps RGBA → minifb `0RGB` using only RGB bytes. Correct for the current opaque white/black palette; would be wrong if semi-transparent drawing were added later.

**Suggestion:** no change needed now; note the limitation if anti-aliased colored layers are introduced.

#### 10. README is a placeholder

`README.md` contains only the repo title. Usage (`./build-and-run.sh`, `./serve-wasm.sh`), prerequisites (Rust, `wasm-pack`, Python 3), and font license pointer live only in scripts and AGENTS.

**Suggestion:** add a short README with build/run instructions for native and WASM.

#### 11. `RefCell` borrow failure skips a WASM frame

If `try_borrow_mut` fails on the canvas (e.g. re-entrant RAF callback), that frame is skipped without logging. Unlikely in practice but possible under devtools throttling or future async work.

**Suggestion:** log once on borrow failure or use a simpler ownership model if the loop grows.

#### 12. Dev server is unauthenticated static HTTP

`serve-wasm.sh` uses `python3 -m http.server`—appropriate for local development only.

**Suggestion:** document that this is not for production deployment.

---

## AGENTS.md checklist

| Step | Status | Notes |
|------|--------|-------|
| 1. Scaffold scripts | Done | `scaffold.sh` needs sync with lib/wasm layout |
| 2. Canvas abstraction | Done | Native + WASM backends behind one API |
| 3. Black circle on white | Done | `draw_background`, `draw_circle` |
| 4. Numerals 1–12 with fontdue | Done | Liberation Sans Bold + license |
| 5. Hour / minute / second hands | Done | Specified lengths and stroke widths |
| 6. Continuous frame loop | Done | Native loop + WASM RAF |
| 7. Local time via chrono | Done | `wasmbind` enabled on WASM |
| 8. WASM build + serve | Done | ~912 KB release `.wasm` after `wasm-opt` |
| 9. Review | Done | This document |

---

## Suggested follow-ups (priority order)

1. Throttle the native frame loop to reduce CPU use.
2. Surface WASM draw/present errors instead of discarding them.
3. Centralize clock dimensions (`512`) and sync with `index.html`.
4. Extend `check.sh` with a WASM compile/clippy step.
5. Update `scaffold.sh` and `README.md` so a fresh clone matches the documented workflow.
6. Deduplicate angle helpers; remove unused `js-sys` if confirmed unnecessary.

None of these are required for the clock to function correctly as a local native or browser demo today.
