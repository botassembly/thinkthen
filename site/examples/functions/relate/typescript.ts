import assert from "node:assert/strict";
import * as tt from "thinkthen";

const names = [
  ["Paul McCartney", "singer"],
  ["Ringo Starr", "singer"],
  ["Yesterday", "song"],
  ["Octopus's Garden", "song"],
] as const;
const whoSings = (await tt.relate(names, {
  relations: ["sings=singer:song"],
})).value;
const sings = whoSings.map((edge) => [
  edge.source.name,
  edge.target.name,
]);
assert.deepEqual(sings, [
  ["Paul McCartney", "Yesterday"],
  ["Ringo Starr", "Octopus's Garden"],
]);
