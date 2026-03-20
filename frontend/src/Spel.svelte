<script lang="ts">
  import * as Wasm from "./lib/wasm/rust_wasm";
  import type { BoterKaasEieren } from "./lib/wasm/rust_wasm";
  let { wasm }: { wasm: typeof Wasm } = $props();

  const spel: BoterKaasEieren = $derived(wasm.speel_boter_kaas_eieren());
</script>

<div style="display: flex; justify-content: center;">
  <table class="bord">
    <tbody>
      {#each spel.bord as rij}
        <tr>
          {#each rij as cel}
            <td>{cel}</td>
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
