#!/bin/sh
wasm-pack build --target web --release --features wasm,console_error_panic_hook,wee_alloc && ./extend_wasm_pkg.sh