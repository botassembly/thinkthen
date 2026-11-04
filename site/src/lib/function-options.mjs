// Function options come from the exact program used to build the site.
import { execFileSync } from 'node:child_process';
import path from 'node:path';
import product from '../../product.config.mjs';
import { functions } from './function-surfaces.mjs';
import { sections } from './help.mjs';
import { inlineCode } from './examples.mjs';

const cache = new Map();
export function functionOptions(name) {
  if (!functions.includes(name)) throw new Error(`Unknown function options: ${name}`);
  if (!cache.has(name)) {
    const help = (...args) => sections(execFileSync(path.resolve(product.binDir, product.binary), args, {
      encoding: 'utf8', timeout: 30000, env: { ...process.env, NO_COLOR: '1', COLUMNS: '200' },
    }));
    const globals = new Set(help('--help').options.map(({ flag }) => flag));
    const command = help(name, '--help');
    const own = command.options.filter(({ flag }) => !globals.has(flag) && !['--help', '--version'].includes(flag));
    cache.set(name, [...command.args.map((arg) => ({ option: arg.name, ...arg })), ...own.map((option) => ({
      ...option, option: [option.short, `${option.flag}${option.value ? ` ${option.value}` : ''}`].filter(Boolean).join(', '),
    }))].map((option) => ({
      option: option.option,
      description: [option.description, option.values && `One of: ${option.values}.`, option.default && `Default: ${option.default}.`].filter(Boolean).join(' '),
    })));

  }
  return cache.get(name);
}
export function functionOptionsMarkdown(name) {
  const cell = (text) => text.replaceAll('|', '\\|').replaceAll('\n', ' ');
  return ['| Option | What it does |', '| --- | --- |', ...functionOptions(name).map((row) => `| ${inlineCode(cell(row.option))} | ${cell(row.description)} |`)].join('\n') + '\n\n[Shared options](/reference/options/)';
}

// The Options reference lists only arguments/options admitted by every function.
let shared;
export function sharedOptions() {
  if (!shared) {
    const commands = functions.map((name) => sections(execFileSync(path.resolve(product.binDir, product.binary), [name, '--help'], {
      encoding: 'utf8', timeout: 30000, env: { ...process.env, NO_COLOR: '1', COLUMNS: '200' },
    })));
    const common = commands[0].options.filter(({ flag }) => commands.every((command) => command.options.some((option) => option.flag === flag)));
    const args = commands[0].args.filter(({ name }) => commands.every((command) => command.args.some((arg) => arg.name === name)));
    shared = [...args.map((arg) => ({ ...arg, option: arg.name })), ...common.map((option) => ({
      ...option, option: [option.short, `${option.flag}${option.value ? ` ${option.value}` : ''}`].filter(Boolean).join(', '),
    }))].map((option) => ({
      option: option.option,
      description: [option.description, option.values && `One of: ${option.values}.`, option.default && `Default: ${option.default}.`].filter(Boolean).join(' '),
      variable: option.env || 'none',
    }));
  }
  return shared;
}
export function sharedOptionsMarkdown() {
  const cell = (text) => text.replaceAll('|', '\\|').replaceAll('\n', ' ');
  return ['| Option | What it does | Variable |', '| --- | --- | --- |', ...sharedOptions().map((row) => `| ${inlineCode(cell(row.option))} | ${cell(row.description)} | ${cell(row.variable)} |`)].join('\n');
}
