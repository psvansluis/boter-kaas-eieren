<script lang="ts">
  import Bord from "./Bord.svelte";
  import type { Speelbaar } from "./lib/wasm";
  import type * as Wasm from "./lib/wasm/rust_wasm";
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
    {#if typeof spel.Ok.spelstatus === "object" && "SpelBezig" in spel.Ok.spelstatus}
      <p>{spel.Ok.spelstatus.SpelBezig.speler_met_beurt} is aan de beurt</p>
    {:else if spel.Ok.spelstatus === "Gelijkspel"}
      <p>Gelijkspel!</p>
    {:else if typeof spel.Ok.spelstatus === "object" && "SpelerWint" in spel.Ok.spelstatus}
      <p>{spel.Ok.spelstatus.SpelerWint.winnaar} heeft gewonnen!</p>
    {/if}
  {/if}
</div>
