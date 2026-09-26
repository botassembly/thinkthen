import { defineConfig } from 'astro/config';
import remarkExamples from './src/lib/remark-examples.mjs';

export default defineConfig({
  site: 'https://thinkthen.dev',
  output: 'static',
  build: { format: 'directory' },
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
    // The earlier preview kept the bench pages at /beatles-bench/.
    '/beatles-bench': '/learn/beatles-bench/',
    '/beatles-bench/the-data': '/learn/beatles-bench/',
    '/beatles-bench/run-it-for-free': '/learn/beatles-bench/',
    '/beatles-bench/every-language': '/install/',
    ...Object.fromEntries(['decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate', 'audit', 'diff', 'what-jev-knows', 'blind-spots', 'rad']
      .map((slug) => [`/beatles-bench/${slug}`, `/learn/beatles-bench/${slug}/`])),
  },
  // The article's code blocks follow the page theme. Shiki writes both colours
  // on every token and site.css picks the dark one under a dark page, so a
  // light page never carries a dark slab. `wrap` keeps a long line inside the
  // pane.
  markdown: {
    remarkPlugins: [remarkExamples],
    shikiConfig: {
      themes: { light: 'github-light', dark: 'github-dark' },
      defaultColor: 'light',
      wrap: true,
    },
  },
});
