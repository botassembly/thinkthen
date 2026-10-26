// Native JSON source text preserves exact numeric IDs without a handwritten parser.
export function fixtureJson(text) {
  return JSON.parse(text, (key, value, context) => key === "id" && typeof value === "number" && Number.isInteger(value) && !Number.isSafeInteger(value)
    ? BigInt(context.source) : value);
}
