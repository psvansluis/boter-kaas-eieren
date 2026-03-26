<script lang="ts">
  import { loadWasm } from "./lib/wasm";
  import Spel from "./Spel.svelte";
  const wasm = $state(loadWasm());
</script>

<main id="app">
  <h1>Boter, Kaas en Eieren</h1>
  {#await wasm}
    <p>Spel start op...</p>
  {:then loadedWasm}
    <Spel wasm={loadedWasm} />
  {/await}
</main>

<style>
  #app {
    width: 80wv;
    max-width: 100%;
    margin: 0 auto;
    text-align: center;
    border-inline: 1px solid var(--border);
    min-height: 100svh;
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
  }
</style>
