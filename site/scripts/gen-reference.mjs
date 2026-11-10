// Generates the reference pages under src/content/docs/reference/ from the built program
// and the schemas it ships: every command's help, every option and environment variable,
// every error code, every exit code and the result row.
//   npm run reference            write the pages
//   node scripts/gen-reference.mjs --check
//                                regenerate in memory and fail if the committed pages differ
// Exit codes come from src/data/exit-codes.json. Each entry names the example that shows it,
// and this script fails when that example's saved exit differs.
// Fix a wrong sentence in the help text or the schema, never in a generated page.
// A page the site menu lists is hand-written. Neither it nor an address in src/data/redirects.json
// gets a generated page, and write mode deletes nothing: --check names what is left behind.
import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import YAML from 'yaml';
import product from '../product.config.mjs';
import site from '../src/data/site.json' with { type: 'json' };
import exitCodes from '../src/data/exit-codes.json' with { type: 'json' };
import errorData from '../src/data/errors.json' with { type: 'json' };
import { VERIFIED, same, stamp } from '../src/lib/verified.mjs';
import { exampleWithFiles, readExample, capturedStreams, inlineCode, exampleInputUrl } from '../src/lib/examples.mjs';
import { linkTerms } from '../src/lib/glossary-links.mjs';
import { commandOptions } from '../src/lib/binary.mjs';
import { sections } from '../src/lib/help.mjs';
import { menuPages } from '../src/lib/sitemap.mjs';
import redirects from '../src/data/redirects.json' with { type: 'json' };

const check = process.argv.includes('--check');
const bin = path.resolve(product.binDir, product.binary);
const OUT = path.resolve('src/content/docs/reference');
const REPO = path.resolve('..');
const EXAMPLES = path.resolve('examples');
const help = (...args) => execFileSync(bin, args, { encoding: 'utf8', env: { ...process.env, NO_COLOR: '1', COLUMNS: '200' } });

