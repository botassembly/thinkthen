// Function options come from the exact program used to build the site.
import { execFileSync } from 'node:child_process';
import path from 'node:path';
import product from '../../product.config.mjs';
import { functions } from './function-surfaces.mjs';
import { sections } from './help.mjs';
import { inlineCode } from './examples.mjs';

const cache = new Map();
export function functionOptions(name, scope = 'function') {
  if (!['function', 'command'].includes(scope)) throw new Error(`Unknown help scope: ${scope}`);
  const complete = scope === 'command';
  if (!complete && !functions.includes(name)) throw new Error(`Unknown function options: ${name}`);
  const key = `${scope}:${name}`;
  if (!cache.has(key)) {
    const help = (...args) => sections(execFileSync(path.resolve(product.binDir, product.binary), args, {
      encoding: 'utf8', timeout: 30000, env: { ...process.env, NO_COLOR: '1', COLUMNS: '200' },
    }));
    const top = help('--help');
    if (complete && !top.commands.some((command) => command.name === name)) throw new Error(`Unknown command help: ${name}`);
    const globals = new Set(top.options.map(({ flag }) => flag));
    const command = help(name, '--help');
    const own = complete ? command.options : command.options.filter(({ flag }) => !globals.has(flag) && !['--help', '--version'].includes(flag));
    cache.set(key, [...command.args.map((arg) => ({ option: arg.name, origin: 'Command argument', ...arg })), ...own.map((option) => ({
      origin: globals.has(option.flag) ? 'Inherited global flag' : 'Command option',
      ...option, option: [option.short, `${option.flag}${option.value ? ` ${option.value}` : ''}`].filter(Boolean).join(', '),
    }))].map((option) => ({
      option: option.option, origin: option.origin,
      description: [option.description, option.values && `One of: ${option.values}.`, option.default && `Default: ${option.default}.`, complete && option.env && `Variable: ${option.env}.`].filter(Boolean).join(' '),
    })));

  }
  return cache.get(key);
}
export function functionOptionsMarkdown(name, scope = 'function') {
  const cell = (text) => text.replaceAll('|', '\\|').replaceAll('\n', ' ');
  const complete = scope === 'command';
  return [complete ? '| Option | Current program help | In help |' : '| Option | What it does |', complete ? '| --- | --- | --- |' : '| --- | --- |', ...functionOptions(name, scope).map((row) => `| ${inlineCode(cell(row.option))} | ${complete && row.description ? inlineCode(cell(row.description)) : cell(row.description)} |${complete ? ` ${row.origin} |` : ''}`)].join('\n') + (complete ? '' : '\n\n[Shared options](/reference/options/)');
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
