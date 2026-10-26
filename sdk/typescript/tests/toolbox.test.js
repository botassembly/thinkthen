import test from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { sdk } from "./support.js";

// Feature: saved toolboxes hydrate native JavaScript definitions and lookup.
test("saved toolbox round trip preserves host question fields and shared errors", async () => {
  const original = JSON.parse(await readFile(new URL("../../../conformance/toolbox/questions.json", import.meta.url), "utf8"));
  const loaded = sdk.parseToolbox(JSON.stringify(original));
  assert.equal(Object.keys(loaded).length, 9);
  assert.equal(loaded.ready.function, "filter");
  assert.equal(loaded.names.rules[0].name, "pays");
  assert.equal(loaded.names.rules[0].phrase, "pays money to");
  assert.equal(loaded.names.rules[0].symmetric, true);
  assert(Object.isFrozen(loaded));
  assert(Object.isFrozen(loaded.names.rules));
  assert.deepEqual(JSON.parse(sdk.serializeToolbox(loaded)), original);
  const client = new sdk.ThinkThen({backend: "stub"});
  try {
    const originalQuestion = {function: "choose", question: "Which topic?", choices: ["billing", "shipping"], threshold: 0.7, evidence: "/body"};
    const actual = await client.chooseStream(loaded.topic, [{body: "Bill"}]).toArray();
    assert.equal(actual.length, 1);
    assert.deepEqual(actual, await client.chooseStream(originalQuestion, [{body: "Bill"}]).toArray());
  } finally { await client.close(); }
  const malformed = JSON.parse(await readFile(new URL("../../../conformance/toolbox/malformed.json", import.meta.url), "utf8"));
  for (const json of malformed) assert.throws(() => sdk.parseToolbox(json), error => error.code === "usage.question");
});
