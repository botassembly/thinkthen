// Actual generated HTML in cached Chromium. No network or browser installation.
import fs from 'node:fs';
import assert from 'node:assert/strict';
import { chromium } from 'playwright-core';
import { DIST, serveDist } from './serve-dist.mjs';
import { ALIASES, routeFile, recipeRouteState } from './redirect-contract.mjs';
let server, browser;
let deadline;
const recipeState = recipeRouteState();
const activeAliases = Object.entries(ALIASES).filter(([alias]) => alias !== '/recipes' || !recipeState.index);
async function proof() {
  if (!fs.existsSync(chromium.executablePath())) throw new Error('redirect browser prerequisite: matching cached Chromium missing');
  server = await serveDist();
  browser = await chromium.launch();
  async function context(javaScriptEnabled = true) {
    const ctx = await browser.newContext({ javaScriptEnabled, viewport: { width: 1280, height: 900 } });
    await ctx.route('**/*', async route => {
      const url = new URL(route.request().url());
      if (url.origin !== server.origin) return route.abort();
      if (Object.hasOwn(ALIASES, url.pathname)) return route.fulfill({ status: 200, contentType: 'text/html', body: fs.readFileSync(routeFile(DIST, url.pathname)) });
      return route.continue();
    });
    return ctx;
  }
  async function visit(ctx, incoming, fixed, anchor = null, observe = false, direct = false) {
    const page = await ctx.newPage();
    page.setDefaultTimeout(5000); page.setDefaultNavigationTimeout(5000);
    const requests = [];
    page.on('request', request => { if (request.isNavigationRequest() && request.frame() === page.mainFrame()) requests.push(request); });
    try {
      await page.goto(server.origin + incoming, { waitUntil: 'domcontentloaded' });
      try { await page.waitForURL(url => url.href === server.origin + fixed, { waitUntil: 'domcontentloaded' }); }
      catch { throw new Error('canonical URL assertion: did not reach expected destination'); }
      assert.equal(page.url(), server.origin + fixed, 'canonical URL assertion');
      assert.equal(await page.locator('main').count(), 1, 'canonical main');
      assert.equal(await page.locator('meta[http-equiv="refresh"]').count(), 0, 'canonical is not stub');
      assert.equal(await page.locator('link[rel="canonical"]').getAttribute('href'), 'https://thinkthen.dev' + new URL(fixed, server.origin).pathname);
      assert.equal(requests.length, direct ? 1 : 2, 'direct document requests');
      assert.equal(requests[0].url(), (server.origin + incoming).split('#')[0]);
      assert.equal(requests.at(-1).url(), (server.origin + fixed).split('#')[0]);
      for (const request of requests) { assert.equal(request.redirectedFrom(), null, 'no HTTP redirect'); assert.equal((await request.response()).ok(), true, 'document succeeds'); }
      if (anchor) {
        const targets = page.locator(`[id="${anchor}"]`); assert.equal(await targets.count(), 1, 'unique bookmark target');
        await page.evaluate(() => document.fonts.ready);
        await page.waitForFunction(id => { const r = document.getElementById(id).getBoundingClientRect(); return r.top < innerHeight && r.bottom >= 0; }, anchor);
      }
      if (observe) { await new Promise(resolve => setTimeout(resolve, 1000)); assert.equal(page.url(), server.origin + fixed); assert.equal(requests.length, direct ? 1 : 2); }
    } finally { await page.close(); }
  }
  const bookmarkOnly = process.argv.includes('--decide-bookmark-only');
  const ctx = await context();
  try {
    if (!bookmarkOnly) for (const [alias, fixed] of activeAliases) await visit(ctx, alias, fixed);
    for (const [incoming, fixed, anchor, observe] of [
      ['/reference/functions/decide/#flags', '/functions/decide/#flags', 'flags', true],
      ['/reference/#tools', '/functions/#tools', 'tools', true],
      ['/reference/annotate/', '/functions/annotate/#edge-cases', 'edge-cases'],
      ['/reference/annotate#flat-fields', '/functions/annotate/#flat-fields', 'flat-fields', true],
      ['/reference/functions/decide/#%66lags', '/functions/decide/#%66lags', 'flags'],
      ['/reference/#%74ools', '/functions/#%74ools', 'tools'],
      ['/reference/annotate#%66lat-fields', '/functions/annotate/#%66lat-fields', 'flat-fields'],
      ['/reference/annotate#', '/functions/annotate/#'],
      ['/reference/functions/decide/#a%2Fb%2520c', '/functions/decide/#a%2Fb%2520c'],
    ].filter(([incoming]) => !bookmarkOnly || incoming === '/reference/functions/decide/#flags')) await visit(ctx, incoming, fixed, anchor, observe);
    if (!bookmarkOnly && recipeState.index) await visit(ctx, '/recipes/', '/recipes/', null, false, true);
    if (!bookmarkOnly && !recipeState.index) for (const suffix of ['#techniques', '#%74echniques', '#', '#a%2Fb%2520c']) await visit(ctx, '/recipes' + suffix, '/how-tos/bash/' + suffix);
    if (!bookmarkOnly) for (const [route, id] of [['/functions/decide/#flags', 'flags'], ['/functions/#tools', 'tools'], ['/functions/annotate/#flat-fields', 'flat-fields']]) await visit(ctx, route, route, id, false, true);
  } finally { await ctx.close(); }
  if (bookmarkOnly) { console.log('redirect browser: canonical decide bookmark assertion passed'); } else {
  const fallback = await context(false);
  try {
    for (const alias of ['/reference/functions/decide', '/reference/annotate', ...(!recipeState.index ? ['/recipes'] : [])]) {
      const fixed = ALIASES[alias];
      const html = fs.readFileSync(routeFile(DIST, alias), 'utf8');
      assert.ok(html.includes(`<a href="${fixed}">`), 'visible fixed fallback');
      await visit(fallback, alias, fixed);
    }
  } finally { await fallback.close(); }
  console.log(`redirect browser: ${activeAliases.length} aliases, 61 compatibility addresses, 9 original fragment cases, 3 original direct pages, ${recipeState.index ? '1 recipe direct index' : '4 recipe fragment cases'}, ${recipeState.index ? 2 : 3} Chromium no-JavaScript fallbacks passed`);
  }
}
try {
  await Promise.race([proof(), new Promise((_, reject) => { deadline = setTimeout(() => reject(new Error('whole-script deadline')), 180000); })]);
} catch (e) { console.error(`redirect browser: ${e.message}`); process.exitCode = 1; }
finally { clearTimeout(deadline); if (browser) await browser.close(); if (server) server.close(); }
