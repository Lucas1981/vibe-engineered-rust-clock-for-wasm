#!/usr/bin/env bash
# Create the clock crate under ./clock and add stack dependencies from AGENTS.md.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

if [[ ! -d clock ]]; then
  # --name: binary/library crate name; --bin: default binary layout (not a library).
  cargo init --name clock --bin clock
fi

cd clock

# Shared rendering + time (native and WASM).
cargo add tiny-skia fontdue
cargo add chrono --features clock

# Native window backend (desktop only).
cargo add minifb --target 'cfg(not(target_arch = "wasm32"))'

# Browser canvas backend (WASM only).
cargo add wasm-bindgen --target 'cfg(target_arch = "wasm32")'
cargo add web-sys \
  --target 'cfg(target_arch = "wasm32")' \
  --features CanvasRenderingContext2d,Document,Element,HtmlCanvasElement,ImageData,Window
cargo add chrono \
  --target 'cfg(target_arch = "wasm32")' \
  --features clock,wasmbind

echo "Scaffold ready in ./clock"
