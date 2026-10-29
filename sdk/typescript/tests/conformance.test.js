import test from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { pathToFileURL } from "node:url";
import { sdk, backend } from "./support.js";
import { streamFor, expectedRows, hostValue } from "./contract.js";
import { fixtureJson } from "./canonical.js";

const directory = new URL("../../../conformance/typed/", import.meta.url);
const manifest = JSON.parse(await readFile(new URL("manifest.json", directory), "utf8"));

// Feature: regenerated function contracts translate identically for both backend wires.
for (const file of manifest.cases) {
  for (const selected of manifest.backends) {
    test(`canonical typed rows: ${selected} ${file}`, async () => {
      const fixture = fixtureJson(await readFile(new URL(file, directory), "utf8"));
      const settings = {...manifest.client, ...fixture.client, backend: selected};
      const server = await backend({replies: fixture.replies});
      let client;
      const options = {...hostSettings(settings), url: server.url, apiKey: "test-only"};
      try {
        const name = fixture.request.question.function;
        const cancel = new AbortController();
        if (fixture.cancelled) cancel.abort();
        if (fixture.expected.error) {
          await assert.rejects(async () => {
            client = new sdk.ThinkThen(options);
            return streamFor(client, fixture.request, cancel.signal).toArray();
          },
            error => error instanceof sdk.ThinkThenError && error.code === fixture.expected.error);
          assert.equal(await server.count(), fixture.requests);
          return;
        }
        client = new sdk.ThinkThen(options);
        const items = await streamFor(client, fixture.request, cancel.signal).toArray();
        const rows = name === "filter" ? items.map(item => item.row) : items;
        const expected = expectedRows(fixture);
        assert.equal(rows.length, expected.length);
        for (const [index, row] of rows.entries()) {
          assert.equal(row.function, name);
          assertRow(row, expected[index], selected, settings.model);
          if (name === "filter") {
            assert.equal(items[index].item, fixture.request.records[row.position - 1]);
            assert.equal(items[index].kept, row.status === "answered" && row.answer === true);
          }
        }
        assert.equal(await server.count(), fixture.requests);
      } finally { await client?.close(); await server.close(); }
    });
  }
}
function assertRow(actual, expected, backend, model) {
  assert.equal(actual.source.backend, backend);
  assert.equal(actual.source.model, model);
  assert.equal(typeof actual.source.questionIdentity, "string");
  assert.deepEqual(normalize(hostValue(actual)), normalize(expected));
}
function normalize(value) {
  if (Array.isArray(value)) return value.map(normalize);
  if (value === null || typeof value !== "object") return value;
  return Object.fromEntries(Object.entries(value).flatMap(([key, child]) => {
    if (key === "message" && typeof value.code === "string") return [];
    if (key === "source" && child && "questionIdentity" in child) {
      child = Object.fromEntries(Object.entries(child).filter(([name]) => !["backend", "model", "questionIdentity"].includes(name)));
      if (!Object.keys(child).length) return [];
    }
    return [[key, normalize(child)]];
  }));
}
function hostSettings(settings) {
  const result = {...settings};
  if (settings.api_key !== undefined) { result.apiKey = settings.api_key; delete result.api_key; }
  if (settings.retry) result.retry = Object.fromEntries(Object.entries(settings.retry).map(([key, value]) => [({max_retries:"maxRetries", initial_backoff_ms:"initialBackoffMs", attempt_timeout_ms:"attemptTimeoutMs"})[key] ?? key, value]));
  return result;
}

// Admission cases run through installed public host boundaries. Question calls
// use a request envelope; settings use the host's camelCase option names.
const admissions = JSON.parse(await readFile(new URL("../../../conformance/public-contract/usage-admission.json", import.meta.url), "utf8"));
for (const fixture of admissions) {
  const route = {question: "projection: tagged question in public request envelope",
    toolbox: "direct: saved toolbox JSON", settings: "projection: camelCase retry options"}[fixture.boundary];
  test(`public admission: ${fixture.name} (${route})`, async () => {
    assert.ok(route, "every admission case must have a public route classification");
    assert.equal(fixture.kind, "admission");
    let client;
    try {
      const expected = fixture.boundary === "question"
        ? `request.${fixture.message}` : fixture.message;
      await assert.rejects(async () => {
        switch (fixture.boundary) {
          case "question":
            client = new sdk.ThinkThen({backend: "stub"});
            await streamFor(client, {question: fixture.input, records: []}).toArray();
            break;
          case "toolbox":
            // Saved JSON keeps evidence arrays invalid; typed questions accept paths.
            sdk.parseToolbox(JSON.stringify(fixture.input));
            break;
          case "settings":
            client = new sdk.ThinkThen(hostSettings(fixture.input));
            break;
          default: assert.fail(`unclassified admission boundary: ${fixture.boundary}`);
        }
      }, error => {
        assert.ok(error instanceof sdk.ThinkThenError);
        assert.equal(error.code, fixture.code);
        assert.equal(error.message, expected);
        assert.ok(!error.message.includes("secret-placeholder"));
        return true;
      });
    } finally { await client?.close(); }
  });
}