const versionLine = help('--version').trim();
const contract = JSON.parse(fs.readFileSync(path.join(REPO, 'schemas/contract.json'), 'utf8'));
const rowSchema = JSON.parse(fs.readFileSync(path.join(REPO, 'schemas/row.schema.json'), 'utf8'));
const pages = new Map();
const fm = (o) => `---\n${YAML.stringify({ ...o, verified: VERIFIED }).trim()}\n---\n`;
// Provenance for maintainers, kept out of the reader's text: an HTML comment (Markdown) or an
// MDX comment.
const generatedFrom = (what, mdx = false) => (mdx ? `{/* Written by scripts/gen-reference.mjs from ${what.replace(/`/g, '')} on ${versionLine}. */}` : `<!-- Written by scripts/gen-reference.mjs from ${what.replace(/`/g, '')} on ${versionLine}. -->`);
const cell = (s) => (s ?? '').replace(/\|/g, '\\|').replace(/\n/g, ' ');
const slug = (code) => code.replace(/\./g, '-');
const route = (rel) => `/reference/${rel.replace(/(index)?\.mdx?$/, '').replace(/\/$/, '')}/`.replace(/\/+$/, '/').replace('//', '/');
const put = (rel, front, body) => pages.set(rel, `${fm(front)}\n${linkTerms(body.trim(), route(rel))}\n`);

// Reuse the release's own limits instead of keeping a second version of its findings.
const releaseSource = fs.readFileSync(path.join(OUT, 'release-notes.mdx'), 'utf8');
const releaseLimits = releaseSource.split('\n## Known limits\n')[1]?.split('\n## ')[0].trim();
if (!releaseLimits?.startsWith('| Limit |')) throw new Error('release-notes.mdx: cannot find known-limits table');
const limitLines = releaseLimits.split('\n');
// New release notes keep precise observations in the canonical resource document.
// The release table remains the fallback for source revisions without that document.
const observationsFile = path.join(REPO, 'docs/resource-observations.md');
const resourceLines = (fs.existsSync(observationsFile) ? fs.readFileSync(observationsFile, 'utf8') : releaseLimits).split('\n');
const memoryRows = resourceLines.filter((line) => /^\| .*memory/.test(line));
if (!memoryRows.length) throw new Error('release resource observations: no memory rows found');
const operationalRows = limitLines.filter((line) => line.startsWith('| ') && !/^\| .*memory/.test(line));
for (const [rel, title, description, lines] of [
  ['release-memory.md', 'Release resource observations', 'Measured resource limits and finite-window memory observations from the release qualification campaign.', [...resourceLines.filter((line) => !line.startsWith('| ') && !line.startsWith('# ' ) && !line.startsWith('## ')), '| Surface | Observation |', '| --- | --- |', ...memoryRows]],
  ['release-limits.md', 'Release operational limits', 'Known installation, database, cancellation and cost limits from the release notes.', operationalRows],
]) {
  put(rel, { title, description, goal: 'Read the release’s measured limits before using paid backends.', reader: 'A developer preparing to run ThinkThen', kind: 'reference' }, `${site.product} ${site.version} records these limits in its release notes.\n\n${lines.join('\n')}\n\n## Next\n\n- [Release notes](/changes/)\n- [Resource observations](/reference/release-memory/)\n- [Operational limits](/reference/release-limits/)`);
}

// ---- Help parsing ------------------------------------------------------------------------
const top = sections(help('--help'));
// Commands the top-level help hides. Each still answers `thinkthen NAME --help`, and the build fails when one stops answering.
const HIDDEN = { man: 'writes the manual page, in roff, to standard output' };
const commands = [
  ...top.commands.filter((c) => c.name !== 'help'),
  ...Object.keys(HIDDEN).map((name) => ({ name, summary: '', hidden: true })),
].map((c) => {
  const text = help(c.name, '--help');
  return { ...c, text, ...sections(text), summary: c.summary || sections(text).about };
});
const globalFlags = new Set(top.options.map((o) => o.flag));
const authored = (name) => {
  for (const r of [`functions/${name}`, name === 'run' ? 'toolboxes' : null, name === 'completions' ? 'install' : null]) {
    if (r && ['.md', '.mdx', '/index.md', '/index.mdx'].some((e) => fs.existsSync(path.resolve('src/content/docs', r + e)))) return `/${r}/`;
  }
  return null;
};

// ---- Commands ----------------------------------------------------------------------------
const NONE = 'The help text gives no description.';
// Plain glosses for arguments the help leaves undescribed, shown until the help describes them.
const ARG_GLOSS = {
  '<QUESTION>': 'The question to ask, in quotes.',
  '[FILE]...': 'Files to read, in order. With none, the command reads standard input; `-` names standard input among files.',
  '<TOOLBOX>': 'The toolbox file of saved questions, or for `run` an entry name in `thinkthen.json` or the `thinkthen/` folder.',
  '<shell>': 'The shell to write completions for.',
};
const RUN_SUMMARY = 'Run one entry of a toolbox';
// The short example each command page shows; the guide that `authored` finds shows more.
const COMMAND_EXAMPLE = { decide: 'decide-hello', annotate: 'annotate-shell', run: 'run-entry', completions: 'completions', man: 'man-page' };
const commandExample = (name) => COMMAND_EXAMPLE[name] ?? (fs.existsSync(path.join(EXAMPLES, `${name}-shell`, 'run.sh')) ? `${name}-shell` : null);
const optionRow = (o) => `| ${o.short ? `\`${o.short}\`, ` : ''}\`${o.flag}\`${o.value ? ` \`${o.value}\`` : ''} | ${(cell(o.description) || NONE).replace(/([^.])$/, '$1.')}${o.values ? ` One of: ${cell(o.values)}.` : ''}${o.default ? ` Default: \`${cell(o.default)}\`.` : ''}${o.env ? ` Variable: \`${cell(o.env).replace(/: .*/, '')}\`.` : ''} |`;
// An option that most commands take is listed by name only; its row lives on the options page.
const optionUse = new Map();
for (const c of commands.filter((x) => !x.hidden)) for (const o of c.options) optionUse.set(o.flag, (optionUse.get(o.flag) ?? 0) + 1);
const SHARED_AT = 6;
for (const c of commands) {
  const summary = c.summary || '';
  const next = authored(c.name);
  const does = (summary || (c.name === 'run' ? RUN_SUMMARY : '')).replace(/^./, (x) => x.toLowerCase()).replace(/\.$/, '');
  const lead = `${site.product} ${site.version} asks a language model questions about your records from the \`thinkthen\` command. ${does ? `Use \`thinkthen ${c.name}\` to ${does}.` : `\`thinkthen ${c.name}\` has no summary in its help.`}${c.hidden ? ` It ${HIDDEN[c.name]}; the top-level \`thinkthen --help\` does not list it.` : ''} This page lists its arguments and options${next ? `; [the guide to ${c.name}](${next}) shows it at work` : ''}.`;
  const args = c.args.length
    ? ['| Argument | Description |', '| --- | --- |', ...c.args.map((a) => `| \`${a.name}\` | ${cell(a.description) || (c.name === 'annotate' && a.name === '<TOOLBOX>' ? 'The toolbox file of saved questions.' : ARG_GLOSS[a.name]) || NONE}${a.values ? ` One of: ${cell(a.values)}.` : ''} |`)].join('\n')
    : 'This command takes no arguments.';
  const listed = c.options.filter((o) => !['--help', '--version'].includes(o.flag));
  const isShared = (o) => globalFlags.has(o.flag) || (!c.hidden && (optionUse.get(o.flag) ?? 0) >= SHARED_AT);
  const own = listed.filter((o) => !isShared(o));
  const shared = listed.filter(isShared);
  const ex = commandExample(c.name);
  const exImport = ex ? "import Example from '../../../../components/Example.astro';\nimport ExampleFile from '../../../../components/ExampleFile.astro';\n" : '';
  put(`commands/${c.name}.mdx`, {
    title: `thinkthen ${c.name}`,
    description: `Reference for thinkthen ${c.name}${summary ? ` (${summary.replace(/\.$/, '').toLowerCase()})` : ''}: its arguments, options, environment variables and defaults.`,
    goal: `Look up every argument and option of thinkthen ${c.name}.`,
    reader: 'A ThinkThen user or agent checking one command',
    kind: 'reference',
  }, `
${exImport}
${generatedFrom(`\`thinkthen ${c.name} --help\``, true)}

${lead}

## Usage

\`\`\`text
${c.usage}
\`\`\`
${ex ? `\n## Example\n\n${exampleWithFiles(ex)}\n` : ''}
## Arguments

${args}

## Options

${own.length ? `| Option | What it does |\n| --- | --- |\n${own.map(optionRow).join('\n')}` : 'This command has no options that most other commands lack.'}

