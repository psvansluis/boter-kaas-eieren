<script lang="ts">
  import { loadWasm } from "./lib/wasm";
  import Spel from "./Spel.svelte";
  const wasm = $state(loadWasm());
</script>

<main>
  {#await wasm}
    <p>Spel start op...</p>
  {:then loadedWasm}
    <Spel wasm={loadedWasm} />
  {/await}
</main>
