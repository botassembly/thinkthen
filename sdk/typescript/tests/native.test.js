import test from "node:test";
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { once } from "node:events";
import { pathToFileURL } from "node:url";
import { runInNewContext } from "node:vm";
import { readFile } from "node:fs/promises";
import { sdk, backend, caseFor } from "./support.js";

const yes = { answer: { type: "noul", noul: 0.9 } };

test("filter retains original objects through a failure and reports it after the last kept record", async () => {
  const server = await backend(caseFor({}, [], [
    { contains: '"bad"', status: 400, body: {} },
    { contains: '"no"', answer: { type: "noul", noul: 0.1 } }, yes,
  ]));
  try {
    const client = new sdk.ThinkThen({ url: server.url, batch: 1 });
    const input = [{ id: "first", body: "good" }, { body: "bad" }, { body: "no" }, { id: "last", body: "good again" }];
    const kept = [];
    await assert.rejects(async () => {
      for await (const record of client.filterStream("Ready?", input, { on: "/body" })) kept.push(record);
    }, error => error instanceof sdk.BackendError && error.code === "backend.status");
    assert.equal(kept.length, 2);
    assert.equal(kept[0], input[0]);
    assert.equal(kept[1], input[3]);
    assert.equal(await server.count(), 4);
  } finally { await server.close(); }
});

test("async input is bounded, runs beside JavaScript, and closes on break or source failure", async () => {
  const server = await backend(caseFor({}, [], [yes]));
  try {
    const client = new sdk.ThinkThen({ url: server.url, batch: 1 });
    let pulled = 0;
    let closed = 0;
    let ticked = false;
    async function* records() {
      try {
        for (let i = 0; i < 1000; i++) {
          pulled++;
          await new Promise(resolve => setImmediate(() => { ticked = true; resolve(); }));
          yield `record ${i}`;
        }
      } finally { closed++; }
    }
    for await (const row of client.decideStream("Ready?", records())) {
      assert.equal(row.status, "answered");
      assert.equal(ticked, true);
      break;
    }
    assert(pulled < 1000);
    assert.equal(closed, 1);
    const failure = new Error('{"code":"usage.question","message":"source failed"}');
    async function* broken() { throw failure; }
    await assert.rejects(async () => {
      for await (const row of client.decideStream("Ready?", broken())) void row;
    }, error => error === failure);
  } finally { await server.close(); }
});

test("abort preserves its reason, closes pending input and leaves the client's next call usable", async () => {
  const server = await backend(caseFor({}, [], [yes]));
  try {
    const client = new sdk.ThinkThen({ url: server.url, batch: 1 });
    const pre = new AbortController();
    const reason = { cancelled: true };
    pre.abort(reason);
    await assert.rejects(client.decide("Ready?", "evidence", { signal: pre.signal }), error => error === reason);
    assert.equal(await server.count(), 0);
    const controller = new AbortController();
    let entered;
    const pulled = new Promise(resolve => { entered = resolve; });
    let release;
    const blocked = new Promise(resolve => { release = resolve; });
    let returned = 0;
    const records = {
      [Symbol.asyncIterator]() { return this; },
      next() { entered(); return blocked; },
      return() { returned++; return Promise.resolve({ done: true }); },
    };
    const stream = client.decideStream("Ready?", records, { signal: controller.signal });
    const waiting = stream[Symbol.asyncIterator]().next();
    await pulled;
    controller.abort(reason);
    await assert.rejects(waiting, error => error === reason);
    assert.equal(returned, 1);
    release({ done: true });
    const row = await client.decide("Ready?", "evidence");
    assert.equal(row.answer, true);
    assert.equal(await server.count(), 1);
  } finally { await server.close(); }
});

test("clients keep their own backend and question options", async () => {
  const a = await backend(caseFor({}, [], [yes]));
  const b = await backend(caseFor({}, [], [{ answer: { type: "noul", noul: 0.1 } }]));
  try {
    const left = new sdk.ThinkThen({ url: a.url, model: "left", batch: 1 });
    const right = new sdk.ThinkThen({ url: b.url, model: "right", batch: 2 });
    const [one, two] = await Promise.all([
      left.decide("Ready?", { body: "yes" }, { on: "/body", threshold: "0.2:0.8" }),
      right.decide("Ready?", "no"),
    ]);
    assert.equal(one.answer, true);
    assert.equal(two.answer, false);
    assert.equal((await left.decide("Ready?", "again")).answer, true);
    assert.equal(await a.count(), 2);
    assert.equal(await b.count(), 1);
  } finally { await a.close(); await b.close(); }
});

