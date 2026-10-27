import test from "node:test";
import assert from "node:assert/strict";
import { sdk } from "./support.js";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { MessageChannel } from "node:worker_threads";

// Feature: lazy input and output advance independently on the native stream.
test("a pending input read allows rows and early return", { timeout: 10000 }, async () => {
  const { port1, port2 } = new MessageChannel();
  port1.on("message", () => {});
  let pulled = 0, returned = 0;
  let finishInput;
  const pendingInput = new Promise(resolve => { finishInput = resolve; });
  let inputEntered;
  const entered = new Promise(resolve => { inputEntered = resolve; });
  const input = {
    [Symbol.asyncIterator]() { return this; },
    next() {
      if (++pulled === 1) return Promise.resolve({ done: false, value: "first record" });
      inputEntered();
      return pendingInput;
    },
    return() { returned++; return Promise.resolve({ done: true }); },
  };
  const client = new sdk.ThinkThen({ backend: "stub", batch: 1 });
  const stream = client.decideStream("Ready?", input);
  try {
    const first = stream.next();
    const result = await first;
    await entered;
    assert.equal(result.done, false);
    assert.equal(result.value.position, 1);
    await stream.return();
    assert.equal(returned, 1);
    assert.equal((await stream.next()).done, true);
  } finally {
    finishInput({ done: true });
    await stream.return();
    await client.close();
    port1.close(); port2.close();
  }
});

// Feature: each lazy stream owns independent bounded input and output.
test("100 concurrent async streams complete with ordered rows", { timeout: 30000 }, async () => {
  const client = new sdk.ThinkThen({ backend: "stub" });
  try {
    await Promise.all(Array.from({ length: 100 }, async (_, stream) => {
      async function* records() {
        for (let row = 0; row < 10; row++) yield { id: `${stream}:${row}`, body: "record" };
      }
      let count = 0;
      for await (const row of client.decideStream("Ready?", records())) {
        assert.equal(row.position, count + 1);
        assert.equal(row.id, `${stream}:${count++}`);
        assert.equal(row.status, "answered");
      }
      assert.equal(count, 10);
    }));
  } finally { await client.close(); }
});

// Feature: a long lazy stream retains one native bridge for its entire lifetime.
test("1000 rows start one addon bridge thread", { skip: process.platform === "win32" }, () => {
  const audit = fileURLToPath(new URL("./thread-start-audit.py", import.meta.url));
  const result = spawnSync("python3", [audit], { encoding: "utf8" });
  assert.equal(result.status, 0, result.stdout + result.stderr);
  console.log(result.stdout.trim());
});
