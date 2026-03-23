import type { WasmResultaat, Spelstatus, Speler } from "./wasm/rust_wasm";
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

export function verwerkResultaat<T, E, O>(
  resultaat: WasmResultaat<T, E>,
  bijOk: (ok: T) => O,
  bijErr: (err: E) => O,
): O {
  if ("Ok" in resultaat) {
    return bijOk(resultaat.Ok);
  } else {
    return bijErr(resultaat.Err);
  }
}

export function verwerkSpelstatus<O>(
  spelstatus: Spelstatus,
  bijSpelerWint: (winnaar: Speler) => O,
  bijGelijkspel: () => O,
  bijSpelBezig: (speler_met_beurt: Speler) => O,
): O {
  if (typeof spelstatus === "object" && "SpelerWint" in spelstatus) {
    return bijSpelerWint(spelstatus.SpelerWint.winnaar);
  } else if (spelstatus === "Gelijkspel") {
    return bijGelijkspel();
  } else if (typeof spelstatus === "object" && "SpelBezig" in spelstatus) {
    return bijSpelBezig(spelstatus.SpelBezig.speler_met_beurt);
  }
  throw new Error("Onbekende spelstatus");
}
