#!/usr/bin/env node
// Edge cases for the named-answer contract in named-answers.mjs. The
// builder's lint rung depends on this contract, so the build runs it.
// Each row gives a language, the code, and the problems expected as
// [line, rule] pairs. Every rule has a pass row and a fail row in each
// language it covers. A throw row gives instead the name that the
// TypeError for an unknown language must quote.

import { namedAnswerProblems } from './named-answers.mjs';
import { articleProblems, NO_CALL } from './check-samples.mjs';

const CASES = [
  ['direct', 'python', 'pass', 'is_spam = tt.decide(question, message)\nassert is_spam', []],
  ['direct', 'python', 'fail', 'assert tt.decide(question, message)', [[1, 'direct']]],
  ['generic', 'python', 'pass', 'urgency = tt.score(question, message)', []],
  ['generic', 'python', 'fail', 'result = tt.decide(question, message)\nassert result', [[1, 'generic']]],

  ['direct', 'typescript', 'pass', 'const isSpam = tt.decide(question, message);\nassert(isSpam);', []],
  ['direct', 'typescript', 'fail', 'assert(tt.decide(question, message));', [[1, 'direct']]],
  ['generic', 'typescript', 'pass', 'const owners = tt.tag(question, message);', []],
  ['generic', 'typescript', 'fail', 'const answer = tt.decide(question, message);', [[1, 'generic']]],

  ['direct', 'ruby', 'pass', 'is_spam = ThinkThen.decide(question, message)\nraise unless is_spam', []],
  ['direct', 'ruby', 'fail', 'raise unless ThinkThen.decide(question, message)', [[1, 'direct']]],
  ['generic', 'ruby', 'pass', 'urgency = ThinkThen.score(question, message)', []],
  ['generic', 'ruby', 'fail', 'answer = ThinkThen.decide(question, message)', [[1, 'generic']]],

  ['direct', 'r', 'pass', 'is_spam <- tt_decide(question, message)\nstopifnot(is_spam)', []],
  ['direct', 'r', 'fail', 'stopifnot(tt_decide(question, message))', [[1, 'direct']]],
  ['generic', 'r', 'pass', 'urgency <- tt_score(question, message)', []],
  ['generic', 'r', 'fail', 'result <- tt_decide(question, message)', [[1, 'generic']]],

  ['direct', 'rust', 'pass', 'let is_spam = tt.decide(&question, &message)?;\nassert!(is_spam);', []],
  ['direct', 'rust', 'fail', 'assert!(tt.decide(&question, &message)?);', [[1, 'direct']]],
  ['generic', 'rust', 'pass', 'for (song, owners) in tt.tag(&question, &songs)? {', []],
  ['generic', 'rust', 'fail', 'let result = tt.decide(&question, &message)?;', [[1, 'generic']]],

  ['direct', 'c', 'pass', 'int is_spam;\nint code = thinkthen_decide(tt, q, m, &is_spam);\nassert(code == 0);', []],
  ['direct', 'c', 'fail', 'int is_spam;\nassert(thinkthen_decide(tt, q, m, &is_spam) == 0);', [[2, 'direct']]],
  ['generic', 'c', 'pass', 'int is_spam;\nint code = thinkthen_decide(tt, q, m, &is_spam);', []],
  ['generic', 'c', 'fail', 'int answer;\nint code = thinkthen_decide(tt, q, m, &answer);', [[2, 'generic']]],

  ['direct', 'bash', 'pass', 'is_spam=$(thinkthen decide "$q" < mail.txt)\nif [ "$is_spam" = yes ]; then\n  echo spam\nfi', []],
  ['direct', 'bash', 'fail', 'if thinkthen decide "$q" < mail.txt; then\n  echo spam\nfi', [[1, 'direct']]],
  ['direct', 'bash', 'fail', 'echo "$(thinkthen decide "$q" < mail.txt)"', [[1, 'direct']]],
  ['direct', 'bash', 'pass', 'thinkthen filter "$q" < mail.jsonl |\nwhile read -r line; do\n  echo "$line"\ndone', []],
  ['generic', 'bash', 'pass', 'is_spam=$(thinkthen decide "$q" < mail.txt)', []],
  ['generic', 'bash', 'fail', 'answer=$(thinkthen decide "$q" < mail.txt)', [[1, 'generic']]],
  ['generic', 'bash', 'pass', 'thinkthen decide "$q" < mail.txt\nrefund_code=$?', []],
  ['generic', 'bash', 'fail', 'thinkthen decide "$q" < mail.txt\nret=$?', [[2, 'generic']]],
  ['bare-exit', 'bash', 'pass', 'thinkthen decide "$q" < mail.txt\nrefund_code=$?\ncase $refund_code in', []],
  ['bare-exit', 'bash', 'fail', 'thinkthen decide "$q" < mail.txt\ncase $? in', [[2, 'bare-exit']]],

  ['unnamed', 'sql', 'pass', "SELECT id, thinkthen_decide('Spam?', body) AS is_spam FROM mail;", []],
  ['unnamed', 'sql', 'fail', "SELECT id FROM mail\nWHERE thinkthen_decide('Spam?', body);", [[2, 'unnamed']]],
  ['generic', 'sql', 'pass', "SELECT thinkthen_score('Urgent?', body) urgency FROM mail;", []],
  ['generic', 'sql', 'fail', "SELECT thinkthen_decide('Spam?', body) AS answer FROM mail;", [[1, 'generic']]],

  ['alias', 'sh', 'fail', 'answer=$(thinkthen decide "$q" < mail.txt)', [[1, 'generic']]],
  ['alias', 'ts', 'fail', 'assert(tt.decide(question, message));', [[1, 'direct']]],
  ['case', 'R', 'fail', 'stopifnot(tt_decide(question, message))', [[1, 'direct']]],
  ['unknown', 'json', 'throw', 'assert tt.decide(question, message)', 'json'],
  ['unknown', 'pyhton', 'throw', 'assert tt.decide(question, message)', 'pyhton'],
  ['unknown', 'constructor', 'throw', 'assert tt.decide(question, message)', 'constructor'],
  ['unknown', '__proto__', 'throw', 'assert tt.decide(question, message)', '__proto__'],
  ['unknown', 'toString', 'throw', 'assert tt.decide(question, message)', 'toString'],
  ['strings', 'python', 'pass', 'is_spam = tt.decide("assert tt.decide(x)", message)', []],
  ['lines', 'python', 'fail', 'import thinkthen as tt\n\nassert tt.decide(q, m)\nresult = tt.decide(q, m)', [[3, 'direct'], [4, 'generic']]],
];

