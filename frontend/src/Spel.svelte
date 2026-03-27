<script lang="ts">
  import Bord from "./Bord.svelte";
  import Errorindicator from "./Errorindicator.svelte";
  import type { Speelbaar } from "./lib/wasm";

  import { match } from "./lib/match";
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
    <Errorindicator error={spel.data} />
  {:else if spel.type === "Ok"}
    <Bord spel={spel.data} speelZet={(zet) => zetten.push(zet)} />
    <Statusindicator spelstatus={spel.data.spelstatus} />
  {/if}
  {#if zetten.length > 0}
    <button onclick={() => zetten.pop()}>Ongedaan maken</button>
  {/if}
  {#if zetten.length > 1}
    <button onclick={() => (zetten.length = 0)}>Nieuw spel</button>
  {/if}
</div>
