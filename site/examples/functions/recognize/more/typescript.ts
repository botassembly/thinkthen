import assert from "node:assert/strict";
import {Client} from "thinkthen";

const tt = new Client();
try {

const text =
  "Maria Chen joined Northwind Freight, " +
  "a company in Chicago.";
const kinds = {person: null, organization: null,
  place: null};
const relations = {
  works_for: ["person", "organization"] as const,
  based_in: ["organization", "place"] as const,
};
const authored = {
  version: 1,
  recognize: {
    kinds,
    relations: Object.entries(relations).map(
      ([name, [source, target]]) => ({
        name, source, target, either: false,
      }),
    ),
  },
};
const facts = (await tt.recognize(
  authored, text,
)).results[0].value;
const names = facts.entities.map((one) => [
  one.text,
  one.kind,
]);
assert.deepEqual(names, [
  ["Maria Chen", "person"],
  ["Northwind Freight", "organization"],
  ["Chicago", "place"],
]);
const links = (facts.relations ?? []).map((one) => [
  one.relation,
  one.source.text,
  one.target.text,
]);
assert.deepEqual(links, [
  ["works_for", "Maria Chen", "Northwind Freight"],
  ["based_in", "Northwind Freight", "Chicago"],
]);

} finally { tt.close(); }
