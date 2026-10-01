import { defineConfig } from 'astro/config';
import remarkExamples from './src/lib/remark-examples.mjs';

export default defineConfig({
  site: 'https://thinkthen.dev',
  output: 'static',
  // Each page carries its CSS in a style tag, so no stylesheet blocks the first paint.
  build: { format: 'directory', inlineStylesheets: 'always' },
  devToolbar: { enabled: false },
  // Pages that moved keep their old address.
  redirects: {
    '/surfaces': '/install/',
    '/backends': '/install/backends/',
    '/tutorial': '/learn/tutorial/',
    '/recipes': '/how-tos/bash/',
    // The shell recipes moved into the Bash techniques section.
    ...Object.fromEntries(['label-a-json-file', 'review-a-diff-by-what-it-does', 'lint-prose-for-hedging', 'fill-a-form-by-selection']
      .map((slug) => [`/how-tos/${slug}`, `/how-tos/bash/${slug}/`])),
    ...Object.fromEntries(['shell', 'python', 'polars', 'typescript', 'ruby', 'r', 'rust', 'c', 'duckdb', 'sqlite', 'postgresql']
      .map((slug) => [`/${slug}`, `/install/${slug}/`])),
    // The launch article replaced the first article on 2026-10-01.
    '/blog/code-that-understands': '/blog/introducing-thinkthen/',
    // The earlier preview kept the bench pages at /beatles-bench/.
    '/beatles-bench': '/learn/beatles-bench/',
    '/beatles-bench/the-data': '/learn/beatles-bench/',
    '/beatles-bench/run-it-for-free': '/learn/beatles-bench/',
    '/beatles-bench/every-language': '/install/',
    ...Object.fromEntries(['decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate', 'audit', 'diff', 'blind-spots', 'rad']
      .map((slug) => [`/beatles-bench/${slug}`, `/learn/beatles-bench/${slug}/`])),
    // The talk cut its what-jev-knows slide. The strings page compares Jev with search.
    '/beatles-bench/what-jev-knows': '/learn/beatles-bench/strings/',
    '/learn/beatles-bench/what-jev-knows': '/learn/beatles-bench/strings/',
    // Each function's options and more examples moved onto its own page.
    ...Object.fromEntries(['decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate', 'question-file']
      .map((slug) => [`/reference/functions/${slug}`, `/functions/${slug}/#reference`])),
  },
  // Articles place examples with <!-- example: --> and <!-- file: -->.
  // src/lib/code.mjs draws them. scripts/check-code.mjs refuses a raw fence.
  markdown: {
    remarkPlugins: [remarkExamples],
  },
});
