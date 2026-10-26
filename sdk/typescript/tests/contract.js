import assert from "node:assert/strict";

// Test-only projection of the shared wire contract into its TypeScript names.
export function hostValue(value) {
  if (Array.isArray(value)) return value.map(hostValue);
  if (value === null || typeof value !== "object") return value;
  const fields = Object.fromEntries(Object.entries(value).filter(([key]) => key !== "message" || typeof value.code !== "string").map(([key, child]) => [key, hostValue(child)]));
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
  if (typeof fields.relation === "string" && typeof fields.source === "object" && typeof fields.target === "object" && "either" in fields) {
    fields.symmetric = fields.either; delete fields.either;
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
  if (name === "annotate") return client.annotateStream(Object.fromEntries(Object.entries(fields.questions).map(([key, value]) => [key, hostQuestion(value)])), request.records, { ...(fields.evidence === undefined ? {} : {evidence: fields.evidence}), ...(request.name === undefined ? {} : {name: request.name}), signal });
  return client[`${name}Stream`](hostQuestion(question), request.records, { ...(request.name === undefined ? {} : {name: request.name}), signal });
}

export function expectedRows(fixture) {
  const expected = structuredClone(fixture.expected);
  if (!Array.isArray(expected)) return expected;
  for (const row of expected) {
    const record = fixture.request.records[row.position - 1];
    const question = fixture.request.question;
    if (question.function === "annotate" && row.status === "answered") {
      for (const [name, child] of Object.entries(row.answer)) {
        const pointer = question.questions[name].evidence ?? question.evidence;
        offsets(child.answer, evidence(record, pointer));
        offsets(child.detail, evidence(record, pointer));
      }
    } else {
      offsets(row.answer, evidence(record, question.evidence));
      offsets(row.detail, evidence(record, question.evidence));
    }
  }
  return hostValue(expected);
}

function offsets(value, text) {
  if (!value || typeof value !== "object") return;
  if ("start" in value && "end" in value) {
    const points = Array.from(text);
    value.start = points.slice(0, value.start).join("").length;
    value.end = points.slice(0, value.end).join("").length;
  }
  for (const child of Object.values(value)) offsets(child, text);
}
function evidence(record, pointer) {
  if (typeof record === "string") return record;
  let value = record;
  if (pointer) for (const part of pointer.slice(1).split("/")) value = value?.[part.replaceAll("~1", "/").replaceAll("~0", "~")];
  if (!pointer && value && typeof value === "object" && !Array.isArray(value)) { const {id, ...fields} = value; value = fields; }
  return typeof value === "string" ? value : JSON.stringify(value) ?? "";
}

function hostQuestion(value) {
  const question = hostValue(value);
  if (question.rules) question.rules = question.rules.map(({relation, reads, either, ...rule}) => ({...rule, name: relation, ...(reads === undefined ? {} : {phrase: reads}), ...(either === undefined ? {} : {symmetric: either})}));
  return question;
}
