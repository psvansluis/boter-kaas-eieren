import init, * as wasm from "./wasm/rust_wasm";

let initialized = false;

export async function loadWasm() {
  if (!initialized) {
    await init();
    initialized = true;
  }
  return wasm;
}
