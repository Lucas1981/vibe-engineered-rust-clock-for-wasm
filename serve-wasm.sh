#!/usr/bin/env bash
# Build (if needed) and serve index.html + clock/pkg over HTTP.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

"$ROOT/build-wasm.sh"

PORT="${PORT:-8080}"
echo "Serving http://127.0.0.1:${PORT}/ (Ctrl+C to stop)"
exec python3 -m http.server "$PORT"
