import { defineConfig } from 'astro/config';

export default defineConfig({
  site: 'https://thinkthen.dev',
  output: 'static',
  build: { format: 'directory' },
  devToolbar: { enabled: false },
});
