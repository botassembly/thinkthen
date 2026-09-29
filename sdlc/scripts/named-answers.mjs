#!/usr/bin/env node
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { namedAnswerProblems } from '../../site/scripts/named-answers.mjs';

const folders = ['demos', 'specification', 'spec', 'libraries', 'databases'];
const skipped = new Set(['', 'text', 'console', 'json', 'toml']);

// A pubspec source dependency is configuration data, not an executable sample.
// Other YAML remains unknown, including YAML in another page or with extra keys.
function pubspecDependency(path, body) {
  return path === 'libraries/dart/README.md' &&
    /^dependencies:\n  thinkthen_dart:\n    path: \/[A-Za-z0-9_./-]+$/.test(body);
}

// The site rule owns the policy. Bridge only Dart's call/assignment spelling to
// its supported TypeScript spelling; blank literals/comments before replacing
// tokens, preserving newlines for the shared rule's line numbers. Interpolation
// can execute code inside a string, so refuse it until this bridge supports it.
function dartForShared(source) {
  let code = '';
  let interpolation = null;
  let block = 0;
  let line = 1;
  const blank = (char) => char === '\n' ? '\n' : ' ';
  for (let i = 0; i < source.length;) {
    const char = source[i];
    if (char === '\n') line += 1;
    if (block) {
      if (source.startsWith('/*', i)) { block += 1; code += '  '; i += 2; continue; }
      if (source.startsWith('*/', i)) { block -= 1; code += '  '; i += 2; continue; }
      code += blank(char); i += 1; continue;
    }
    if (source.startsWith('//', i)) {
      const end = source.indexOf('\n', i);
      if (end < 0) { code += ' '.repeat(source.length - i); break; }
      code += ' '.repeat(end - i); i = end; continue;
    }
    if (source.startsWith('/*', i)) { block = 1; code += '  '; i += 2; continue; }
    if (char === "'" || char === '"') {
      const width = source.startsWith(char.repeat(3), i) ? 3 : 1;
      const raw = i > 0 && /[rR]/.test(source[i - 1]) && (i === 1 || !/[\w$]/.test(source[i - 2]));
      code += ' '.repeat(width); i += width;
      while (i < source.length && !source.startsWith(char.repeat(width), i)) {
        if (source[i] === '\n') line += 1;
        if (!raw && source[i] === '$' && /[A-Za-z_{]/.test(source[i + 1] || '')) interpolation ??= line;
        if (source[i] === '\\' && i + 1 < source.length) {
          code += blank(source[i]) + blank(source[i + 1]);
          if (source[i + 1] === '\n') line += 1;
          i += 2; continue;
        }
        code += blank(source[i]); i += 1;
      }
      if (i === source.length) return { code, unsupported: line };
      code += ' '.repeat(width); i += width;
      continue;
    }
    code += char; i += 1;
  }
  if (block) return { code, unsupported: line };
  const calls = /\b[A-Za-z_]\w*[ \t]*\.[ \t]*(ask|decide|many|recognize|relate)[ \t]*\(/g;
  code = code.replace(calls, (_, method) => `tt.${method === 'many' || method === 'ask' ? 'decide' : method}(`);
  code = code.replace(/\bfinal\b/g, 'const');
  return { code, unsupported: interpolation };
}

function pages() {
  return execFileSync('git', ['ls-files', '-co', '--exclude-standard', '--', ...folders], { encoding: 'utf8' })
    .trim().split('\n').filter((path) => path.endsWith('.md'))
    .map((path) => [path, readFileSync(path, 'utf8')]);
}

function scan(path, source) {
  const problems = [];
  let blocks = 0;
  let ignored = 0;
  let opened = null;
  let body = [];
  source.split('\n').forEach((line, index) => {
    if (!opened) {
      const match = /^\s*(`{3,}|~{3,})(.*)$/.exec(line);
      if (match) {
        opened = { line: index + 1, mark: match[1][0], width: match[1].length, tag: match[2].trim().split(/\s+/)[0].toLowerCase() };
        body = [];
      }
      return;
    }
    const close = /^\s*(`+|~+)\s*$/.exec(line);
    if (!close || close[1][0] !== opened.mark || close[1].length < opened.width) {
      body.push(line);
      return;
    }
    const contents = body.join('\n');
    if (skipped.has(opened.tag) || (opened.tag === 'yaml' && pubspecDependency(path, contents))) ignored += 1;
    else {
      try {
        const bridged = opened.tag === 'dart' ? dartForShared(contents) : null;
        namedAnswerProblems('', bridged ? 'typescript' : opened.tag);
        blocks += 1;
        if (bridged?.unsupported) {
          problems.push(`named answers: ${path}:${opened.line + bridged.unsupported}: unsupported: Dart string interpolation or an unclosed literal/comment needs a named answer outside that construct`);
        }
        for (const problem of namedAnswerProblems(bridged ? bridged.code : contents, bridged ? 'typescript' : opened.tag)) {
          problems.push(`named answers: ${path}:${opened.line + problem.line}: ${problem.rule}: ${problem.message}`);
        }
      } catch {
        problems.push(`named answers: ${path}:${opened.line}: the rule reads no \`${opened.tag}\` block; tag data or output as text, console, json, or toml`);
      }
    }
    opened = null;
  });
  if (opened) problems.push(`named answers: ${path}:${opened.line}: this fence never closes`);
  return { problems, blocks, ignored };
}

function check(entries) {
  const lines = [];
  let blocks = 0;
  let ignored = 0;
  for (const [path, source] of entries) {
    const result = scan(path, source);
    lines.push(...result.problems);
    blocks += result.blocks;
    ignored += result.ignored;
  }
  return { lines, blocks, ignored };
}

function selfTest() {
  const real = pages();
  const before = check(real).lines;
  const plants = [
    ['demos', 'bash', "if thinkthen decide 'Q?' --quiet < m.txt; then echo yes; fi", 'direct'],
    ['specification', 'sh', "thinkthen decide 'Q?' --quiet < m.txt\ncase $? in", 'bare-exit'],
    ['spec', 'bash', "answer=$(thinkthen decide 'Q?' < m.txt)", 'generic'],
    ['libraries', 'python', 'assert tt.decide(question, message)', 'direct'],
    ['databases', 'sql', "SELECT id FROM t WHERE thinkthen_decide('Q?', body);", 'unnamed'],
  ];
  for (const [folder, tag, code, rule] of plants) {
    const picked = real.find(([path]) => path.startsWith(`${folder}/`));
    if (!picked) throw new Error(`no page under ${folder}/`);
    const [path, source] = picked;
    const line = source.split('\n').length + 2;
    const planted = `${source}\n\n\`\`\`${tag}\n${code}\n\`\`\`\n`;
    const after = check(real.map(([name, text]) => [name, name === path ? planted : text])).lines;
    const added = after.filter((item) => !before.includes(item));
    const at = line + (rule === 'bare-exit' ? 2 : 1);
    if (after.length !== before.length + 1 || added.length !== 1 ||
        !added[0].startsWith(`named answers: ${path}:${at}: ${rule}: `) ||
        added[0] === `named answers: ${path}:${at}: ${rule}: `) {
      throw new Error(`folder plant failed under ${folder}/: ${added.join(' | ')}`);
    }
  }
  const plant = "answer=$(thinkthen decide 'Q?' < m.txt)";
  const rows = [
    ['skip tag', `\`\`\`text\n${plant}\n\`\`\``, []],
    ['no tag', `\`\`\`\n${plant}\n\`\`\``, []],
    ['tildes', `~~~bash\n${plant}\n~~~`, ['generic', 2]],
    ['long fence', `\`\`\`\`bash\n\`\`\`\n${plant}\n\`\`\`\``, ['generic', 3]],
    ['indented', `   \`\`\`bash\n${plant}\n   \`\`\``, ['generic', 2]],
    ['unknown tag', `\`\`\`yaml\n${plant}\n\`\`\``, ['unknown', 1]],
    ['unclosed', `\`\`\`bash\n${plant}`, ['unclosed', 1]],
  ];
  for (const [name, source, expected] of rows) {
    const lines = scan('plant.md', source).problems;
    const prefix = expected.length === 0 ? null : expected[0] === 'generic'
      ? `named answers: plant.md:${expected[1]}: generic: `
      : expected[0] === 'unknown'
        ? 'named answers: plant.md:1: the rule reads no `yaml` block; tag data or output as text, console, json, or toml'
        : 'named answers: plant.md:1: this fence never closes';
    if (lines.length !== (prefix ? 1 : 0) || (prefix && !(expected[0] === 'generic' ? lines[0].startsWith(prefix) && lines[0].length > prefix.length : lines[0] === prefix))) throw new Error(`${name}: ${lines}`);
  }
  const dartRows = [
    ['direct', 'print(door.ask(engine, {}));', 'direct'],
    ['generic', 'final result = door.ask(engine, {});', 'generic'],
    ['typed direct', 'if (door.decide(engine, "Q?", "text")) {}', 'direct'],
    ['named', 'final refundDecision = door.ask(engine, {});\nprint(refundDecision);', null],
    ['string', 'final note = "final result = door.ask(engine, {})";', null],
    ['line comment', '// final result = door.ask(engine, {});', null],
    ['block comment', '/* final result = door.ask(engine, {}); */', null],
    ['interpolation', 'print("${door.ask(engine, {})}");', 'unsupported'],
  ];
  for (const [name, code, rule] of dartRows) {
    const lines = scan('plant.md', `\`\`\`dart\n${code}\n\`\`\``).problems;
    if (lines.length !== (rule ? 1 : 0) || (rule && !lines[0].startsWith(`named answers: plant.md:2: ${rule}: `))) {
      throw new Error(`Dart ${name}: ${lines}`);
    }
  }
  const config = 'dependencies:\n  thinkthen_dart:\n    path: /source/libraries/dart';
  if (scan('libraries/dart/README.md', `\`\`\`yaml\n${config}\n\`\`\``).problems.length ||
      scan('plant.md', `\`\`\`yaml\n${config}\n\`\`\``).problems.length !== 1 ||
      scan('libraries/dart/README.md', `\`\`\`yaml\n${config}\n  run: door.ask()\n\`\`\``).problems.length !== 1) {
    throw new Error('pubspec YAML boundary changed');
  }
  const dartReadme = real.find(([path]) => path === 'libraries/dart/README.md');
  if (!dartReadme || scan(...dartReadme).problems.length) throw new Error('real Dart README is not clean');
  const genericReadme = dartReadme[1].replaceAll('decisionEnvelope', 'result');
  if (genericReadme === dartReadme[1] ||
      !scan(dartReadme[0], genericReadme).problems.some((line) => /: generic: /.test(line))) {
    throw new Error('real Dart call did not reach the shared generic-name rule');
  }
  console.log('named answers: self-test passed');
}

try {
  if (process.argv[2] === '--self-test') selfTest();
  else {
    const entries = pages();
    const result = check(entries);
    if (result.lines.length) {
      for (const line of result.lines) console.error(line);
      process.exitCode = 1;
    } else console.log(`named answers: ${result.blocks} code blocks in ${entries.length} pages name their answers; ${result.ignored} skipped by tag`);
  }
} catch (error) {
  console.error(`named answers: self-test: ${error.message}`);
  process.exitCode = 1;
}
