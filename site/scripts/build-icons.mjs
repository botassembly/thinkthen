#!/usr/bin/env node
// Renders the raster icons from the dark ThinkThen mark with resvg. Run it by
// hand with `npm run icons` when the mark changes, and commit what it writes.
// The build never renders, and check-head.mjs checks each file's size.
//
// favicon.ico holds one PNG each at 16, 32 and 48 pixels. Every browser in
// use reads PNG entries in an ICO. The PNG icons fill the square with the
// mark's ground, because iOS and Android round the corners themselves and
// would show black in transparent ones.

import fs from 'node:fs';
import path from 'node:path';
import { Resvg } from '@resvg/resvg-js';

const PUBLIC = path.join(process.cwd(), 'public');
const mark = fs.readFileSync(path.join(PUBLIC, 'brand/thinkthen-mark-dark.svg'), 'utf8');
const square = mark.replace(/ rx="[^"]*"/, '');
if (square === mark) throw new Error('the dark mark has no rounded ground to square off');

function png(svg, px) {
  return new Resvg(svg, { fitTo: { mode: 'width', value: px } }).render().asPng();
}

// An ICO is a six-byte header, a sixteen-byte entry per image, then the images.
function ico(images) {
  const head = Buffer.alloc(6 + 16 * images.length);
  head.writeUInt16LE(0, 0);
  head.writeUInt16LE(1, 2);
  head.writeUInt16LE(images.length, 4);
  let offset = head.length;
  images.forEach(({ px, data }, i) => {
    const at = 6 + 16 * i;
    head.writeUInt8(px, at);
    head.writeUInt8(px, at + 1);
    head.writeUInt16LE(1, at + 4);
    head.writeUInt16LE(32, at + 6);
    head.writeUInt32LE(data.length, at + 8);
    head.writeUInt32LE(offset, at + 12);
    offset += data.length;
  });
  return Buffer.concat([head, ...images.map((i) => i.data)]);
}

const files = {
  'favicon.ico': ico([16, 32, 48].map((px) => ({ px, data: png(mark, px) }))),
  'apple-touch-icon.png': png(square, 180),
  'icon-192.png': png(square, 192),
  'icon-512.png': png(square, 512),
};
for (const [name, data] of Object.entries(files)) {
  fs.writeFileSync(path.join(PUBLIC, name), data);
  console.log(`wrote public/${name}, ${data.length} bytes`);
}
