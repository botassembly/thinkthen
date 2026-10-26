import test from "node:test";
import assert from "node:assert/strict";
import { sdk, backend, caseFor } from "./support.js";
const yes = { answer: { type: "noul", noul: 0.9 } };

// Ordinary arrays are finite input; invalid records still retain their slots.
test("finite arrays batch valid records and preserve intervening failed positions", async () => {
  const server = await backend(caseFor({}, [], [yes]));
  let client;
  try {
    client = new sdk.ThinkThen({ url: server.url, batch: 16 });
    const input = [{ id: "first", body: "first" }, { id: true }, null, { id: "last", body: "last" }];
    const original = structuredClone(input);
    const rows = [];
    for await (const row of client.decideStream("Ready?", input)) rows.push(row);
    assert.equal(rows.length, 4);
    assert.deepEqual(rows.map(row => row.position), [1, 2, 3, 4]);
    assert.deepEqual(rows.map(row => row.status), ["answered", "failed", "failed", "answered"]);
    assert.equal(rows[0].id, "first");
    assert.equal(rows[3].id, "last");
    assert.equal(rows[1].error.code, "record.invalid");
    assert.equal(rows[2].error.code, "record.missing");
    assert.deepEqual(input, original);
    assert.equal(await server.count(), 1);
  } finally { await client?.close(); await server.close(); }
});

// Request admission wins before any caller-provided iterator or record access.
test("invalid questions cannot invoke array getters proxy traps or iterator getters", async () => {
  const client = new sdk.ThinkThen({ backend: "stub" });
  let accessed = 0;
  const array = [];
  Object.defineProperty(array, 0, { get() { accessed++; return "evidence"; } });
  const custom = {
    get [Symbol.iterator]() { accessed++; return function* () { yield "evidence"; }; },
  };
  const proxy = new Proxy(["evidence"], {
    get(target, name, receiver) { accessed++; return Reflect.get(target, name, receiver); },
    getPrototypeOf(target) { accessed++; return Reflect.getPrototypeOf(target); },
    getOwnPropertyDescriptor(target, name) { accessed++; return Reflect.getOwnPropertyDescriptor(target, name); },
    has(target, name) { accessed++; return Reflect.has(target, name); },
  });
  for (const input of [array, custom, proxy]) {
    const stream = client.decideStream("", input);
    await assert.rejects(stream[Symbol.asyncIterator]().next(), error => error instanceof sdk.UsageError && error.code === "usage.question");
    assert.equal(accessed, 0);
  }
});

// An array can supply a caller-owned iterator whose extent is unknown.
for (const kind of ["own iterator", "async iterator", "subclass iterator"]) {
  test(`custom array ${kind} stays on the streaming path`, async () => {
    const server = await backend(caseFor({}, [], [yes]));
    let client;
    try {
      client = new sdk.ThinkThen({ url: server.url, batch: 1 });
      let pulled = 0;
      let returned = 0;
      class Input extends Array {}
      const input = kind === "subclass iterator" ? new Input("unused array element") : ["unused array element"];
      const source = function* () {
        try { for (let i = 0; i < 1000; i++) { pulled++; yield `evidence ${i}`; } }
        finally { returned++; }
      };
      if (kind === "subclass iterator") Input.prototype[Symbol.iterator] = source;
      else input[kind === "async iterator" ? Symbol.asyncIterator : Symbol.iterator] = source;
      for await (const row of client.decideStream("Ready?", input)) {
        assert.equal(row.status, "answered");
        assert.equal(row.position, 1);
        break;
      }
      assert(pulled > 0 && pulled < 1000);
      assert.equal(returned, 1);
      const requests = await server.count();
      assert(requests > 0 && requests <= pulled);
    } finally { await client?.close(); await server.close(); }
  });
}

// A cancelled finite collection must stop before its first backend request.
for (const when of ["before", "during"]) {
  test(`abort ${when} finite array collection stops input and makes no backend request`, async () => {
    const server = await backend(caseFor({}, [], [yes]));
    let client;
    try {
      client = new sdk.ThinkThen({ url: server.url, batch: 16 });
      const controller = new AbortController();
      let first = 0;
      let second = 0;
      const input = [];
      Object.defineProperties(input, {
        0: { get() { first++; controller.abort("stop collection"); return "first"; } },
        1: { get() { second++; return "second"; } },
      });
      if (when === "before") controller.abort("do not start");
      const stream = client.decideStream("Ready?", input, { signal: controller.signal });
      await assert.rejects(stream[Symbol.asyncIterator]().next(), error => error instanceof sdk.CancelledError && error.code === "local.cancelled");
      assert.equal(first, when === "before" ? 0 : 1);
      assert.equal(second, 0);
      assert.equal(await server.count(), 0);
    } finally { await client?.close(); await server.close(); }
  });
}
