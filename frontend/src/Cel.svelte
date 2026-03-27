<script lang="ts">
  import type { Cel } from "./lib/wasm/rust_wasm";
  const { cel }: { cel: Cel } = $props();
</script>

{#if cel.type === "Leeg"}
  <div class="cel leeg">&nbsp;</div>
{:else if cel.type === "Gespeeld"}
  <div class="cel gespeeld">{cel.data.door.type}</div>
{/if}