test("answer spans use String.slice through find, names, relations and annotation with detail in the same string units", async () => {
  const tags = { type: "choice", probabilities: { BEGIN: 0, INSIDE: 0, END: 0, SINGLE: 1, OUT: 0 } };
  const server = await backend(caseFor({}, [], [
    { criterion: "u1", answer: { type: "choice", probabilities: { u1: 0, u2: 1, none: 0 } } },
    { criterion: "BEGIN", answer: tags },
    { criterion: "person", contains: "[[Ada]]", answer: { type: "choice", probabilities: { person: 1, company: 0, "none of these": 0 } } },
    { criterion: "person", contains: "[[Acme]]", answer: { type: "choice", probabilities: { person: 0, company: 1, "none of these": 0 } } },
    { criterion: "person", answer: { type: "choice", probabilities: { person: 0, company: 0, "none of these": 1 } } },
    yes,
  ]));
  try {
    const client = new sdk.ThinkThen({ url: server.url });
    const text = "😀 Ada Acme";
    const rules = [{ name: "works_for", source: "person", target: "company" }];
    const found = await client.find("Who?", "😀\nAda");
    assert.equal("😀\nAda".slice(found.answer.start, found.answer.end), "Ada");
    const names = await client.recognize(["person", "company"], text);
    assert.deepEqual(names.answer.map(n => text.slice(n.start, n.end)), ["Ada", "Acme"]);
    assert.equal(names.detail.probabilities.pieces[0].end, 2);
    const relations = await client.relate(rules, text);
    assert.equal(relations.answer.length, 1);
    const relation = relations.answer[0];
    assert.equal(text.slice(relation.source.start, relation.source.end), "Ada");
    assert.equal(text.slice(relation.target.start, relation.target.end), "Acme");
    const row = await client.annotate({ questions: {
      found: { find: "Who?", on: "/lines" },
      names: { recognize: ["person", "company"], on: "/text" },
      relations: { relate: rules, on: "/text" },
    } }, { lines: "😀\nAda", text });
    assert.equal(row.answer.found.answer.start, 3);
    assert.equal(row.answer.names.answer[0].start, 3);
    assert.equal(row.answer.relations.answer[0].target.start, 7);
    assert.equal(row.answer.names.detail.pieces[0].end, 2);
  } finally { await server.close(); }
});

test("module admission follows promise and iteration contracts and error constructors match types", async () => {
  delete process.env.THINKTHEN_API_KEY;
  try {
    let promise;
    assert.doesNotThrow(() => { promise = sdk.decide("Ready?", "evidence"); });
    await assert.rejects(promise, error => error instanceof sdk.UsageError && error.code === "usage.key-missing");
    let stream;
    assert.doesNotThrow(() => { stream = sdk.decideStream("Ready?", ["evidence"]); });
    await assert.rejects(stream[Symbol.asyncIterator]().next(), error => error instanceof sdk.UsageError && error.code === "usage.key-missing");
    const error = new sdk.UsageError({ code: "usage.question", message: "Question failed" });
    assert.equal(error.message, "Question failed");
    assert.equal(error.code, "usage.question");
    assert(error instanceof sdk.ThinkThenError);
  } finally { process.env.THINKTHEN_API_KEY = "sk-test-0123456789"; }
});

test("JSON values stay single records and malformed host text fails before sending", async () => {
  const server = await backend(caseFor({}, [], [yes]));
  try {
    const client = new sdk.ThinkThen({ url: server.url });
    const rows = [];
    for await (const row of client.decideStream("Ready?", [null, true, 42, ["a", "b"]])) rows.push(row);
    assert.equal(rows.length, 4);
    assert.deepEqual(rows.map(row => row.id), [1, 2, 3, 4]);
    assert(rows.every(row => row.status === "answered" && row.answer === true));
    const one = await client.decide("Ready?", ["a", "b"]);
    assert.equal(one.id, 1);
    assert.equal(one.answer, true);
    const sent = await server.count();
    await assert.rejects(client.decide("Ready?", "\ud800"), error => error instanceof sdk.UsageError && error.code === "usage.record");
    assert.equal(await server.count(), sent);
  } finally { await server.close(); }
});

test("tagged in-memory questions keep client routing and literal question strings", async () => {
  const client = new sdk.ThinkThen({ adapter: "stub" });
  const inline = await client.decide("literal.json", "evidence");
  const named = await client.decide({ type: "toolbox", value: { questions: { ready: { decide: "literal.json" } } }, name: "ready" }, "evidence");
  assert.deepEqual(named, inline);
  assert.equal(inline.detail.source.backend, "stub");
});

