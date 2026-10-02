// Put an example into an article. The comment
//
//   <!-- example: functions/decide/3-one -->
//
// becomes the script examples/functions/decide/3-one.sh, then its output
// and its exit code. The comment
//
//   <!-- file: functions/question-file/files/refund.json -->
//
// becomes that file. The comment
//
//   <!-- install: shell -->
//
// becomes that binding's first install line from SURFACES, with a copy
// button, as InstallLine draws it. scripts/smoke.mjs runs every script, so
// an article shows only what a real run printed. The comment
//
//   <!-- video: Ut3LOjKNJaE | How Jev Turns AI Into Software | a16z -->
//
// becomes a small card that opens the video on YouTube. Its thumbnail is
// public/video/<id>.jpg, served from this site, so no reader loads an
// image from YouTube. scripts/emit-md.mjs writes the card as a plain link.

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { codeBlock, esc, exampleBlock } from './code.mjs';
import { SURFACES } from '../data/catalog.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..', 'examples');

const pub = path.resolve(root, '..', 'public');

// A card for one video: its thumbnail, its title, and its channel.
function videoCard(spec) {
  const [id, title, channel, extra] = spec.split('|').map((part) => part.trim());
  if (!/^[A-Za-z0-9_-]{11}$/.test(id ?? '') || !title || !channel || extra !== undefined) {
    throw new Error(`video: write <!-- video: ID | Title | Channel -->, not ${spec}`);
  }
  if (!fs.existsSync(path.join(pub, 'video', `${id}.jpg`))) throw new Error(`video: public/video/${id}.jpg is missing`);
  const watch = `https://www.youtube.com/watch?v=${id}`;
  return `<a class="video-card" href="${esc(watch)}"><img src="/video/${id}.jpg" alt="" width="160" height="90" loading="lazy"><span><b>${esc(title)}</b><span>${esc(channel)} · YouTube</span></span></a>`;
}

const read = (p) => fs.readFileSync(path.join(root, p), 'utf8').replace(/\n+$/, '');

// The same blocks the pages draw, from src/lib/code.mjs, so an article's
// code has colour and a copy button too.
function expand(kind, name) {
  if (kind === 'install') {
    const surface = SURFACES.find((s) => s.slug === name);
    if (!surface) throw new Error(`install: no binding named ${name}`);
    const line = surface.install[0][0];
    return [{ type: 'html', value: codeBlock({ code: line, lang: 'bash', caption: 'Install the command', prompt: true, label: 'Copy the install line', copyText: line }) }];
  }
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
        const m = child.type === 'html' && /^<!--\s*(example|file|install):\s*(\S+)\s*-->$/.exec(child.value.trim());
        if (m) return expand(m[1], m[2]);
        const v = child.type === 'html' && /^<!--\s*video:\s*(.+?)\s*-->$/.exec(child.value.trim());
        if (v) return [{ type: 'html', value: videoCard(v[1]) }];
        walk(child);
        return [child];
      });
    };
    walk(tree);
  };
}
