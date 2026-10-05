// Shared reference tables and sample captions. Defaults and ranges come from specification/settings.md.

import { setting } from '../lib/settings-table.mjs';
import { cutOn, CUT, LIST_RULE } from './flags.mjs';

// ------------------------------------------------------------ thresholds

export { CUT };
export const DEFAULT_CUT = cutOn('decide');

// What one function's cut applies to, and its default. A function with no
// entry takes no threshold. Text marks code with backticks.
export const THRESHOLDS = {
  decide: { band: true, rule: 'A cut or a band. `decide` is the one function that takes both. The rule applies to the probability of yes.', dflt: cutOn('decide') },
  choose: { rule: 'A cut only. A band is a usage error. The cut applies to the winning option\'s probability.', dflt: cutOn('choose') },
  tag: { rule: 'A cut only, applied to each label\'s own probability of yes. A band is refused.', dflt: cutOn('tag') },
  filter: { rule: 'A cut only, applied to each record\'s probability of yes. A band is exit 2 from the command line and exit 5 from a question file.', dflt: cutOn('filter') },
  annotate: { band: true, rule: 'Each question in the set carries its own `threshold` key. A top-level `threshold` applies to every `decide` question that names none. One set can mix cuts and bands. A cut is a number, such as `"threshold": 0.8`. A band is a string, such as `"threshold": "0.1:0.9"`.', dflt: 'per question' },
  recognize: { rule: 'A cut on each name\'s printed strength. ThinkThen computes strength as a product. It is not a probability. `--relation-threshold` is a separate cut on each relation edge\'s probability of yes.', dflt: cutOn('recognize') },
  relate: { rule: 'A cut on each edge\'s probability of yes. An edge exactly at the cut is kept.', dflt: cutOn('relate') },
};

// ------------------------------------------------------------ arguments

// What each positional argument takes, past the signature in catalog.mjs.
const range = (name) => `${setting(name).range.min} to ${setting(name).range.max}`;
const TYPED = 'A value typed beside `@FILE` replaces the file\'s value.';
export const ARGUMENTS = {
  decide: `\`QUESTION\` is one argument that names one visible fact. \`@FILE\` reads a saved \`decide\` question from a question file instead. ${TYPED} A question that must begin with \`@\` is written in a file.`,
  choose: `\`QUESTION\` states what decides the pick, or \`@FILE\` names a question file. Each \`OPTION\` is one label, ${range('Options')} of them, sent in the order given. Keep the order fixed once a cut is tuned. ${LIST_RULE}`,
  tag: `\`QUESTION\` frames the labels, or \`@FILE\` names a question file. Each \`LABEL\` is judged on its own, ${range('Labels')} of them, in request and output order. ${LIST_RULE}`,
  score: `\`QUESTION\` names what is being placed, or \`@FILE\` names a question file. The levels come lowest first, ${range('Levels')} of them. The first level is 0. ${LIST_RULE}`,
  filter: `\`QUESTION\` is asked of each record. \`@FILE\` names a saved \`decide\` question instead. ${TYPED}`,
  rank: `\`QUESTION\` is asked of each record. \`@FILE\` names a saved \`decide\` or \`score\` question instead. A \`score\` file ranks by its weighted level position. \`rank\` refuses a threshold on the command line and in a question file.`,
  find: '`QUESTION` is the question the selected line or record best answers. It is one argument. Quote a question of several words.',
  annotate: '`FILE` is a saved question set: one JSON file of named questions. Each question names its verb, and it may carry its own threshold.',
  recognize: `Each \`KIND\` is one kind of name to look for, ${range('Kinds')} of them. With no kinds, every name has the kind \`ENTITY\`. \`none of these\`, \`ENTITY\` and \`ANY\` are reserved. \`@FILE\` names a \`recognize\` question file instead.`,
  relate: 'Each `RELATION` is one rule, written `NAME=SOURCE_KIND:TARGET_KIND`. A bare `NAME` relates any two kinds. `*` or `ANY` on a side means any kind. `@FILE` names a rules file instead, and it never mixes with inline rules.',
};

// ------------------------------------------------------------ input

const frameOn = (fn, rest) => setting('Framing').defaultOn(fn, { rest });
const cap = (s) => s.charAt(0).toUpperCase() + s.slice(1);
const ONE = (fn, what) => `${cap(frameOn(fn, true))} by default. The whole input is ${what}.`;

