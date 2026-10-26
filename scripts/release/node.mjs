import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { ThinkThen } from 'thinkthen';

for (const fixture of JSON.parse(await readFile(process.argv[2], 'utf8'))) {
  const { question, records, name: executionName } = fixture.request;
  const { function: name, questions, evidence } = question;
  const hostQuestion = hostValue(question);
  const value = name === 'annotate' ? hostQuestion.questions : hostQuestion;
  const options = { ...(name === 'annotate' && evidence !== undefined ? {evidence} : {}),
    ...(executionName === undefined ? {} : {name: executionName}) };
  const client = new ThinkThen(fixture.client);
  let actual;
  try {
    const items = await client[`${name}Stream`](value, records, options).toArray();
    actual = name === 'filter' ? items.map(item => item.row) : items;
    if (name === 'filter') for (const item of items) {
      assert.equal(item.item, records[item.row.position - 1]);
      assert.equal(item.kept, item.row.status === 'answered' && item.row.answer === true);
    }
  } finally { await client.close(); }
  const expected = structuredClone(fixture.expected);
  for (const row of expected) {
    const record = records[row.position - 1];
    if (name === 'annotate' && row.status === 'answered') {
      for (const [entry, outcome] of Object.entries(row.answer)) {
        if (['find', 'recognize', 'relate'].includes(outcome.function)) {
          projectOffsets(outcome, selectedText(record, questions[entry].evidence ?? evidence));
        }
      }
    } else if (['find', 'recognize', 'relate'].includes(name)) {
      projectOffsets(row.answer, selectedText(record, evidence));
      projectOffsets(row.detail, selectedText(record, evidence));
    }
  }
  assert.deepEqual(actual, hostValue(expected), name);
}
console.log('candidate TypeScript: ten shared stub cases pass');

// Shared fixtures count code points; the installed Node API uses String.slice units.
function projectOffsets(value, text) {
  if (value === null || typeof value !== 'object') return;
  if ('start' in value && 'end' in value) {
    const points = Array.from(text);
    value.start = points.slice(0, value.start).join('').length;
    value.end = points.slice(0, value.end).join('').length;
  }
  for (const child of Object.values(value)) projectOffsets(child, text);
}

function selectedText(record, pointer) {
  if (typeof record === 'string') return record;
  let value = record;
  if (pointer) {
    for (const segment of pointer.slice(1).split('/')) value = value?.[segment.replaceAll('~1', '/').replaceAll('~0', '~')];
  }
  return typeof value === 'string' ? value : JSON.stringify(value) ?? '';
}

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
    if (result.function === 'relate' && Array.isArray(result.rules)) result.rules = result.rules.map(({relation,reads,either,...rule}) => ({...rule,name:relation,...(reads === undefined ? {} : {phrase:reads}),...(either === undefined ? {} : {symmetric:either})}));
  }
  if (typeof result.relation === 'string' && typeof result.source === 'object' && typeof result.target === 'object' && 'either' in result) {
    result.symmetric = result.either; delete result.either;
  }
  return result;
}
