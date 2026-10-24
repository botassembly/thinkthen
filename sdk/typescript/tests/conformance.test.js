import test from "node:test";
import assert from "node:assert/strict";
import { readdir, readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { sdk, backend } from "./support.js";

const directory = new URL("../../../conformance/", import.meta.url);
for (const file of (await readdir(directory)).filter(name => name.endsWith(".json")).sort()) {
  test(`shared contract: ${file}`, async () => {
    const path = new URL(file, directory);
    const fixture = JSON.parse(await readFile(path, "utf8"));
    const server = await backend(fileURLToPath(path));
    try {
      const request = fixture.request;
      const settings = fixture.client ?? {};
      const client = new sdk.ThinkThen({ url: server.url,
        ...(settings.batch === undefined ? {} : { batch: Number(settings.batch) }),
        ...(settings.model === undefined ? {} : { model: settings.model }),
      });
      const controller = new AbortController();
      if (fixture.cancelled) controller.abort();
      const options = { signal: controller.signal,
        ...(request.options ?? {}),
      };
      const records = request.records;
      const functionName = request.function;
      const question = request.question.value;
      const delivered = [];
      let actual;
      try {
        actual = delivered;
        for await (const row of client[`${functionName}Stream`](question, records, options)) actual.push(row);
      } catch (error) {
        if (controller.signal.aborted && error === controller.signal.reason) actual = { error: "local.cancelled" };
        else actual = { error: error.code };
      }
      let expected = structuredClone(fixture.expected);
      if (Array.isArray(expected) && functionName !== "filter") {
        for (const [index, row] of expected.entries()) {
          if (functionName === "annotate") {
            for (const [name, outcome] of Object.entries(row.answer ?? {})) {
              projectOffsets(outcome, evidence(records[index], question.questions?.[name]?.on ?? options.on));
            }
          } else {
            projectOffsets(row.answer, evidence(records[index], options.on));
            projectOffsets(row.detail, evidence(records[index], options.on));
          }
        }
      }
      if (functionName === "filter") {
        expected = fixture.expected.flatMap((row, index) => row.status === "answered" && row.answer === true ? [records[index]] : []);
      } else {
        for (const row of delivered) {
          if (row.error) delete row.error.message;
          if (functionName === "annotate") {
            for (const entry of Object.values(row.answer ?? {})) if (entry.error) delete entry.error.message;
          }
        }
      }
      if (fixture.expected_rows) assert.deepEqual(delivered, fixture.expected_rows);
      assert.deepEqual(actual, expected);
      assert.equal(await server.count(), fixture.requests);
    } catch (error) {
      if (fixture.expected.error && !fixture.expected_rows) {
        assert.equal(error.code, fixture.expected.error);
        assert.equal(await server.count(), fixture.requests);
      } else throw error;
    } finally { await server.close(); }
  });
}

// Shared fixtures use code points; this surface exposes UTF-16 in answers and detail.
function projectOffsets(answer, text) {
  if (Array.isArray(answer)) { for (const item of answer) projectOffsets(item, text); }
  else if (answer && typeof answer === "object") {
    if ("start" in answer && "end" in answer) {
      const points = Array.from(text);
      answer.start = points.slice(0, answer.start).join("").length;
      answer.end = points.slice(0, answer.end).join("").length;
    }
    for (const value of Object.values(answer)) projectOffsets(value, text);
  }
}

function evidence(record, pointer) {
  if (typeof record === "string") return record;
  let value = record;
  if (pointer) {
    for (const part of pointer.slice(1).split("/")) value = value?.[part.replaceAll("~1", "/").replaceAll("~0", "~")];
  }
  return typeof value === "string" ? value : JSON.stringify(value);
}
