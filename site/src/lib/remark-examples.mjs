// Put an example into an article. The comment
//
//   <!-- example: functions/decide/3-one -->
//
// becomes the script examples/functions/decide/3-one.sh, then its output
// and its exit code. The comment
//
//   <!-- file: functions/question-file/files/refund.json -->
//
// becomes that file. scripts/smoke.mjs runs every script, so an article
// shows only what a real run printed.

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { exitMark } from '../data/catalog.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..', 'examples');
const LANG = { '.json': 'json', '.sh': 'bash', '.jq': 'jq' };

const read = (p) => fs.readFileSync(path.join(root, p), 'utf8').replace(/\n+$/, '');

function expand(kind, name) {
  if (kind === 'file') {
    return [{ type: 'code', lang: LANG[path.extname(name)] || 'text', value: read(name) }];
  }
  const script = read(`${name}.sh`);
  const nodes = [{ type: 'code', lang: 'bash', value: script }];
  const output = read(`${name}.out`);
  if (output) nodes.push({ type: 'code', lang: 'text', value: output });
  const exitFile = path.join(root, `${name}.exit`);
  const exit = fs.existsSync(exitFile) ? Number(fs.readFileSync(exitFile, 'utf8').trim()) : 0;
  const mark = exitMark(script, exit);
  const word = mark.word ? `: ${mark.word}` : '';
  nodes.push({ type: 'html', value: `<p class="exit-row"><span class="exit ${mark.cls}">exit ${exit}${word}</span></p>` });
  return nodes;
}

export default function remarkExamples() {
  return (tree) => {
    const walk = (node) => {
      if (!node.children) return;
      node.children = node.children.flatMap((child) => {
        const m = child.type === 'html' && /^<!--\s*(example|file):\s*(\S+)\s*-->$/.exec(child.value.trim());
        if (m) return expand(m[1], m[2]);
        walk(child);
        return [child];
      });
    };
    walk(tree);
  };
}
