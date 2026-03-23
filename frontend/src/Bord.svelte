<script lang="ts">
  export type ZetVerwerker = (zet: Zet) => void;
  import type { BoterKaasEieren, Zet } from "./lib/wasm/rust_wasm";
  import Cel from "./Cel.svelte";
  import { verwerkSpelstatus } from "./lib/wasm";
  const { spel, speelZet }: { spel: BoterKaasEieren; speelZet: ZetVerwerker } =
    $props();

  const handleClick = (x: number, y: number): void => {
    verwerkSpelstatus(
      spel.spelstatus,
      (_winnaar) => {},
      () => {},
      (speler_met_beurt) => {
        speelZet({
          x,
          y,
          speler: speler_met_beurt,
        });
      },
    );
  };
</script>

<table class="bord">
  <tbody>
    {#each spel.bord as rij, rijIndex}
      <tr>
        {#each rij as cel, celIndex}
          <td
            onclick={() => {
              handleClick(celIndex, rijIndex);
            }}><Cel {cel} /></td
          >
        {/each}
      </tr>
    {/each}
  </tbody>
</table>

<style>
  .bord {
    border-collapse: collapse;
    margin: 20px 0;
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
