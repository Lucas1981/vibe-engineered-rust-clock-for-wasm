#!/usr/bin/env bash
# Build the clock crate for wasm32-unknown-unknown with wasm-pack.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

if ! rustup target list --installed | grep -q 'wasm32-unknown-unknown'; then
  echo "Adding wasm32-unknown-unknown target…"
  rustup target add wasm32-unknown-unknown
fi

if ! command -v wasm-pack >/dev/null 2>&1; then
  echo "Installing wasm-pack…"
  cargo install wasm-pack
fi

# --target web: ES-module glue for static index.html
# --out-dir pkg: emit JS/WASM next to the crate
# --out-name clock: stable import path clock.js
wasm-pack build clock \
  --target web \
  --out-dir pkg \
  --out-name clock

echo "WASM build ready in ./clock/pkg"