${shared.length ? `It also takes these options, which most commands share. [Command options](/reference/options/) describes each one: ${shared.map((o) => `\`${o.flag}\``).join(', ')}.` : ''}

## Next

${next ? `- [The guide to ${c.name}](${next})\n` : ''}- [All commands](/reference/)
- [Exit codes](/reference/exit-codes/)
`);
}

// ---- Options and environment -------------------------------------------------------------
const allNames = commands.filter((c) => !c.hidden).map((c) => c.name);
const usedBy = (list) => (list.length === allNames.length ? 'every command' : list.map((n) => `[${n}](/reference/commands/${n}/)`).join(', '));
const flags = commandOptions(commands);
put('options.md', {
  title: 'Command options',
  description: `Every ${site.product} command option: its value, description, environment variable, default and the commands that accept it.`,
  goal: 'Look up any thinkthen option and see which commands accept it.',
  reader: 'A ThinkThen user or agent checking one flag',
  kind: 'reference',
}, `
${site.product} ${site.version} asks a language model questions about your records from the \`thinkthen\` command. This page lists every option of every command, from each command's help. A flag on the command line wins over its environment variable. \`--version\` prints \`${versionLine}\`.

| Option | What it does | Commands |
| --- | --- | --- |
${flags.map((o) => `${optionRow(o).replace(/ \|$/, '')} | ${usedBy(o.commands)} |`).join('\n')}

${generatedFrom('`thinkthen COMMAND --help` for every command')}

## Next

- [Environment variables](/reference/environment/)
- [All commands](/reference/)
`);

const envs = new Map();
for (const o of flags) for (const e of o.env.split(', ').filter(Boolean)) {
  const [name, note] = e.split(': ');
  if (!envs.has(name)) envs.set(name, { name, note: note ?? '', flags: [] });
  envs.get(name).flags.push(o);
}
// Variables a backend reads. They are not options, so the help does not list them; this list is
// compared with crates/thinkthen/src/backend.rs and the build fails when the two differ.
const BACKEND_VARIABLES = [
  { name: 'THINKTHEN_API_KEY', backend: 'systemone', holds: 'The key sent to the systemone backend.' },
  { name: 'THINKTHEN_URL', backend: 'systemone', holds: 'The System One address. An explicit `--url` overrides it for the chosen backend.' },
  { name: 'THINKTHEN_BASE_URL', backend: 'systemone', holds: 'The System One address when neither `--url` nor `THINKTHEN_URL` is given.' },
  { name: 'OPENAI_BASE_URL', backend: 'openai', holds: 'The OpenAI address. An explicit `--url` overrides it for the chosen backend.' },
  { name: 'OPENAI_API_KEY', backend: 'openai', holds: 'The key sent to the openai backend.' },
];
{
  const source = fs.readFileSync(path.join(REPO, 'crates/thinkthen/src/backend.rs'), 'utf8');
  const found = new Set([...source.matchAll(/"([A-Z][A-Z0-9_]*_(?:API_KEY|URL))"/g)].map((m) => m[1]));
  const declared = new Set(BACKEND_VARIABLES.map((v) => v.name));
  const missing = [...found].filter((n) => !declared.has(n));
  const extra = [...declared].filter((n) => !found.has(n));
  if (missing.length || extra.length) {
    console.error(`gen-reference: backend variables differ from crates/thinkthen/src/backend.rs; add ${missing.join(', ') || 'none'}, remove ${extra.join(', ') || 'none'}`);
    process.exit(1);
  }
}
// Variables the command reads outside the options and the backends: every std::env read in
// crates/thinkthen/src must name an option's variable from the help, a backend variable above, or
// one of these. The build fails on a read this page does not list.
// The measurement harness owns packing diagnostics; they are not public user settings.
const INTERNAL_VARIABLE_SOURCES = new Set(['crates/thinkthen/src/engine/input_report.rs']);
const OTHER_VARIABLES = [
  { name: 'THINKTHEN_CA_FILE', source: 'crates/thinkthen/src/transport/tls.rs', holds: 'A PEM file of extra certificate authorities that HTTPS requests trust, in addition to the operating system certificate store. A file that is missing, unreadable or holds no valid certificate fails every HTTPS request with `backend.transport`.' },
];
{
  const reads = new Map();
  const walk = (dir) => {
    for (const f of fs.readdirSync(dir, { withFileTypes: true })) {
      const p = path.join(dir, f.name);
      if (f.isDirectory()) walk(p);
      else if (f.name.endsWith('.rs')) for (const m of fs.readFileSync(p, 'utf8').matchAll(/env::var(?:_os)?\(\s*"([A-Z][A-Z0-9_]*)"\s*\)/g)) reads.set(m[1], path.relative(REPO, p));
    }
  };
  walk(path.join(REPO, 'crates/thinkthen/src'));
  const known = new Set([...envs.keys(), ...BACKEND_VARIABLES.map((v) => v.name), ...OTHER_VARIABLES.map((v) => v.name)]);
  const unlisted = [...reads].filter(([name, source]) => !known.has(name) && !INTERNAL_VARIABLE_SOURCES.has(source));
  const stale = OTHER_VARIABLES.filter((v) => !reads.has(v.name));
  if (unlisted.length || stale.length) {
    console.error(`gen-reference: environment reads differ from crates/thinkthen/src; add ${unlisted.map(([n, f]) => `${n} (${f})`).join(', ') || 'none'} to OTHER_VARIABLES, remove ${stale.map((v) => v.name).join(', ') || 'none'}`);
    process.exit(1);
  }
}
put('environment.md', {
  title: 'Environment variables',
  description: `Every environment variable the ${site.product} command reads: the variable for each option, and each backend's key and address variables.`,
  goal: 'Look up which environment variable sets which thinkthen option.',
  reader: 'A ThinkThen user or agent configuring a script or a CI job',
  kind: 'reference',
}, `
The environment variables that set \`thinkthen\` options in ${site.product} ${site.version}, from the help text. An option given on the command line wins over its variable. Backend keys are not options; [Models and backends](/install/backends/) names each backend's key variable.

| Variable | Sets | Format | Commands |
| --- | --- | --- | --- |
${[...envs.values()].sort((a, b) => a.name.localeCompare(b.name)).map((e) => `| \`${e.name}\` | ${e.flags.map((o) => `\`${o.flag}\``).join(', ')} | ${cell(e.note)} | ${usedBy([...new Set(e.flags.flatMap((o) => o.commands))])} |`).join('\n')}

${generatedFrom('`thinkthen COMMAND --help` for every command')}

## Backend variables

Each backend reads its key, and the systemone backend its address, from these variables. The help does not list them. A shell step strips backend keys unless its \`env\` list names them, and requires \`--allow-shell\`; see [Shell steps see your keys](/toolboxes/shell-steps/#shell-steps-see-your-keys).

| Variable | Backend | Holds |
| --- | --- | --- |
${BACKEND_VARIABLES.map((v) => `| \`${v.name}\` | \`${v.backend}\` | ${v.holds} |`).join('\n')}

