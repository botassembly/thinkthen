// The named-answer rule, shared by the site's sample check and the
// builder's lint rung.
//
// Contract. Changing the signature, the problem shape, the rule ids, the
// accepted languages, or the unknown-language behavior needs notice to the
// ThinkThen queue owner first. The builder's lint rung imports this file.
//
//   namedAnswerProblems(code, language) -> Array<{ line, rule, message }>
//
//   code      The source text of one example or code block, as a string.
//   language  One of the names below. Case does not matter.
//   line      The 1-based line in `code` where the problem starts.
//   rule      One of four ids:
//               direct     an assert, print, if, or while acts on a
//                          ThinkThen call, a Bash until does, or a Bash
//                          test, [, [[, echo, or printf reads the call's
//                          output in place, through a $(...) that holds
//                          the call
//               unnamed    a SQL call has no alias
//               generic    an answer or exit code takes a generic name
//               bare-exit  Bash reads $? in case, test, if, while, [, or [[
//   message   One short sentence or two that say how to fix it.
//
// The array is empty when the code keeps the rule. Problems come in line
// order, and several may share a line.
//
// Accepted languages: bash, sh, python, typescript, ts, ruby, r, rust, c,
// and sql. sh is bash, and ts is typescript. An unknown language throws a
// TypeError. Its message names the bad language and lists the accepted
// names. A name that every JavaScript object inherits, such as constructor
// or __proto__, is unknown too. A caller skips text that holds no
// ThinkThen call, such as JSON or program output, before it calls.
//
// Generic names, compared without case: result, results, answer, answers,
// output, outputs, out, res, ret, response, responses, value, values,
// data, found, kept, ranked, pick, picks, hits, matches, scores, tags,
// edges, forms, tmp, x, y, n, r, v.
//
// The function is pure. It reads no file, makes no network call, keeps no
// state between calls, and imports nothing.

