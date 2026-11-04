// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import { satteri } from '@astrojs/markdown-satteri';
import product from './product.config.mjs';
import redirects from './src/data/redirects.json' with { type: 'json' };

export default defineConfig({
  site: product.site,
  trailingSlash: 'always',
  // Keep -- and quotes as typed: option names such as --check must never become dashes.
  markdown: { processor: satteri({ features: { smartPunctuation: false } }) },
  // Old addresses that have no page of their own now. src/data/old-addresses.txt lists every old address; scripts/check.mjs fails if one no longer resolves.
  redirects: Object.fromEntries(Object.entries(redirects).map(([from, to]) => [from.replace(/\/$/, ''), to])),
  integrations: [
    starlight({
      title: product.name,
      pagefind: product.pagefind,
      description: product.description,
      logo: product.logo,
      favicon: product.favicon,
      social: product.social,
      customCss: ['./src/styles/brand.css'],
      pagination: false,
      // Preserve tabs in shown/copied source and its Markdown twin.
      expressiveCode: { tabWidth: 0 },
      disable404Route: true,
      lastUpdated: false,
      sidebar: product.sidebar,
      components: {
        Head: './src/components/Head.astro',
        Header: './src/components/Header.astro',
        Footer: './src/components/Footer.astro',
        MarkdownContent: './src/components/MarkdownContent.astro',
      },
      head: product.head ?? [],
    }),
  ],
});