<!-- Compared with crates/thinkthen/src/backend.rs by scripts/gen-reference.mjs. -->

## Other variables

The command also reads these variables. The help does not list them.

| Variable | Holds | Read in |
| --- | --- | --- |
${OTHER_VARIABLES.map((v) => `| \`${v.name}\` | ${v.holds} | [\`${path.basename(v.source)}\`](https://github.com/botassembly/thinkthen/blob/v${site.version}/${v.source}) |`).join('\n')}

<!-- scripts/gen-reference.mjs fails when crates/thinkthen/src reads a variable this page does not list. -->

## Next

- [Command options](/reference/options/)
- [Models and backends](/install/backends/)
`);

// ---- Exit codes --------------------------------------------------------------------------
const exampleOutput = (id) => {
  const dir = path.join(EXAMPLES, id);
  if (readExample(id).unverified) return null;
  if (!fs.existsSync(path.join(dir, 'output.txt'))) throw new Error(`exit-codes.json: example ${id} has no output.txt`);
  return { exit: Number(fs.readFileSync(path.join(dir, 'exit'), 'utf8').trim()), output: fs.readFileSync(path.join(dir, 'output.txt'), 'utf8') };
};
for (const e of exitCodes) if (e.example && exampleOutput(e.example) && exampleOutput(e.example).exit !== e.code) throw new Error(`exit-codes.json: example ${e.example} exits ${exampleOutput(e.example).exit}, not ${e.code}`);
const exitFor = (code) => {
  const hit = exitCodes.find((e) => (e.codes ?? []).includes(code.name)) ?? exitCodes.find((e) => (code.record ? e.rowFailures : (e.families ?? []).includes(code.family)));
  const beforeOutput = exitCodes.find((e) => (e.beforeOutputCodes ?? []).includes(code.name));
  // A row error of this family changes the exit when every row failed with it (Completion::code).
  const allRowsFailed = code.record ? exitCodes.find((e) => (e.allRowsFailedFamilies ?? []).includes(code.family)) : undefined;
  return { ...hit, family: code.family, ...(beforeOutput && { beforeOutput }), ...(allRowsFailed && { allRowsFailed }) };
};
const exitLink = (n) => `[${n}](/reference/exit-codes/${n}/)`;
const rowExit = (exit) => exit.allRowsFailed
  ? `${exitLink(exit.code)}, or ${exitLink(exit.allRowsFailed.code)} when every row failed with a ${exit.family} error`
  : exitLink(exit.code);
const exitLinks = (exit) => exit.beforeOutput
  ? `${exitLink(exit.beforeOutput.code)} before output; after output, ${rowExit(exit)}`
  : rowExit(exit);
const exitImport = "import Example from '../../../../components/Example.astro';\nimport ExampleFile from '../../../../components/ExampleFile.astro';\n";
for (const e of exitCodes) {
  const errs = contract.codes.filter((c) => { const exit = exitFor(c); return [exit.code, exit.beforeOutput?.code, exit.allRowsFailed?.code].includes(e.code); });
  put(`exit-codes/${e.code}.mdx`, {
    title: `Exit code ${e.code}: ${e.name}`,
    description: `What exit code ${e.code} (${e.name}) means when the ${site.product} command ends, when you see it, and a run that returns it.`,
    goal: `Explain exit code ${e.code} of the thinkthen command and show a run that returns it.`,
    reader: 'A script author or agent branching on the thinkthen exit status',
    kind: 'reference',
  }, `
${e.example ? exitImport : ''}
{/* Written by scripts/gen-reference.mjs from src/data/exit-codes.json; the example's exit is checked. Set in ${e.source}. */}