// What each framing flag makes one record.
export const FRAMING = {
  '--lines': 'Each line is one text record. A trailing newline ends the last record.',
  '--jsonl': 'Each line is one JSON value and one record. No blank lines.',
  '--csv': 'The first logical row is a header. Each later row becomes one JSON object of string cells.',
  '--tsv': 'The CSV rules, with a tab delimiter.',
};

// What each function reads when no framing flag is given.
export const FRAMING_DEFAULT = {
  decide: ONE('decide', 'one text and one record'),
  choose: ONE('choose', 'one text and one record'),
  tag: ONE('tag', 'one text and one record'),
  score: ONE('score', 'one text and one record'),
  annotate: ONE('annotate', 'one text or one JSON value'),
  recognize: ONE('recognize', 'one text'),
  filter: `${cap(frameOn('filter'))} by default, or JSON Lines when a pointer is given. One document is not a stream.`,
  rank: `${cap(frameOn('rank'))} by default, or JSON Lines when a pointer is given. One document is not a stream.`,
  find: `${cap(frameOn('find'))} by default, or \`--jsonl\`. CSV and TSV are refused as unexpected arguments. Every line or record goes out together in one request.`,
  relate: 'One JSON array of entities by default. Under `--jsonl`, `--csv` or `--tsv` each record is one entity of the same set. Under `--lines` each line is one name of kind `*`, and only bare or `*:*` rules apply.',
};

// ------------------------------------------------------------ output

// What standard output holds without --details.
export const BARE = {
  decide: '`true`, `false`, or `null`.',
  choose: 'A JSON string, or `null`. `--raw` prints the bare label.',
  tag: 'A JSON array of every label that reaches the bar. An empty array `[]` is a good answer.',
  score: 'A JSON number, from 0 to the number of levels minus one.',
  filter: 'Each kept line or JSONL record as it arrived, in input order. A kept table row prints as compact JSON.',
  rank: 'Every line or record again, most likely yes first. An exact tie keeps input order. With a saved `score` question, `rank @FILE` puts the highest score first.',
  find: 'The selected line or JSONL record as it arrived. Nothing when `--none` wins or ties.',
  annotate: 'One JSON object per record, with one field added per question.',
  recognize: 'One JSON object, `{"entities":[...]}`. Each name carries `text`, `start`, `end`, `length`, `kind` and `strength`. `start` and `end` count Unicode characters, and `end` is exclusive. With relation rules the object also holds `relations`, a list of edges.',
  relate: 'One compact edge per line, in rule and pair order. Each edge holds `relation`, `source`, `target` and `probability`. An edge of a both-ways rule ends with `"either":true`.',
};

// What a row looks like in record mode.
const VALUE_ROW = 'In record mode each row is `{"input":RECORD,"value":ANSWER}`, in that key order.';
export const RECORD = {
  decide: VALUE_ROW,
  choose: VALUE_ROW,
  tag: VALUE_ROW,
  score: VALUE_ROW,
  recognize: 'In record mode each row is `{"input":RECORD,"value":OBJECT}`, in input order.',
  filter: '`filter` returns records, not answers. CSV and TSV rows come back as JSON objects.',
  rank: '`rank` returns records, not answers. CSV and TSV rows come back as JSON objects.',
  find: '`find` returns the line or record it picked, not an answer.',
  annotate: 'In record mode an object record gains one top-level field per question. A line, a JSON scalar, or a JSON array keeps its parsed record under `input` and its named answers under `value`. CSV and TSV rows are objects, and they stay flat.',
  relate: '`relate` reads one whole set of entities. It prints edges, and no row per record.',
};

// ------------------------------------------------------------ limits

// The limits one function holds beyond the request limits every function shares.
const RECORD_CAP = 'A record is capped at 16 MiB of encoded bytes.';
const count = (name, noun) => `A run takes ${setting(name).range.min} to ${setting(name).range.max} ${noun}.`;
export const LIMITS = {
  decide: [RECORD_CAP],
  choose: [count('Options', 'options'), RECORD_CAP],
  tag: [count('Labels', 'labels'), RECORD_CAP],
  score: [count('Levels', 'levels'), RECORD_CAP],
  filter: [RECORD_CAP],
  rank: [RECORD_CAP, '`rank` holds every record until the input ends. Cut an endless stream into windows first.'],
  find: ['The set holds 2 to 255 lines or records, or 2 to 254 with `--none`. `find` reads at most 16 MiB of input in all.'],
  annotate: [RECORD_CAP, 'The questions and the evidence count together against the token limit. A long question set can pass the limit on evidence that fits on its own. Fewer questions per file is the answer.'],
  recognize: [`A text over ${setting('Recognize text limit').default} UTF-8 bytes exits 2 before any request. \`--max-text-bytes\` raises the limit.`, 'A relation plan admits at most 255 names and 4,000 pair questions.'],
  relate: ['A set holds at most 255 entities. A larger set exits 2 before any request.'],
};

