<script lang="ts">
  import Bord from "./Bord.svelte";
  import type { Speelbaar } from "./lib/wasm";
  import type * as Wasm from "./lib/wasm/rust_wasm";
  import Statusindicator from "./Statusindicator.svelte";
  const { wasm }: { wasm: Speelbaar } = $props();
  const zetten: Wasm.Zet[] = $state([]);
  const spel: Wasm.WasmResultaat<Wasm.BoterKaasEieren, Wasm.OngeldigeZet> =
    $derived(wasm.speel_boter_kaas_eieren(zetten));
</script>

<div style="display: flex; justify-content: center;">
  {#if typeof spel === "object" && "Err" in spel}
    <p>{spel.Err}</p>
  {:else if typeof spel === "object" && "Ok" in spel}
    <Bord spel={spel.Ok} speelZet={(zet) => zetten.push(zet)} />
    <Statusindicator spelstatus={spel.Ok.spelstatus} />
  {/if}
</div>
