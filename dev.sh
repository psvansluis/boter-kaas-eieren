#!/bin/sh
echo "validating Rust"
cd ./rust-wasm
cargo clippy --fix --allow-dirty --allow-staged -- -D warnings
cargo fmt -- --check
cargo test

echo compiling Rust to Wasm
wasm-pack build --target web --out-dir ../frontend/src/lib/wasm --dev

echo "serving frontend"
cd ../frontend
npm ci
npm run dev