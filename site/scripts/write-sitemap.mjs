#!/usr/bin/env node
// Writes dist/sitemap.xml from the built pages. Redirect stubs, the 404 page
// and the search page stay out. robots.txt names the sitemap only once
// NOINDEX is false.

import fs from 'node:fs';
import path from 'node:path';
import { listedUrls } from '../src/lib/listed-pages.mjs';

const DIST = path.join(process.cwd(), 'dist');
const urls = listedUrls(DIST);
const xml = [
  '<?xml version="1.0" encoding="UTF-8"?>',
  '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">',
  ...urls.map((u) => `  <url><loc>${u}</loc></url>`),
  '</urlset>',
  '',
].join('\n');
fs.writeFileSync(path.join(DIST, 'sitemap.xml'), xml);
console.log(`wrote sitemap.xml with ${urls.length} pages`);
