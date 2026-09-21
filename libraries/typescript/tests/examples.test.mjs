// Every TypeScript function's example, run as one test.
//
// The file `examples.json` is keyed by function: the call and the answer
// the null backend gives. The site's function pages and surface pages draw
// their TypeScript tab from this file (site.md's one source), so an
// example nobody runs cannot reach the site.
//
// The call text is evaluated with `tt` in scope and awaited; the answer is
// compared to the file's JSON. Offline, no stub, no key.

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

import * as tt from '../index.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const file = JSON.parse(readFileSync(join(here, '..', 'examples.json'), 'utf8'));

test('every function example answers as the file says', async () => {
  const names = Object.keys(file.examples);
  assert.equal(names.length, 10, 'the ten functions are all here');
  for (const name of names) {
    const example = file.examples[name];
    const run = new Function('tt', `return (async () => (${example.ts}))();`);
    const value = await run(tt);
    assert.deepEqual(
      JSON.parse(JSON.stringify(value)),
      JSON.parse(example.expected),
      `${name}: ${example.ts}`,
    );
  }
});
