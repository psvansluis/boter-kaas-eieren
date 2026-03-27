import type { Cel, Spelstatus, Zet } from "./lib/wasm/rust_wasm";
import { match } from "./lib/wasm";

/**
 * Mapt een Cel naar een Zet, of geeft een foutmelding als de zet niet geldig is.
 *
 * @param spelstatus De huidige status van het spel.
 * @param cel De Cel die gemapt moet worden.
 * @param x De x-coördinaat van de zet.
 * @param y De y-coördinaat van de zet.
 * @returns Een Zet-object als de zet geldig is, of een string met een foutmelding als de zet niet geldig is.
 *
 */
export const mapZet = (
  spelstatus: Spelstatus,
  cel: Cel,
  x: number,
  y: number,
): Zet | string =>
  match(cel, {
    Leeg: () =>
      match(spelstatus, {
        SpelerWint: ({ winnaar }) =>
          `Spel is al afgelopen. ${winnaar.type} heeft gewonnen.`,
        Gelijkspel: () => "Spel is afgelopen in gelijkspel.",
        SpelBezig: ({ speler_met_beurt }) => ({
          x,
          y,
          speler: speler_met_beurt,
        }),
      }),

    Gespeeld: ({ door }) => `Cel (${x}, ${y}) is al bezet door ${door.type}`,
  });
