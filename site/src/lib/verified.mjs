// The dates generated pages carry, so no generator copies a hand-typed date.
import fs from 'node:fs';
import path from 'node:path';
import { EXAMPLES } from './examples.mjs';

export const today = () => new Date().toISOString().slice(0, 10);

// The day an example last ran and saved its output: examples/<id>/ran, written by npm run examples.
export function ranOn(id) {
  const f = path.join(EXAMPLES, id, 'ran');
  if (!fs.existsSync(f)) throw new Error(`example ${id}: no ran file; run npm run examples -- ${id}`);
  return fs.readFileSync(f, 'utf8').trim();
}

// Generators write `verified: VERIFIED` in front matter. stamp() fills in the date: the date the
// page already carries when nothing else changed, else today. same() compares two texts apart
// from that date, for --check.
export const VERIFIED = 'VERIFIED';
const blank = (t) => t.replace(/^verified: .*$/m, `verified: ${VERIFIED}`);
export const same = (a, b) => a !== null && b !== null && blank(a) === blank(b);
export const stamp = (text, old) => (same(text, old) ? old : text.replace(new RegExp(`^verified: ${VERIFIED}$`, 'm'), `verified: ${today()}`));
