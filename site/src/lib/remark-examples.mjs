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
import { codeBlock, exampleBlock } from './code.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..', 'examples');

const read = (p) => fs.readFileSync(path.join(root, p), 'utf8').replace(/\n+$/, '');

// The same blocks the pages draw, from src/lib/code.mjs, so an article's
// code has colour and a copy button too.
function expand(kind, name) {
  if (kind === 'file') {
    const file = path.basename(name);
    return [{ type: 'html', value: codeBlock({ code: read(name), file, caption: file }) }];
  }
  const exitFile = path.join(root, `${name}.exit`);
  const exit = fs.existsSync(exitFile) ? Number(fs.readFileSync(exitFile, 'utf8').trim()) : 0;
  const run = { command: read(`${name}.sh`), output: read(`${name}.out`), exit };
  return [{ type: 'html', value: exampleBlock(run) }];
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
