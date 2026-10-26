import test from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { sdk, backend } from "./support.js";
import { questionOf, streamFor, expectedRows, hostValue } from "./contract.js";

// Feature: shared deterministic constructions run through the installed package.
test("stub constructions produce typed rows with no credential or HTTP access", async () => {
  const cases = JSON.parse(await readFile(new URL("../../../construction/stub.json", import.meta.url), "utf8"));
  const server = await backend({ replies: [] });
  const client = new sdk.ThinkThen({ backend: "stub", url: server.url, model: "ignored", apiKey: "unused", batch: 1 });
  try {
    for (const fixture of cases) {
      const name = questionOf(fixture.request).function;
      const rows = await streamFor(client, fixture.request).toArray();
      const expected = expectedRows(fixture);
      assert.equal(rows.length, expected.length, name);
      if (name === "filter") {
        for (const [index, item] of rows.entries()) {
          assert.equal(item.item, fixture.request.records[expected[index].position - 1]);
          assert.equal(item.kept, expected[index].status === "answered" && expected[index].answer === true);
          assert.deepEqual(hostValue(item.row), expected[index]);
        }
      } else assert.deepEqual(hostValue(rows), expected, name);
    }
    assert.equal(await server.count(), 0);
  } finally { await client.close(); await server.close(); }
});