ThinkThen ${site.version} asks a language model questions about your records from the \`thinkthen\` command. The command ends with exit code ${e.code}, ${e.name}, in this case: ${e.meaning.replace(/^./, (x) => x.toLowerCase())} A script reads the code from \`$?\` to decide what to do next.

| Field | Value |
| --- | --- |
| Code | ${e.code} |
| Name | ${e.name} |
| When | ${cell(e.when)} |
| Error codes | ${errs.length ? errs.map((c) => `[\`${c.name}\`](/reference/errors/${slug(c.name)}/)`).join(', ') : 'none'} |

## ${e.example ? 'Example' : 'What you see'}

${e.example ? `This run ends with exit code ${e.code}:\n\n${exampleWithFiles(e.example)}` : e.noExample}

## Next

- [All exit codes](/reference/exit-codes/)
- [${e.next.title}](${e.next.href})
`);
}
put('exit-codes/index.md', {
  title: 'Exit codes',
  description: `Every exit code the ${site.product} command returns, from 0 to 141, with what each one means and a run that shows it.`,
  goal: 'Look up what each thinkthen exit code means.',
  reader: 'A script author or agent branching on the thinkthen exit status',
  kind: 'reference',
}, `
${site.product} ${site.version} asks a language model questions about your records from the \`thinkthen\` command, which ends with one of these exit codes. A script reads the code from \`$?\`. Without \`--check\`, a yes, a no and a not sure answer all end with 0. With \`--check\`, the verdict becomes the exit code: 0, 1 or 3. A failed row wins over every verdict.

| Code | Name | Meaning |
| --- | --- | --- |
${exitCodes.map((e) => `| [${e.code}](/reference/exit-codes/${e.code}/) | ${e.name} | ${cell(e.meaning)} |`).join('\n')}

${generatedFrom('`src/data/exit-codes.json`')}

## Next

- [Error codes](/reference/errors/)
- [Troubleshooting](/troubleshooting/)
`);

// ---- Error codes -------------------------------------------------------------------------
// src/data/errors.json holds each code's meaning, when a reader sees it, its cause and fix, and
// the example that reproduces it. The build fails when a code lacks an entry or an entry's
// example does not print its code.
const outputs = fs.readdirSync(EXAMPLES).filter((id) => fs.existsSync(path.join(EXAMPLES, id, 'output.txt'))).sort()
  .map((id) => ({ id, text: capturedStreams(readExample(id)).join('\n') }));
const errImport = "import Example from '../../../../components/Example.astro';\nimport ExampleFile from '../../../../components/ExampleFile.astro';\n";
const facts = (s) => s.replace(/\{limits\.(\w+)\}/g, (_, k) => site.limits[k]);
const prints = (id, name) => {
  const o = outputs.find((x) => x.id === id);
  if (!o) throw new Error(`errors.json: example ${id} has no output.txt`);
  return o.text.includes(name);
};
// An entry's `standIn` says what its stand-in server answers. The page names that and links to
// the one page that shows such a server, instead of repeating the server's code on every page.
const STAND_IN = '/install/backends/your-server/#what-a-model-server-must-accept';
const reproduceLead = (name, id, standIn) => {
  if (name === 'local.cancelled') return 'This script starts a command that waits for input, then sends it the same signal as Ctrl-C:';
  const shown = readExample(id).show;
  const allFailed = () => exitLink(exitCodes.find((e) => (e.allRowsFailedFamilies ?? []).includes('backend')).code);
  if (standIn) return `This command sends the request to a local [stand-in server](${STAND_IN}) that ${standIn}. Its one record fails with a backend error, so the command ends with exit code ${allFailed()}:`;
  if (/\$port\b/.test(shown)) return `This command reproduces it against a stand-in service on the local machine, so no request leaves the machine. Its one record fails with a backend error, so the command ends with exit code ${allFailed()}:`;
  if (/127\.0\.0\.1:1\b/.test(shown)) return 'This command reproduces it by sending the request to a port on the local machine where nothing listens:';
  return 'This command reproduces it:';
};
const errorTitle = (c) => `Error ${c.name}: ${errorData.codes[c.name].means}`;
for (const c of contract.codes) {
  const e = errorData.codes[c.name];
  if (!e) throw new Error(`errors.json: no entry for ${c.name}`);
  for (const id of [e.example, e.also].filter(Boolean)) {
    if (!readExample(id).unverified && !prints(id, c.name) && c.name !== 'local.cancelled') throw new Error(`errors.json: example ${id} does not print ${c.name}`);
  }
  const exit = exitFor(c);
  const seen = outputs.filter((o) => o.text.includes(`thinkthen: ${c.name}: `));
  const messages = [...new Set(seen.flatMap((o) => [...o.text.matchAll(new RegExp(`thinkthen: ${c.name.replace(/[.-]/g, '\\$&')}: (.*)`, 'g'))].map((m) => m[1])))].sort();
  const scope = exit.beforeOutput
    ? `Before the first output row, this error stops the command with exit ${exitLinks(exit)}. After output starts, affected rows fail and the other records can still answer.`
    : c.record
    ? `It fails one [row](/how-thinkthen-judges/#the-terms), the answer for one record. The other records still get answers, and the command ends with exit code ${rowExit(exit)}.`
    : `It stops the whole call before any answer returns, and the command ends with exit code ${exitLink(exit.code)}.`;
  const cases = e.cases ? `\n\n| Message holds | Fix |\n| --- | --- |\n${e.cases.map(([m, f]) => `| ${cell(m)} | ${cell(f)} |`).join('\n')}` : '';
  const example = e.example
    ? `## Reproduce it\n\n${reproduceLead(c.name, e.example, e.standIn)}\n\n${e.standIn ? `[server.py](${exampleInputUrl(e.example, 'server.py')})\n\n<Example id="${e.example}" />` : exampleWithFiles(e.example)}${e.also ? `\n\nA second case:\n\n${exampleWithFiles(e.also)}` : ''}`
    : `## When you cannot reproduce it\n\n${facts(e.instead)}`;
  const shown = messages.length
    ? `## What you see\n\n${exit.beforeOutput ? 'The command prints the message on standard error. Before any output, no row is written; after output starts, a failed row carries the code and message in `error`:' : c.record ? 'The command prints the message on standard error, and the failed row carries the same code and message in `error`:' : 'The command prints one line on standard error:'}\n\n${messages.map((m) => `- ${inlineCode(`thinkthen: ${c.name}: ${m}`)}`).join('\n')}\n\n`
    : '';
  put(`errors/${slug(c.name)}.mdx`, {
    title: errorTitle(c),
    description: `What the ThinkThen error ${c.name} means (${e.means}), when you see it, what causes it and how to fix it.`,
    goal: `Explain the ${c.name} error: when it appears, its cause and its fix.`,
    reader: 'A ThinkThen user or agent who saw this code in a message or a failed row',
    kind: 'reference',
  }, `
${e.example ? errImport : ''}
{/* Written by scripts/gen-reference.mjs from schemas/contract.json and src/data/errors.json. */}

ThinkThen ${site.version} asks a language model questions about your records. Its error \`${c.name}\` means ${e.means}. You see it when ${e.when}. ${scope}

## Cause

${facts(e.cause)}${cases}

## Fix

${facts(e.fix)}

${shown}${example}

## Code details

| Field | Value |
| --- | --- |
| Code | \`${c.name}\` |
| Number | ${c.value}, for the C interface and the language packages |
| Family | ${c.family} |
| Exit code | ${exitLinks(exit)} |

## Next

- [All error codes](/reference/errors/)
- [Troubleshooting](/troubleshooting/#${c.family}-errors)
`);
}
put('errors/index.md', {
  title: 'Error codes',
  description: `All ${contract.codes.length} ${site.product} error codes: what each one means, whether it stops the call or fails one row, and its exit code.`,
  goal: 'Look up any ThinkThen error code.',
  reader: 'A ThinkThen user or agent who saw an error code',
  kind: 'reference',
}, `
${site.product} ${site.version} names every failure with one of ${contract.codes.length} error codes, such as \`usage.url\`. The \`thinkthen\` command prints an error as \`thinkthen: CODE: message\` on standard error. When one record fails, its [row](/how-thinkthen-judges/#the-terms) also carries \`"status": "failed"\` with \`error.code\` and \`error.message\`, and the other records still get answers. The language packages and the database extensions return the same codes. Find your code below; each page gives the cause and the fix.

| Code | Means | Stops | Exit code |
| --- | --- | --- | --- |
${contract.codes.map((c) => `| [\`${c.name}\`](/reference/errors/${slug(c.name)}/) | ${errorData.codes[c.name].means} | ${exitFor(c).beforeOutput ? 'the call before output; rows after output' : c.record ? 'one row' : 'the call'} | ${exitLinks(exitFor(c))} |`).join('\n')}

