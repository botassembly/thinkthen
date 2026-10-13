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
    '/surfaces': '/start/',
    '/backends': '/models/',
    '/tutorial': '/start/tutorial/',
    ...(recipeSelection(RECIPE_PAGES, process.env.THINKTHEN_DRAFTS === '1' || process.argv.includes('dev')).index ? {} : { '/recipes': '/languages/bash/' }),
    // The how-tos, the shell jobs and the guides became the Recipes section
    // on 2026-10-12. The shell jobs once sat at /how-tos/<slug> too.
    '/how-tos': '/recipes/',
    '/how-tos/bash': '/recipes/',
    '/guides': '/recipes/',
    ...Object.fromEntries([...['triage-a-support-inbox', 'join-two-tables-by-meaning', 'code-open-ended-survey-answers', 'screen-studies-for-a-review', 'group-alerts-into-incidents', 'rank-the-inbound-leads', 'check-an-expense-against-the-policy', 'split-a-scanned-packet-into-documents', 'screen-a-post-before-it-goes-up', 'search-youtube-transcripts-by-meaning'], ...['label-a-json-file', 'review-a-diff-by-what-it-does', 'lint-prose-for-hedging', 'fill-a-form-by-selection', 'set-aside-bad-records']]
      .map((slug) => [`/how-tos/${slug}`, `/recipes/${slug}/`])),
    ...Object.fromEntries(['label-a-json-file', 'review-a-diff-by-what-it-does', 'lint-prose-for-hedging', 'fill-a-form-by-selection', 'set-aside-bad-records']
      .map((slug) => [`/how-tos/bash/${slug}`, `/recipes/${slug}/`])),
    '/shell': '/languages/bash/',
    ...Object.fromEntries(['python', 'polars', 'typescript', 'ruby', 'r', 'rust', 'c']
      .map((slug) => [`/${slug}`, `/languages/${slug}/`])),
    ...Object.fromEntries(['duckdb', 'sqlite', 'postgresql']
      .map((slug) => [`/${slug}`, `/databases/${slug}/`])),
    // The database pages moved under /databases/ on 2026-10-12.
    ...Object.fromEntries(['duckdb', 'sqlite', 'postgresql']
      .map((slug) => [`/install/${slug}`, `/databases/${slug}/`])),
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
    '/beatles-bench/every-language': '/languages/',
    ...Object.fromEntries(['decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate', 'audit', 'diff', 'blind-spots', 'rad']
      .map((slug) => [`/beatles-bench/${slug}`, `/learn/beatles-bench/#${slug}`])),
    // The talk cut its what-jev-knows slide. The strings page compares Jev with search.
    '/beatles-bench/what-jev-knows': '/learn/beatles-bench/#strings',
    '/learn/beatles-bench/what-jev-knows': '/learn/beatles-bench/#strings',
    // annotate's edge cases moved onto its reference page.
    '/reference/annotate': '/functions/annotate/#edge-cases',
    ...Object.fromEntries(['decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate', 'question-file']
      .map((name) => [`/reference/functions/${name}`, `/functions/${name}/`])),
    ...Object.fromEntries(['audit', 'diff', 'check', 'transform']
      .map((name) => [`/reference/${name}`, name === 'check' ? '/models/check/' : `/functions/${name}/`])),
    ...Object.fromEntries(['answers', 'question-sets', 'recording']
      .map((name) => [`/reference/${name}`, { answers: '/functions/answers/', 'question-sets': '/functions/question-file/#question-sets', recording: '/start/replay/' }[name]])),
    '/reference': '/functions/',
    // The talk merged its bench-run slide into the bench-field slide.
    '/learn/beatles-bench/bench-run': '/learn/beatles-bench/#bench-field',
    // The talk became one page with a section per slide on 2026-10-12.
    ...Object.fromEntries(['strings', 'jev', 'runs-in', 'decide', 'choose', 'tag', 'score', 'score-bands', 'filter', 'rank', 'find', 'recognize', 'relate', 'question-file', 'annotate', 'bash', 'languages', 'data', 'blind-spots', 'rad', 'audit', 'diff', 'use-cases', 'shell-uses', 'data-uses', 'retrieval', 'competitors', 'bench-field', 'backends']
      .map((slug) => [`/learn/beatles-bench/${slug}`, `/learn/beatles-bench/#${slug}`])),
    // The install page and the how-it-works pages moved under /start/ on
    // 2026-10-12, rebuilt around what each one gains the reader.
    '/install': '/start/',
    '/install/settings': '/start/settings/',
    '/install/configuration': '/start/configuration/',
    '/learn/tutorial': '/start/tutorial/',
    '/learn/functional': '/start/pay-once/',
    '/learn/caching': '/start/replay/',
    '/learn/recording': '/start/replay/',
    '/learn/files': '/start/files/',
    '/learn/overhead': '/start/speed/',
    // The backends pages became the Models section on 2026-10-12.
    '/install/backends': '/models/',
    ...Object.fromEntries(['typesafe', 'liquid', 'openai', 'perplexity', 'openrouter', 'ollama', 'local', 'system-one', 'other-servers']
      .map((slug) => [`/install/backends/${slug}`, `/models/${slug}/`])),
    '/functions/check': '/models/check/',
    // The Learn section closed on 2026-10-12. Its pages moved to Functions
    // and Start, and Beatles Bench sits in the footer.
    '/learn': '/start/',
    '/learn/answers': '/functions/answers/',
    '/learn/question-sets': '/functions/question-file/#question-sets',
  },
  // Articles place examples with <!-- example: --> and <!-- file: -->.
  // src/lib/code.mjs draws them. scripts/check-code.mjs refuses a raw fence.
  markdown: {
    remarkPlugins: [remarkExamples],
  },
});
