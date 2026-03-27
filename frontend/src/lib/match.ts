type Variant<T, K extends string> = Extract<T, { type: K }>;
type DataVariant<T, K extends string> =
  Variant<T, K> extends { data: infer D } ? D : void;
type MatchHandlers<T extends { type: string }, O> = {
  [K in T["type"]]: (value: DataVariant<T, K>) => O;
};

export const match = <T extends { type: string }, O>(
  value: T,
  handlers: MatchHandlers<T, O>,
): O => handlers[value.type as T["type"]]((value as any)?.data);
