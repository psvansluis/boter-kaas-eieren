<script lang="ts">
  import Actieknop from "./Actieknop.svelte";
  import { kiesWillekeurig } from "./lib/kiesWillekeurig";
  import type { Zet } from "./lib/wasm/rust_wasm";
  import type { ZettenTransformatie, ZettenUpdater } from "./Spel.svelte";

  const {
    zetten,
    updateZetten,
    suggesties,
  }: {
    zetten: Zet[];
    updateZetten: ZettenUpdater;
    suggesties: Zet[];
  } = $props();

  type Actie = {
    label: string;
    onclick: ZettenTransformatie;
    disabled: boolean;
    sneltoets: string;
  };

  const acties: Actie[] = $derived([
    {
      label: "Ongedaan maken",
      onclick: (zetten: Zet[]) => zetten.slice(0, -1),
      disabled: zetten.length === 0,
      sneltoets: "o",
    },
    {
      label: "Nieuw spel",
      onclick: (_zetten: Zet[]) => [],
      disabled: zetten.length < 2,
      sneltoets: "n",
    },
    {
      label: "Suggereer zet",
      onclick: (zetten: Zet[]) => {
        const suggestie = kiesWillekeurig(suggesties);
        return suggestie ? [...zetten, suggestie] : zetten;
      },
      disabled: suggesties.length < 1,
      sneltoets: "s",
    },
  ]);

  const knoppen = $derived(
    acties.map((actie) => ({
      ...actie,
      onclick: () => updateZetten(actie.onclick),
    })),
  );

  const isTekstinvoerveld = (element: EventTarget | null): boolean => {
    if (!(element instanceof HTMLElement)) return false;
    const tag = element.tagName.toLowerCase();
    return tag === "input" || tag === "textarea" || element.isContentEditable;
  };

  const handleSneltoetsen = (e: KeyboardEvent) => {
    if (isTekstinvoerveld(e.target)) return;
    const toets = e.key.toLowerCase();
    const knop = knoppen.find((k) => k.sneltoets === toets);
    if (knop && !knop.disabled) {
      e.preventDefault();
      knop.onclick();
    }
  };

  $effect(() => {
    window.addEventListener("keydown", handleSneltoetsen);
    return () => window.removeEventListener("keydown", handleSneltoetsen);
  });
</script>

<div class="knoppen" role="group" aria-label="Spel acties">
  {#each knoppen as knop (knop.sneltoets)}
    <Actieknop actie={knop} />
  {/each}
</div>
<p class="tip" aria-label="sneltoetsen" aria-hidden="true" id="hotkeys">
  Navigeer met pijltjestoetsen. Doe een zet met spatiebalk of enter. Gebruik O
  om een zet ongedaan te maken en N om een nieuw spel te starten.
</p>