The first word of a code is its family: \`usage\` for a wrong option or setting, \`record\` for one bad record, \`backend\` for the model service, \`local\` for files and interrupts, \`internal\` for a defect in ${site.product}.

${generatedFrom('`schemas/contract.json`')}

## Next

- [Exit codes](/reference/exit-codes/)
- [Troubleshooting](/troubleshooting/)
`);

// ---- Result rows -------------------------------------------------------------------------
const defs = rowSchema.$defs;
const refName = (r) => r.replace('#/$defs/', '');
function type(s) {
  if (!s) return '';
  if (s === false) return 'none';
  if (s.$ref) { const n = refName(s.$ref); return defs[n] === false ? 'none' : `\`${n}\``; }
  if (s.anyOf) return s.anyOf.map(type).join(' or ');
  if (s.type === 'array') return `array of ${type(s.items)}`;
  if (s.type === 'object' && s.additionalProperties) return `object of ${type(s.additionalProperties)}`;
  if (Array.isArray(s.type)) return s.type.join(' or ');
  if (s.const !== undefined) return `\`"${s.const}"\``;
  return s.type + (s.minimum !== undefined && s.type === 'integer' && s.minimum > 0 ? ` from ${s.minimum}` : '');
}
const rows = rowSchema.oneOf.map((r) => defs[refName(r.$ref)]).map((r) => {
  const fn = defs[refName(r.properties.function.$ref)].const;
  const answered = r.oneOf.find((v) => v.properties.status.const === 'answered').properties;
  return { fn, answer: type(answered.answer), detail: type(answered.detail) };
});
// One table row per type keeps the page short: its name (with the anchor it had as a heading),
// its description, then each field with its type, or the values a plain type takes.
const typeRow = (n) => {
  const d = defs[n];
  const what = d.properties
    ? Object.entries(d.properties).map(([k, v]) => `\`${k}\` (${type(v)}${(d.required ?? []).includes(k) ? '' : ', optional'})`).join(', ')
    : d.anyOf ? d.anyOf.map((a) => a.description.replace(/\.$/, '')).join('; ') : `${d.type} from ${d.minimum} to ${d.maximum}`;
  return `| <span id="${n.toLowerCase()}"></span>\`${n}\` | ${(d.description ?? '').split('\n')[0].replace(/\.$/, '')} | ${what} |`;
};
const shared = defs.Row;
// ---- Toolbox format ------------------------------------------------------------------------
// The question fields come from schemas/request.schema.json, the schema of a saved request. The
// file shapes around them (an annotate `questions` map, run entries and steps) are fixed by
// crates/thinkthen/src/runs/entry.rs and the annotate command.
const requestSchema = JSON.parse(fs.readFileSync(path.join(REPO, 'schemas/request.schema.json'), 'utf8'));
const reqDefs = requestSchema.$defs;
const fieldType = (v) => {
  const ref = v.$ref?.split('/').pop();
  if (ref === 'Pointer') return 'JSON Pointer string, such as `"/text"`';
  if (ref === 'Threshold') return 'a cut, a number above 0 and at most 1, or a band `{"low": CUT, "high": CUT}`';
  if (ref === 'Cut') return 'a cut, a number above 0 and at most 1';
  if (ref === 'RawNamed') return 'object: name → question, at least one';
  if (v.type === 'array') return v.items?.$ref?.endsWith('RuleWire') ? 'array of rules' : 'array of strings';
  const t = [v.type].flat().filter((x) => x !== 'null');
  return t.join(' or ') + ([v.type].flat().includes('null') ? ', or null' : '');
};
// One row per function: every field it takes, its value, and whether it is required.
const questionTable = (def) => {
  const rows = [];
  for (const alt of reqDefs[def].oneOf) {
    const fn = alt.properties.function.const;
    const fields = Object.entries(alt.properties).filter(([k]) => k !== 'function').map(([k, v]) => `\`${k}\`: ${fieldType(v)}${alt.required.includes(k) ? ', required' : ''}${v.default === undefined ? '' : `, default \`${JSON.stringify(v.default)}\``}`);
    rows.push(`| ${fn} | ${fields.join('; ')} |`);
  }
  return ['| Function | Fields |', '| --- | --- |', ...rows].join('\n');
};
const ruleTable = Object.entries(reqDefs.RuleWire.properties).map(([k, v]) => `| \`${k}\` | ${fieldType(v)} | ${reqDefs.RuleWire.required.includes(k) ? 'yes' : 'no'} | ${v.default === undefined ? '' : `\`${JSON.stringify(v.default)}\``} |`).join('\n');
const annotatable = reqDefs.RawAnnotatableQuestion.oneOf.map((a) => a.properties.function.const);
const runOnly = reqDefs.RawQuestion.oneOf.map((a) => a.properties.function.const).filter((f) => !annotatable.includes(f));
put('toolbox-format.md', {
  title: 'Toolbox format',
  description: `The ${site.product} toolbox file format: the annotate questions map, run entries and steps, and every field of a saved question, from the shipped request schema.`,
  goal: 'Look up every field a toolbox file accepts.',
  reader: 'A developer or agent writing a toolbox for thinkthen annotate or thinkthen run',
  kind: 'reference',
}, `
A toolbox is a JSON file of saved questions for ${site.product} ${site.version}. \`thinkthen annotate\` reads a questions map, and \`thinkthen run\` reads entries. Every object rejects a field this page does not list, with \`usage.question\` or \`local.toolbox\` and the closest field name. [Toolboxes](/toolboxes/) shows each shape in use.

## An annotate toolbox

\`thinkthen annotate TOOLBOX FILE...\` reads one object:

| Field | Value | Required |
| --- | --- | --- |
| \`questions\` | object: name → question, at least one | yes |
| \`field\` | JSON Pointer string; the field of every question that names none | no |

A question in the map takes the fields in [Question fields](#question-fields). ${runOnly.map((f) => `\`${f}\``).join(' and ')} cannot be a question in the map.

