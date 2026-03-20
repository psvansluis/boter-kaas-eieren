<script lang="ts">
  import type { Speelbaar } from "./lib/wasm";
  import type * as Wasm from "./lib/wasm/rust_wasm";
  const { wasm }: { wasm: Speelbaar } = $props();
  const zetten: Wasm.Zet[] = $state([]);
  const spel: Wasm.BoterKaasEieren = $derived(
    wasm.speel_boter_kaas_eieren(zetten),
  );
</script>

<div style="display: flex; justify-content: center;">
  <table class="bord">
    <tbody>
      {#each spel.bord as rij, rijIndex}
        <tr>
          {#each rij as cel, celIndex}
            <td
              onclick={() => {
                zetten.push({
                  x: celIndex,
                  y: rijIndex,
                  speler: spel.speler_met_beurt,
                });
              }}>{cel}</td
            >
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<p>{spel.speler_met_beurt} is aan de beurt</p>

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

  .bord td:hover {
    background-color: #f0f0f0;
  }
</style>
