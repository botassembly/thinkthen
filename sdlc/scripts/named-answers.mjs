#!/usr/bin/env node
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { namedAnswerProblems } from '../../site/scripts/named-answers.mjs';

const folders = ['demos', 'specification', 'spec', 'libraries', 'databases'];
const skipped = new Set(['', 'text', 'console', 'json', 'toml']);

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
    if (skipped.has(opened.tag)) ignored += 1;
    else {
      try {
        namedAnswerProblems('', opened.tag);
        blocks += 1;
        for (const problem of namedAnswerProblems(body.join('\n'), opened.tag)) {
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
