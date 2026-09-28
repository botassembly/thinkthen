import assert from "node:assert/strict";
import * as tt from "thinkthen";

const rules = [
  "Book economy class for every flight under six hours.",
  "Submit receipts within 30 days of the trip.",
  "Hotel stays are capped at 200 dollars a night.",
  "Employees may book business class on any flight.",
  "Rental cars need a manager's approval.",
  "Receipts may be submitted at any time, " +
    "with no deadline.",
  "Meals are reimbursed up to 60 dollars a day.",
  "Use the company travel portal for all bookings.",
];
const entities = rules.map(
  (rule): readonly [string, string] => [rule, "rule"],
);
const contradictions = (await tt.relate(entities, {
  relations: ["contradicts"],
  either: ["contradicts"],
  threshold: 0.5,
})).value;
const pairs = contradictions.map((edge) => [
  edge.source.name,
  edge.target.name,
  edge.probability,
]);
assert.deepEqual(pairs, [
  [rules[0], rules[3], 0.84],
  [rules[1], rules[5], 0.99],
]);
