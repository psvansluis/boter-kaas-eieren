import type { Cel, Spelstatus, Zet } from "./wasm/rust_wasm";
import { match } from "./match";
import type { Resultaat, ZetFout } from "./resultaat";
import { ok, err } from "./resultaat";

export const mapZet = (
  spelstatus: Spelstatus,
  cel: Cel,
  x: number,
  y: number,
): Resultaat<Zet, ZetFout> =>
  match(cel, {
    Leeg: () =>
      match(spelstatus, {
        SpelerWint: ({ winnaar }) =>
          err({ type: "SpelAfgelopen", data: { winnaar: winnaar.type } }),

        Gelijkspel: () => err({ type: "Gelijkspel" }),

        SpelBezig: ({ speler_met_beurt }) =>
          ok({
            x,
            y,
            speler: speler_met_beurt,
          }),
      }),

    Gespeeld: ({ door }) =>
      err({ type: "CelBezet", data: { door: door.type } }),
  });
