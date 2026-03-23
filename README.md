# Boter, kaas en eieren

Dit is boter kaas en eieren, met domeinlogica in Rust compileerd naar WebAssembly en een frontend in TypeScript Svelte, gecompileerd naar een statische website.

Compileer Rust als volgt:

```bash
wasm-pack build \
            --target web \
            --out-dir ../frontend/src/wasm \
            --release

```
