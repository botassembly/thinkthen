import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { ThinkThen } from 'thinkthen';
for (const fixture of JSON.parse(await readFile(process.argv[2], 'utf8'))) {
  const request = fixture.request;
  const actual = [];
  const client = new ThinkThen({ adapter: 'stub' });
  for await (const row of client[`${request.function}Stream`](request.question.value, request.records, request.options)) actual.push(row);
  const expected = request.function === 'filter'
    ? fixture.expected.flatMap((row, i) => row.answer === true ? [request.records[i]] : [])
    : fixture.expected;
  assert.deepEqual(actual, expected, request.function);
}
console.log('candidate TypeScript: ten shared stub cases pass');
