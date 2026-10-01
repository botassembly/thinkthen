// Every code block on the site comes from here. Code.astro, Terminal.astro,
// InstallLine.astro and the article examples all call these functions, so a
// block looks and copies the same everywhere.
//
// Shiki colours the code once, at build time, with the one theme in
// code-theme.mjs. Output panes and plain-text files stay in ink.
// scripts/check-code.mjs reads the built pages and fails on a block that
// skipped this file.

import path from 'node:path';
import { createHighlighter } from 'shiki';
import { exitMark } from '../data/catalog.mjs';
import { CODE_THEME, JQ_GRAMMAR } from './code-theme.mjs';

const BY_EXT = {
  '.sh': 'bash', '.json': 'json', '.jsonl': 'jsonl', '.jq': 'jq', '.diff': 'diff',
  '.py': 'python', '.ts': 'typescript', '.rb': 'ruby', '.r': 'r', '.rs': 'rust',
  '.c': 'c', '.cpp': 'cpp', '.m': 'objective-c', '.cob': 'cobol',
  '.adb': 'ada', '.java': 'java', '.kt': 'kotlin', '.scala': 'scala',
  '.cs': 'csharp', '.csproj': 'xml', '.go': 'go', '.swift': 'swift',
  '.zig': 'zig', '.zon': 'zig', '.php': 'php', '.dart': 'dart', '.yaml': 'yaml', '.toml': 'toml', '.sql': 'sql', '.txt': 'text',
};

// One highlighter for the whole build. It loads every language the site
// uses, once.
const highlighter = await createHighlighter({
  themes: [CODE_THEME],
  langs: ['bash', 'json', 'jsonl', 'diff', 'python', 'typescript', 'ruby', 'r', 'rust', 'c', 'cpp', 'objective-c', 'cobol', 'ada', 'java', 'kotlin', 'scala', 'csharp', 'xml', 'go', 'swift', 'zig', 'php', 'dart', 'yaml', 'toml', 'sql', JQ_GRAMMAR],
});

export const esc = (s) => String(s)
  .replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;').replaceAll('"', '&quot;');

// The language of a file, from its name. A name the map does not hold
// fails the build.
export function langOf(name) {
  const lang = BY_EXT[path.extname(name).toLowerCase()];
  if (!lang) throw new Error(`code: no language for ${name}`);
  return lang;
}

// The lines inside a heredoc body are input. They are muted, as the reader
// skims them. The line that closes the heredoc is script.
function heredocLines(code) {
  const input = new Set();
  let until = null;
  code.split('\n').forEach((l, i) => {
    if (until) {
      if (l.trim() === until) until = null;
      else input.add(i + 1);
      return;
    }
    const doc = /<<-?\s*'?(\w+)'?/.exec(l);
    if (doc) until = doc[1];
  });
  return input;
}

// A coloured pre. data-copy holds the exact text the copy button copies.
// `prompt` shows a "$ " before the first line, outside the copied text.
// `copyText` is the text to copy when it is not the whole block, as for an
// install line with a note under it.
function highlight(code, lang, { prompt = false, copyText = code } = {}) {
  const input = lang === 'bash' ? heredocLines(code) : new Set();
  return highlighter.codeToHtml(code, {
    lang,
    theme: CODE_THEME.name,
    transformers: [{
      pre(node) {
        delete node.properties.tabindex;
        delete node.properties.style;
        node.properties.class = prompt ? 'prompt' : null;
        node.properties['data-lang'] = lang;
      },
      line(node, n) {
        if (input.has(n)) this.addClassToHast(node, 'in');
        if (prompt && n === 1) node.properties['data-copy-range'] = '';
      },
    }],
  }).replace('<pre ', `<pre data-copy="${esc(copyText)}" `);
}

// A pre in ink, for an output pane or a plain-text file. A plain-text
// file keeps its copy text.
function plain(text, lang, file = null) {
  const named = file ? ` data-copy="${esc(text)}" data-file="${esc(file)}"` : '';
  return `<pre data-lang="${lang}"${named}>${esc(text)}</pre>`;
}

const captionText = (caption) => (caption ?? '')
  .split('`').map((part, i) => (i % 2 ? `<code>${esc(part)}</code>` : esc(part))).join('');

const copyButton = (label) => `<button class="copy" type="button" aria-label="${esc(label)}">Copy</button>`;

// One block: a caption row and the code. `lang` is a language, "output",
// or "text". `file` names the file the code came from. Only a .txt file
// may show as text.
export function codeBlock({ code, lang = null, file = null, caption = null, copy = true, prompt = false, label = null, copyText = code }) {
  const kind = lang ?? (file ? langOf(file) : null);
  if (!kind) throw new Error(`code: a block needs a lang or a file: ${code.split('\n')[0]}`);
  if (kind === 'text' && !(file && file.endsWith('.txt'))) {
    throw new Error(`code: only a .txt file shows as plain text: ${file ?? code.split('\n')[0]}`);
  }
  const pre = kind === 'output' || kind === 'text' ? plain(code, kind, file) : highlight(code, kind, { prompt, copyText });
  const button = copy ? copyButton(label ?? (file ? `Copy ${file}` : 'Copy the code')) : '';
  return `<div class="block"><div class="caption"><span class="grow">${captionText(caption)}</span>${button}</div>${pre}</div>`;
}

// One example: the script in one block, then what it printed in its own
// block, closed by the exit code in its outcome colour. exitMark in
// catalog.mjs gives the word and the colour. Only a single answer gets one.
export function exampleBlock(run, caption = null) {
  const exit = run.exit ?? 0;
  const mark = exitMark(run.command, exit);
  const out = run.output ?? '';
  const exitSpan = `<span class="exit ${mark.cls}">exit ${exit}${mark.word ? `: ${esc(mark.word)}` : ''}</span>`;
  const output = out
    ? `<div class="caption"><span class="grow">Output</span></div>${plain(out, 'output')}<div class="caption">${exitSpan}</div>`
    : `<div class="caption solo"><span class="grow">No output</span>${exitSpan}</div>`;
  return `<div class="example"><div class="block"><div class="caption"><span class="grow">${captionText(caption)}</span>${copyButton('Copy the script')}</div>${highlight(run.command, 'bash')}</div><div class="block output">${output}</div></div>`;
}
