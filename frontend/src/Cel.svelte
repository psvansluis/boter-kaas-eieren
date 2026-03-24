<script lang="ts">
  import type { Cel } from "./lib/wasm/rust_wasm";
  const { cel }: { cel: Cel } = $props();
</script>

{#if cel.type === "Leeg"}
  <div class="cel leeg">&nbsp;</div>
{:else if cel.type === "Gespeeld"}
  <div class="cel gespeeld">{cel.data.door.type}</div>
{/if}

<style>
  .cel {
    width: 60px;
    height: 60px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 24px;
    font-weight: bold;
  }
  .leeg:hover {
    background-color: #f0f0f0;
  }
</style>
