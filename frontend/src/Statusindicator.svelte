<script lang="ts">
  import type { Spelstatus } from "./lib/wasm/rust_wasm";
  import { verwerkSpelstatus } from "./lib/wasm";
  const { spelstatus }: { spelstatus: Spelstatus } = $props();
  const bericht = $derived(
    verwerkSpelstatus(
      spelstatus,
      (winnaar) => `${winnaar} heeft gewonnen!`,
      () => "Gelijkspel!",
      (speler_met_beurt) => `${speler_met_beurt} is aan de beurt`,
    ),
  );
</script>

<p>{bericht}</p>
