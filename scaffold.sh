#!/usr/bin/env bash
# Create the clock crate under ./clock and add stack dependencies from AGENTS.md.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

LIBERATION_FONTS_URL="https://github.com/liberationfonts/liberation-fonts/files/7261482/liberation-fonts-ttf-2.1.5.tar.gz"

if [[ ! -d clock ]]; then
  # --name: binary/library crate name; --bin: default binary layout (not a library).
  cargo init --name clock --bin clock
fi

cd clock

# wasm-pack needs a cdylib in addition to the native binary.
if ! grep -q '^\[lib\]' Cargo.toml; then
  cat >> Cargo.toml <<'EOF'

[lib]
crate-type = ["cdylib", "rlib"]
EOF
fi

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

fetch_font_assets() {
  local assets_dir="$ROOT/clock/assets"
  mkdir -p "$assets_dir"

  if [[ -f "$assets_dir/LiberationSans-Bold.ttf" && -f "$assets_dir/FONT-LICENSE.txt" ]]; then
    echo "Font assets already present in ./clock/assets"
    return
  fi

  echo "Fetching Liberation Sans Bold (2.1.5) and OFL license…"
  local tmp
  tmp="$(mktemp -d)"
  curl -fsSL -o "$tmp/liberation-fonts.tar.gz" "$LIBERATION_FONTS_URL"
  tar -xzf "$tmp/liberation-fonts.tar.gz" -C "$tmp" \
    liberation-fonts-ttf-2.1.5/LiberationSans-Bold.ttf \
    liberation-fonts-ttf-2.1.5/LICENSE
  cp "$tmp/liberation-fonts-ttf-2.1.5/LiberationSans-Bold.ttf" "$assets_dir/"
  cp "$tmp/liberation-fonts-ttf-2.1.5/LICENSE" "$assets_dir/FONT-LICENSE.txt"
  rm -rf "$tmp"
  echo "Installed ./clock/assets/LiberationSans-Bold.ttf and FONT-LICENSE.txt"
}

fetch_font_assets

echo "Scaffold ready in ./clock"
