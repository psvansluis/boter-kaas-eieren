<script lang="ts">
  import { onMount } from 'svelte';
  import { loadWasm } from './lib/wasm';
  let message = 'Loading...';

  onMount(async () => {
    const wasm = await loadWasm();
    message = wasm.greet('Svelte + WASM');
  });
</script>

<main>
  <h1>{message}</h1>
</main>