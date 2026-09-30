// robots.txt follows NOINDEX. Before launch it turns every crawler away.
// At launch NOINDEX flips, and robots.txt and the noindex tag change together.
import { NOINDEX } from '../data/catalog.mjs';

export function GET({ site }) {
  const body = NOINDEX
    ? 'User-agent: *\nDisallow: /\n'
    : `User-agent: *\nAllow: /\n\nSitemap: ${new URL('/sitemap.xml', site).href}\n`;
  return new Response(body, { headers: { 'Content-Type': 'text/plain; charset=utf-8' } });
}
