<script lang="ts">
  import type { BoterKaasEieren, Zet } from "./lib/wasm/rust_wasm";

  import Cel from "./Cel.svelte";
  import { mapZet } from "./lib/mapZet";
  import { match } from "./lib/match";
  import type { ZettenUpdater } from "./Spel.svelte";

  const {
    spel,
    updateZetten,
  }: {
    spel: BoterKaasEieren;
    updateZetten: ZettenUpdater;
  } = $props();

  let focusedX = $state(0);
  let focusedY = $state(0);
  let knoppen: HTMLButtonElement[][] = [];

  const bordLengte = $derived(spel.bord.length);
  const knopMetFocus = $derived(knoppen[focusedY]?.[focusedX]);

  const beweegFocus = (dx: number, dy: number) => {
    focusedX = (focusedX + dx + bordLengte) % bordLengte;
    focusedY = (focusedY + dy + bordLengte) % bordLengte;
  };

  const toetsEffecten: Record<string, () => void> = {
    ArrowUp: () => beweegFocus(0, -1),
    ArrowDown: () => beweegFocus(0, 1),
    ArrowLeft: () => beweegFocus(-1, 0),
    ArrowRight: () => beweegFocus(1, 0),
    Enter: () => knopMetFocus?.click(),
    " ": () => knopMetFocus?.click(),
  };

  const verwerkKlik = (resultaat: ReturnType<typeof mapZet>): (() => void) =>
    match(resultaat, {
      Ok: (zet) => () => {
        focusedX = zet.x;
        focusedY = zet.y;
        updateZetten((zetten) => [...zetten, zet]);
      },
      Err: () => () => {},
    });

  const registreerKnop = (x: number, y: number) => (el: HTMLButtonElement) => {
    if (!knoppen[y]) knoppen[y] = [];
    knoppen[y][x] = el;
  };

  const onkeydown = (e: KeyboardEvent) => {
    const handler: () => void | undefined = toetsEffecten[e.key];
    if (handler) {
      e.preventDefault();
      handler();
    }
  };

  $effect(() => knopMetFocus?.focus());
</script>

<table class="bord" role="grid" {onkeydown}>
  <tbody>
    {#each spel.bord as rij, y}
      <tr>
        {#each rij as cel, x}
          {@const resultaat = mapZet(spel.spelstatus, cel, x, y)}
          {@const heeftFocus = focusedX === x && focusedY === y}

          <td role="gridcell">
            <Cel
              {cel}
              {resultaat}
              {heeftFocus}
              onclick={verwerkKlik(resultaat)}
              registreer={registreerKnop(x, y)}
            />
          </td>
        {/each}
      </tr>
    {/each}
  </tbody>
</table>
