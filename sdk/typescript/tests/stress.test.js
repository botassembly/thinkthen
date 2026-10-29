import test from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { sdk } from "./support.js";

import { spawn, spawnSync } from "node:child_process";
import { once } from "node:events";
import { createInterface } from "node:readline";
import { fileURLToPath } from "node:url";

async function consume(client, scenario, count, probability, call, basis, widths) {
  call += basis;
  function* records() { for (let i = 0; i < count; i++) yield { id: `call-${call}-record-${basis + i}`, body: `record ${i}` }; }
  let received = 0, failed = 0;
  for await (const row of client.decideStream("Ready?", records())) {
    widths.add(row.id.length);
    assert.equal(row.position, ++received);
    assert.equal(row.id, `call-${call}-record-${basis + received - 1}`);
    if (row.status === "failed") { failed++; assert.equal(row.error.code, scenario.code); }
    else { assert.equal(row.status, "answered"); assert.equal(row.answer, true); assert.equal(row.probability, probability); }
    if (scenario.status === "answered" || scenario.name === "refused") assert.equal(row.status, scenario.status);
  }
  assert.equal(received, count);
  assert.equal(failed > 0, scenario.status === "failed");
  return failed;
}

async function adapter(sample) {
  const c = sample.client;
  const client = new sdk.ThinkThen({ backend: c.backend, url: c.url, apiKey: c.api_key, model: c.model,
    batch: c.batch, retry: { maxRetries: c.retry.max_retries,
      initialBackoffMs: c.retry.initial_backoff_ms, attemptTimeoutMs: c.retry.attempt_timeout_ms } });
  let calls = 0, rows = 0, failed = 0, pulled = 0;
  const widths = new Set();
  try {
    if (sample.cancel) {
      const controller = new AbortController();
      function* records() { for (let i = 0; i < sample.records; i++) { pulled++; if (sample.read_ahead_limit !== null) assert.ok(pulled <= sample.read_ahead_limit, "input read ahead exceeded the engine bound"); yield "body"; } }
      const iterator = client.decideStream("Ready?", records(), { signal: controller.signal })[Symbol.asyncIterator]();
      const pending = assert.rejects(iterator.next(), e => e instanceof sdk.CancelledError && e.code === "local.cancelled");
      const lines = createInterface({ input: process.stdin });
      for await (const line of lines) { assert.equal(line, "cancel"); controller.abort(); break; }
      await pending;
      assert.equal((await iterator.next()).done, true);
      calls = 1;
    } else {
      for (let round = 0; round < sample.rounds; round++) {
        const results = await Promise.all(Array.from({ length: sample.concurrency }, (_, index) => consume(client, sample.scenario, sample.records, sample.probability, round * sample.concurrency + index, sample.identifier_base, widths)));
        for (const result of results) { calls++; rows += sample.records; failed += result; }
        if (process.env.THINKTHEN_STRESS_CHECKPOINT_COMMAND) assert.equal(spawnSync(process.env.THINKTHEN_STRESS_CHECKPOINT_COMMAND, { shell: true, stdio: "inherit" }).status, 0);
      }
    }
    console.log(JSON.stringify({ calls, rows, failed, read_ahead_records: pulled, cancelled: sample.cancel, identifier_width_mask: [...widths].reduce((mask, width) => mask + 2 ** width, 0) }));
  } finally { await client.close(); }
}

if (process.env.THINKTHEN_STRESS_CASE) {
  await adapter(JSON.parse(await readFile(process.env.THINKTHEN_STRESS_CASE)));
} else {
  test("shared stress preserves positions, concurrent calls and batch cancellation", async () => {
    const runner = fileURLToPath(new URL("../../../scripts/stress-adapters.py", import.meta.url));
    const child = spawn("python3", [runner, "--surface", "typescript", "--", process.execPath, fileURLToPath(import.meta.url)], { stdio: "inherit" });
    const [code] = await once(child, "exit");
    assert.equal(code, 0);
  });
}
