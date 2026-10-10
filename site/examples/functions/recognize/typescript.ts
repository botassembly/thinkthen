import assert from "node:assert/strict";
import {Client} from "thinkthen";

const tt = new Client();
try {
  const kinds = {person: null, organization: null,
    place: null};
  const text =
    "Maria Chen joined Northwind Freight, " +
    "a company in Chicago.";
  const facts = (await tt.recognize(
    {version: 1, recognize: {kinds}}, text,
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
} finally { tt.close(); }
