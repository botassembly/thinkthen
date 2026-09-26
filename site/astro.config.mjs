import { defineConfig } from 'astro/config';
import remarkBench from './src/lib/remark-bench.mjs';

export default defineConfig({
  site: 'https://thinkthen.dev',
  output: 'static',
  build: { format: 'directory' },
  devToolbar: { enabled: false },
  // The article's code blocks follow the page theme. Shiki writes both colours
  // on every token and site.css picks the dark one under a dark page, so a
  // light page never carries a dark slab. `wrap` keeps a long line inside the
  // pane.
  markdown: {
    // The Beatles Bench pages set each command beside its output.
    remarkPlugins: [remarkBench],
    shikiConfig: {
      themes: { light: 'github-light', dark: 'github-dark' },
      defaultColor: 'light',
      wrap: true,
    },
  },
});