// An unknown language throws a TypeError that names it and lists the
// accepted names. The expected value is the name the message must quote.
const ACCEPTED = 'Accepted names: bash, sh, python, typescript, ts, ruby, r, rust, c, sql.';
function thrown(code, language) {
  try {
    namedAnswerProblems(code, language);
  } catch (error) {
    return error;
  }
  return null;
}

// The site never hands a kind with no ThinkThen call to the function, and
// a fence tag it does not know fails the build. Each row gives an article
// and the number of problems check-samples finds in it.
const bad = 'assert tt.decide(q, m)';
const ARTICLES = [
  ['python fence', `\`\`\`python\n${bad}\n\`\`\``, 1],
  ['skipped fences', ['text', 'json', 'console', 'output', ''].map((t) => `\`\`\`${t}\n${bad}\n\`\`\``).join('\n'), 0],
  ['mistyped fence', `\`\`\`pyhton\n${bad}\n\`\`\``, 1, /pyhton is not a kind check-samples knows/],
  ['c++ fence', `\`\`\`c++\n${bad}\n\`\`\``, 1, /c\+\+ is not a kind/],
  ['inherited name', `\`\`\`constructor\n${bad}\n\`\`\``, 1, /constructor is not a kind/],
];

const failures = [];
for (const kind of ['.json', '.out', '.txt', 'text', 'json', 'console', 'output']) {
  if (!NO_CALL.has(kind)) failures.push(`skip list: ${kind} is missing from NO_CALL`);
}
for (const [label, text, count, message] of ARTICLES) {
  const problems = articleProblems('planted.md', text);
  if (problems.length !== count || (message && !message.test(problems[0]))) {
    failures.push(`check-samples ${label}: expected ${count} problems, got ${JSON.stringify(problems)}`);
  }
}
for (const [rule, language, kind, code, expected] of CASES) {
  if (kind === 'throw') {
    const error = thrown(code, language);
    if (!(error instanceof TypeError) || !error.message.includes(`"${expected}"`) || !error.message.includes(ACCEPTED)) {
      failures.push(`${rule} ${language}: expected a TypeError naming "${expected}", got ${error}`);
    }
    continue;
  }
  const problems = namedAnswerProblems(code, language);
  const got = problems.map((p) => [p.line, p.rule]);
  const shaped = problems.every((p) => Object.keys(p).join() === 'line,rule,message' && typeof p.message === 'string' && p.message.length > 0);
  if (JSON.stringify(got) !== JSON.stringify(expected) || !shaped) {
    failures.push(`${rule} ${language} ${kind}: expected ${JSON.stringify(expected)}, got ${JSON.stringify(problems)}`);
  }
}

if (failures.length) {
  console.error(`named-answers: ${failures.length} of ${CASES.length + ARTICLES.length} cases fail\n  ${failures.join('\n  ')}`);
  process.exit(1);
}
console.log(`named-answers: all ${CASES.length} cases and ${ARTICLES.length} check-samples rows keep the contract.`);
