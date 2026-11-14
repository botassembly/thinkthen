// Package already prepared runtime files; never build or execute a consumer.
import fs from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { validatePreparedExamples } from '../src/lib/prepared-examples.mjs';

const out = process.argv[2];
if (!out || !path.isAbsolute(out) || fs.existsSync(out)) throw new Error('Use a fresh absolute owned runtime bundle');
const checkout = fs.realpathSync('..');
if (!path.relative(checkout, out).startsWith(`..${path.sep}`)) throw new Error('Runtime bundle must be outside the checkout');
const packages = fs.realpathSync(process.env.SITE_PACKAGES);
const packageSource = fs.readFileSync(path.join(packages, 'source-commit'), 'utf8').trim();
const prepared = validatePreparedExamples(process.env.SITE_PREPARED_EXAMPLES);
const roots = JSON.parse(fs.readFileSync(path.join(packages, 'runtime-roots.json'), 'utf8'));
const names = ['dotnet', 'java', 'node', 'python', 'ruby', 'swift'];
if (Object.keys(roots).sort().join() !== names.join()) throw new Error('Unexpected staged runtime roots');
const mapped = {};
for (const name of names) {
  const root = roots[name].root;
  if (!path.isAbsolute(root) || root.split(path.sep).filter(Boolean).length < 3 || !fs.statSync(root).isDirectory()) throw new Error(`Refuse broad or missing runtime root: ${root}`);
  if (name !== 'swift' && (!path.isAbsolute(roots[name].executable) || !fs.statSync(roots[name].executable).isFile())) throw new Error(`Missing ${name} runtime executable`);
  mapped[name] = path.join(out, 'roots', root.slice(1));
}
fs.mkdirSync(out, { recursive: true });
for (const name of names) {
  const root = roots[name].root;
  const destination = mapped[name];
  // These are runtime slices, not copies of the JDK or .NET build SDK.
  const slices = name === 'java' ? ['bin/java', 'lib', 'conf', 'release']
    : name === 'dotnet' ? ['dotnet', 'host', 'shared']
      : name === 'node' ? [path.relative(root, roots[name].executable)]
        : name === 'ruby' ? ['bin/ruby', 'lib'] : ['.'];
  for (const slice of slices) {
    const source = path.join(root, slice);
    const target = path.join(destination, slice);
    if (!fs.existsSync(source)) throw new Error(`Missing ${name} runtime slice: ${source}`);
    fs.mkdirSync(path.dirname(target), { recursive: true });
    fs.cpSync(source, target, { recursive: true, dereference: false, verbatimSymlinks: true });
  }
}
// Close absolute runtime links over the copied files. Configuration links to
// system files (for example Java's public CA store) become immutable copies.
for (const name of names) {
  const closeLinks = (directory) => {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
      const destination = path.join(directory, entry.name);
      if (entry.isDirectory()) { closeLinks(destination); continue; }
      if (!entry.isSymbolicLink()) continue;
      const target = fs.readlinkSync(destination);
      const selected = names.find((candidate) => path.isAbsolute(target) &&
        (target === roots[candidate].root || target.startsWith(roots[candidate].root + path.sep)));
      if (selected) {
        fs.unlinkSync(destination);
        fs.symlinkSync(path.join(mapped[selected], path.relative(roots[selected].root, target)), destination);
      } else if (!fs.existsSync(destination) || !Object.values(mapped).some((root) => fs.realpathSync(destination).startsWith(root + path.sep))) {
        const original = path.join(roots[name].root, path.relative(mapped[name], destination));
        const resolved = fs.realpathSync(original);
        fs.unlinkSync(destination);
        fs.cpSync(resolved, destination, { recursive: true, dereference: true });
      }
    }
  };
  closeLinks(mapped[name]);
}
// Ruby embeds its original load paths even when its executable is copied.
// Query the selected runtime rather than duplicating its version/architecture.
const rubyPaths = spawnSync(roots.ruby.executable,
  ['--disable-gems', '-rjson', '-e', 'puts JSON.generate($LOAD_PATH)'],
  { encoding: 'utf8', env: { ...process.env, RUBYLIB: '', RUBYOPT: '' } });
