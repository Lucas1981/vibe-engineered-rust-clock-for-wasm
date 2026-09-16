#!/usr/bin/env bash
# Format and lint the clock crate.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT/clock"

echo "==> cargo fmt"
cargo fmt --all

echo "==> cargo clippy"
# -D warnings: treat clippy warnings as errors
# --all-targets: include tests/benches/examples if present
# -- -D warnings: forward deny to rustc for any remaining warnings
cargo clippy --all-targets -- -D warnings

echo "Check passed."
