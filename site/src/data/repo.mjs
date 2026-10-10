// The public repository. It sits in its own module so that catalog.mjs and
// lib/settings-table.mjs can both import it without importing each other.
export const REPO = 'https://github.com/botassembly/thinkthen';

// Package guides own executable language examples and platform evidence.
export function packageGuide(slug) {
  if (slug === 'shell') return `${REPO}/blob/main/README.md`;
  const folder = ['java', 'kotlin', 'scala'].includes(slug) ? 'jvm'
    : ['pandas', 'polars'].includes(slug) ? 'python' : slug;
  const parent = ['duckdb', 'sqlite', 'postgresql'].includes(slug) ? 'databases' : 'libraries';
  return `${REPO}/blob/main/${parent}/${folder}/README.md`;
}