// ------------------------------------------------------------ exit codes

// The signal codes every function shares.
export const SIGNALS = { code: '130, 143', what: 'SIGINT or SIGTERM stopped the command. It ends by that signal, and the shell reports 128 plus the signal number.' };

// The rule every list of exit codes repeats.
export const RUN_RULE = 'In record mode the exit code reports the run and never one record\'s answer.';

// Every exit code the command uses, for the table on How answers work.
export const EXITS = [
  [0, 'yes', 'The command finished. On a single piece of evidence, `decide` answers yes.'],
  [1, 'no', 'A single piece of evidence under `decide` answers no. No other function exits 1.'],
  [2, 'broken', 'A usage error or an input error. The failing record sent nothing.'],
  [3, 'unsure', 'A single piece of evidence under `decide` or `choose` is not sure. Under `find --none`, nothing fits.'],
  [4, 'broken', 'The backend failed, or sent a reply the adapter refused. Also a key that is unset or blank at any address other than `localhost`, `127.0.0.1` or `[::1]`, evidence past the token limit, and a critical line in a check report.'],
  [5, 'broken', 'A local failure: a file or a saved answer. Also an unreadable question file, a band in a question file for `choose`, `tag`, or `filter`, and a threshold in a `rank` question file.'],
  [6, 'broken', '`annotate` or `relate` only. The run finished with at least one valid and one failed question. It prints no diagnostic. `annotate` marks each failed question in its result. `relate` prints only the edges that succeeded.'],
  [7, 'broken', '`annotate --jsonl --details --batch 1 --on-error continue` finished, and at least one record had a missing-pointer error row. When a run has both failed questions and such rows, 7 wins over 6.'],
  [70, 'broken', 'A defect in the tool. Its message begins `defect:`.'],
  [SIGNALS.code, 'broken', SIGNALS.what],
];

// Captions for the annotate edge-case page, keyed by script name.
export const ANNOTATE_SEE = {
  '1-lines': 'A line is text, not an object. Its answers come back under value, and the line itself under input.',
  '2-clash': 'The record already holds steps. annotate refuses it at exit 2 and sends nothing for it.',
  '3-plan': 'The second record holds area. The plan refuses it at exit 2 and sends nothing.',
  '4-details': 'Under --details the record keeps its own steps under input. The answer named steps sits under value.',
  '5-continue': 'B-8 has no /body. It gets one error row in its place. B-7 and B-9 get their answers. The run exits 7, and the test line checks it.',
};

// Captions for the question-set page, keyed by script name.
export const QUESTION_SETS_SEE = {
  '1-plan': 'The plan shows each question\'s pointers. urgent reads both fields as one object, keyed subject and body. team reads only the body.',
};

// Captions for the recording page, keyed by script name. Its examples sit
// in examples/reference/answer-cache/, because check-samples skips any folder
// named recording.
export const RECORDING_SEE = {
  '1-plan': 'The plan ends with a summary line: one record, one request, and the size of what it would send.',
  '2-dry-run': 'The old flag is refused at exit 2, and the message names the new one.',
  '3-miss': 'The folder holds one answer, and this question is not it. The run exits 5 and names the missing key.',
  '4-facts': 'The answer came from a replay folder. The facts line counts one record, no request sent and no cache answer. Seconds change on every run. jq drops them.',
};

// The caption for the status run on the Configuration page.
export const STATUS_SEE = {
  '1-json': 'The schema names version 2. No backend is named, so the key would come from THINKTHEN_API_KEY.',
};

// Captions for the four tool pages, keyed by page and script name.
export const TOOLS_SEE = {
  audit: {
    '1-counts': 'Ten songs at the band 0.2:0.8: 3 right, 2 wrong, 5 not sure, and no ties.',
  },
  diff: {
    '1-cuts': 'One saved run, read at 0.5 and then at the band 0.2:0.8. Five answers become not sure, and each is withdrawn. No request is sent.',
  },
  check: {
    '1-probes': 'The plan names the four probes in the order a live check sends them.',
  },
  transform: {
    '1-list': 'The ten transforms, in name order.',
    '2-show': 'counts reads a saved run and counts its answers. jq runs it. thinkthen does not.',
  },
};