## A run toolbox

\`thinkthen run TOOLBOX\` reads one of three shapes:

- A file with an \`entries\` object: entry name → entry. Name the entry with \`--entry\`; a file with one entry needs none.
- A file that holds one entry.
- A \`thinkthen/\` folder, where each \`NAME.json\` file holds the entry \`NAME\`. \`thinkthen run NAME\` reads \`thinkthen.json\` or the \`thinkthen/\` folder of the current directory.

An entry is one question or a list of steps:

| Field | Value | Required |
| --- | --- | --- |
| \`question\` | a question, with the fields in [Question fields](#question-fields) | yes, unless \`run\` is given |
| \`name\` | string; the row's \`source.name\`, like \`--name\` | no |
| \`run\` | array of steps, at least one; an entry with \`run\` holds no other field | yes, unless \`question\` is given |

A step is \`{"question": QUESTION}\`, with an optional \`name\`, or \`{"shell": ["PROGRAM", "ARG", ...]}\`. A shell step is a program and its arguments, never a shell line, and it requires \`--allow-shell\`. Backend key variables are removed unless the step explicitly lists them in an optional \`env\` array; other environment variables pass through. Each step reads the previous step's output on standard input.

## Question fields

Every question names its \`function\`. A run entry takes every function; an annotate map takes all but ${runOnly.map((f) => `\`${f}\``).join(' and ')}.

${questionTable('RawQuestion')}

A rule in \`rules\` takes these fields:

| Field | Value | Required | Default |
| --- | --- | --- | --- |
${ruleTable}

${generatedFrom('`schemas/request.schema.json`')}

## Next

- [Toolboxes](/toolboxes/)
- [Command options](/reference/options/)
`);

