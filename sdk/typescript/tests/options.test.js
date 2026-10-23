import test from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { sdk } from "./support.js";

async function execute(client, request, single) {
  const question = request.question.type === "toolbox" ? request.question : request.question.value;
  const rows = [];
  try {
    if (single && request.function !== "rank") {
      for (const record of request.records) rows.push(await client[request.function](question, record, request.options));
    } else {
      for await (const row of client[`${request.function}Stream`](question, request.records, request.options)) rows.push(row);
    }
    return rows;
  } catch (error) { return { error: error.code }; }
}

test("shared saved domains, rank levels and unsupported thresholds", async () => {
  const cases = JSON.parse(await readFile(new URL("../../../construction/options.json", import.meta.url), "utf8"));
  for (const sample of cases) {
    const client = new sdk.ThinkThen(sample.client);
    for (const single of [false, true]) {
      const actual = await execute(client, sample.request, single);
      const expected = sample.expected_error ? { error: sample.expected_error }
        : await execute(client, sample.equivalent_request, single);
      assert.deepEqual(actual, expected, sample.name);
      if (!sample.expected_error) assert.equal(actual.length, sample.request.records.length);
    }
  }
});
