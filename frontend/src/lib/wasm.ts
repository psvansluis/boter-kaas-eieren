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

type Variant<T, K extends string> = Extract<T, { type: K }>;

type DataVariant<T, K extends string> =
  Variant<T, K> extends { data: infer D } ? D : void;

type MatchHandlers<T extends { type: string }, O> = {
  [K in T["type"]]: (value: DataVariant<T, K>) => O;
};

export function match<T extends { type: string }, O>(
  value: T,
  handlers: MatchHandlers<T, O>,
): O {
  const handler = handlers[value.type as T["type"]];
  const arg = "data" in value ? (value as any).data : undefined;
  return handler(arg);
}
