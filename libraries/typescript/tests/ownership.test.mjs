// Punch-list item 3 for TypeScript: canonical results and ownership. The
// results are the host's own plain objects; mutating them at any depth
// must not change the engine's next answer, and they stay readable
// through later calls. The offsets count UTF-16 units, the host's own
// indexing. Offline: the stand-in answers from the recordings.
import test from "node:test";
import assert from "node:assert/strict";

import * as tt from "../index.mjs";

const TWICE =
  "Chicago sent a delegation in March, and Chicago hosted the reply in June.";

test("recognize results are the host's own, at every depth", async () => {
  const first = await tt.recognize(TWICE, { kinds: ["place"] });
  assert.equal(first.entities.length, 2);
  assert.equal(first.entities[0].text, "Chicago");
  // Mutate every layer the caller was handed.
  first.entities[0].text = "Oslo";
  first.entities[0].kind = "organization";
  first.entities[0].start = 999;
  first.entities.length = 0;
  first.relations.push({ name: "junk", source: 0, target: 0, probability: 0 });

  const second = await tt.recognize(TWICE, { kinds: ["place"] });
  assert.equal(second.entities.length, 2);
  assert.equal(second.entities[0].text, "Chicago");
  assert.equal(second.entities[0].kind, "place");
  assert.equal(second.entities[0].start, 0);
  assert.equal(second.relations.length, 0);
  // UTF-16 units slice the name, repeated names included.
  assert.equal(TWICE.slice(second.entities[0].start, second.entities[0].end), "Chicago");
  assert.equal(TWICE.slice(second.entities[1].start, second.entities[1].end), "Chicago");
});

test("decide_many answers are the host's own and stay readable", async () => {
  const records = ["I want a refund", "hello team"];
  const first = await tt.decide_many("Is this a complaint?", records);
  assert.deepEqual(first, [true, false]);
  first[0] = false;
  first.length = 0;
  if (global.gc) global.gc();
  const second = await tt.decide_many("Is this a complaint?", records);
  assert.deepEqual(second, [true, false]);
});

test("relation endpoints are typed ids on the result", async () => {
  const found = await tt.recognize(
    "Maria Chen joined Northwind Freight in Chicago last spring.",
    { kinds: ["person", "organization", "place"], relations: { works_for: ["person", "organization"] } },
  );
  const relation = found.relations[0];
  assert.equal(relation.name, "works_for");
  assert.equal(relation.source, 1);
  assert.equal(relation.target, 2);
  assert.equal(typeof relation.probability, "number");
});
