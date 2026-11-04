// The build checks the current docs decision, not the retired L1–L9 rubric.
import words from '../data/words.json' with { type: 'json' };

export function bannedWords(text, route) {
  const found = [];
  for (const rule of words.banned) {
    if (rule.exceptRoutes?.includes(route) || (rule.routes && !new RegExp(rule.routes).test(route))) continue;
    for (const match of text.matchAll(new RegExp(`\\b(?:${rule.word})\\b`, 'gi'))) {
      const before = text.slice(0, match.index);
      if (rule.unlessPrecededBy && before.toLowerCase().endsWith(rule.unlessPrecededBy.toLowerCase())) continue;
      if (rule.csvContext) {
        // Fail closed at Markdown block starts; CSV context never leaks
        // through another heading, a nested block, or a real blank line.
        const blocks = [...text.matchAll(/^[ \t]*\r?$|^ {0,3}#{1,6}(?:[ \t]+[^\r\n]*)?\r?$|^[^\r\n]+\r?\n {0,3}(?:=+|-+)[ \t]*\r?$|^ {0,3}(?:>[ \t]*|[-+*][ \t]+|\d+[.)][ \t]+)[^\r\n]*\r?$/gm)];
        const active = blocks.filter((item) => item.index <= match.index && item[0].trim()).at(-1);
        const nested = active && /^ {0,3}(?:>|[-+*][ \t]+|\d+[.)][ \t]+)/.test(active[0]);
        const heading = active && !nested ? active[0] : '';
        const previous = blocks.filter((item) => item.index < match.index).at(-1);
        const next = blocks.find((item) => item.index > match.index);
        const paragraph = active && match.index < active.index + active[0].length
          ? active[0] : text.slice(previous ? previous.index + previous[0].length : 0, next?.index ?? text.length);
        if (/\bCSV\b/i.test(heading) || /\bCSV\b/i.test(paragraph)) continue;
      }
      if (rule.near) {
        const sentence = text.slice(Math.max(before.lastIndexOf('.'), before.lastIndexOf('\n')) + 1).split(/[.\n]/)[0];
        if (!new RegExp(`\\b${rule.near}\\b`, 'i').test(sentence)) continue;
      }
      found.push({ word: match[0], offset: match.index, use: rule.use });
    }
  }
  return found;
}

// Fences close only with the opener's character and at least its length.
// This is also the boundary used for commands, banned prose and twin literals.
export function markdownSections(text) {
  const sections = [];
  const open = /^[ \t]*(`{3,}|~{3,})([^\n]*)\n/gm;
  let end = 0;
  for (let match; (match = open.exec(text));) {
    if (match.index > end) sections.push({ kind: 'prose', text: text.slice(end, match.index) });
    const close = new RegExp('^[ \\t]*' + match[1][0] + '{' + match[1].length + ',}[ \\t]*\\r?$', 'gm');
    close.lastIndex = open.lastIndex;
    const closing = close.exec(text);
    sections.push({ kind: 'block', language: match[2].trim().split(/\s/)[0], text: text.slice(open.lastIndex, closing?.index ?? text.length) });
    end = closing ? close.lastIndex : text.length;
    open.lastIndex = end;
  }
  if (end < text.length) sections.push({ kind: 'prose', text: text.slice(end) });
  return sections;
}

export function commandLines(markdown) {
  return markdownSections(markdown).filter((part) => part.kind === 'block' && ['sh', 'bash', 'shell', 'powershell', 'ps1'].includes(part.language))
    .flatMap((part) => part.text.split('\n').filter((line) => [...line].length > 72).map((line) => ({ line, length: [...line].length })));
}

export function proseOnly(markdown) {
  return markdownSections(markdown).filter(({ kind }) => kind === 'prose').map(({ text }) => text).join('\n')
    .replace(/(`+)[\s\S]*?\1/g, '')
    .replace(/\]\([^)]*\)/g, ']')
    .replace(/^import .*$/gm, '')
    .replace(/<[^>]*>/g, '');
}

export function markdownShape(markdown) {
  const literals = [];
  const keep = (kind, text) => {
    // A fence's final newline separates its content from the closing marker.
    // Other spaces, tabs, CRs and blank lines are literal. Captured DOM/copy
    // has a separate byte-for-byte comparison, including the terminal newline.
    literals.push({ kind, text: kind === 'block' ? text.replace(/\n$/, '') : text });
    return ` \u0000${literals.length - 1}\u0000 `;
  };
  const text = markdownSections(markdown.replace(/^---\n[\s\S]*?\n---\n*/, ''))
    .map((part) => part.kind === 'block' ? keep('block', part.text)
      : part.text.replace(/(`+)([\s\S]*?)\1/g, (_, fence, code) => keep('inline', /^ .* $/.test(code) && code.trim() ? code.slice(1, -1) : code)))
    .join('')
    .replace(/<!--[^]*?-->/g, '').replace(/<[^>]*>/g, '')
    .replace(/!?\[([^\]]*)\]\([^)]*\)/g, '$1')
    .replace(/(\*\*|__)(.*?)\1/g, '$2')
    .replace(/^\s{0,3}(?:#{1,6}\s+|[-*+]\s+|\d+\.\s+|>\s?)/gm, '')
    .replace(/^\s*\|?(?:\s*:?-+:?\s*\|)+\s*$/gm, '')
    .replace(/\\\|/g, '|').replace(/\|/g, ' ');
  return { prose: text.replace(/\s+/g, ' ').trim(), literals };
}
