#!/bin/bash

RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack test --headless --firefox
