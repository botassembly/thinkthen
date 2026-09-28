// The social card each page shares on Open Graph and Twitter. A Beatles Bench
// page uses its own slide. A function page uses its function's slide.
// export-slides.mjs writes one card per page in slides.json to
// public/og/<page>.png, 1200 by 630. Base.astro falls back to ROUTES below,
// then to the ThinkThen card.

import slides from './slides.json';
import { PAGES } from './beatles.mjs';

export const DEFAULT_CARD = { image: '/brand/thinkthen-card.png', alt: 'ThinkThen' };

// The card of one Beatles Bench page, by its key in slides.json, or null.
export function slideCard(key) {
  if (!slides.slides[key]) return null;
  const page = PAGES.find((p) => (p.slug || 'beatles-bench') === key);
  const title = page.slug ? page.title : 'Jev, ThinkThen, and Beatles Bench';
  return { image: `/og/${key}.png`, alt: `The talk's slide: ${title}` };
}

// Other pages with a matching slide.
const LANGUAGES = ['python', 'typescript', 'ruby', 'r', 'rust', 'c'];
const DATA = ['polars', 'duckdb', 'sqlite', 'postgresql'];
const ROUTES = {
  '/functions/': 'runs-in',
  '/install/': 'runs-in',
  '/install/shell/': 'runs-in',
  '/install/backends/': 'backends',
  '/trust/': 'audit',
  ...Object.fromEntries(LANGUAGES.map((s) => [`/install/${s}/`, 'languages'])),
  ...Object.fromEntries(DATA.map((s) => [`/install/${s}/`, 'data'])),
};

export const routeCard = (route) => (ROUTES[route] ? slideCard(ROUTES[route]) : null);
