import test from "node:test";
import assert from "node:assert/strict";
import { sdk } from "./support.js";

function deferred() {
  let resolve;
  const promise = new Promise(done => { resolve = done; });
  return { promise, resolve };
}

// Feature: stop paths interrupt a pending pull without joining its next promise.
for (const action of ["abort", "return", "dispose", "close"]) {
  test(`${action} interrupts pending next and awaits iterator return exactly once`, { timeout: 10000 }, async () => {
    const entered = deferred();
    const inputReply = deferred();
    const returnEntered = deferred();
    const returnReply = deferred();
    const controller = new AbortController();
    let returned = 0;
    const input = {
      [Symbol.asyncIterator]() { return this; },
      next() { entered.resolve(); return inputReply.promise; },
      return() { returned++; returnEntered.resolve(); return returnReply.promise; },
    };
    const client = new sdk.ThinkThen({ backend: "stub" });
    const stream = client.decideStream("Ready?", input, { signal: controller.signal });
    const pending = stream.next();
    const outcome = pending.then(value => ({ value }), error => ({ error }));
    await entered.promise;
    let stopping;
    if (action === "abort") controller.abort({ callerReason: true });
    else if (action === "return") stopping = stream.return();
    else if (action === "dispose") stopping = stream[Symbol.asyncDispose]();
    else stopping = client.close();
    try {
      await returnEntered.promise;
      assert.equal(returned, 1);
      // Observe stop before releasing the input's next promise.
      returnReply.resolve({ done: true });
      if (stopping) await stopping;
      const result = await outcome;
      if (action === "abort") {
        assert(result.error instanceof sdk.CancelledError);
        assert.equal(result.error.code, "local.cancelled");
        assert.notEqual(result.error, controller.signal.reason);
      } else {
        assert.equal(result.error, undefined);
        assert.equal(result.value.done, true);
      }
      inputReply.resolve({ done: false, value: "late input must not publish" });
      assert.equal((await stream.next()).done, true);
      assert.equal(returned, 1);
    } finally {
      inputReply.resolve({ done: true });
      returnReply.resolve({ done: true });
      await stream.return();
      await client.close();
    }
  });
}

// Feature: AbortSignal cancellation has one typed error regardless of reason.
test("already aborted signals never acquire the source and always use CancelledError", async () => {
  const client = new sdk.ThinkThen({ backend: "stub" });
  let pulled = 0;
  const source = { [Symbol.iterator]() { return { next() { pulled++; return { done: false, value: "evidence" }; } }; } };
  try {
    for (const reason of [undefined, "stop", { stop: true }, new Error("caller error")]) {
      const controller = new AbortController();
      controller.abort(reason);
      const isCancel = error => error instanceof sdk.CancelledError && error.code === "local.cancelled";
      await assert.rejects(client.decide("Ready?", "evidence", { signal: controller.signal }), isCancel);
      const stream = client.decideStream("Ready?", source, { signal: controller.signal });
      await assert.rejects(stream.next(), isCancel);
      assert.equal((await stream.next()).done, true);
    }
    assert.equal(pulled, 0);
  } finally { await client.close(); }
});

// Feature: breaking iteration closes its source while retained rows remain usable.
test("break and repeated disposal release one iterator and preserve other client calls", async () => {
  const client = new sdk.ThinkThen({ backend: "stub", batch: 1 });
  let returned = 0;
  async function* source() {
    try { yield "first"; yield "second"; }
    finally { returned++; }
  }
  const stream = client.chooseStream("Team?", source(), { choices: ["billing", "shipping"] });
  let retained;
  try {
    for await (const row of stream) { retained = row; break; }
    assert.equal(returned, 1);
    await stream.return();
    await stream[Symbol.asyncDispose]();
    assert.equal(returned, 1);
    const independent = await client.decide("Ready?", "still open");
    assert.equal(independent.function, "decide");
    await client.close();
    await client.close();
    assert.equal(retained.status, "answered");
    assert(["billing", "shipping"].includes(retained.answer));
    assert(Object.isFrozen(retained.detail));
    assert(Object.isFrozen(retained.detail.labels));
    await assert.rejects(client.decide("Ready?", "closed"), error => error instanceof sdk.ThinkThenError);
  } finally { await stream.return(); await client.close(); }
});

// Feature: source exceptions keep their identity after owned resources close.
test("source failure is not parsed as an engine error and closes once", async () => {
  const client = new sdk.ThinkThen({ backend: "stub" });
  const failure = new Error('{"code":"usage.question","message":"source error"}');
  let returned = 0;
  const input = {
    [Symbol.asyncIterator]() { return this; },
    async next() { throw failure; },
    async return() { returned++; return { done: true }; },
  };
  try {
    const stream = client.decideStream("Ready?", input);
    await assert.rejects(stream.next(), error => error === failure);
    assert.equal(returned, 1);
    assert.equal((await stream.next()).done, true);
  } finally { await client.close(); }
});

// Feature: active native cancellation closes the remote operation and spares its client.
test("abort after request arrival closes native I/O", { timeout: 10000 }, async () => {
  const { backend } = await import("./support.js");
  const held = await backend(["--held", "body"]);
  const controller = new AbortController();
  const client = new sdk.ThinkThen({ backend: "systemone", url: held.url, apiKey: "test-only", batch: 1 });
  const stream = client.decideStream("Ready?", ["evidence"], { signal: controller.signal });
  const outcome = stream.next().then(value => ({ value }), error => ({ error }));
  try {
    assert.equal(await held.event("arrived"), "arrived");
    controller.abort("stop");
    const result = await outcome;
    assert(result.error instanceof sdk.CancelledError);
    assert.equal(result.error.code, "local.cancelled");
    assert.equal(await held.event("closed"), "closed");
    assert.equal((await stream.next()).done, true);
  } finally { controller.abort(); await stream.return(); await client.close(); await held.close(); }
});

// Feature: concurrent next calls serialize and each call owns its cancellation.
test("aborting one call leaves another call's queued reads in order", async () => {
  const client = new sdk.ThinkThen({backend:"stub",batch:1});
  const entered = deferred();
  const pendingInput = deferred();
  const controller = new AbortController();
  let returned = 0;
  const blocked = client.decideStream("Ready?", {
    [Symbol.asyncIterator]() { return this; },
    next() { entered.resolve(); return pendingInput.promise; },
    async return() { returned++; return {done:true}; },
  }, {signal:controller.signal});
  const active = blocked.next().then(value => ({value}), error => ({error}));
  await entered.promise;
  const independent = client.decideStream("Ready?", ["one","two","three"]);
  try {
    assert.equal((await independent.next()).value.position,1);
    controller.abort("stop one call");
    assert((await active).error instanceof sdk.CancelledError);
    const values = await Promise.all([independent.next(),independent.next(),independent.next()]);
    assert.deepEqual(values.slice(0,2).map(value => value.value.position),[2,3]);
    assert.equal(values[2].done,true);
    assert.equal(returned,1);
  } finally { pendingInput.resolve({done:true}); await blocked.return(); await independent.return(); await client.close(); }
});
