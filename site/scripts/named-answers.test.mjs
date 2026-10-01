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

  ['direct', 'cpp', 'pass', 'auto is_refund = tt::decide(engine, q, m);\nassert(is_refund.value.outcome == tt::Outcome::yes);', []],
  ['direct', 'cpp', 'fail', 'assert(tt::decide(engine, q, m).value.outcome\n       == tt::Outcome::yes);', [[1, 'direct']]],
  ['direct', 'cpp', 'fail', 'std::cout << tt::decide(engine, q, m).ok;', [[1, 'direct']]],
  ['direct', 'cpp', 'pass', 'const char *q = R"({"decide": "Refund?"})";\nauto is_refund = tt::decide(engine, q, m);', []],
  ['generic', 'cpp', 'pass', 'is_refund = tt::decide(\n    engine, q, m);', []],
  ['generic', 'cpp', 'fail', 'auto result = tt::decide(engine, q, m);', [[1, 'generic']]],
  ['direct', 'objc', 'pass', '[client decide:q\n          text:m\n        answer:&is_refund];\nassert(is_refund.outcome == TTOutcomeYes);', []],
  ['direct', 'objective-c', 'fail', 'if ([client decide:q text:m answer:&is_refund] == TTErrorNone) {', [[1, 'direct']]],
  ['generic', 'objective-c', 'fail', '[client decide:q\n          text:m\n        answer:&answer];', [[1, 'generic']]],
  ['generic', 'objc', 'pass', 'TTDecision is_refund = {0};', []],
  ['direct', 'cobol', 'pass', 'call "TT-DECIDE" using engine\n    question tt-answer.\nif not outcome-yes\n    call "TT-DECIDE" using engine question tt-answer\nend-if', []],
  ['direct', 'cobol', 'pass', "move '(call \"TT-DECIDE\" using x)' to question", []],
  ['generic', 'cobol', 'fail', 'call "TT-DECIDE" using engine\n    question question-length\n    result tt-facts.', [[1, 'generic']]],
  ['generic', 'cobol', 'pass', 'call "tt-decide" using engine question tt-answer.', []],
  ['direct', 'ada', 'pass', 'Decide\n  (Client,\n   Question,\n   Is_Refund);\npragma Assert (Is_Refund.Value = Yes);', []],
  ['direct', 'ada', 'pass', 'if Ready then\n   Decide (Client, Question, Is_Refund);\nend if;', []],
  ['direct', 'ada', 'fail', 'pragma Assert\n  (Decide (Client, Question) = Yes);', [[1, 'direct']]],
  ['direct', 'ada', 'fail', 'Put_Line (Decide (Client, Question));', [[1, 'direct']]],
  ['generic', 'ada', 'fail', 'Decide\n  (Client,\n   Question,\n   Result);', [[1, 'generic']]],
  ['generic', 'ada', 'pass', 'Refund : constant String :=\n  "{""decide"": ""Refund?""}";', []],

  ['direct', 'java', 'pass', 'var isRefund = tt.decide(\n    question,\n    text);\nassert Door.outcome(isRefund.value()) == Outcome.YES;', []],
  ['direct', 'java', 'fail', 'assert Door.outcome(tt.decide(q, m).value())\n    == Outcome.YES;', [[1, 'direct']]],
  ['direct', 'java', 'fail', 'System.out.println(tt.decide(q, m));', [[1, 'direct']]],
  ['generic', 'java', 'fail', 'var result = tt.decide(q, m);', [[1, 'generic']]],
  ['generic', 'java', 'pass', 'TypedResult<Answer> isRefund =\n    tt.decide(q, m);', []],
  ['direct', 'kotlin', 'pass', 'var isRefund = tt.decide(\n    question,\n    text,\n)\ncheck(Door.outcome(isRefund.value()) == Outcome.YES)', []],
  ['direct', 'kotlin', 'fail', 'check(tt.decide(q, m).value().outcome() == 1)', [[1, 'direct']]],
  ['generic', 'kotlin', 'fail', 'val answer = tt.decide(q, m)', [[1, 'generic']]],
  ['generic', 'kotlin', 'pass', 'val refund = """\n    {"decide": "tt.decide(x)"}\n""".trimIndent()', []],
  ['direct', 'scala', 'pass', 'val isRefund = tt.decide(q, m)\nassert(Door.outcome(isRefund.value()) == Outcome.YES)', []],
  ['direct', 'scala', 'fail', 'assert(\n  tt.decide(q, m).value().outcome() == 1\n)', [[1, 'direct']]],
  ['generic', 'scala', 'fail', 'var res = tt.decide(\n  q,\n  m\n)', [[1, 'generic']]],
  ['direct', 'csharp', 'pass', 'var isRefund = tt.Decide(\n    question,\n    text);\nTrace.Assert(isRefund.Value.OutcomeKind == Outcome.Yes);', []],
  ['direct', 'cs', 'fail', 'Trace.Assert(tt.Decide(q, m).Value.OutcomeKind\n    == Outcome.Yes);', [[1, 'direct']]],
  ['direct', 'csharp', 'fail', 'Console.WriteLine(tt.Decide(q, m).Value);', [[1, 'direct']]],
  ['generic', 'csharp', 'fail', 'var response = tt.Decide(q, m);', [[1, 'generic']]],
  ['generic', 'csharp', 'pass', 'const string refund = """\n    {"decide": "Refund?"}\n    """;', []],
  ['direct', 'go', 'pass', 'isRefund, err := tt.Decide(\n\tctx,\n\tq,\n\tm,\n)\nif isRefund.Value.Outcome != thinkthen.Yes {', []],
  ['direct', 'go', 'fail', 'if r, _ := tt.Decide(ctx, q, m); r.Value.Outcome == 1 {', [[1, 'direct']]],
  ['direct', 'go', 'fail', 'fmt.Println(tt.Decide(ctx, q, m))', [[1, 'direct']]],
  ['generic', 'go', 'fail', 'result, err := tt.Decide(ctx, q, m)', [[1, 'generic']]],
  ['generic', 'go', 'pass', 'refund := `{"decide": "tt.Decide(x)"}`', []],
  ['direct', 'swift', 'pass', 'var isRefund = try tt.decide(\n    q,\n    m\n)\nprecondition(isRefund.value.outcome == .yes)', []],
  ['direct', 'swift', 'fail', 'precondition(try tt.decide(q, m).value.outcome == .yes)', [[1, 'direct']]],
  ['generic', 'swift', 'fail', 'let answer = try tt.decide(q, m)', [[1, 'generic']]],
  ['direct', 'zig', 'pass', 'const asked = try tt.decide(q, m, .{});\nstd.debug.assert(is_refund.value.outcome == .yes);', []],
  ['direct', 'zig', 'fail', 'std.debug.assert((try tt.decide(q, m, .{})).ok.value.outcome == .yes);', [[1, 'direct']]],
  ['generic', 'zig', 'fail', 'const result = try tt.decide(\n    q,\n    m,\n    .{},\n);', [[1, 'generic']]],
  ['direct', 'php', 'pass', "$isRefund = $tt->decide(\n    $question,\n    $text,\n);\nassert($isRefund['value']['outcome'] === ThinkThen::YES);", []],
  ['direct', 'php', 'fail', "assert($tt->decide($q, $m)['value']['outcome'] === 1);", [[1, 'direct']]],
  ['direct', 'php', 'fail', 'echo json_encode($tt->decide($q, $m));', [[1, 'direct']]],
  ['generic', 'php', 'fail', '$result = $tt->decide(\n    $q,\n    $m,\n);', [[1, 'generic']]],
  ['generic', 'php', 'pass', "$refund = '{\"decide\": \"$tt->decide(x)\"}';", []],
  ['direct', 'dart', 'pass', 'final isRefund = tt.decide(\n  engine,\n  q,\n  m,\n);\nassert(isRefund.value.outcome == Outcome.yes);', []],
  ['direct', 'dart', 'fail', 'assert(tt.decide(engine, q, m).value.outcome == Outcome.yes);', [[1, 'direct']]],
  ['direct', 'dart', 'fail', 'print(tt.many(engine, q, texts).value);', [[1, 'direct']]],
  ['generic', 'dart', 'fail', 'final answer = tt.decide(\n  engine,\n  q,\n  m,\n);', [[1, 'generic']]],
  ['generic', 'dart', 'pass', "const refund = '{\"decide\": \"tt.decide(x)\"}';", []],

  ['direct', 'bash', 'pass', 'is_spam=$(thinkthen decide "$q" < mail.txt)\nif [ "$is_spam" = yes ]; then\n  echo spam\nfi', []],
  ['direct', 'bash', 'fail', 'if thinkthen decide "$q" < mail.txt; then\n  echo spam\nfi', [[1, 'direct']]],
  ['direct', 'bash', 'fail', 'echo "$(thinkthen decide "$q" < mail.txt)"', [[1, 'direct']]],
  ['direct', 'bash', 'pass', 'thinkthen filter "$q" < mail.jsonl |\nwhile read -r line; do\n  echo "$line"\ndone', []],
  ['direct', 'bash', 'pass', 'printf \'%s\' "$(jq -r .body m.jsonl)" | thinkthen decide "$q"', []],
  ['direct', 'bash', 'pass', 'printf \'x\' | thinkthen recognize --replay "$(git rev-parse --show-toplevel)/r"', []],
  ['direct', 'bash', 'pass', 'printf \'%s\' "$(jq -r .body m.jsonl)" \\\n  | thinkthen decide "$q"', []],
  ['direct', 'bash', 'fail', 'test -z "$(printf x | thinkthen decide "$q")"', [[1, 'direct']]],
  ['direct', 'bash', 'fail', '[ "$(thinkthen decide "$q" < m.txt)" = yes ]', [[1, 'direct']]],
  ['direct', 'bash', 'fail', '[[ $(thinkthen decide "$q" < m.txt) == yes ]]', [[1, 'direct']]],
  ['direct', 'bash', 'pass', '[ "$(jq -r .id m.json)" = C-01 ]', []],
  ['direct', 'bash', 'fail', 'case "$(thinkthen choose "$q" billing shipping --raw < m.txt)" in\n  billing) route billing ;;\nesac', [[1, 'direct']]],
  ['direct', 'bash', 'pass', 'team=$(thinkthen choose "$q" billing shipping --raw < m.txt)\ncase "$team" in\n  billing) route billing ;;\nesac', []],
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
const ACCEPTED = 'Accepted names: bash, sh, python, typescript, ts, ruby, r, rust, c, cpp, objective-c, objc, cobol, ada, java, kotlin, scala, csharp, cs, go, swift, zig, php, dart, sql.';
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
  ['spaced fence', `\`\`\` python\n${bad}\n\`\`\``, 1, /direct assert|acts on a ThinkThen call/],
  ['braced fence', `\`\`\`{.python}\n${bad}\n\`\`\``, 1, /\{\.python\} is not a kind/],
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