if (rubyPaths.status !== 0) throw new Error(`Cannot read selected Ruby load paths: ${rubyPaths.stderr}`);
const rubyLoadPaths = JSON.parse(rubyPaths.stdout);
if (!Array.isArray(rubyLoadPaths) || rubyLoadPaths.length === 0) throw new Error('Missing selected Ruby load paths');
const rubyLib = rubyLoadPaths.map((directory) => {
  if (typeof directory !== 'string' || !path.isAbsolute(directory)) throw new Error('Invalid selected Ruby load path');
  const relative = path.relative(roots.ruby.root, directory);
  if (relative === '..' || relative.startsWith(`..${path.sep}`) || path.isAbsolute(relative)) {
    throw new Error(`Ruby load path is outside its runtime root: ${directory}`);
  }
  return path.join(mapped.ruby, relative);
}).join(path.delimiter);
const runtimeExecutable = (name) => path.join(mapped[name], path.relative(roots[name].root, roots[name].executable));
fs.mkdirSync(path.join(out, 'bin'));
fs.symlinkSync(path.join(packages, 'bin/thinkthen'), path.join(out, 'bin/thinkthen'));
const bins = ['python', 'ruby', 'node', 'java', 'dotnet'].map((name) => path.dirname(runtimeExecutable(name)));
const environment = {
  SITE_PACKAGES: packages,
  SITE_PREPARED_EXAMPLES: prepared.directory,
  SITE_BIN_DIR: path.join(out, 'bin'),
  THINKTHEN_BUILD_TOOLS: path.dirname(path.dirname(mapped.java)),
  DOTNET_ROOT: mapped.dotnet,
  RUBYLIB: rubyLib,
  PATH: [...new Set([path.join(out, 'bin'), ...bins, path.join(packages, 'bin'), '/usr/bin', '/bin'])].join(path.delimiter),
  LD_LIBRARY_PATH: [mapped.swift, path.join(packages, 'swift-libs'), path.join(packages, 'c/lib')].join(path.delimiter),
};
fs.writeFileSync(path.join(out, 'environment.json'), JSON.stringify(environment, null, 2) + '\n');
// Redirect staged interpreter links into the copied closure before freezing it.
// Other package links must already resolve when the closure is consumed.
const stagedPython = path.join(packages, 'bin/python3');
if (fs.readdirSync(path.dirname(stagedPython)).includes('python3')) {
  if (!fs.lstatSync(stagedPython).isSymbolicLink()) throw new Error('Staged Python must be a runtime link');
  fs.unlinkSync(stagedPython);
  fs.symlinkSync(runtimeExecutable('python'), stagedPython);
}
const files = {}, links = {};
const inventory = (directory, base = out, targetFiles = files, targetLinks = links) => {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const file = path.join(directory, entry.name);
    const relative = path.relative(base, file);
    if (entry.isSymbolicLink()) {
      targetLinks[relative] = fs.readlinkSync(file);
      if (!fs.existsSync(file)) throw new Error(`Unresolved runtime link: ${file}`);
    } else if (entry.isDirectory()) inventory(file, base, targetFiles, targetLinks);
    else if (entry.isFile()) targetFiles[relative] = createHash('sha256').update(fs.readFileSync(file)).digest('hex');
    else throw new Error(`Unsupported runtime file: ${file}`);
  }
};
inventory(out);
const packageFiles = {}, packageLinks = {};
inventory(packages, packages, packageFiles, packageLinks);
const source = spawnSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' });
if (source.status !== 0) throw new Error('Cannot identify runtime bundle source');
fs.writeFileSync(path.join(out, 'manifest.json'), JSON.stringify({ source: source.stdout.trim(), packageSource,
  packages, preparedSource: prepared.source, prepared: prepared.directory, roots, mapped, files, links, packageFiles, packageLinks }, null, 2) + '\n');
console.log(`Bundled six prepared runtimes (${Object.keys(files).length} files) in ${out}`);
