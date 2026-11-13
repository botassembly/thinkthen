import test from "node:test";
import assert from "node:assert/strict";
import { sdk } from "./support.js";

test("client usage copies exact counters and survives awaited close", async () => {
  const client = new sdk.ThinkThen({backend: "stub"});
  const before = client.usage();
  assert.equal(before.attempts, 0n);
  assert.equal(before.activeCalls, 0n);
  assert.equal(before.inputTokens.reported, 0n);
  assert.equal(before.inputTokens.unknownAttempts, 0n);
  assert.ok(Object.isFrozen(before));
  assert.ok(Object.isFrozen(before.inputTokens));
  const row = await client.decide("Ready?", "yes").row();
  assert.equal(row.status, "answered");
  await client.close();
  await client.close();
  const after = client.usage();
  assert.equal(after.attempts, 0n);
  assert.equal(after.activeCalls, 0n);
  assert.equal(after.outputTokens.reported, 0n);
  assert.equal(before.attempts, 0n);
  await assert.rejects(client.decide("Ready?", "yes").row(), {code: "usage.closed"});
});
