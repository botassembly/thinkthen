// Each Beatles Bench slide at 800 pixels wide. sharp shrinks the 1600-pixel
// slide in public/ when the site builds, so no copy sits in the repository.
import fs from 'node:fs';
import path from 'node:path';
import sharp from 'sharp';

const DIR = path.join(process.cwd(), 'public/learn/beatles-bench');

export function getStaticPaths() {
  return fs.readdirSync(DIR).filter((f) => f.endsWith('.webp'))
    .map((f) => ({ params: { name: f.slice(0, -'.webp'.length) } }));
}

export async function GET({ params }) {
  const body = await sharp(path.join(DIR, `${params.name}.webp`)).resize({ width: 800 }).webp({ quality: 85 }).toBuffer();
  return new Response(body, { headers: { 'content-type': 'image/webp' } });
}
