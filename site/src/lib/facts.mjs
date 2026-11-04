// One home per fact: src/data/site.json. Pages read facts through <Fact k="..."/>.
import site from '../data/site.json' with { type: 'json' };
import contract from '../../../schemas/contract.json' with { type: 'json' };

export function fact(key) {
  if (key === 'counts.errorCodes') return String(contract.codes.length);
  const v = key.split('.').reduce((o, k) => (o == null ? o : o[k]), site);
  if (v === undefined) throw new Error(`unknown fact: ${key}`);
  return String(v);
}

// A command a page shows on several lines, as one line for a table cell: an open bracket and a
// backslash-newline join the next line; any other line break separates commands with "; ".
export function oneLine(cmd) {
  return cmd.replace(/([([])\n\s*/g, '$1').replace(/\s*\\\n\s*/g, ' ').replace(/\n\s*/g, '; ');
}
