<script lang="ts">
  import type { Cel, Zet } from "./lib/wasm/rust_wasm";
  import type { Resultaat, ZetFout } from "./lib/resultaat";
  import { match } from "./lib/match";
  import { onMount } from "svelte";

  let buttonEl: HTMLButtonElement;

  const {
    cel,
    resultaat,
    heeftFocus,
    onclick,
    registreer,
  }: {
    cel: Cel;
    resultaat: Resultaat<Zet, ZetFout>;
    heeftFocus: boolean;
    onclick: () => void;
    registreer: (el: HTMLButtonElement) => void;
  } = $props();

  const disabled: boolean = $derived(
    match(resultaat, {
      Ok: () => false,
      Err: () => true,
    }),
  );
  const klasse: string = $derived(
    match(resultaat, {
      Ok: () => "speelbaar",
      Err: () => "onspeelbaar",
    }),
  );
  const label: string = $derived(
    match(resultaat, {
      Ok: (zet) => `Speelbare zet op rij ${zet.y + 1}, kolom ${zet.x + 1}`,
      Err: (error) =>
        match(error, {
          SpelAfgelopen: ({ winnaar }) =>
            `Spel afgelopen, ${winnaar} heeft gewonnen`,
          Gelijkspel: () => "Spel afgelopen in gelijkspel",
          CelBezet: ({ door }) => `Cel bezet door ${door}`,
        }),
    }),
  );
  const inhoud: string = $derived(
    match(cel, {
      Leeg: () => " ",
      Gespeeld: ({ door }) => door.type,
    }),
  );

  onMount(() => registreer(buttonEl));
</script>

<button
  class="cel {klasse}"
  bind:this={buttonEl}
  tabindex={heeftFocus ? 0 : -1}
  aria-disabled={disabled}
  aria-label={label}
  title={label}
  {onclick}
>
  {inhoud}
</button>
