import { defineConfig } from 'astro/config';
import { RECIPE_PAGES } from './src/data/catalog.mjs';
import { recipeSelection } from './src/data/recipes.mjs';
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
    ...(recipeSelection(RECIPE_PAGES, process.env.THINKTHEN_DRAFTS === '1' || process.argv.includes('dev')).index ? {} : { '/recipes': '/how-tos/bash/' }),
    // The shell recipes moved into the Bash techniques section.
    ...Object.fromEntries(['label-a-json-file', 'review-a-diff-by-what-it-does', 'lint-prose-for-hedging', 'fill-a-form-by-selection']
      .map((slug) => [`/how-tos/${slug}`, `/how-tos/bash/${slug}/`])),
    '/shell': '/languages/bash/',
    ...Object.fromEntries(['python', 'polars', 'typescript', 'ruby', 'r', 'rust', 'c']
      .map((slug) => [`/${slug}`, `/languages/${slug}/`])),
    ...Object.fromEntries(['duckdb', 'sqlite', 'postgresql']
      .map((slug) => [`/${slug}`, `/install/${slug}/`])),
    // The language pages moved under /languages/ on 2026-10-12, and the Bash
    // techniques moved onto the Bash page.
    '/install/shell': '/languages/bash/',
    ...Object.fromEntries(['python', 'pandas', 'polars', 'typescript', 'ruby', 'r', 'rust', 'c', 'cpp', 'objective-c', 'cobol', 'ada', 'java', 'kotlin', 'scala', 'csharp', 'go', 'swift', 'zig', 'php', 'dart']
      .map((slug) => [`/install/${slug}`, `/languages/${slug}/`])),
    ...Object.fromEntries(['if', 'case', 'not-sure', 'threshold-band', 'while-read', 'pipeline', 'batches', 'ci-gate', 'long-lived-loop', 'judge-paragraphs', 'agent-tool-guard']
      .map((slug) => [`/how-tos/bash/${slug}`, `/languages/bash/${slug}/`])),
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
    // annotate's edge cases moved onto its reference page.
    '/reference/annotate': '/functions/annotate/#edge-cases',
    ...Object.fromEntries(['decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate', 'question-file']
      .map((name) => [`/reference/functions/${name}`, `/functions/${name}/`])),
    ...Object.fromEntries(['audit', 'diff', 'check', 'transform']
      .map((name) => [`/reference/${name}`, `/functions/${name}/`])),
    ...Object.fromEntries(['answers', 'question-sets', 'recording']
      .map((name) => [`/reference/${name}`, `/learn/${name}/`])),
    '/reference': '/functions/',
    // The talk merged its bench-run slide into the bench-field slide.
    '/learn/beatles-bench/bench-run': '/learn/beatles-bench/bench-field/',
  },
  // Articles place examples with <!-- example: --> and <!-- file: -->.
  // src/lib/code.mjs draws them. scripts/check-code.mjs refuses a raw fence.
  markdown: {
    remarkPlugins: [remarkExamples],
  },
});
