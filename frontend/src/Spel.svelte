<script lang="ts">
  export type ZettenTransformatie = (z: Zet[]) => Zet[];
  export type ZettenUpdater = (fn: ZettenTransformatie) => void;
  import Bord from "./Bord.svelte";
  import Errorindicator from "./Errorindicator.svelte";
  import type { Speelbaar, Suggereerder } from "./lib/wasm";

  import { match } from "./lib/match";
  import type {
    Zet,
    WasmResultaat,
    BoterKaasEieren,
    OngeldigeZet,
  } from "./lib/wasm/rust_wasm";
  import Statusindicator from "./Statusindicator.svelte";
  import Actiepaneel from "./Actiepaneel.svelte";
  const { wasm }: { wasm: Speelbaar & Suggereerder } = $props();
  let zetten: Zet[] = $state([]);
  const spel: WasmResultaat<BoterKaasEieren, OngeldigeZet> = $derived(
    wasm.speel_boter_kaas_eieren(zetten),
  );
  const suggesties = $derived(wasm.suggereer_zetten(zetten));

  const updateZetten: ZettenUpdater = (fn) => {
    zetten = fn(zetten);
  };
  $effect(() =>
    match(spel, {
      Err: console.error,
      Ok: console.log,
    }),
  );
</script>

<section class="spel" aria-label="Boter kaas en eieren spel">
  {#if spel.type === "Err"}
    <Errorindicator error={spel.data} />
  {:else if spel.type === "Ok"}
    <Bord spel={spel.data} {updateZetten} />
    <Statusindicator spelstatus={spel.data.spelstatus} />
  {/if}
  <Actiepaneel {zetten} {updateZetten} {suggesties} />
</section>
