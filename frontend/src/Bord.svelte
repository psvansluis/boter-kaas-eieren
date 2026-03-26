<script lang="ts">
  export type ZetVerwerker = (zet: Zet) => void;
  import type {
    BoterKaasEieren,
    Zet,
    Cel as WasmCel,
  } from "./lib/wasm/rust_wasm";
  import Cel from "./Cel.svelte";
  import { ZetMapper } from "./zetmapper";
  const { spel, speelZet }: { spel: BoterKaasEieren; speelZet: ZetVerwerker } =
    $props();

  const handleClick = (cel: WasmCel, x: number, y: number): void => {
    const zetOfFout = new ZetMapper(spel.spelstatus).mapZet(cel, x, y);
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

<style>
  .bord {
    border-collapse: collapse;
    margin: 0px auto;
  }

  .bord td {
    width: 60px;
    height: 60px;
    border: 2px solid #333;
    text-align: center;
    font-size: 24px;
    font-weight: bold;
    cursor: pointer;
  }
</style>
