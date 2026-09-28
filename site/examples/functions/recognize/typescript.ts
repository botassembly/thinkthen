import assert from "node:assert/strict";
import * as tt from "thinkthen";

const text =
  "Maria Chen joined Northwind Freight, " +
  "a company in Chicago.";
const kinds = ["person", "organization", "place"];
const relations = {
  works_for: ["person", "organization"],
  based_in: ["organization", "place"],
};
const facts = (await tt.recognize(text, {
  kinds,
  relations,
})).value;
const names = facts.entities.map((one) => [
  one.name,
  one.kind,
]);
assert.deepEqual(names, [
  ["Maria Chen", "person"],
  ["Northwind Freight", "organization"],
  ["Chicago", "place"],
]);
const links = facts.relations.map((one) => [
  one.relation,
  one.source.name,
  one.target.name,
]);
assert.deepEqual(links, [
  ["works_for", "Maria Chen", "Northwind Freight"],
  ["based_in", "Northwind Freight", "Chicago"],
]);