put('rows.md', {
  title: 'Result rows',
  description: `The ${site.product} result row: shared fields, statuses, each function's answer and detail shape, from the shipped row schema.`,
  goal: 'Look up the exact shape of a ThinkThen result row for any function.',
  reader: 'A developer or agent parsing thinkthen --json output or a package row',
  kind: 'reference',
}, `
Every ${site.product} ${site.version} function returns one row per record. \`thinkthen --json\` writes each row as one line of JSON, and every language package returns the same fields.

## Shared fields

| Field | Type | Required |
| --- | --- | --- |
${Object.entries(shared.properties).map(([k, v]) => `| \`${k}\` | ${type(v)} | ${shared.required.includes(k) ? 'yes' : 'no'} |`).join('\n')}
| \`status\` | ${contract.statuses.map((s) => `\`"${s.name}"\``).join(', ')} | yes |
| \`answer\` | depends on the function | on answered rows |
| \`probability\` | \`Probability\` | no |
| \`detail\` | depends on the function | no |
| \`error\` | object with \`code\`, \`message\` and optional \`http_status\` | on failed rows |

## Statuses

| Status | Number | The row carries |
| --- | --- | --- |
${contract.statuses.map((s) => {
  const v = shared.oneOf.find((o) => o.properties.status.const === s.name);
  return `| \`${s.name}\` | ${s.value} | ${v.required.filter((k) => k !== 'status').map((k) => `\`${k}\``).join(', ')}: ${v.description.replace(/\.$/, '')} |`;
}).join('\n')}

## Answers by function

| Function | \`answer\` on an answered row | \`detail\` |
| --- | --- | --- |
${rows.map((r) => `| [${r.fn}](/reference/commands/${r.fn}/) | ${r.answer} | ${r.detail} |`).join('\n')}

## Types

| Type | What it holds | Fields or values |
| --- | --- | --- |
${['Source', 'Probability', 'RecordId', 'Found', 'Name', 'Relation', 'RelationName', 'LabelDetail', 'LevelDetail', 'RecognizeDetail', 'KindDetail', 'PieceDetail', 'PairDetail'].map(typeRow).join('\n')}

${generatedFrom('`schemas/row.schema.json` and `schemas/contract.json`')}

## Next

- [How ThinkThen judges](/how-thinkthen-judges/)
- [Error codes](/reference/errors/)
`);

// ---- Index -------------------------------------------------------------------------------
put('index.md', {
  title: 'Reference',
  description: `The ${site.product} ${site.version} reference: every command, option, environment variable, result row field, error code and exit code.`,
  goal: 'Find the reference entry for any thinkthen command, option, error or exit code.',
  reader: 'A ThinkThen user or agent looking up an exact fact',
  kind: 'reference',
}, `
${site.product} ${site.version} asks a language model questions about your records, from the \`thinkthen\` command, the language packages and the database extensions. This reference lists every command, option, variable, result field, error code and exit code, taken from the program's own help and schemas. \`thinkthen --version\` prints:

\`\`\`text
${versionLine}
\`\`\`

## Commands

${top.about}

| Command | Summary |
| --- | --- |
${commands.map((c) => `| [\`thinkthen ${c.name}\`](/reference/commands/${c.name}/) | ${cell(c.summary) || 'The help text gives no summary.'} |`).join('\n')}

## Lookups

- [Command options](/reference/options/): every flag and the commands that accept it.
- [Environment variables](/reference/environment/): every \`THINKTHEN_*\` variable.
- [Result rows](/reference/rows/): the row each function returns.
- [Toolbox format](/reference/toolbox-format/): every field of a toolbox file.
- [Error codes](/reference/errors/): all ${contract.codes.length} codes.
- [Exit codes](/reference/exit-codes/): every exit status.

## Usage

\`\`\`text
${top.usage}
\`\`\`

${generatedFrom('`thinkthen --help`')}

## Next

- [Answers](/answers/): one question per page, each with a command and its real output.
`);

// ---- Write or check ----------------------------------------------------------------------
const existing = new Map();
const walk = (dir) => {
  if (!fs.existsSync(dir)) return;
  for (const f of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, f.name);
    if (f.isDirectory()) walk(p);
    else existing.set(path.relative(OUT, p), fs.readFileSync(p, 'utf8'));
  }
};
walk(OUT);
const menuRoutes = new Set(menuPages.map((p) => p.route));
const output = new Map([...pages].filter(([rel]) => !menuRoutes.has(route(rel)) && !Object.hasOwn(redirects, route(rel))));
const handWritten = (rel) => menuRoutes.has(route(rel)) && !pages.has(rel);
if (check) {
  const diffs = [];
  for (const [rel, text] of output) if (!same(existing.get(rel) ?? null, text)) diffs.push(`${rel}: differs from the build`);
  for (const rel of existing.keys()) if (!output.has(rel) && !handWritten(rel)) diffs.push(`${rel}: not generated by this build`);
  if (diffs.length) {
    console.error(`reference: ${diffs.length} page(s) out of date; run npm run reference\n${diffs.join('\n')}`);
    process.exit(1);
  }
  console.log(`reference: ${output.size} pages match the build`);
} else {
  for (const [rel, text] of output) {
    fs.mkdirSync(path.dirname(path.join(OUT, rel)), { recursive: true });
    fs.writeFileSync(path.join(OUT, rel), stamp(text, existing.get(rel) ?? null));
  }
  console.log(`reference: wrote ${output.size} pages`);
}
