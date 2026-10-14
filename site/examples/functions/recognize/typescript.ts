import assert from "node:assert/strict";
import {Client} from "thinkthen";

const tt = new Client();
try {
  const kinds = {person: null, product: null,
    place: null};
  const text =
    "Sam Patel says his Kindle arrived broken " +
    "in Denver.";
  const facts = (await tt.recognize(
    {version: 1, recognize: {kinds}}, text,
  )).recognizeAnswer()[0].value;
  const names = facts.entities.map((one) => [
    one.text,
    one.kind,
  ]);
  assert.deepEqual(names, [
    ["Sam Patel", "person"],
    ["Kindle", "product"],
    ["Denver", "place"],
  ]);
} finally { tt.close(); }
