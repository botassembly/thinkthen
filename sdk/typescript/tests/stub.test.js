import test from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { sdk, backend } from "./support.js";
import { questionOf, streamFor, expectedRows, hostValue, assertKept } from "./contract.js";

// Feature: shared deterministic constructions run through the installed package.
test("stub constructions produce typed rows with no credential or HTTP access", async () => {
  const cases = JSON.parse(await readFile(new URL("../../../construction/stub.json", import.meta.url), "utf8"));
  const server = await backend({ replies: [] });
  const client = new sdk.Client({ backend: "stub", url: server.url, model: "ignored", apiKey: "unused", batch: 1 });
  try {
    for (const fixture of cases) {
      const name = questionOf(fixture.request).function;
      if (name === "filter") await assertKept(client, fixture);
      else {
        const rows = await streamFor(client, fixture.request).toArray();
        assert.deepEqual(hostValue(rows), expectedRows(fixture), name);
      }
    }
    assert.equal(await server.count(), 0);
  } finally { await client.close(); await server.close(); }
});
