export type Resultaat<T, E> =
  | { type: "Ok"; data: T }
  | { type: "Err"; data: E };

export const ok = <T>(value: T): Resultaat<T, never> => ({
  type: "Ok",
  data: value,
});

export const err = <E>(error: E): Resultaat<never, E> => ({
  type: "Err",
  data: error,
});

export type ZetFout =
  | { type: "SpelAfgelopen"; data: { winnaar: string } }
  | { type: "Gelijkspel" }
  | { type: "CelBezet"; data: { door: string } };
