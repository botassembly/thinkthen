// Links the first use of each glossary term (src/data/glossary.json) in a generated page body
// to the page that defines it, so a reader who lands on the generated page can look the term up.
// Only plain prose is touched: headings, code blocks, inline code, existing links, import lines,
// MDX comments and component tags stay as they are. The term's own page is left alone.
import glossary from '../data/glossary.json' with { type: 'json' };

const terms = Object.entries(glossary.terms).map(([term, t]) => ({
  term,
  page: [t.page].flat()[0],
  pages: [t.page].flat().map((p) => p.replace(/#.*$/, '')),
  re: new RegExp(t.pattern ?? `\\b${term.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}s?\\b`, 'i'),
}));

// Inline code and Markdown links, which are never rewritten.
const PROTECTED = /(`[^`]*`|\[[^\]]*\]\([^)]*\))/;

export function linkTerms(body, route) {
  const lines = body.split('\n');
  const done = new Set();
  let fence = false;
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    if (/^\s*(```|~~~)/.test(line)) { fence = !fence; continue; }
    if (fence || /^\s*(#|<|import |\{\/\*|---)/.test(line) || !line.trim()) continue;
    for (const t of terms) {
      if (done.has(t.term)) continue;
      if (route && t.pages.includes(route)) { done.add(t.term); continue; }
      const parts = lines[i].split(PROTECTED);
      let hit = false;
      for (let k = 0; k < parts.length && !hit; k++) {
        if (k % 2) {
          // Inline code that is the whole term counts as a use; link it as it stands.
          const code = parts[k].match(/^`([^`]*)`$/);
          if (code && new RegExp(`^(?:${t.re.source})$`, 'i').test(code[1])) { parts[k] = `[${parts[k]}](${t.page})`; hit = true; }
          continue;
        }
        const m = parts[k].match(t.re);
        if (!m) continue;
        parts[k] = `${parts[k].slice(0, m.index)}[${m[0]}](${t.page})${parts[k].slice(m.index + m[0].length)}`;
        hit = true;
      }
      if (hit) { lines[i] = parts.join(''); done.add(t.term); }
    }
  }
  return lines.join('\n');
}
