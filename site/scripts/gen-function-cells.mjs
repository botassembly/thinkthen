// Route scaffolding copies metadata and the authored introduction, never new prose or answers.
// An authored cell replaces this scaffold at the same path.
import fs from 'node:fs';
import path from 'node:path';
import YAML from 'yaml';
import { functions, surfaces, cellId } from '../src/lib/function-surfaces.mjs';
import { inputFiles } from '../src/lib/examples.mjs';
const docs = 'src/content/docs';
for (const fn of functions) {
  const source = fs.readFileSync(`${docs}/functions/${fn}.mdx`, 'utf8');
  const frontmatter = source.match(/^---\n[\s\S]*?\n---\n/)[0];
  const title = YAML.parse(frontmatter.replace(/^---\n|\n---\n$/g, '')).title;
  const introduction = source.slice(frontmatter.length).replace(/^import[^\n]*\n/gm, '').trimStart().split(/\n\s*\n/, 1)[0];
  if (!introduction || introduction.startsWith('<')) throw new Error(`Function ${fn}: require an authored opening paragraph`);
  for (const { id, label } of surfaces) {
    const file = path.join(docs, `function-cells/${fn}-${id}.mdx`);
    if (fs.existsSync(path.join(docs, `${cellId(fn, id)}.mdx`))) { fs.rmSync(file, { force: true }); continue; }
    const inputs = inputFiles(cellId(fn, id));
    const inputImport = inputs.length ? "import ExampleFile from '../../../components/ExampleFile.astro';\n" : '';
    const inputBlocks = inputs.map((name) => `<ExampleFile id="${cellId(fn, id)}" file="${name}" />\n\n`).join('');
    fs.mkdirSync(path.dirname(file), { recursive: true });
    fs.writeFileSync(file, `${frontmatter.replace('---\n', `---\nslug: ${cellId(fn, id)}\n`).replace(/^title:.*$/m, `title: ${JSON.stringify(`${title} (${label})`)}`)}import Example from '../../../components/Example.astro';\n${inputImport}\n{/* Function cell scaffold. */}\n\n<Example id="${cellId(fn, id)}" wrap />\n\n${introduction}\n\n${inputBlocks}## Next\n\n[${fn}](/functions/${fn}/)\n`);
  }
}
console.log(`Static function routes: ${functions.length * surfaces.length} cells`);
