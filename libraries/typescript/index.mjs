// The ESM face of the same package: one wrapper, two doors. The CommonJS
// file loads through createRequire, because Deno's ESM loader does not
// synthesize a default export for a sibling CommonJS file the way Node
// does.

import { createRequire } from 'node:module';

const cjs = createRequire(import.meta.url)('./index.js');

export const ThinkThenError = cjs.ThinkThenError;
export const annotate = cjs.annotate;
export const choose = cjs.choose;
export const decide = cjs.decide;
export const decide_many = cjs.decide_many;
export const details = cjs.details;
export const filter = cjs.filter;
export const find = cjs.find;
export const probe = cjs.probe;
export const question = cjs.question;
export const rank = cjs.rank;
export const reset_usage = cjs.reset_usage;
export const score = cjs.score;
export const tag = cjs.tag;
export const usage = cjs.usage;
export default cjs;
