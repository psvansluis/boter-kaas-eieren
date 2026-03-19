<script lang="ts">
  import { onMount } from 'svelte';
  import { loadWasm } from './lib/wasm';
  let message = 'Loading...';

  let spel = null;
  onMount(async () => {
    const wasm = await loadWasm();
    message = wasm.greet('Svelte + WASM');
    spel = wasm.speel_boter_kaas_eieren();
  });
</script>

<main>
  <h1>{message}</h1>
  <p>JS: {JSON.stringify(spel)}</p>
</main>