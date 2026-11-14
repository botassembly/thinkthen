import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { Client, FailedError } from 'thinkthen';

for (const fixture of JSON.parse(await readFile(process.argv[2], 'utf8'))) {
  const { question, records, name: executionName } = fixture.request;
  const { function: name, questions, field } = question;
  const hostQuestion = hostValue(question);
  const value = name === 'annotate' ? hostQuestion.questions : hostQuestion;
  const options = { ...(name === 'annotate' && field !== undefined ? {field} : {}),
    ...(executionName === undefined ? {} : {name: executionName}) };
  const client = new Client(fixture.client);
  let actual;
  try {
    if (name === 'filter') {
      const kept = [];
      let failures = [];
      try { for await (const record of client.filter(value, records, options)) kept.push(record); }
      catch (error) { if (!(error instanceof FailedError)) throw error; failures = error.rows; }
      assert.deepEqual(kept, fixture.expected.filter(row => row.status === 'answered' && row.answer).map(row => records[row.position - 1]));
      assert.deepEqual(failures.map(row => row.position), fixture.expected.filter(row => row.status === 'failed').map(row => row.position));
      continue;
    }
    const items = await (name === 'recognize' || name === 'relate'
      ? client[`${name}Stream`](records, {...hostQuestion, ...options})
      : client[name === 'rank' ? 'rank' : `${name}Stream`](value, records, options)).toArray();
    for (const row of items) assert.equal(row.item, records[row.position - 1]);
    actual = JSON.parse(JSON.stringify(items));
  } finally { await client.close(); }
  const expected = structuredClone(fixture.expected);
  assert.deepEqual(actual, expected, name);
}
console.log('candidate TypeScript: ten shared stub cases pass');

function hostValue(value) {
  if (Array.isArray(value)) return value.map(hostValue);
  if (value === null || typeof value !== 'object') return value;
  const result = Object.fromEntries(Object.entries(value).map(([key, child]) => [key, hostValue(child)]));
  if (typeof result.function === 'string') {
    if (result.source && 'question_identity' in result.source) {
      const {question_identity,...source} = result.source;
      result.source = {...source,questionIdentity:question_identity};
    }
    if (result.error && 'http_status' in result.error) {
      const {http_status,...error} = result.error;
      result.error = {...error,httpStatus:http_status};
    }
  }
  return result;
}
