<script lang="ts">
  import { match } from "./lib/wasm";
  import type { OngeldigeZet } from "./lib/wasm/rust_wasm";

  const { error }: { error: OngeldigeZet } = $props();

  const bericht: string = $derived(
    match(error, {
      CelBezet: () => `Cel is al bezet`,
      SpelAfgerond: () => "Het spel is al afgelopen",
      OngeldigeCoordinaten: () => "Ongeldige coördinaten",
      VerkeerdeSpeler: () => "Verkeerde speler",
    }),
  );
</script>

<p class="error">{bericht}</p>
