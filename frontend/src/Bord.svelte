<script lang="ts">
  import type { BoterKaasEieren, Zet } from "./lib/wasm/rust_wasm";

  import Cel from "./Cel.svelte";
  import { mapZet } from "./lib/mapZet";
  import { match } from "./lib/match";
  import { onMount } from "svelte";

  export type ZetVerwerker = (zet: Zet) => void;

  const {
    spel,
    speelZet,
  }: {
    spel: BoterKaasEieren;
    speelZet: ZetVerwerker;
  } = $props();

  const bordLengte = $derived(spel.bord.length);

  const toetsEffecten: Record<string, () => void> = {
    ArrowUp: () => {
      focusedY = (focusedY - 1 + bordLengte) % bordLengte;
    },
    ArrowDown: () => {
      focusedY = (focusedY + 1) % bordLengte;
    },
    ArrowLeft: () => {
      focusedX = (focusedX - 1 + bordLengte) % bordLengte;
    },
    ArrowRight: () => {
      focusedX = (focusedX + 1) % bordLengte;
    },
    Enter: () => {
      buttons[focusedY]?.[focusedX]?.click();
    },
    " ": () => {
      buttons[focusedY]?.[focusedX]?.click();
    },
  };

  let focusedX = $state(0);
  let focusedY = $state(0);

  let buttons: HTMLButtonElement[][] = [];

  const verwerkKlik = (resultaat: ReturnType<typeof mapZet>): (() => void) =>
    match(resultaat, {
      Ok: (zet) => () => {
        focusedX = zet.x;
        focusedY = zet.y;
        speelZet(zet);
        buttons[focusedY]?.[focusedX]?.focus();
      },
      Err: () => () => {},
    });

  const registreerKnop = (x: number, y: number) => (el: HTMLButtonElement) => {
    if (!buttons[y]) buttons[y] = [];
    buttons[y][x] = el;
  };

  const onkeydown = (e: KeyboardEvent) => {
    const handler: () => void | undefined = toetsEffecten[e.key];
    if (handler) {
      e.preventDefault();
      handler();
      buttons[focusedY]?.[focusedX]?.focus();
    }
  };

  onMount(() => {
    buttons[focusedY]?.[focusedX]?.focus();
  });

  $effect(() => {
    // Refocus na renderen, bijvoorbeeld na een zet
    console.log("Effect: refocus op", focusedX, focusedY);
    buttons[focusedY]?.[focusedX]?.focus();
  });
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
