import assert from "node:assert/strict";
import * as tt from "thinkthen";

const kinds = ["person", "organization", "place"];
const text =
  "Maria Chen joined Northwind Freight, " +
  "a company in Chicago.";
const facts = (await tt.recognize(text, { kinds })).value;
const names = facts.entities.map((one) => [
  one.text,
  one.kind,
]);
assert.deepEqual(names, [
  ["Maria Chen", "person"],
  ["Northwind Freight", "organization"],
  ["Chicago", "place"],
]);
