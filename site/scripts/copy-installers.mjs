import { copyFileSync } from 'node:fs';

for (const name of ['install.sh', 'install.ps1']) {
  copyFileSync(new URL(`../../${name}`, import.meta.url), new URL(`../dist/${name}`, import.meta.url));
}
