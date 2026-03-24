<script lang="ts">
  import Bord from "./Bord.svelte";
  import type { Speelbaar } from "./lib/wasm";
  import { match } from "./lib/wasm";
  import type {
    Zet,
    WasmResultaat,
    BoterKaasEieren,
    OngeldigeZet,
  } from "./lib/wasm/rust_wasm";
  import Statusindicator from "./Statusindicator.svelte";
  const { wasm }: { wasm: Speelbaar } = $props();
  const zetten: Zet[] = $state([]);
  const spel: WasmResultaat<BoterKaasEieren, OngeldigeZet> = $derived(
    wasm.speel_boter_kaas_eieren(zetten),
  );
  $effect(() =>
    match(spel, {
      Err: console.error,
      Ok: console.log,
    }),
  );
</script>

<div>
  {#if spel.type === "Err"}
    <p>{spel.data}</p>
  {:else if spel.type === "Ok"}
    <Bord spel={spel.data} speelZet={(zet) => zetten.push(zet)} />
    <Statusindicator spelstatus={spel.data.spelstatus} />
    <button onclick={() => (zetten.length = 0)}> Nieuw spel</button>
  {/if}
</div>
