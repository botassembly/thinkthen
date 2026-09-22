// Read the pulled examples. The build fails when a function-and-surface cell
// is missing, when a cell carries neither an example nor a status, or when a
// recorded run has no see sentence telling the reader what to look for.

import { FUNCTIONS, SURFACES } from './catalog.mjs';

const files = import.meta.glob('./examples/*.json', { eager: true, import: 'default' });

function load(name) {
  const found = files[`./examples/${name}`];
  if (!found) throw new Error(`examples: ${name} is missing. Run npm run pull.`);
  return found;
}

const STATUSES = new Set(['run', 'drawn', 'planned']);
const cells = {};
for (const fn of FUNCTIONS) {
  for (const surface of SURFACES) {
    const key = `${fn.name}|${surface.slug}`;
    const cell = load(`${fn.name}__${surface.slug}.json`);
    if (!STATUSES.has(cell.status)) throw new Error(`examples: ${key} has no status`);
    if (cell.status === 'run' && !cell.runs?.length) {
      throw new Error(`examples: ${key} is marked run and carries no example`);
    }
    if (cell.status === 'run') {
      for (const run of cell.runs) {
        if (typeof run.see !== 'string' || !run.see.trim()) {
          throw new Error(`examples: ${fn.name}__${surface.slug}.json run ${run.name} has no see sentence`);
        }
      }
    }
    if (cell.status === 'drawn' && !cell.code) {
      throw new Error(`examples: ${key} is marked drawn and carries no code`);
    }
    cells[key] = cell;
  }
}

export const CELLS = cells;
export const SURFACE_SAMPLES = load('_surfaces.json');
export const HOWTO_RUNS = load('_howtos.json');
export const RECIPE_RUNS = load('_recipes.json');
export const PAGE_RUNS = load('_pages.json');
export const INDEX = load('_index.json');

export function cell(fn, surface) {
  const found = CELLS[`${fn}|${surface}`];
  if (!found) throw new Error(`examples: no cell for ${fn} on ${surface}`);
  return found;
}
