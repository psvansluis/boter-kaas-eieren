import init, * as wasm from "./wasm/rust_wasm";

type Wasm = typeof wasm;

export interface Speelbaar {
  speel_boter_kaas_eieren: Wasm["speel_boter_kaas_eieren"];
}

let initialized = false;

export async function loadWasm(): Promise<Speelbaar> {
  if (!initialized) {
    await init();
    initialized = true;
  }
  return wasm;
}
