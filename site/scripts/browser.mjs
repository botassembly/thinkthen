// The browser-box entry point supplies the only browser launcher.
export async function launchBrowser() {
  const launch = globalThis.__thinkthenBrowserLaunch;
  if (!launch) throw new Error('Run browser checks through scripts/browser-box-job.mjs');
  const browser = await launch({ args: ['--host-resolver-rules=MAP docs-preview.test 127.0.0.1', '--no-proxy-server'] });
  if (browser.process().spawnargs.some((arg) => ['--no-sandbox', '--disable-setuid-sandbox'].includes(arg))) {
    await browser.close();
    throw new Error('Chromium sandbox disabled');
  }
  return browser;
}
