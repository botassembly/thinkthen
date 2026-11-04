// The Markdown form of this site's own components, for the Markdown twins and llms-full.txt.
// scripts/pages.mjs renders the shared components; a component missing from both fails the build.
import { functionOptionsMarkdown, sharedOptionsMarkdown } from './function-options.mjs';
import site from '../data/site.json' with { type: 'json' };
import { surfaces } from './function-surfaces.mjs';
import { fact, oneLine } from './facts.mjs';

export default {
  SharedOptions: () => sharedOptionsMarkdown(),
  FunctionOptions: (a) => functionOptionsMarkdown(a.name),
  LanguageGroup: () => '',
  LanguageVariant: (a) => `### ${surfaces.find(({id}) => id === a.surface)?.label ?? a.surface}\n\n`,
  InstallLine: (a) => '```' + (a.lang ?? 'sh') + '\n' + fact(a.k ?? 'install.brew') + '\n```',
  LanguageTable: () => {
    const rows = Object.values(site.languages ?? {}).map((l) => `| [${l.name}](${l.page}) | ${l.package} | \`${oneLine(l.install)}\` | ${l.requires} |`);
    return ['| Language | Package | Install | Requires |', '| --- | --- | --- | --- |', ...rows].join('\n');
  },
};
