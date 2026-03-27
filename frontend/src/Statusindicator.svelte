<script lang="ts">
  import type { Spelstatus } from "./lib/wasm/rust_wasm";
  import { match } from "./lib/wasm";
  const { spelstatus }: { spelstatus: Spelstatus } = $props();
  const bericht = $derived(
    match(spelstatus, {
      SpelerWint: ({ winnaar }) => `${winnaar.type} heeft gewonnen!`,
      Gelijkspel: () => "Gelijkspel!",
      SpelBezig: ({ speler_met_beurt }) =>
        `${speler_met_beurt.type} is aan de beurt`,
    }),
  );
</script>

<p class="status">{bericht}</p>