// Raw strings and structured evidence must preserve the same caller bytes.
test("raw text keeps empty strings and embedded or trailing NUL characters", async () => {
  const client = new sdk.ThinkThen({ adapter: "stub" });
  for (const text of ["", "a\0b\0", 'quoted "text"\n😀']) {
    const raw = await client.decide("Ready?", text, { on: "/body" });
    const structured = await client.decide("Ready?", { body: text }, { on: "/body" });
    assert.deepEqual(raw, structured);
  }
});

test("row detail decodes only when read", async () => {
  const original = JSON.parse;
  let decoded = 0;
  JSON.parse = (...args) => {
    const value = original(...args);
    if (value !== null && typeof value === "object" && "source" in value) decoded++;
    return value;
  };
  try {
    const row = await new sdk.ThinkThen({ adapter: "stub" }).decide("Ready?", "text");
    assert.equal(decoded, 0);
    const detail = row.detail;
    assert.equal(detail.source.backend, "stub");
    assert.equal(decoded, 1);
    assert.equal(row.detail, detail);
    assert.equal(decoded, 1);
  } finally { JSON.parse = original; }
});

test("JSON conversion refuses non-finite numbers in settings, calls and nested records", async () => {
  const client = new sdk.ThinkThen({ adapter: "stub" });
  const usage = error => error instanceof sdk.UsageError && error.code === "usage.record";
  const numbers = [NaN, Infinity, -Infinity];
  const values = numbers.flatMap(value => [value, new Number(value), runInNewContext("new Number(value)", { value })]);
  for (const value of values) {
    assert.throws(() => new sdk.ThinkThen({ adapter: "stub", batch: value }), usage);
    await assert.rejects(client.decide("Ready?", "alpha", { threshold: value }), usage);
    await assert.rejects(client.decide("Ready?", { body: [value] }), usage);
    const stream = client.decideStream("Ready?", [{ body: { number: value } }]);
    await assert.rejects(stream[Symbol.asyncIterator]().next(), usage);
  }
  assert.equal((await client.decide("Ready?", { body: [null, 1] })).status, "answered");
  assert.deepEqual(await client.decide("Ready?", { body: new Number(1) }), await client.decide("Ready?", { body: 1 }));
});

test("unordered relation answers retain their direction flag", async () => {
  const fixture = JSON.parse(await readFile(new URL("../../../conformance/relate-undirected.json", import.meta.url), "utf8"));
  const server = await backend(fixture);
  try {
    const client = new sdk.ThinkThen({ url: server.url });
    const text = fixture.request.records[0];
    const row = await client.relate(fixture.request.question.value, text);
    assert.equal(row.answer.length, 1);
    assert.equal(row.answer[0].either, true);
    assert.equal(text.slice(row.answer[0].source.start, row.answer[0].source.end), row.answer[0].source.text);
  } finally { await server.close(); }
});


test("abandoned stream releases Node's event loop", async () => {
  const url = pathToFileURL(process.env.THINKTHEN_NODE_PACKAGE).href;
  for (const mode of ["after-row", "pending-input"]) {
    const child = spawn(process.execPath, ["--input-type=module", "-e", `
      import { ThinkThen } from ${JSON.stringify(url)};
      const input = ${JSON.stringify(mode)} === "pending-input"
        ? { [Symbol.asyncIterator]() { return this; }, next() { return new Promise(() => {}); } }
        : ["first", "second"];
      const stream = new ThinkThen({ adapter: "stub", batch: 1 }).decideStream("Ready?", input);
      const pending = stream.next();
      if (${JSON.stringify(mode)} === "after-row") await pending;
    `], { stdio: ["ignore", "pipe", "pipe"] });
    const done = once(child, "exit");
    const timeout = setTimeout(() => child.kill(), 10000);
    try {
      const [code, signal] = await done;
      assert.equal(signal, null, `abandoned stream kept Node alive: ${mode}`);
      assert.equal(code, 0);
    } finally { clearTimeout(timeout); child.kill(); }
  }
});

test("future engine error families preserve code and message in the base error", async () => {
  let family;
  const parse = JSON.parse;
  JSON.parse = (...args) => {
    const value = parse(...args);
    if (value?.family && value?.code) value.family = family;
    return value;
  };
  try {
    for (family of ["future", "constructor", "__proto__"]) {
      assert.throws(() => new sdk.ThinkThen({ adapter: "stub", batch: 0 }),
        error => error.constructor === sdk.ThinkThenError && error.code === "usage.batch" && error.message.length > 0);
    }
  } finally { JSON.parse = parse; }
});
