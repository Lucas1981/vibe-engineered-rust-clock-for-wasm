#!/usr/bin/env bash
# Build and run the native desktop clock binary.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT/clock"

# cargo run:
#   --release  optimized build (slower compile, faster runtime)
#   --         pass remaining args to the binary
exec cargo run --release -- "$@"
