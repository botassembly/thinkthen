// Browser proof for the current docs rules: four reader journeys, three
// viewport pairs, both themes. No retired picker, cell matrix or L1–L9 policy.
import fs from 'node:fs';
import path from 'node:path';
import http from 'node:http';
import { launchBrowser } from './browser.mjs';
import { markdownTwin } from '../src/lib/routes.mjs';
import { readExample, outputHtml } from '../src/lib/examples.mjs';
import { menuPages } from '../src/lib/sitemap.mjs';
import { markdownShape } from '../src/lib/docs-rules.mjs';

const args = process.argv.slice(2);
const only = args.includes('--pages') ? new RegExp(args[args.indexOf('--pages') + 1]) : null;
const templateReview = args.includes('--templates');
const searchOnly = args.includes('--search-only');
if (templateReview && searchOnly) throw new Error('Choose template geometry or the search-only reader walk');
// The additional review measures each actual function/SDK template on the
// required phone viewport; the normal four journeys retain all six views.
const selectedRoutes = templateReview
  ? menuPages.filter(({ route }) => /^\/(?:functions|sdks)\/[^/]+\/$/.test(route)).map(({ route }) => route)
  : ['/', '/functions/decide/', '/sdks/python/', '/reference/troubleshooting/'];
const routes = selectedRoutes.filter((route) => !only || only.test(route));
const viewports = templateReview ? [[375, 667]] : [[375, 667], [820, 1180], [1280, 800]];
if (!routes.length) throw new Error('No required reader routes selected');
const dist = path.resolve('dist');
const shots = path.resolve(process.env.SITE_LOOK_SHOTS || 'landing-shots');
fs.mkdirSync(shots, { recursive: true });
const types = { '.html': 'text/html', '.css': 'text/css', '.js': 'text/javascript', '.mjs': 'text/javascript', '.json': 'application/json', '.md': 'text/markdown', '.svg': 'image/svg+xml', '.png': 'image/png', '.woff2': 'font/woff2' };
const server = http.createServer((request, response) => {
  const name = decodeURIComponent(new URL(request.url, 'http://docs.invalid').pathname);
  let file = path.resolve(dist, '.' + name);
  if (!file.startsWith(dist + path.sep) && file !== dist) { response.writeHead(403).end(); return; }
  if (fs.existsSync(file) && fs.statSync(file).isDirectory()) file = path.join(file, 'index.html');
  if (!fs.existsSync(file)) { response.writeHead(404).end(); return; }
  response.writeHead(200, { 'content-type': types[path.extname(file)] ?? 'application/octet-stream' });
  fs.createReadStream(file).pipe(response);
});
await new Promise((resolve) => server.listen(0, '127.0.0.1', resolve));
const origin = `http://127.0.0.1:${server.address().port}`;
const browser = await launchBrowser();
const cases = [];
const redirectVisits = [];
const readerWalk = [];
const redirects = JSON.parse(fs.readFileSync('src/data/redirects.json', 'utf8'));
const oldAddresses = fs.readFileSync('src/data/old-addresses.txt', 'utf8').split('\n').filter((line) => line.startsWith('/'));
const retained = new Set([...menuPages.map(({ route }) => route), '/about/', '/refusals/', '/404/', '/blog/', '/blog/introducing-thinkthen/', '/blog/let-your-shell-branch-on-meaning/']);
// Visit the frozen old sitemap, including all matrix cells. Static Astro hosts
// express redirects as refresh documents; the final destination must be a
// real page with the requested section, never another refresh or an error.
for (const address of searchOnly ? [] : oldAddresses) {
  if (address === '/install.sh') {
    // The frozen checklist includes this installer asset, not a removed page.
    const response = await fetch(origin + address);
    const bytes = Buffer.from(await response.arrayBuffer());
    redirectVisits.push({ address, retainedAsset: true,
      failure: !response.ok || !bytes.equals(fs.readFileSync('../install.sh')) ? 'Published installer differs from its source' : undefined });
    continue;
  }
  if (retained.has(address)) continue;
  const target = redirects[address];
  const response = await fetch(origin + address);
  const html = await response.text();
  const refresh = html.match(/<meta\b[^>]*http-equiv=["']refresh["'][^>]*>/i)?.[0];
  const content = refresh?.match(/content=["']([^"']+)["']/i)?.[1];
  const actual = content?.match(/url=(.*)$/i)?.[1]?.replaceAll('&amp;', '&');
  let failure;
  if (!response.ok || !target || !actual || new URL(actual, origin).href !== new URL(target, origin).href) failure = 'Old address does not redirect to its assigned home';
  else {
    const destination = await fetch(new URL(target, origin));
    const body = await destination.text();
    const hash = new URL(target, origin).hash.slice(1);
    if (!destination.ok || /http-equiv=["']refresh["']/i.test(body) || (hash && !body.includes(`id="${hash}"`))) failure = 'Redirect destination or section is not live';
  }
  redirectVisits.push({ address, target, failure });
}

const normalize = (text) => text.replace(/\s+/g, ' ').trim();

try {
  for (const route of searchOnly ? [] : routes) for (const [width, height] of viewports) for (const theme of ['light', 'dark']) {
    const context = await browser.createBrowserContext();
    await context.overridePermissions(origin, ['clipboard-read', 'clipboard-write', 'clipboard-sanitized-write']);
    const page = await context.newPage();
    const result = { route, width, height, theme, failures: [] };
    const fail = (problem, detail) => result.failures.push({ problem, detail });
    try {
      await page.setViewport({ width, height });
      await page.emulateMediaFeatures([{ name: 'prefers-color-scheme', value: theme }]);
      await page.setRequestInterception(true);
      page.on('request', (request) => new URL(request.url()).origin === origin ? request.continue() : request.abort());
      const response = await page.goto(origin + route, { waitUntil: 'networkidle0' });
      if (!response?.ok()) throw new Error(`Reader page returned ${response?.status()}`);
      await page.evaluate(() => document.fonts.ready);
      const state = await page.evaluate(() => {
        const main = document.querySelector('main');
        const content = main?.querySelector('.sl-markdown-content');
        const rect = (node) => node?.getBoundingClientRect().toJSON();
        const example = content?.querySelector('[data-example]');
        const code = example?.querySelector(':scope > .expressive-code pre');
        const outputs = [...(example?.querySelectorAll('[data-captured-stream]') ?? [])];
        const inputs = [...(content?.querySelectorAll('.expressive-code pre') ?? [])].filter((node) => !node.closest('[data-example]') && example && Boolean(node.compareDocumentPosition(example) & Node.DOCUMENT_POSITION_FOLLOWING));
        const inside = (node) => {
          if (!node || !node.checkVisibility()) return false;
          const r = node.getBoundingClientRect();
          return r.top >= 0 && r.bottom <= innerHeight && r.left >= 0 && r.right <= innerWidth;
        };
        // Inputs may follow their call in normal reading flow. Check every
        // visible code/output port independently of first-fold input ordering.
        const flowFailures = [...(content?.querySelectorAll('pre') ?? [])].filter((node) => node.checkVisibility()
          && (node.scrollHeight > node.clientHeight || node.scrollWidth > node.clientWidth))
          .map((node) => ({ bounds: rect(node), language: node.dataset.language,
            example: node.closest('[data-example]')?.dataset.example,
            scrollWidth: node.scrollWidth, clientWidth: node.clientWidth,
            scrollHeight: node.scrollHeight, clientHeight: node.clientHeight }));
        const naturalFlow = flowFailures.length === 0;
        const codeBounds = code?.getBoundingClientRect();
        const starts = Boolean(code?.checkVisibility() && codeBounds.top >= 0 && codeBounds.top < innerHeight
          && codeBounds.left >= 0 && codeBounds.right <= innerWidth);
        const firstOutputCode = example?.querySelector('[data-captured-stream] code');
        const firstOutputText = firstOutputCode?.textContent ?? '';
        const firstLine = firstOutputText.split(/[\r\n]/)[0];
        const firstLineRects = [];
        if (firstOutputCode?.firstChild && firstLine) {
          const range = document.createRange();
          range.setStart(firstOutputCode.firstChild, 0);
          range.setEnd(firstOutputCode.firstChild, firstLine.length);
          firstLineRects.push(...range.getClientRects());
        }
        const emptyNotice = example?.querySelector('[data-captured-stream] .empty-output');
        const firstOutputVisible = firstLine ? firstLineRects.length > 0 && firstLineRects.every((rect) =>
          rect.top >= 0 && rect.bottom <= innerHeight && rect.left >= 0 && rect.right <= innerWidth)
          : Boolean(emptyNotice && inside(emptyNotice));
        const clone = content?.cloneNode(true);
        clone?.querySelectorAll('button, .copy, .frame > .header, [data-pagefind-ignore], script, style').forEach((node) => node.remove());
        const literals = [];
        for (const node of [...(clone?.querySelectorAll('pre, code') ?? [])].filter((node) => node.tagName === 'PRE' || !node.closest('pre'))) {
          const block = node.tagName === 'PRE';
          const lines = [...node.querySelectorAll('.ec-line')];
          const text = lines.length ? lines.map((line) => {
            const value = line.querySelector('.code')?.textContent ?? '';
            // Expressive Code puts one LF in an empty visual line. The
            // join below supplies that line boundary, not another blank line.
            return value === '\n' ? '' : value;
          }).join('\n') : node.textContent ?? '';
          literals.push({ kind: block ? 'block' : 'inline', text: block ? text.replace(/\n$/, '') : text });
          node.replaceWith(document.createTextNode(` \u0000${literals.length - 1}\u0000 `));
        }
        clone?.querySelectorAll('p,li,td,th,tr,h1,h2,h3,h4,h5,h6,summary').forEach((node) => node.append(' '));
        const title = main?.querySelector('h1');
        const titleRange = document.createRange();
        if (title) titleRange.selectNodeContents(title);
        const titleLines = new Set([...titleRange.getClientRects()].filter((r) => r.width > 0).map((r) => Math.round(r.top))).size;
        const h1 = title?.textContent ?? '';
        const topCopy = document.querySelector('.header .copy-md.icon button');
        const topLabel = topCopy?.querySelector('.label');
        const labelStyle = topLabel && getComputedStyle(topLabel);
        const labelBounds = topLabel?.getBoundingClientRect();
        const buttonBounds = topCopy?.getBoundingClientRect();
        const copyLabelVisible = Boolean(topLabel?.checkVisibility() && Number(labelStyle.opacity) > 0
          && labelBounds.width > 0 && labelBounds.height > 0 && buttonBounds
          && labelBounds.left >= buttonBounds.left && labelBounds.right <= buttonBounds.right
          && labelBounds.top >= buttonBounds.top && labelBounds.bottom <= buttonBounds.bottom);

        return {
          header: {
            blog: Boolean(document.querySelector('.header a[href="/blog/"]')),
            footerBlog: Boolean(document.querySelector('footer a[href="/blog/"]')),
            copyText: topLabel?.textContent,
            copyLabelVisible,
            copyTarget: rect(topCopy),
          },
          pageHeight: Math.max(document.body.scrollHeight, document.documentElement.scrollHeight),
          theme: document.documentElement.dataset.theme,
          titleLines,
          picker: Boolean(document.querySelector('[data-surface-picker], [data-language-group], [data-language-variant]')),
          overflow: document.documentElement.scrollWidth > innerWidth,
          scroll: { x: scrollX, y: scrollY },
          first: { id: example?.dataset.example, inputs: inputs.map(rect), code: rect(code), codePort: code && { scrollWidth: code.scrollWidth, clientWidth: code.clientWidth, scrollHeight: code.scrollHeight, clientHeight: code.clientHeight }, outputs: outputs.map(rect), starts, naturalFlow, flowFailures, firstOutputLine: { text: firstLine, rects: firstLineRects.map((value) => value.toJSON()), visible: firstOutputVisible }, functionFits: inside(code) && firstOutputVisible && code.scrollWidth <= code.clientWidth && code.scrollHeight <= code.clientHeight },
          text: h1 + ' ' + (clone?.textContent ?? ''),
          literals,
          examples: [...(content?.querySelectorAll('[data-example]') ?? [])].map((node) => {
            const frame = node.querySelector(':scope > .expressive-code');
            const label = node.querySelector(':scope > .example-label');
            const output = node.querySelector('[data-captured-stream]');
            return { id: node.dataset.example, label: label?.textContent,
              transform: label && getComputedStyle(label).textTransform,
              gaps: frame && label && output ? [rect(label).top - rect(frame).bottom, rect(output).top - rect(label).bottom] : null,
              streams: [...node.querySelectorAll('[data-captured-stream]')].map((stream) => ({ name: stream.dataset.capturedStream, text: stream.querySelector('code')?.textContent })) };
          }),
          codeTargets: [...document.querySelectorAll('.expressive-code .copy button')].filter((node) => node.checkVisibility()).map((node) => ({ target: rect(node), port: rect(node.closest('.frame')) })),
          outputTargets: [...document.querySelectorAll('[data-copy-output]')].filter((node) => node.checkVisibility()).map((node) => ({ target: rect(node), port: rect(node.closest('pre')) })),
        };
      });
      result.geometry = state.first;
      result.header = state.header;
      if (state.header.blog) fail('Header retains the Blog link');
      if (!state.header.footerBlog) fail('Footer Blog link is missing');
      if (width >= 820 && (state.header.copyText !== 'Copy page' || !state.header.copyLabelVisible)) {
        fail('Top copy button lacks visible Copy page text', state.header);
      }
      if (!state.header.copyTarget || state.header.copyTarget.width < 44 || state.header.copyTarget.height < 44) {
        fail('Top copy target is smaller than 44px', state.header.copyTarget);
      }
      result.pageHeight = state.pageHeight;
      if (state.theme !== theme) fail('Theme does not match selected theme', state.theme);
      if (state.titleLines !== 1) fail('Page title wraps', state.titleLines);
      if (state.picker) fail('Language picker or hidden language variants remain');
      if (state.overflow) fail('Page scrolls sideways');
      if (state.scroll.x || state.scroll.y) fail('Initial page was scrolled', state.scroll);
      const requiresExample = route !== '/reference/troubleshooting/';
      if (requiresExample && !state.first.id) fail('Required first example is missing');
      if (width === 375 && (requiresExample || state.first.id)) {
        if (!state.first.naturalFlow) fail('First example uses a nested scroll frame', state.first);
        if (route.startsWith('/functions/')) {
          if (!state.first.functionFits) fail('Function command and first output line do not fit the phone screen', state.first);
        } else if (!state.first.starts) fail('First example does not begin on the phone screen', state.first);
      }
      for (const target of [...state.codeTargets, ...state.outputTargets]) {
        const { target: button, port } = target;
        if (button.width < 44 || button.height < 44 || button.left < port.left || button.right > port.right || button.top < port.top || button.bottom > port.bottom) fail('Copy target is not 44px inside its box', target);
      }
      if (width === 375 && route.startsWith('/functions/') && state.pageHeight > 6 * height) {
        fail('Function page exceeds six phone screens', { actual: state.pageHeight, maximum: 6 * height });
      }
      // This bounded mode supplies missing physical template evidence; exact
      // twins, clipboard and shared controls remain covered by the four journeys.
      if (templateReview) continue;
      for (const example of state.examples) {
        const captured = readExample(example.id);
        if (!captured.unverified && (example.label !== 'Output' || example.transform !== 'none' || !example.gaps || example.gaps.some((gap) => Math.abs(gap) > 1))) fail('Output label or command/output spacing differs', example);
        for (const stream of example.streams) if (stream.text !== captured[stream.name === 'stdout' ? 'output' : 'stderr']) fail('Displayed output differs from captured bytes', { id: example.id, stream: stream.name });
        if (!captured.unverified && 1 + Number(Boolean(captured.stderr)) !== example.streams.length) fail('Captured stream missing from page', example.id);
      }
      const twin = fs.readFileSync(path.join(dist, markdownTwin(route)), 'utf8');
      const expectedShape = markdownShape(twin);
      if (normalize(state.text) !== expectedShape.prose || JSON.stringify(state.literals) !== JSON.stringify(expectedShape.literals)) fail('Markdown twin differs from page text or literal code', { page: { prose: normalize(state.text), literals: state.literals }, twin: expectedShape });
      await page.click('.copy-md.icon button');
      await page.waitForSelector('.copy-md.icon button[data-state="copied"]');
      if (await page.evaluate(() => navigator.clipboard.readText()) !== twin) fail('Page copy differs from Markdown twin');
      await page.waitForFunction(() => {
        const button = document.querySelector('.copy-md.icon button');
        return button && !button.dataset.state;
      });
      if (await page.$eval('.copy-md.icon .label', (node) => node.textContent) !== 'Copy page') fail('Top copy label changed after reset');
      if (await page.$eval('.copy-md.button .label', (node) => node.textContent) !== 'Copy page as Markdown') fail('Bottom copy label changed');
      if (state.first.id) {
        const source = readExample(state.first.id).show;
        await page.click('[data-example] > .expressive-code .copy button');
        await page.waitForFunction(async (expected) => await navigator.clipboard.readText() === expected, {}, source);
        if (await page.evaluate(() => navigator.clipboard.readText()) !== source) fail('Command copy changes the shown program');
      }
      // Copy actual displayed output; do not synthesize clipboard contents in the test.
      const output = await page.$('[data-captured-stream]');
      if (output) {
        const expected = await output.$eval('code', (node) => node.textContent);
        await output.$eval('button', (node) => node.click());
        await page.waitForSelector('[data-captured-stream] button[data-state="copied"]');
        if (await page.evaluate(() => navigator.clipboard.readText()) !== expected) fail('Output copy changes captured text');
      }
      // One browser-only fixture exercises HTML parsing and the real output
      // copy handler. Restore the actual capture before measuring or saving it.
      if (route === routes[0] && width === 375 && theme === 'light' && state.first.id) {
        if (!output) fail('Literal-output fixture requires a rendered output');
        else {
          const literal = 'a  b\t<NA>\r\n\r\nlone\rreturn\n```\n{"n":9007199254740993123}\n';
          const original = await output.$eval('code', (node) => node.innerHTML);
          try {
            await output.$eval('code', (node, html) => { node.innerHTML = html; }, outputHtml(literal));
            if (await output.$eval('code', (node) => node.textContent) !== literal) fail('HTML parsing changes literal output bytes');
            await output.$eval('button', (node) => { delete node.dataset.state; node.click(); });
            await page.waitForSelector('[data-captured-stream] button[data-state="copied"]');
            if (await page.evaluate(() => navigator.clipboard.readText()) !== literal) fail('Output clipboard changes CR, tabs, spaces or blank lines');
          } finally {
            await output.$eval('code', (node, html) => { node.innerHTML = html; }, original);
            await output.$eval('button', (node) => { delete node.dataset.state; node.setAttribute('aria-label', 'Copy output'); });
          }
        }
      }
      // Open the actual phone navigation, then measure all 45 labels after
      // opening their native group disclosures. Restore before screenshots.
      const menuButton = await page.$('nav.sidebar > button.sl-menu-button[popovertarget="starlight__sidebar"]');
      if (width < 800) {
        if (!menuButton) throw new Error('Phone navigation button is missing');
        await menuButton.click();
        await page.waitForFunction(() => document.querySelector('#starlight__sidebar')?.matches(':popover-open'));
      }
      const navigation = await page.evaluate((expected) => {
        const nav = document.querySelector('nav.sidebar');
        const pane = nav?.querySelector('#starlight__sidebar');
        if (!pane?.checkVisibility()) return { missingNavigation: true };
        const closed = [...nav.querySelectorAll('details:not([open])')];
        closed.forEach((node) => { node.open = true; });
        const failures = [];
        for (const { route, label } of expected) {
          const link = [...nav.querySelectorAll('a')].find((node) => new URL(node.href).pathname === route);
          if (!link) { failures.push({ route, problem: 'missing menu page' }); continue; }
          if (link.textContent.trim() !== label) failures.push({ route, problem: 'menu label differs', text: link.textContent });
          const glyphs = [];
          const walker = document.createTreeWalker(link, NodeFilter.SHOW_TEXT);
          for (let node; (node = walker.nextNode());) {
            if (!node.textContent.trim()) continue;
            const range = document.createRange(); range.selectNodeContents(node);
            glyphs.push(...[...range.getClientRects()].filter((rect) => rect.width > 0));
          }
          const rows = new Set(glyphs.map((rect) => Math.round(rect.top)));
          const port = pane.getBoundingClientRect();
          const bounds = link.getBoundingClientRect();
          if (!link.checkVisibility() || rows.size !== 1 || glyphs.some((rect) => rect.left < Math.max(port.left, bounds.left) || rect.right > Math.min(port.right, bounds.right))) {
            failures.push({ route, problem: 'menu label wraps or clips', port: port.toJSON(), bounds: bounds.toJSON(), glyphs: glyphs.map((rect) => rect.toJSON()) });
          }
        }
        closed.forEach((node) => { node.open = false; });
        return { failures };
      }, menuPages);
      if (navigation.missingNavigation || navigation.failures?.length) fail('Navigation does not meet the sitemap', navigation);
      if (width < 800 && menuButton) {
        await menuButton.click();
        // Starlight clears main-content inertness in the asynchronous toggle
        // event, after the native popover's open state already changed.
        await page.waitForFunction(() => !document.querySelector('#starlight__sidebar')?.matches(':popover-open')
          && [...document.querySelectorAll('.main-frame, .sl-skip-link')].every((node) => !node.hasAttribute('inert')));
      }
      // Native disclosures remain keyboard-accessible, including folded SDK
      // sections; their contents were included in the twin comparison above.
      const summary = await page.$('main details:not([open]) > summary');
      if (summary) {
        await summary.focus();
        const focused = await summary.evaluate((node) => ({
          active: document.activeElement === node,
          inert: Boolean(node.closest('[inert]')),
          activeElement: document.activeElement?.outerHTML.slice(0, 300),
        }));
        if (!focused.active || focused.inert) fail('Folded section did not receive keyboard focus', focused);
        await page.keyboard.press('Enter');
        if (!await summary.evaluate((node) => node.parentElement.open)) fail('Keyboard cannot open folded section');
        await page.keyboard.press('Enter');
        if (await summary.evaluate((node) => node.parentElement.open)) fail('Keyboard cannot close folded section');
      }
    } catch (error) { fail('Browser check did not complete', error.message); }
    finally {
      const name = `${route.replace(/[^a-z0-9]+/gi, '-') || 'home'}-${width}-${theme}.png`;
      await page.screenshot({ path: path.join(shots, name), fullPage: true }).catch((error) => fail('Screenshot failed', error.message));
      cases.push(result);
      console.log(`${result.failures.length ? 'FAIL' : 'PASS'} ${route} ${width}x${height} ${theme}: ${result.failures.length} finding(s)`);
      await context.close();
    }
  }
  if (!templateReview) {
    // One real desktop journey supplies the docs review's findability evidence.
    // These are clicks/searches on this private build, not direct route shortcuts.
    const context = await browser.createBrowserContext();
    const page = await context.newPage();
    const resourceErrors = [];
    page.on('requestfailed', (request) => resourceErrors.push({ url: request.url(), error: request.failure()?.errorText }));
    page.on('pageerror', (error) => resourceErrors.push({ error: error.message }));
    page.on('response', (response) => { if (response.status() >= 400) resourceErrors.push({ url: response.url(), status: response.status() }); });
    await page.setViewport({ width: 1280, height: 800 });
    await page.emulateMediaFeatures([{ name: 'prefers-color-scheme', value: 'light' }]);
    await page.setRequestInterception(true);
    page.on('request', (request) => new URL(request.url()).origin === origin ? request.continue() : request.abort());
    const home = () => page.goto(origin + '/', { waitUntil: 'networkidle0' });
    const menu = async (group, destination, notes) => {
      for (const summary of await page.$$('nav.sidebar summary')) {
        if ((await summary.evaluate((node) => node.textContent.trim())) === group
          && !await summary.evaluate((node) => node.parentElement.open)) await summary.click();
      }
      const selector = `nav.sidebar a[href="${destination}"]`;
      const link = await page.waitForSelector(selector, { visible: true });
      notes.push({ action: 'menu', group, label: await link.evaluate((node) => node.textContent.trim()), destination });
      await Promise.all([page.waitForNavigation({ waitUntil: 'networkidle0' }), link.click()]);
    };
    const answer = async (needles, selector = 'main .sl-markdown-content') => {
      const text = await page.$eval(selector, (node) => node.innerText);
      return needles.map((needle) => {
        const offset = text.indexOf(needle);
        if (offset < 0) throw new Error(`Answer not found: ${needle}`);
        return text.slice(Math.max(0, offset - 50), offset + needle.length + 100);
      });
    };
    const tasks = [
      ['Find how decide handles not sure', async (notes) => {
        await menu('Functions', '/functions/decide/', notes);
        return answer(['Send not sure to a person', 'Never treat it as a no']);
      }],
      ['Find CSV input and output with search', async (notes) => {
        const search = await page.waitForSelector('site-search button[data-open-modal]:not([disabled])');
        await search.click();
        await page.waitForSelector('site-search dialog[open] input.pagefind-ui__search-input', { visible: true });
        await page.type('site-search dialog[open] input.pagefind-ui__search-input', 'CSV');
        notes.push({ action: 'search query', query: 'CSV' });
        const resultsSelector = 'site-search dialog[open] a.pagefind-ui__result-link';
        const parentResults = 'site-search dialog[open] .pagefind-ui__result';
        await page.waitForSelector(resultsSelector, { visible: true });
        await page.waitForFunction((selector) => {
          const items = [...document.querySelectorAll(selector)];
          return items.length > 0 && items.every((node) => node.querySelector('a.pagefind-ui__result-link'));
        }, {}, parentResults);
        const total = await page.$eval('site-search dialog[open]', (dialog) => {
          const count = [...dialog.querySelectorAll('.pagefind-ui__message')]
            .map((node) => node.textContent.match(/\b(\d+)\s+results?\b/i)).find(Boolean);
          return count ? Number(count[1]) : 0;
        });
        if (!Number.isSafeInteger(total) || total < 1) throw new Error('Search result total is missing');
        const results = () => page.$$eval(resultsSelector, (nodes) => nodes.map((node) => ({ text: node.innerText, href: node.getAttribute('href') })));
        let found = await results();
        let rendered = await page.$$eval(parentResults, (nodes) => nodes.length);
        notes.push({ action: 'search results', total, count: rendered, results: found });
        const targetSelector = 'site-search dialog[open] a[href*="/command-line/file-types/"]';
        let result = await page.$(targetSelector);
        // The rendered total bounds this reader's real pagination clicks.
        for (let clicks = 0; !result && clicks < total; clicks++) {
          if (rendered >= total) throw new Error('All CSV results exhausted without File types');
          const more = await page.$('site-search dialog[open] button.pagefind-ui__button');
          if (!more) throw new Error('No Load more results control before File types');
          const label = await more.evaluate((node) => node.innerText);
          const before = rendered;
          await more.click();
          notes.push({ action: 'load more', label, click: clicks + 1, before });
          await page.waitForFunction((selector, count) => {
            const items = [...document.querySelectorAll(selector)];
            return items.length > count && items.every((node) => node.querySelector('a.pagefind-ui__result-link'));
          }, {}, parentResults, before);
          rendered = await page.$$eval(parentResults, (nodes) => nodes.length);
          found = await results();
          notes.push({ action: 'search results', total, count: rendered, results: found });
          result = await page.$(targetSelector);
        }
        if (!result) throw new Error('CSV search reached its finite pagination limit');
        await page.waitForSelector(targetSelector, { visible: true });
        notes.push({ action: 'search', query: 'CSV', label: await result.evaluate((node) => node.textContent.trim()), href: await result.evaluate((node) => node.getAttribute('href')) });
        await Promise.all([page.waitForNavigation({ waitUntil: 'networkidle0' }), result.click()]);
        if (new URL(page.url()).pathname !== '/command-line/file-types/') throw new Error('Search did not reach File types');
        return answer(['--column', '--into', 'messages.thinkthen.csv']);
      }],
      ['Find Python choose and not sure', async (notes) => {
        await menu('SDKs', '/sdks/python/', notes);
        const summary = await page.$('#choose + details > summary, .sl-heading-wrapper:has(#choose) + details > summary');
        if (!summary) throw new Error('Python choose disclosure is missing');
        notes.push({ action: 'disclosure', label: await summary.evaluate((node) => node.textContent.trim()) });
        await summary.click();
        if (!await summary.evaluate((node) => node.parentElement.open)) throw new Error('Choose did not open');
        return answer(['topic == "billing"', 'topic.unsure']);
      }],
    ];
    try {
      for (const [task, run] of searchOnly ? tasks.slice(1, 2) : tasks) {
        const entry = { task, actions: [] };
        try {
          await home();
          entry.answer = await run(entry.actions);
          entry.destination = new URL(page.url()).pathname;
          entry.completed = true;
        } catch (error) {
          entry.failure = error.message; entry.completed = false;
          entry.diagnostic = await page.evaluate(() => {
            const dialog = document.querySelector('site-search dialog');
            return { url: location.href, dialogOpen: dialog?.open,
              query: dialog?.querySelector('input')?.value,
              text: dialog?.innerText.slice(0, 10000),
              links: [...(dialog?.querySelectorAll('a') ?? [])].map((node) => ({ text: node.innerText, href: node.getAttribute('href') })),
              busy: [...(dialog?.querySelectorAll('[aria-busy]') ?? [])].map((node) => node.getAttribute('aria-busy')) };
          }).catch((failure) => ({ error: failure.message }));
          entry.resourceErrors = [...resourceErrors];
          await page.screenshot({ path: path.join(shots, 'reader-walk-failure.png'), fullPage: true });
        }
        readerWalk.push(entry);
      }
      if (!searchOnly) {
        const agent = { task: 'Follow agent Markdown links', actions: [] };
        try {
          const response = await fetch(origin + '/llms.txt');
          if (!response.ok) throw new Error('llms.txt is not live');
          const llms = await response.text();
          const links = [...llms.matchAll(/\[[^\]]*\]\(([^)]+)\)/g)].map((match) => match[1]);
          for (const route of ['/', '/functions/decide/', '/sdks/python/']) {
            const target = markdownTwin(route);
            const advertised = links.find((link) => new URL(link, origin).pathname === target);
            if (route !== '/' && !advertised) throw new Error(`llms.txt does not link ${target}`);
            const markdown = await fetch(origin + target);
            const bytes = Buffer.from(await markdown.arrayBuffer());
            if (!markdown.ok || !bytes.equals(fs.readFileSync(path.join(dist, target)))) throw new Error(`Markdown link differs: ${target}`);
            agent.actions.push({ advertised, target, bytes: bytes.length });
          }
          agent.completed = true;
        } catch (error) { agent.failure = error.message; agent.completed = false; }
        readerWalk.push(agent);
      }
    } finally { await context.close(); }
  }
} finally { await browser.close(); await new Promise((resolve) => server.close(resolve)); }
const failures = [...redirectVisits.filter(({ failure }) => failure).map(({ address, target, failure }) => ({ route: address, problem: failure, target })), ...cases.flatMap((entry) => entry.failures.map((failure) => ({ route: entry.route, width: entry.width, theme: entry.theme, ...failure }))), ...readerWalk.filter((entry) => !entry.completed).map(({ task, failure }) => ({ problem: 'Reader walk did not complete', task, detail: failure }))];
fs.writeFileSync('landing-report.json', JSON.stringify({ mode: searchOnly ? 'search-only' : templateReview ? 'templates' : 'journeys', rules: '2026-11-03-docs-rules-and-sitemap', redirectVisits, cases, readerWalk, failures, passed: !failures.length }, null, 2) + '\n');
fs.writeFileSync('landing-geometry.json', JSON.stringify(cases.map(({ route, width, height, theme, geometry, pageHeight }) => ({ route, width, height, theme, geometry, pageHeight })), null, 2) + '\n');
console.log(`Docs browser: ${cases.length} cases, ${failures.length} findings; landing-report.json`);
if (failures.length) process.exitCode = 1;
