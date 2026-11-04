// browser-box mounts only this launcher and the public site's isolated scratch copy.
import { launch } from 'browser-box';
import fs from 'node:fs';
globalThis.__thinkthenBrowserLaunch = launch;
const { script, args, shots, routes } = JSON.parse(fs.readFileSync('/out/browser-request.json', 'utf8'));
process.env.SITE_LOOK_SHOTS = shots ? '/out/review-shots/after' : '';
process.env.SITE_REVIEW_ROUTES = routes ?? '';
process.chdir('/out/site');
process.argv = [process.argv[0], `/out/site/${script}`, ...args];
await import(`/out/site/${script}`);
