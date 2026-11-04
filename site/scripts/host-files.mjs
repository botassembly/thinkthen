// Writes the files hosts and crawlers read at fixed addresses:
//   robots.txt     allows every crawler and names the sitemap
//   sitemap.xml    the same sitemap index as Astro's sitemap-index.xml, at the classic address
//   version.json   the version, the commit of this site's source, the program's build commit
//                  when it reports one, and the build time
// Ship the repository installers unchanged at their documented addresses.
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import product from '../product.config.mjs';
import site from '../src/data/site.json' with { type: 'json' };
import { binaryInfo } from '../src/lib/binary.mjs';
import errors from '../src/data/errors.json' with { type: 'json' };
import { EXAMPLES, exampleInputUrl } from '../src/lib/examples.mjs';

const dist = path.resolve('dist');
for (const name of ['install.sh', 'install.ps1']) fs.copyFileSync(path.resolve('..', name), path.join(dist, name));
// The error registry explicitly selects these reproducible local stand-in inputs.
for (const id of new Set(Object.values(errors.codes).filter((entry) => entry.standIn).map((entry) => entry.example))) {
  const source = path.join(EXAMPLES, id, 'server.py');
  const out = path.join(dist, exampleInputUrl(id, 'server.py'));
  if (!fs.lstatSync(source).isFile() || !fs.realpathSync(source).startsWith(fs.realpathSync(EXAMPLES) + path.sep)) {
    throw new Error(`Example download must be a regular input inside examples: ${id}/server.py`);
  }
  fs.mkdirSync(path.dirname(out), { recursive: true });
  const destination = fs.lstatSync(out, { throwIfNoEntry: false });
  if (!fs.realpathSync(path.dirname(out)).startsWith(fs.realpathSync(dist) + path.sep)
      || (destination && !destination.isFile())) {
    throw new Error(`Example download destination leaves dist or is not a regular file: ${id}/server.py`);
  }
  fs.copyFileSync(source, out);
}
const abs = (p) => new URL(p, product.site).href;

fs.writeFileSync(path.join(dist, 'robots.txt'), `User-agent: *\nAllow: /\n\nSitemap: ${abs('/sitemap.xml')}\n`);
fs.copyFileSync(path.join(dist, 'sitemap-index.xml'), path.join(dist, 'sitemap.xml'));

const git = (...a) => {
  const r = spawnSync('git', a, { encoding: 'utf8' });
  return r.status === 0 ? r.stdout.trim() : null;
};
const info = binaryInfo();
const commit = git('rev-parse', 'HEAD');
const version = {
  version: site.version,
  commit,
  short: commit ? commit.slice(0, 7) : null,
  branch: git('rev-parse', '--abbrev-ref', 'HEAD'),
  built: new Date().toISOString(),
  program: { version: info.version, commit: info.commit },
};
fs.writeFileSync(path.join(dist, 'version.json'), JSON.stringify(version, null, 2) + '\n');
console.log(`host files: robots.txt, sitemap.xml, version.json (${site.version}${info.commit ? `, program commit ${info.commit.slice(0, 7)}` : ''})`);
