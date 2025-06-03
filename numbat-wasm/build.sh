#!/bin/bash

set -euo pipefail

rm -rf www/pkg
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --target=web --release --out-dir=www/pkg

echo "$(stat -f%z www/pkg/numbat_wasm_bg.wasm)"
