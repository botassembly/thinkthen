import test from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { sdk, backend } from "./support.js";

test("all function constructions match shared stub rows without HTTP", async () => {
  const cases = JSON.parse(await readFile(new URL("../../../construction/stub.json", import.meta.url), "utf8"));
  const server = await backend({ replies: [] });
  delete process.env.THINKTHEN_API_KEY;
  delete process.env.OPENAI_API_KEY;
  process.env.THINKTHEN_BASE_URL = server.url;
  try {
    for (const fixture of cases) {
      const request = fixture.request;
      const client = new sdk.ThinkThen({ adapter: "stub", url: server.url, model: "ignored", batch: 1 });
      const actual = [];
      for await (const row of client[`${request.function}Stream`](request.question.value, request.records, request.options)) actual.push(row);
      const expected = request.function === "filter"
        ? fixture.expected.flatMap((row, i) => row.status === "answered" && row.answer === true ? [request.records[i]] : [])
        : fixture.expected;
      assert.deepEqual(actual, expected, request.function);
    }
    assert.equal(await server.count(), 0);
  } finally { await server.close(); }
});
