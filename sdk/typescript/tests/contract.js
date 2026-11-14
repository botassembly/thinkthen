import assert from "node:assert/strict";
import {fixtureJson} from "./canonical.js";

// Test-only projection of the shared wire contract into its TypeScript names.
export function hostValue(value) {
  if (value && typeof value.toJSON === "function") value = fixtureJson(JSON.stringify(value));
  if (Array.isArray(value)) return value.map(hostValue);
  if (value === null || typeof value !== "object") return value;
  const fields = Object.fromEntries(Object.entries(value).filter(([key]) => (key !== "message" || typeof value.code !== "string") && (key !== "item" || value.function !== "rank")).map(([key, child]) => [key, hostValue(child)]));
  if (typeof fields.function === "string") {
    if (fields.source && "question_identity" in fields.source) {
      const {question_identity,...source} = fields.source;
      fields.source = {...source,questionIdentity:question_identity};
    }
    if (fields.error && "http_status" in fields.error) {
      const {http_status,...error} = fields.error;
      fields.error = {...error,httpStatus:http_status};
    }
  }
  return fields;
}

export function questionOf(request) {
  assert.equal(typeof request.question?.function, "string", "shared fixtures must use regenerated flat questions");
  return request.question;
}

export function streamFor(client, request, signal) {
  const question = questionOf(request);
  const { function: name, ...fields } = question;
  if (name === "annotate") return client.annotateStream(Object.fromEntries(Object.entries(fields.questions).map(([key, value]) => [key, hostQuestion(value)])), request.records, { ...(fields.field === undefined ? {} : {field: fields.field}), ...(request.name === undefined ? {} : {name: request.name}), signal });
  const options = {...(request.name === undefined ? {} : {name: request.name}), signal};
  if (name === "recognize" || name === "relate") return client[`${name}Stream`](request.records, {...fields, ...options});
  return client[name === "filter" || name === "rank" ? name : `${name}Stream`](hostQuestion(question), request.records, options);
}

export function expectedRows(fixture) { return hostValue(fixture.expected); }

function hostQuestion(value) { return value; }

export async function assertKept(client, fixture, signal) {
  const received = [];
  let failed = [];
  try {
    for await (const record of streamFor(client, fixture.request, signal)) received.push(record);
  } catch (error) {
    if (!Array.isArray(error.rows)) throw error;
    failed = error.rows;
  }
  const expected = fixture.expected;
  assert.deepEqual(received, expected.filter(row => row.status === "answered" && row.answer)
    .map(row => fixture.request.records[row.position - 1]));
  assert.deepEqual(failed.map(row => ({position:row.position, code:row.error.code})),
    expected.filter(row => row.status === "failed").map(row => ({position:row.position, code:row.error.code})));
  for (const row of failed) assert.equal(row.item, fixture.request.records[row.position - 1]);
}
