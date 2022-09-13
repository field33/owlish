#!/bin/sh
wasm-pack build --target web --release --features wasm,console_error_panic_hook && ./extend_wasm_pkg.sh && node patch_pkg_json.js