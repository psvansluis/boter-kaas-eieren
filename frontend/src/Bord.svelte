<script lang="ts">
  export type ZetVerwerker = (zet: Zet) => void;
  import type {
    BoterKaasEieren,
    Zet,
    Cel as WasmCel,
  } from "./lib/wasm/rust_wasm";
  import Cel from "./Cel.svelte";
  import { mapZet } from "./lib/mapZet";
  const { spel, speelZet }: { spel: BoterKaasEieren; speelZet: ZetVerwerker } =
    $props();

  const handleClick = (cel: WasmCel, x: number, y: number): void => {
    const zetOfFout = mapZet(spel.spelstatus, cel, x, y);
    if (typeof zetOfFout === "string") {
      console.warn(zetOfFout);
    } else {
      speelZet(zetOfFout);
    }
  };
</script>

<table class="bord">
  <tbody>
    {#each spel.bord as rij, y}
      <tr>
        {#each rij as cel, x}
          <td onclick={() => handleClick(cel, x, y)}><Cel {cel} /></td>
        {/each}
      </tr>
    {/each}
  </tbody>
</table>
