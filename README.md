# Boter, kaas en eieren

Dit is boter kaas en eieren, met domeinlogica in Rust compileerd naar WebAssembly en een frontend in TypeScript Svelte, gecompileerd naar een statische website.

Het doel van dit project is om beter inzicht te krijgen in de integratie van Rust WebAssembly in een webproject.

Compileer Rust als volgt:

```bash
cd ./rust-wasm
wasm-pack build \
            --target web \
            --out-dir ../frontend/src/wasm \
            --release
```

Compileer de frontend als volgt (voor development):

```bash
cd ./frontend
npm run dev
```