const FNS = 'decide|choose|score|tag|filter|rank|find|annotate|recognize|relate';
const LANGUAGE = {
  bash: 'bash', sh: 'bash', python: 'python', typescript: 'typescript',
  ts: 'typescript', ruby: 'ruby', r: 'r', rust: 'rust', c: 'c', sql: 'sql',
};
const CALL = {
  python: new RegExp(`\\btt\\s*\\.(${FNS})\\(`),
  typescript: new RegExp(`\\btt\\s*\\.(${FNS})\\(`),
  rust: new RegExp(`\\btt\\s*\\.(${FNS})\\(`),
  ruby: new RegExp(`\\bThinkThen\\.(${FNS})\\(`),
  r: new RegExp(`\\btt_(${FNS})\\(`),
  c: new RegExp(`\\bthinkthen_(call|${FNS})\\(`),
  bash: new RegExp(`\\bthinkthen\\s+(${FNS})\\b`),
};
const SQL_CALL = new RegExp(`\\bthinkthen_(${FNS}|probability|relations)\\s*\\(`, 'g');
const GENERIC = new Set([
  'result', 'results', 'answer', 'answers', 'output', 'outputs', 'out',
  'res', 'ret', 'response', 'responses', 'value', 'values', 'data', 'found',
  'kept', 'ranked', 'pick', 'picks', 'hits', 'matches', 'scores', 'tags',
  'edges', 'forms', 'tmp', 'x', 'y', 'n', 'r', 'v',
]);
const PRINT = /\b(print\(|console\.log\(|puts\b|println!|printf\(|cat\()/;
const ASSIGN = {
  python: /^\s*([A-Za-z_]\w*)\s*=(?!=)/,
  ruby: /^\s*([A-Za-z_]\w*)\s*=(?!=)/,
  r: /^\s*([A-Za-z_.][\w.]*)\s*(?:<-|=(?!=))/,
  typescript: /^\s*(?:const|let|var)\s+([A-Za-z_$][\w$]*)\s*=/,
  rust: /^\s*(?:let\s+(?:mut\s+)?([A-Za-z_]\w*)\s*(?::[^=]+)?=|for\s+\(?([\w, ]+?)\)?\s+in\b)/,
  c: /^\s*(?:const\s+)?[A-Za-z_]\w*\s*\*?\s*([A-Za-z_]\w*)\s*(?:=|;)/,
};
const ASSERT = {
  python: /^\s*assert\b/,
  typescript: /^\s*assert(\.\w+)?\(/,
  ruby: /\braise\s+(unless|if)\b/,
  r: /^\s*stopifnot\(/,
  rust: /^\s*(debug_)?assert(_eq|_ne)?!\(/,
  c: /^\s*assert\(/,
};
const SQL_WORDS = new Set(['from', 'where', 'order', 'group', 'is', 'and', 'or', 'not', 'in', 'desc', 'asc', 'limit', 'on', 'join', 'union', 'having', 'select', 'with', 'case', 'when', 'then', 'else', 'end']);

const generic = (name) => GENERIC.has(name.toLowerCase());
const named = (name) => `the answer is named ${name}. Name it for its meaning.`;

// Blank out string literals, so brackets and words inside them do not count.
function unquote(text, lang) {
  const single = ['python', 'ruby', 'r', 'typescript', 'sql'].includes(lang);
  let out = '';
  let quote = null;
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (quote) {
      if (c === '\\' && quote === '"') { i += 1; continue; }
      if (c === quote) { out += c; quote = null; }
      else if (c === '\n') out += c;
      continue;
    }
    if (c === '"' || (single && c === "'")) quote = c;
    out += c;
  }
  return out;
}

// The lines from `start` to the end of the statement that begins there.
function statement(lines, start, lang) {
  let depth = 0;
  for (let j = start; j < lines.length; j++) {
    const line = lines[j];
    depth += (line.match(/[([]/g) || []).length - (line.match(/[)\]]/g) || []).length;
    if (lang === 'ruby') depth += (line.match(/\bdo\b/g) || []).length - (line.match(/^\s*end\b/g) || []).length;
    const tail = line.trimEnd();
    const more = ['typescript', 'rust', 'c'].includes(lang)
      ? !tail.endsWith(';') && !tail.endsWith('{') && !tail.endsWith('}')
      : /(\\|[+,=.]|<-|\|\||&&)$/.test(tail);
    if (depth <= 0 && !more) return lines.slice(start, j + 1).join('\n');
  }
  return lines.slice(start).join('\n');
}

function sql(text) {
  const found = [];
  const bare = unquote(text, 'sql');
  for (const m of bare.matchAll(SQL_CALL)) {
    let depth = 0;
    let i = m.index + m[0].length - 1;
    for (; i < bare.length; i++) {
      if (bare[i] === '(') depth += 1;
      else if (bare[i] === ')' && --depth === 0) break;
    }
    // A call wrapped in another function, such as unnest(), closes that first.
    const wraps = (/((?:\w+\s*\(\s*)+)$/.exec(bare.slice(0, m.index))?.[1].match(/\(/g) || []).length;
    let rest = bare.slice(i + 1);
    for (let w = 0; w < wraps; w++) rest = rest.replace(/^\s*\)/, '');
    const alias = /^\s*(?:AS\s+)?([A-Za-z_]\w*)/i.exec(rest);
    const line = bare.slice(0, m.index).split('\n').length;
    if (!alias || SQL_WORDS.has(alias[1].toLowerCase())) {
      found.push({ line, rule: 'unnamed', message: `thinkthen_${m[1]} has no name. Give its answer an alias named for its meaning, and filter or sort on that name.` });
    } else if (generic(alias[1])) {
      found.push({ line, rule: 'generic', message: named(alias[1]) });
    }
  }
  return found;
}

// The text inside each $(...), nested ones included.
function substitutions(text) {
  const found = [];
  for (let at = text.indexOf('$('); at !== -1; at = text.indexOf('$(', at + 2)) {
    let depth = 0;
    let i = at + 1;
    for (; i < text.length; i++) {
      if (text[i] === '(') depth += 1;
      else if (text[i] === ')' && --depth === 0) break;
    }
    found.push(text.slice(at + 2, i));
  }
  return found;
}

function bash(text) {
  const found = [];
  const call = CALL.bash;
  const lines = [];
  let until = null;
  for (const line of text.split('\n')) {
    if (until) { lines.push(''); if (line.trim() === until) until = null; continue; }
    const doc = /<<-?\s*'?(\w+)'?/.exec(line);
    if (doc) until = doc[1];
    lines.push(line.replace(/'[^']*'/g, "''"));
  }
  // A Bash command runs on while a line ends in a pipe or a backslash.
  const piped = (i) => {
    let j = i;
    while (j + 1 < lines.length && /(\||\\)\s*$/.test(lines[j])) j += 1;
    return lines.slice(i, j + 1).join('\n');
  };
  lines.forEach((line, i) => {
    const at = i + 1;
    const capture = /^\s*([A-Za-z_]\w*)=\$\(/.exec(line);
    const code = /^\s*([A-Za-z_]\w*)=\$\?/.exec(line);
    const fn = /^\s*([A-Za-z_]\w*)\s*\(\)\s*\{/.exec(line);
    if (capture || fn) {
      let depth = 0;
      let j = i;
      for (; j < lines.length; j++) {
        depth += (lines[j].match(fn ? /\{/g : /\(/g) || []).length - (lines[j].match(fn ? /\}/g : /\)/g) || []).length;
        if (depth <= 0) break;
      }
      const name = (capture || fn)[1];
      if (call.test(lines.slice(i, j + 1).join('\n')) && generic(name)) found.push({ line: at, rule: 'generic', message: named(name) });
    }
    if (code && generic(code[1])) found.push({ line: at, rule: 'generic', message: `the exit code is named ${code[1]}. Name it for its meaning.` });
    if (/^\s*(case|test|if|while|\[)\b.*\$\?/.test(line) || /^\s*\[\[?\s.*\$\?/.test(line)) {
      found.push({ line: at, rule: 'bare-exit', message: 'reads $? directly. Name the exit code first, such as refund_code=$?.' });
    }
    if (/^\s*(if|while|until)\s/.test(line) && !/^\s*while\s+read\b/.test(line) && call.test(piped(i))) {
      found.push({ line: at, rule: 'direct', message: 'a branch acts on a thinkthen call. Wrap the call in a function named for its meaning, or name the answer first.' });
    }
    if (/^\s*(test|\[\[?|echo|printf)\b/.test(line) && /\$\(/.test(line)) {
      if (substitutions(statement(lines, i, 'python')).some((inner) => call.test(inner))) found.push({ line: at, rule: 'direct', message: 'an assert or print acts on a thinkthen call. Name the answer first.' });
    }
  });
  return found;
}

function library(text, lang) {
  const found = [];
  const call = CALL[lang];
  const lines = unquote(text, lang).split('\n');
  const assign = ASSIGN[lang];
  const assert = ASSERT[lang];
  lines.forEach((line, i) => {
    const at = i + 1;
    if ((assert.test(line) || PRINT.test(line) || /^\s*(if|while)\b/.test(line)) && call.test(statement(lines, i, lang))) {
      found.push({ line: at, rule: 'direct', message: 'an assert, print, or branch acts on a ThinkThen call. Name the answer first, then assert on the name.' });
    }
    const a = assign.exec(line);
    if (a && call.test(statement(lines, i, lang))) {
      for (const name of (a[1] || a[2]).split(',').map((s) => s.trim()).filter(Boolean)) {
        if (generic(name)) found.push({ line: at, rule: 'generic', message: named(name) });
      }
    }
    if (lang === 'c' && call.test(line)) {
      for (const [, name] of statement(lines, i, lang).matchAll(/&\s*([A-Za-z_]\w*)/g)) {
        if (generic(name)) found.push({ line: at, rule: 'generic', message: named(name) });
      }
    }
  });
  return found;
}

export function namedAnswerProblems(code, language) {
  const key = String(language).toLowerCase();
  if (!Object.hasOwn(LANGUAGE, key)) {
    throw new TypeError(`unknown language ${JSON.stringify(String(language))}. Accepted names: ${Object.keys(LANGUAGE).join(', ')}.`);
  }
  const lang = LANGUAGE[key];
  if (lang === 'sql') return sql(code);
  if (lang === 'bash') return bash(code);
  return library(code, lang);
}
