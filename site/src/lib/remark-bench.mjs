// Draws the Beatles Bench pages the way the site draws a run.
//
// It touches only the pages under src/pages/beatles-bench/. The bench wrote
// them for GitHub, so this plugin:
//   - sets each command beside the output the bench tests check under it,
//   - keeps output values in the ink colour and colours only the marks: green
//     right, amber not sure, red wrong,
//   - folds "Everything in the folder" and "The slide" shut, because the slide
//     already sits at the top of the page.

const text = (s) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
const esc = (s) => text(s).replace(/"/g, '&quot;');

// The words a page marks an answer with. Only these carry colour.
const MARKS = [
  [/("(?:mark|effect)":)("right"|"gained"|"resolved")/g, 'ok'],
  [/("(?:mark|effect)":)("wrong"|"lost")/g, 'bad'],
  [/("(?:mark)":)("not sure")/g, 'unsure'],
  [/("(?:value|jev|kept)":)(null)/g, 'unsure'],
  [/(: )(gained|resolved)$/gm, 'ok'],
  [/(: )(lost)$/gm, 'bad'],
];

function output(raw) {
  let html = text(raw);
  for (const [re, cls] of MARKS) html = html.replace(re, (_, lead, word) => `${lead}<span class="mark ${cls}">${word}</span>`);
  return html;
}

// A line starts a new command unless the line before it runs on, or it is
// indented, or it closes a loop.
function command(cmd) {
  const lines = cmd.split('\n');
  return lines.map((l, i) => {
    const runsOn = i > 0 && (/[\\|]$/.test(lines[i - 1]) || /^\s/.test(l) || /^(done|fi|esac)\b/.test(l) || /^'/.test(l));
    return `<span class="p">${runsOn ? '  ' : '$ '}</span>${text(l)}`;
  }).join('\n');
}

function block(cmd, out, caption) {
  const parts = ['<div class="block bench-run">'];
  if (cmd !== null) {
    parts.push(`<div class="caption"><span class="grow">${caption}</span><button class="copy" type="button">Copy</button></div>`);
    parts.push(`<pre class="cmd" data-copy="${esc(cmd)}">${command(cmd)}</pre>`);
  }
  if (out !== null) parts.push(`<pre class="out">${output(out)}</pre>`);
  parts.push('</div>');
  return { type: 'html', value: parts.join('') };
}

const textOf = (node) => (node.value ?? (node.children || []).map(textOf).join(''));
const isOutput = (n) => n && n.type === 'code' && (n.lang === 'json' || n.lang === 'text');

export default function remarkBench() {
  return (tree, file) => {
    const where = (file.path || file.history?.[0] || '').split('\\').join('/');
    if (!where.includes('/src/pages/beatles-bench/')) return;
    const out = [];
    const kids = tree.children;
    let slideOpen = false;
    for (let i = 0; i < kids.length; i += 1) {
      const node = kids[i];
      if (node.type === 'heading' && node.depth <= 2 && slideOpen) {
        out.push({ type: 'html', value: '</details>' });
        slideOpen = false;
      }
      if (node.type === 'heading' && node.depth === 2 && textOf(node) === 'The slide') {
        out.push({ type: 'html', value: '<details class="more"><summary>About the slide</summary>' });
        slideOpen = true;
        continue;
      }
      if (node.type === 'paragraph' && textOf(node) === 'Everything in the folder:' && kids[i + 1]?.type === 'list') {
        out.push({ type: 'html', value: '<details class="more"><summary>Everything in the folder</summary>' });
        out.push(kids[i + 1]);
        out.push({ type: 'html', value: '</details>' });
        i += 1;
        continue;
      }
      if (node.type === 'code' && node.lang === 'sh') {
        if (isOutput(kids[i + 1])) {
          out.push(block(node.value, kids[i + 1].value, 'The bench tests check this output.'));
          i += 1;
        } else {
          out.push(block(node.value, null, 'No saved output.'));
        }
        continue;
      }
      if (isOutput(node)) {
        out.push(block(null, node.value, ''));
        continue;
      }
      out.push(node);
    }
    if (slideOpen) out.push({ type: 'html', value: '</details>' });
    tree.children = out;
  };
}
