import test from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
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
      const client = new sdk.ThinkThen({...hostSettings(settings), url: server.url, apiKey: "test-only"});
      try {
        const name = fixture.request.question.function;
        if (fixture.expected.error) {
          await assert.rejects(async () => streamFor(client, fixture.request).toArray(),
            error => error instanceof sdk.ThinkThenError && error.code === fixture.expected.error);
          assert.equal(await server.count(), fixture.requests);
          return;
        }
        const items = await streamFor(client, fixture.request).toArray();
        const rows = name === "filter" ? items.map(item => item.row) : items;
        const expected = expectedRows(fixture);
        assert.equal(rows.length, expected.length);
        for (const [index, row] of rows.entries()) {
          assert.equal(row.source.backend, selected);
          assert.equal(typeof row.source.questionIdentity, "string");
          assert.equal(typeof row.source.model, "string");
          assert.equal(row.function, name);
          assert.deepEqual(project(hostValue(row), expected[index]), expected[index]);
          if (name === "filter") {
            assert.equal(items[index].item, fixture.request.records[row.position - 1]);
            assert.equal(items[index].kept, row.status === "answered" && row.answer === true);
          }
        }
        assert.equal(await server.count(), fixture.requests);
      } finally { await client.close(); await server.close(); }
    });
  }
}
function project(value, expected) {
  if (Array.isArray(expected)) return value.map((child, index) => project(child, expected[index]));
  if (expected === null || typeof expected !== "object" || value === null || typeof value !== "object") return value;
  return Object.fromEntries(Object.entries(expected).map(([key, child]) => [key, project(value[key], child)]));
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