// Feature: canonical settings and saved execution options reach the public host API.
const optionsCases = JSON.parse(await readFile(new URL("../../../construction/options.json", import.meta.url), "utf8"));
for (const fixture of optionsCases) {
  test(`shared settings options: ${fixture.name}`, async () => {
    let client;
    try {
      const invoke = async () => {
        client = new sdk.ThinkThen(hostSettings(fixture.client));
        return streamFor(client, {...fixture.request, records: fixture.request.records ?? []}).toArray();
      };
      if (fixture.expected_error) {
        await assert.rejects(invoke, error => error instanceof sdk.ThinkThenError && error.code === fixture.expected_error);
      } else {
        const rows = await invoke();
        assert.deepEqual(hostValue(rows), hostValue(fixture.expected));
      }
    } finally { await client?.close(); }
  });
}

// Feature: the conformance boundary refuses undeclared fields on non-answer rows.
test("strict row comparison rejects a seeded extra answer", () => {
  const expected = {function: "decide", position: 1, status: "unsure", probability: 0.6};
  assert.throws(() => assert.deepEqual(normalize({...expected, answer: null}), expected), assert.AssertionError);
});

// Feature: shared row provenance matches the selected client before normalization.
test("strict row comparison rejects wrong backend and model", () => {
  const expected = {function: "decide", position: 1, status: "unsure", probability: 0.6};
  const row = {...expected, source: {backend: "openai", model: "fixture-model", questionIdentity: "fixture"}};
  assertRow(row, expected, "openai", "fixture-model");
  for (const source of [{...row.source, backend: "stub"}, {...row.source, model: "wrong-model"}]) {
    assert.throws(() => assertRow({...row, source}, expected, "openai", "fixture-model"), assert.AssertionError);
  }
});

// Feature: native error families select the installed public exception factory.
test("registry families select exact public error classes", async () => {
  const registry = JSON.parse(await readFile(new URL(manifest.registry, directory), "utf8"));
  const {engineError} = await import(new URL("./bridge.js", pathToFileURL(process.env.THINKTHEN_NODE_PACKAGE)));
  for (const entry of registry.codes) {
    const expected = entry.name === "local.cancelled" ? sdk.CancelledError
      : entry.family === "usage" ? sdk.UsageError
      : entry.family === "backend" ? sdk.BackendError : sdk.ThinkThenError;
    const error = engineError({code: entry.name, family: entry.family, message: "fixture error"});
    assert.equal(error.constructor, expected, entry.name);
    assert.equal(error.code, entry.name);
    assert.equal(error.message, "fixture error");
  }
});

// Feature: standalone single and streaming exports consume shared rows.
test("shared case through standalone exports", async () => {
  const fixture = fixtureJson(await readFile(new URL("decide-precision.json", directory), "utf8"));
  const server = await backend({replies: fixture.replies});
  const previous = process.env.THINKTHEN_BASE_URL;
  process.env.THINKTHEN_BASE_URL = server.url;
  try {
    const request = fixture.request;
    const single = await sdk.decide(request.question, request.records[0]);
    const rows = await sdk.decideStream(request.question, request.records).toArray();
    for (const row of [single, ...rows]) {
      assert.deepEqual(normalize(hostValue(row)), expectedRows(fixture)[0]);
    }
    assert.equal(await server.count(), fixture.requests * 2);
  } finally {
    if (previous === undefined) delete process.env.THINKTHEN_BASE_URL;
    else process.env.THINKTHEN_BASE_URL = previous;
    await server.close();
  }
});
