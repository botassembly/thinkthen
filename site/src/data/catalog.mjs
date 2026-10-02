// The names, the order, and the one line for every function and surface.
//
// The one line for each function is the first line of its help text. The
// option lists come from specification/. The examples live in examples/, and
// scripts/smoke.mjs runs them.

export const TAGLINE = 'ThinkThen: code that knows what you mean';

// Set to false the day the site is linked from anywhere.
export const NOINDEX = false;

export const KEY_VARIABLE = 'THINKTHEN_API_KEY';

export { REPO } from './repo.mjs';
import { setting } from '../lib/settings-table.mjs';
import { backend } from '../lib/backends-table.mjs';
import { flag } from './flags.mjs';

// The four outcomes. One color each, everywhere a number or an answer shows.
export const OUTCOMES = [
  { key: 'yes', code: 0, label: 'yes', prints: 'true' },
  { key: 'no', code: 1, label: 'no', prints: 'false' },
  { key: 'unsure', code: 3, label: 'not sure', prints: 'null' },
  { key: 'broken', code: '2, 4, 5, 6, 7, 70', label: 'broken', prints: null },
];

export function outcomeOf(exit) {
  if (exit === 0) return 'yes';
  if (exit === 1) return 'no';
  if (exit === 3) return 'unsure';
  return 'broken';
}

// Each function's own flags, with each default and range from
// specification/settings.md through setting(). flags.mjs holds the flags
// several functions share, and the global flags every function takes.
const cutOn = (fn) => setting('Threshold').defaultOn(fn);
const CUT = setting('Threshold').bounds;
const CUT_TAKES = `a number above ${CUT.above} and at most ${CUT.atMost}`;
const TEXT = setting('What true and false mean');
const MEANS = [
  flag('--true TEXT', TEXT.allowed.toLowerCase(), TEXT.default.toLowerCase(), 'One sentence that says what a yes means, sent beside the question.'),
  flag('--false TEXT', TEXT.allowed.toLowerCase(), TEXT.default.toLowerCase(), 'One sentence that says what a no means.'),
];
const QUIET = flag('--quiet', 'nothing', 'off', 'Prints nothing. The exit code carries the answer. It works on one document only.');
// The shared flags every record function takes. find takes none of them.
const RECORD_FLAGS = ['--csv', '--tsv', '--context', '--batch', '--max-request-bytes', '--jobs'];
// recognize and relate do not batch records.
const SET_FLAGS = ['--csv', '--tsv', '--max-request-bytes', '--jobs'];
// A list typed beside @FILE replaces the question file's whole list.
const lower = (s) => s.charAt(0).toLowerCase() + s.slice(1);
const LIST_RULE = 'A list typed beside `@FILE` replaces the question file\'s whole list. The two never merge.';

// The failure codes every function shares, from specification/channels.md.
// A function's own list puts its answers first and then these.
export const COMMON_EXITS = [
  [2, 'usage or input error'],
  [4, 'the backend failed or refused, as it does for evidence over the size limit'],
  [5, 'a local failure'],
  [70, 'a defect in the tool'],
];

// A function page's lede reads "You give it <takes>. You get back <gives>.",
// so both are lower-case phrases with no closing period. `lede` replaces them
// on the question file, as HTML. `toPerson` marks the functions whose
// not-sure answers a person should read.
export const FUNCTIONS = [
  {
    name: 'decide',
    goal: 'decide answers one yes or no question with true, false, or null, and the exit code carries the answer.',
    primitive: 'Yes or no',
    line: 'Answer one yes or no question about the evidence.',
    takes: 'one question and one piece of evidence',
    gives: 'true, false, or null',
    toPerson: true,
    requests: 'One request for one piece of evidence. One request for each record in a stream.',
    args: 'QUESTION or @FILE',
    options: [
      flag('--threshold T or LOW:HIGH', `${CUT_TAKES}, or a band LOW:HIGH`, cutOn('decide'), 'The bar the probability of yes must reach. One number is a cut. Two numbers are a band, and the middle answers not sure.'),
      ...MEANS,
      QUIET,
    ],
    shared: RECORD_FLAGS,
    exits: [[0, 'yes'], [1, 'no'], [3, 'not sure'], ...COMMON_EXITS],
    unsure: 'On one piece of evidence, a probability inside the band prints null and exits 3. In a stream, that record prints null and the run exits 0.',
    howtos: ['screen-studies-for-a-review', 'group-alerts-into-incidents', 'screen-a-post-before-it-goes-up', 'check-an-expense-against-the-policy', 'split-a-scanned-packet-into-documents'],
    see: {
      '0-refund': "The customer asks for money back. decide prints true and exits 0.",
      '1-lines': "The refund request answers true and the thank-you note false. \"I want to send this back.\" could mean an exchange or money back. It lands inside the band 0.2:0.8 as null.",
      '2-case': "The send-back line lands in the band. decide exits 3, and the case sends it to a person. The script then exits 0.",
      '3-one': "The ticket asks for a refund. decide prints true and exits 0.",
      '5-means': "Under these words, a cancellation is not money back. decide says false and exits 1.",
    },
  },
  {
    name: 'choose',
    goal: 'choose picks one option from your list, or says not sure when no option clears the bar.',
    primitive: 'Pick one',
    line: 'Pick one option from your list.',
    takes: `one question, one piece of evidence, and ${setting('Options').range.min} to ${setting('Options').range.max} options`,
    gives: 'one of your options, or null',
    toPerson: true,
    requests: 'One request for one piece of evidence. One request for each record in a stream.',
    args: 'QUESTION or @FILE, then OPTION...',
    options: [
      flag('--threshold T', CUT_TAKES, cutOn('choose'), 'The bar the winning option must reach. One number only. A band is a usage error.'),
      flag('--option LABEL=DESCRIPTION', 'a label and what it means, and it may repeat', 'none', `One option and what it means. It replaces the positional options. ${LIST_RULE}`),
      flag('--options POINTER', 'an RFC 6901 pointer', 'none', 'Takes the options from each record. It needs `--jsonl`.'),
      flag('--raw', 'nothing', 'off', 'Prints the label without quotation marks. CSV and TSV refuse it.'),
      flag('--quiet', 'nothing', 'off', 'Prints nothing. The exit code says whether an option came back. It works on one document only.'),
    ],
    shared: RECORD_FLAGS,
    exits: [[0, 'an option came back'], [3, 'not sure'], ...COMMON_EXITS],
    unsure: 'On one piece of evidence, a pick under the threshold or an exact tie at the top prints null and exits 3. In a stream, the run exits 0. choose never exits 1.',
    howtos: ['rank-the-inbound-leads', 'split-a-scanned-packet-into-documents'],
    see: {
      '0-one': 'A parcel sent to the wrong address belongs to shipping. choose prints "shipping" and exits 0.',
      '1-lines': "Each of the first three messages names one team: billing, shipping, and account. The fourth names a parcel and a login. No team reaches 0.9, and it comes back null.",
    },
    moreSee: {
      pandas: 'A Series of four tickets gets one team each at 0.9. The fourth names two teams, and it comes back as a missing value.',
      duckdb: 'The same four tickets as rows of a table. The fourth row gets NULL at 0.9.',
    },
  },
  {
    name: 'tag',
    goal: 'tag returns every label that clears the bar, each judged on its own.',
    primitive: 'Yes or no, per label',
    line: 'Name every label that fits.',
    takes: `one question, one piece of evidence, and ${setting('Labels').range.min} to ${setting('Labels').range.max} labels`,
    gives: 'the labels that fit, as a list',
    requests: 'Every label rides in one request. One request for one piece of evidence, whatever the label count.',
    args: 'QUESTION or @FILE, then LABEL...',
    options: [
      flag('--threshold T', CUT_TAKES, cutOn('tag'), 'The bar every label must reach on its own. A band is refused.'),
      flag('--label LABEL=DESCRIPTION', 'a label and what it means, and it may repeat', 'none', `A label and what it means. It replaces the positional labels. ${LIST_RULE}`),
    ],
    shared: RECORD_FLAGS,
    exits: [[0, 'the run finished'], ...COMMON_EXITS],
    unsure: 'A label under the threshold is left out. An empty list is a good answer and exits 0.',
    howtos: ['code-open-ended-survey-answers'],
    see: {
      '1-one': "The message praises the dashboard, reports a crash, and names a double charge. It gets praise, bug, and billing.",
      '3-labels': "At 0.9, each line gets the one label its description fits.",
    },
  },
  {
    name: 'score',
    goal: 'score places text on a scale you name, and it orders a queue rather than gating one.',
    primitive: 'Place on a scale',
    line: 'Place the evidence on a scale you name.',
    takes: `one question, one piece of evidence, and ${setting('Levels').range.min} to ${setting('Levels').range.max} levels, least first`,
    gives: 'a number along your levels. The first level is 0',
    requests: 'One request for one piece of evidence. One request for each record in a stream.',
    args: 'QUESTION or @FILE, then LEVEL...',
    options: [],
    shared: RECORD_FLAGS,
    exits: [[0, 'the run finished'], ...COMMON_EXITS],
    unsure: 'score has no threshold and no not-sure answer. It always lands somewhere on the scale. It orders a queue a person reads. Do not use it to decide yes or no.',
    howtos: ['code-open-ended-survey-answers'],
    see: {
      '1-one': "The outage scores 2.0, and 2 is Immediate on this scale.",
      '3-lines': "The address change lands near 0, the Friday deadline near 1, and the login outage at 2.",
    },
    moreSee: {
      polars: 'A data frame of three messages gets an urgency column. Each message lands on the scale.',
      duckdb: 'The same three messages as rows of a table, each with its urgency.',
    },
  },
  {
    name: 'filter',
    goal: 'filter keeps the records where the answer is yes, unchanged and in order.',
    primitive: 'Yes or no, per record',
    line: 'Keep the records where the answer is yes.',
    takes: 'one yes-or-no question and many records',
    gives: 'the records that pass, byte for byte, in the order they went in',
    requests: 'One request for each record.',
    args: 'QUESTION or @FILE',
    argsNote: 'It reads one record per line. A pointer from `--field` or a question file makes it read JSON Lines.',
    options: [
      flag('--threshold T', CUT_TAKES, cutOn('filter'), 'The bar a record must reach to be kept. A band is a usage error.'),
      ...MEANS,
    ],
    shared: RECORD_FLAGS,
    exits: [[0, 'the run finished'], ...COMMON_EXITS],
    unsure: 'No record sets the exit code. A record under the threshold is dropped.',
    howtos: ['triage-a-support-inbox', 'join-two-tables-by-meaning', 'rank-the-inbound-leads'],
    see: {
      '1-lines': "filter keeps the two complaints: the broken zipper and the snapped strap.",
      '2-means': "The zipper, the color, and the strap are about the item. They stay, and the early delivery drops out.",
      '3-csv': "Only R-2 complains, and it comes back as one JSON object, id and all.",
    },
  },
  {
    name: 'rank',
    goal: 'rank sorts every record by how likely the answer is yes, and drops none.',
    primitive: 'Yes or no, per record',
    line: 'Sort records by how likely the answer is yes.',
    takes: 'one yes-or-no question and many records',
    gives: 'every record again, most likely first',
    requests: 'One request for each record. --top trims the printed list and saves nothing.',
    args: 'QUESTION or @FILE',
    argsNote: 'It reads one record per line. A pointer from `--field` or a question file makes it read JSON Lines.',
    options: [
      flag('--top N', lower(setting('Top').allowed), lower(setting('Top').default), 'Prints the first N records of the order. Every record is still judged, so it saves no request.'),
      ...MEANS,
    ],
    shared: RECORD_FLAGS,
    exits: [[0, 'the run finished'], ...COMMON_EXITS],
    unsure: 'rank takes no threshold. It drops nothing, and nothing is not sure. The sort happens on this machine.',
    howtos: ['rank-the-inbound-leads'],
    see: {
      '1-lines': "The outage comes first and the quote due today second. The bill due in a month comes third, and the newsletter that needs no reply comes last.",
      '2-jsonl': "The checkout crash stops customers from buying. `--top 1` prints it alone, id and all.",
    },
  },
  {
    name: 'find',
    goal: 'find reads all the lines together and returns the one that answers the question, or none.',
    primitive: 'Pick one line of the evidence',
    line: 'Pick the one line that best answers a question.',
    takes: 'a question and 2 to 255 lines or records, or 2 to 254 with --none',
    gives: 'the one line that fits best',
    requests: 'It sends one request for the whole document.',
    args: 'QUESTION',
    options: [
      flag('--none', 'nothing', 'off', 'Lets it answer that nothing fits. It then prints nothing and exits 3.'),
    ],
    shared: [],
    exits: [[0, 'a line came back'], [3, 'nothing fits, under --none'], ...COMMON_EXITS],
    unsure: 'Without --none, find must pick a line. When nothing fits, the line it picks is wrong. --none lets it say nothing fits.',
    howtos: ['group-alerts-into-incidents', 'check-an-expense-against-the-policy'],
    see: {
      '1-find': "find prints the line with the 30 day refund deadline.",
      '3-none': "No line says how to cancel. find prints nothing and exits 3.",
      '4-jsonl': "Q1 holds the reset steps, and the whole record comes back.",
    },
  },
  {
    name: 'annotate',
    goal: 'annotate answers a saved set of questions for every record and adds one field per question.',
    primitive: 'Every kind of answer at once',
    line: 'Fill out a form for every record.',
    takes: 'a saved set of questions and your JSON',
    gives: 'the same JSON with one field added per question. Nested fields ride through unchanged',
    requests: 'It sends one request for each record and each part the questions read.',
    toPerson: true,
    args: 'FILE',
    argsNote: 'The file is a saved question set.',
    options: [
      flag('--on-error POLICY', '`continue`', 'stop at the first failed record', 'Skips a record whose question set `on` pointer finds nothing, and prints one error row in its place. It needs `--jsonl`, `--details` and `--batch 1`, and `--plan` refuses it. Every other failure still stops the run.'),
    ],
    shared: RECORD_FLAGS,
    exits: [[0, 'every question was answered'], [6, 'the run finished with failed questions'], [7, 'the run finished and skipped records under --on-error continue'], ...COMMON_EXITS.map(([code, what]) => [code, code === 5 ? 'the question set could not be read' : what])],
    unsure: 'Each question carries its own threshold. One question set can mix cuts and bands. A question the backend could not answer is marked failed and counted. It never turns into null.',
    howtos: ['triage-a-support-inbox'],
    see: {
      '0-json': "One JSON document goes in. The same document comes back with three answers added: steps, area, and impact. The nested report rides through unchanged.",
      '1-jsonl': "Each report keeps its id and lands in its own area: login, billing, and export. The two with steps say true, and the blue button blocks nothing.",
    },
    moreSee: {
      pandas: 'A data frame of three reports gets one new column for each question in form.json.',
      duckdb: 'The same three reports as rows of a table. Each row gets its answers as one JSON object.',
    },
  },
  {
    name: 'recognize',
    goal: 'recognize finds the names in a text, gives each a kind from your list, and keeps those that clear the bar.',
    primitive: 'Pick one, per word',
    line: 'Find every name in the evidence and say what kind it is.',
    takes: 'the evidence and the kinds of name you allow',
    gives: 'each name, its kind, where it sits, and a strength',
    requests: 'It runs three steps. It finds the names. It labels each name with one of your kinds. This step works like choose. When you name a relation, it relates the names. --plan prints the first request it would send.',
    args: 'KIND... or @FILE',
    argsNote: 'The file is a question file.',
    options: [
      flag('--kind KIND=DESCRIPTION', 'a kind and what it means, and it may repeat', 'none', 'One kind and what it means. With no kinds, every name has the kind `ENTITY`.'),
      flag('--threshold T', CUT_TAKES, cutOn('recognize'), 'Keeps names whose strength reaches this cut.'),
      flag('--relation NAME=SOURCE:TARGET', 'a rule, and it may repeat', 'none', 'Also links the names it finds. A bare NAME relates any two kinds.'),
      flag('--relation-threshold T', CUT_TAKES, setting('Relation threshold').number, 'Keeps relation edges whose probability reaches this cut.'),
      flag('--max-text-bytes N', `${setting('Recognize text limit').allowed}`, `${setting('Recognize text limit').default} bytes`, 'The largest text it takes, in UTF-8 bytes. A longer text exits 2 before any request.'),
    ],
    shared: SET_FLAGS,
    exits: [[0, 'the run finished'], ...COMMON_EXITS],
    unsure: 'With kinds, the model picks each name\'s kind from them. A name that is not in the evidence cannot come back. The number on a name is its strength. ThinkThen computes it, and it is not a probability. Your threshold decides which names you keep.',
    howtos: [],
    see: {
      '0-kinds': 'recognize finds three names and gives each one of the three kinds: a person, an organization, and a place.',
      '1-names': 'Each --kind gives a kind a description. recognize finds three names. Each comes back with its kind and its strength.',
    },
    moreSee: {
      typescript: 'Two relation rules ask how the names connect. Maria Chen works for Northwind Freight, and Northwind Freight is based in Chicago.',
      duckdb: 'The same text as a row of a table. The first query lists the names. The second reads the relation rules from names.json and lists the links.',
    },
  },
  {
    name: 'relate',
    goal: 'relate asks the model about named entities, one possible edge per pair and rule, or one menu per source for a single rule.',
    primitive: 'Yes or no per pair, or pick one',
    line: 'Find relationships among named entities.',
    lede: 'You give it a set of names, the kind of each name, and the relations you care about. <code>relate</code> reads no other text. Jev answers from what it knows about the names. You get back one edge for each related pair, with its probability. A rule asks one yes or no question for each pair. In a rules file, a rule marked <code>"single": true</code> asks one choice for each source instead, and gives that source at most one edge. For the links a text states, use <code>recognize --relation</code>.',
    takes: 'one set of named entities and relation rules',
    gives: 'one edge for each related pair, with a probability, and at most one edge per source for a single rule',
    requests: 'It reads one complete set of up to 255 entities and asks a yes/no question per allowed pair and rule. A single rule asks one choice per source instead, with none of these as an option. --plan prints the first request it would send.',
    args: 'RELATION... or @FILE',
    argsNote: 'Each relation is `NAME=SOURCE_KIND:TARGET_KIND` or a bare `NAME`.',
    options: [
      flag('--either', 'nothing', 'off', 'Treats every relation as reading the same both ways.'),
      flag('--threshold T', CUT_TAKES, cutOn('relate'), 'Keeps edges whose probability reaches this cut.'),
      flag('--kind-field POINTER', 'an RFC 6901 pointer', setting('Kind pointer').default, 'Reads each entity\'s kind from this pointer.'),
    ],
    shared: SET_FLAGS,
    exits: [[0, 'the run finished'], [6, 'the run finished with failed questions'], ...COMMON_EXITS],
    unsure: 'A relation has a direction, or it reads the same both ways. The number on an edge is a probability. Your threshold decides which edges you keep.',
    howtos: [],
    see: {
      '1-sings': 'Paul McCartney sings Yesterday, and Ringo Starr sings Octopus\'s Garden. The two wrong pairs do not reach the default threshold.',
    },
    moreSee: {
      r: 'Eight travel rules in a data frame. One both-ways rule, contradicts, finds the two pairs of rules that disagree, at a bar of 0.5.',
      duckdb: 'The same eight rules as rows of a table. relate reads them through a query and returns the contradicting pairs by id.',
    },
  },
  {
    name: 'question-file',
    title: '@question',
    goal: 'A question file saves one question with its threshold, and every command that reads it asks the same question.',
    primitive: 'Not a function',
    line: 'A saved question that six functions accept.',
    lede: 'Save one question in a JSON file. Pass it as <code>@FILE</code> to decide, choose, tag, score, filter, or rank. Every command that reads the file then asks the same question.',
    requests: 'None of its own. The function that reads it sends the requests.',
    args: '@FILE',
    argsNote: 'It takes the place of the question words on decide, choose, tag, score, filter, and rank.',
    options: [
      flag('the verb key', '', '', 'One of decide, choose, tag, or score. It names the verb and carries the question text.'),
      flag('true, false', '', '', 'What a yes and a no mean, for a decide question.'),
      flag('options, labels', '', '', 'A list, or a map from label to what it means.'),
      flag('levels', '', '', 'The scale, lowest first.'),
      flag('threshold', '', '', 'A cut, or a band written LOW:HIGH.'),
      flag('on', '', '', 'One JSON Pointer, or a list of them.'),
      flag('model, profile', '', '', 'The model to ask and the calibration profile to apply.'),
    ],
    exits: [[5, 'the file could not be read, is not one JSON object, or breaks a rule'], [2, 'the command names the wrong verb for the file']],
    unsure: 'A band in the file marks the middle answers not sure. Send those to a person.',
    howtos: [],
    see: {
      '1-yes': "The saved question answers true for a plain request for money back.",
      '2-unsure': "Sending it back could mean an exchange or money back. The band in the file calls it not sure. decide prints null and exits 3.",
    },
    moreSee: {
      duckdb: 'Two messages as rows of a table, asked with refund.json. The money-back message gets true. The send-back message lands inside the band and gets NULL.',
    },
    notAFunction: true,
  },
];

// The function whose exit code a script ends with. The last command of
// the script counts, with heredoc bodies and `test` asserts set aside. A
// script that ends in a block, or in another tool such as jq, gives null,
// because that block or tool set the code.
export function lastFunction(script) {
  const lines = [];
  let until = null;
  for (const line of script.split('\n')) {
    if (until) { if (line.trim() === until) until = null; continue; }
    const doc = /<<-?\s*'?(\w+)'?/.exec(line);
    if (doc) until = doc[1];
    if (/^\s*test\s/.test(line) || !line.trim()) continue;
    lines.push(line);
  }
  // Join the lines a trailing backslash or pipe continues.
  const commands = [];
  for (const line of lines) {
    const prev = commands.at(-1);
    if (prev !== undefined && /(\\|\|)\s*$/.test(prev)) commands[commands.length - 1] = prev.replace(/\\\s*$/, ' ') + ' ' + line.trim();
    else commands.push(line.trim());
  }
  const last = commands.at(-1) || '';
  let quote = null;
  let start = 0;
  for (let i = 0; i < last.length; i++) {
    const c = last[i];
    if (quote) { if (c === quote) quote = null; continue; }
    if (c === "'" || c === '"') quote = c;
    else if (c === '|' && last[i + 1] !== '|' && last[i - 1] !== '|') start = i + 1;
  }
  const found = /^thinkthen\s+([a-z-]+)/.exec(last.slice(start).trim());
  return found ? found[1] : null;
}

// The mark after a script's output: its exit code, a word, and a colour.
// The word is the function's own meaning for the code. Green, red, and
// amber go only to a single answer: yes, no, or not sure. A record run, a
// finished run, and a script that ends in another tool stay plain.
export function exitMark(script, exit) {
  const name = lastFunction(script || '');
  const entry = FUNCTIONS.find((f) => f.name === name);
  const records = /--(lines|jsonl|csv|tsv)\b/.test(script || '');
  if (!entry) return { word: null, cls: exit === 0 ? 'plain' : outcomeOf(exit) };
  if (records && exit === 0) return { word: 'the run finished', cls: 'plain' };
  const row = entry.exits.find(([code]) => code === exit);
  const word = row ? row[1] : null;
  const answer = exit !== 0 || !['the run finished', 'every question was answered'].includes(word);
  return { word, cls: answer ? outcomeOf(exit) : 'plain' };
}

export const CODE_FUNCTIONS = FUNCTIONS.filter((f) => !f.notAFunction);


// The C library row on every page of a binding that calls it.
const C_ARCHIVE = ['thinkthen-c-VERSION-TARGET.tar.gz', 'The C library it calls, from the same release.'];

export const SURFACES = [
  {
    slug: 'shell', name: 'Bash', deckHeading: null,
    backends: 'cli',
    lang: 'bash', tab: 'Bash',
    blurb: 'Pipe text in, read the answer out, and branch on the exit code.',
    unsureWord: '`null`, and exit code 3',
    facts: '`--facts` prints one line of run facts on standard error, after the answers.',
    install: [
      ['curl -fsSL https://thinkthen.dev/install.sh | sh', 'Download script.'],
      ['brew install botassembly/thinkthen/thinkthen', 'Homebrew, an option on a Mac.'],
    ],
    uninstall: {
      text: 'To remove the download, delete the binary and its receipt. If you set `THINKTHEN_INSTALL_DIR`, delete the same two files there. With Homebrew, run `brew uninstall thinkthen`.',
      code: 'rm ~/.local/bin/thinkthen \\\n  ~/.local/bin/thinkthen.install.json',
    },
    particular: [
      'Standard input carries the evidence. Standard output carries the answer and nothing else.',
      'On one piece of evidence, the exit code is the answer. `if` and `case` read it directly.',
      '`--jobs` sets how many requests run at once.',
      '`--plan` prints the plan and needs no key.',
    ],
  },
  {
    slug: 'python', name: 'Python', deckHeading: 'Python',
    backends: 'env',
    lang: 'python', tab: 'Python',
    blurb: 'Pass a string or a list. Read the answer from `Call.value`.',
    unsureWord: '`None`',
    facts: '`Call.facts` counts this call.',
    errors: 'Every failure raises a `ThinkThenError`. Its subclass names the kind: `UsageError`, `BackendError`, `LocalError`, `DeadlineError`, `DefectError` or `Cancelled`.',
    settings: '`tt.Engine` takes each setting as a keyword and reads the environment for the rest. `record=` writes a recording to a folder. `replay=` answers from that recording with no connection.',
    install: [['pip install thinkthen', null], ['uv add thinkthen', null]],
    particular: [
      'A list goes in and `Call.value` holds the answered list. The list crosses into the engine once.',
      '`tt.question()` builds a question that carries its own threshold. Reuse it wherever you ask.',
    ],
  },
  {
    slug: 'polars', name: 'Polars', deckHeading: 'Polars',
    backends: 'python',
    lang: 'python', tab: 'Python',
    blurb: 'A Polars frame goes in. Read the frame with its new columns from `Call.value`.',
    unsureWord: '`None`',
    facts: '`Call.facts` counts the whole frame call. The new columns hold only the answers.',
    errors: 'Every failure raises a `ThinkThenError`, as in Python. A failed row in a column call ends the call with `BackendError`.',
    settings: '`tt.Engine` takes each setting as a keyword, as in Python. `record=` writes a recording to a folder. `replay=` answers from that recording with no connection.',
    install: [['pip install thinkthen[polars]', null]],
    particular: [
      '`decide`, `choose`, `score`, and `tag` send a whole column to the engine in one call.',
      '`on=` names the column the questions read.',
    ],
  },
  {
    slug: 'pandas', name: 'pandas', deckHeading: null,
    backends: 'python',
    lang: 'python', tab: 'Python',
    blurb: 'A pandas Series goes in, and a Series with the same index comes back in `Call.value`.',
    unsureWord: '`pd.NA`',
    facts: '`Call.facts` counts this call.',
    errors: 'Every failure raises a `ThinkThenError`, as in Python. Its subclass names the kind.',
    settings: '`tt.Engine` takes each setting as a keyword, as in Python. `record=` writes a recording to a folder. `replay=` answers from that recording with no connection.',
    install: [['pip install thinkthen pandas', null]],
    particular: [
      '`decide`, `choose`, `score` and `tag` take a Series and keep its index and name.',
      '`decide` gives a `boolean` Series, `score` gives `Float64`, `choose` gives `string`, and `tag` gives one list of labels per row.',
      '`annotate` takes a DataFrame with `on=` and adds one column per question.',
    ],
  },
  {
    slug: 'typescript', name: 'TypeScript', deckHeading: 'TypeScript',
    backends: 'env',
    lang: 'ts', tab: 'TypeScript',
    blurb: 'Pass the question and the text. Await the call, then read its `.value`.',
    unsureWord: '`null`',
    facts: '`Call.facts` counts this call.',
    errors: 'A failure rejects with a `ThinkThenError`. Its `kind` is `usage`, `backend`, `local`, `deadline`, `cancelled` or `defect`.',
    settings: '`new Engine({...})` starts from the environment, and each key overrides one setting. `record` writes a recording to a folder. `replay` answers from that recording with no connection.',
    install: [['npm install thinkthen', null], ['pnpm add thinkthen', null], ['bun add thinkthen', null]],
    particular: [
      'An AbortSignal cancels the call, and the promise rejects at once.',
      'Every call returns a promise for a `Call`. An array crosses once.',
    ],
  },
  {
    slug: 'ruby', name: 'Ruby', deckHeading: 'Ruby',
    backends: 'env',
    lang: 'ruby', tab: 'Ruby',
    blurb: 'Any Enumerable goes in. Read the answer from `Call#value`.',
    unsureWord: '`nil`',
    facts: '`Call#facts` counts this call.',
    errors: 'Every failure raises a `ThinkThen::Error`. Its subclass names the kind: `UsageError`, `BackendError`, `LocalError`, `DeadlineError`, `DefectError` or `CancelledError`.',
    settings: '`Engine.new` takes each setting as a keyword and reads the environment for the rest. `record:` writes a recording to a folder. `replay:` answers from that recording with no connection.',
    install: [['gem install thinkthen', null]],
    particular: ['Any Enumerable crosses to the engine once.'],
  },
  {
    slug: 'r', name: 'R', deckHeading: 'R',
    backends: 'env',
    lang: 'r', tab: 'R',
    blurb: 'The ten functions work inside dplyr pipelines. Read the answer from `$value`.',
    unsureWord: '`NA`',
    facts: '`$facts` counts this call.',
    errors: 'Each failure arrives as an R condition named for its kind, such as `thinkthen_usage`. Each condition carries `retryable`.',
    settings: '`tt_engine()` takes each setting as an argument and reads the environment for the rest. `record =` writes a recording to a folder. `replay =` answers from that recording with no connection.',
    install: [['install.packages("thinkthen")', null]],
    particular: [
      'A column goes in and the answered column is in `$value`.',
    ],
    frames: 'A verb inside `mutate()` answers a whole column in one call. dplyr\'s `filter()` drops NA rows, so a not-sure answer leaves the pipeline on its own.',
  },
  {
    slug: 'rust', name: 'Rust', deckHeading: 'Rust',
    backends: 'builder',
    lang: 'rust', tab: 'Rust',
    blurb: 'Call the engine directly. The compiler makes you handle not sure.',
    unsureWord: '`Answer::Unsure` from `decide`, and `None` from `choose`',
    facts: '`Call::facts()` counts this call.',
    errors: 'Every call returns a `Result`. Its `Error` names one of six kinds. A started call that fails keeps its facts in `Error::facts()`.',
    settings: '`EngineBuilder` sets each setting. `record` writes a recording to a folder. `replay` answers from that recording and sends nothing, even on a miss.',
    install: [['cargo add thinkthen', null]],
    fragment: 'Put this code inside `fn main() -> Result<(), Box<dyn std::error::Error>>` and end it with `Ok(())`. `main` returns a `Result`, so `?` compiles.',
    particular: [
      'Calls block. No async runtime comes with it.',
    ],
  },
  {
    slug: 'c', name: 'C', deckHeading: 'C',
    backends: 'env',
    lang: 'c', tab: 'Rust',
    blurb: 'One header over a shared or a static library. Bind ThinkThen to any language that can call C.',
    unsureWord: 'the outcome `THINKTHEN_UNSURE`',
    facts: 'A JSON call returns `{"value":...,"facts":...}`.',
    errors: 'A failed call returns NULL, or a code from `THINKTHEN_EUSAGE` to `THINKTHEN_EDEFECT`. `thinkthen_error_message` reads the message on the same thread.',
    settings: '`thinkthen_engine_new_with` takes the settings as JSON. `"record"` writes a recording to a folder. `"replay"` answers from that recording with no connection. The key stays in `THINKTHEN_API_KEY`.',
    install: [['thinkthen.h + libthinkthen', 'Each release ships the header, the shared library, and the static library.']],
    fragment: 'Keep the `#include` lines on top. Put the rest inside `int main(void)` and end it with `return 0;`.',
    particular: [
      'The JSON examples parse the `value` and `facts` members with json-c. Install its development headers and link with `pkg-config --cflags --libs json-c` beside libthinkthen.',
      'A failed JSON call returns NULL. A successful `value` can itself be JSON null. Typed calls use a result struct and an error code.',
    ],
  },
  {
    slug: 'cpp', name: 'C++', deckHeading: null,
    backends: 'env',
    lang: 'cpp', tab: 'C++',
    blurb: 'One C++17 header over the C library. Calls return `CallResult` values. The engine frees itself when it goes out of scope.',
    unsureWord: '`tt::Outcome::notSure`',
    facts: '`decide` returns a `CallResult`. Its `.value` holds the answer, and its `.facts` holds this call\'s run facts as `tt::Json`.',
    errors: 'A failed call throws a `tt::Failure`. Its `tt::ErrorKind` names one of six kinds, with the message and the retry flag.',
    settings: '`tt::create(settings)` takes the settings as JSON. `"record"` writes a recording to a folder. `"replay"` answers from that recording with no connection.',
    install: [
      ['thinkthen-cpp-VERSION-TARGET.tar.gz', 'The header and a CMake package, from each release.'],
      C_ARCHIVE,
    ],
    particular: [
      'The CMake package gives `find_package(thinkthen-cpp)`.',
      '`tt::Engine` and `tt::CancelToken` move but do not copy. Join every thread that uses them before they go.',
    ],
  },
  {
    slug: 'objective-c', name: 'Objective-C', deckHeading: null,
    backends: 'env',
    lang: 'objective-c', tab: 'Objective-C',
    blurb: 'A `TTClient` over the C library, for GNU Objective-C with no Foundation.',
    unsureWord: '`TTOutcomeNotSure`',
    facts: 'Each typed call sets `facts:` to this call\'s run facts as JSON text. Free it with `free`.',
    errors: 'A failed call returns a `TTErrorKind` and fills the `TTFailure`. A failed call leaves the answer untouched.',
    settings: '`createWithSettings:length:failure:` takes the settings as JSON. `"record"` writes a recording to a folder. `"replay"` answers from that recording with no connection.',
    install: [
      ['thinkthen-objective-c-VERSION-TARGET.tar.gz', 'The binding\'s source, from each release.'],
      C_ARCHIVE,
    ],
    particular: [
      'The `*Bytes` forms, such as `decideBytes`, take the question with its length and refuse a NUL inside it.',
    ],
  },
  {
    slug: 'cobol', name: 'COBOL', deckHeading: null,
    backends: 'env',
    lang: 'cobol', tab: 'COBOL',
    blurb: 'A copybook and called programs over the C library, for GnuCOBOL.',
    unsureWord: '`outcome-not-sure`',
    facts: '`TT-DECIDE` fills `tt-facts` with this call\'s run facts as JSON text. `TT-JSON-MEMBER` reads one member.',
    errors: '`tt-failure` holds the kind as a code from 1 to 6, the retry flag and the message. Its level-88 names, such as `failure-backend`, test each kind.',
    settings: '`TT-ENGINE-NEW` takes the settings as JSON. `"record"` writes a recording to a folder. `"replay"` answers from that recording with no connection.',
    install: [
      ['thinkthen-cobol-VERSION-TARGET.tar.gz', 'The copybook and the called programs, from each release.'],
      C_ARCHIVE,
    ],
    particular: [
      'Compile the programs you call from `src/` with your own, as the build line does with `TT-DECIDE`.',
      'The copybook names no, yes and not sure as level-88 conditions, so `if outcome-yes` reads the answer.',
      '`TT-DECIDE` writes into the question buffer, so set the question again before each call.',
    ],
  },
  {
    slug: 'ada', name: 'Ada', deckHeading: null,
    backends: 'env',
    lang: 'ada', tab: 'Ada',
    blurb: 'A `Thinkthen` package over the C library, for GNAT. Each call is a procedure with out parameters.',
    unsureWord: '`Not_Sure`',
    facts: '`Decide` sets `Facts` to this call\'s run facts as JSON text. `Member` reads one member.',
    errors: '`Decide` sets `Error`. Its `Kind` runs from `Usage` to `Defect`, and `None` means the call answered.',
    settings: '`Configure` takes the settings as JSON. `"record"` writes a recording to a folder. `"replay"` answers from that recording with no connection.',
    install: [
      ['thinkthen-ada-VERSION-TARGET.tar.gz', 'The package and its GNAT project, from each release.'],
      C_ARCHIVE,
    ],
    particular: [
      '`gprbuild -P thinkthen.gpr` builds the package as a library.',
      '`pragma Assert` runs only under `-gnata`.',
    ],
  },
  {
    slug: 'java', name: 'Java', deckHeading: null,
    backends: 'env',
    lang: 'java', tab: 'Java',
    blurb: 'One `Door` class over the C library, for Java 21. Each call returns its answer and its run facts.',
    unsureWord: '`Outcome.NOT_SURE`',
    facts: '`decide` returns a `TypedResult`. Its `value()` holds the answer, and its `facts()` holds this call\'s run facts as a `Map`.',
    errors: 'A failed call throws a `Door.NativeFailure`. Its `failure` names one of six `FailureKind` values, with the message and the retry flag.',
    settings: '`new Door(settingsJson)` takes the settings as JSON. `"record"` writes a recording to a folder. `"replay"` answers from that recording with no connection.',
    install: [
      ['io.github.botassembly:thinkthen-jvm:VERSION', 'The Maven coordinate.'],
      ['thinkthen-jvm-VERSION-TARGET.tar.gz', 'The door, Kotlin and Scala JARs, from each release.'],
      C_ARCHIVE,
    ],
    particular: [
      'The door calls the C library through Java 21\'s foreign function API. `javac` and `java` take `--enable-preview`.',
      '`-Dthinkthen.library` names the C library by its absolute path.',
      '`assert` runs only under `java -ea`.',
    ],
  },
  {
    slug: 'kotlin', name: 'Kotlin', deckHeading: null,
    backends: 'env',
    lang: 'kotlin', tab: 'Kotlin',
    blurb: 'A `KotlinFacade` over the Java door. Its calls take strings.',
    unsureWord: '`Outcome.NOT_SURE`',
    facts: '`decide` returns the door\'s `TypedResult`. `value()` holds the answer, and `facts()` holds this call\'s run facts as a `Map`.',
    errors: 'A failed call throws a `Door.NativeFailure`, as in Java. Its `failure` names the kind, the message and the retry flag.',
    settings: '`Door(settingsJson)` takes the settings as JSON. `"record"` writes a recording to a folder. `"replay"` answers from that recording with no connection.',
    install: [
      ['io.github.botassembly:thinkthen-jvm:VERSION:kotlin', 'The Kotlin JAR. Add the Java coordinate beside it.'],
      ['thinkthen-jvm-VERSION-TARGET.tar.gz', 'The door, Kotlin and Scala JARs, from each release.'],
      C_ARCHIVE,
    ],
    particular: [
      '`decideAsync` runs a call on its own thread. `await()` gives the answer. Close it before the door.',
      'The run takes the same `java` flags as Java: `--enable-preview` and `-Dthinkthen.library`.',
    ],
  },
  {
    slug: 'scala', name: 'Scala', deckHeading: null,
    backends: 'env',
    lang: 'scala', tab: 'Scala',
    blurb: 'A `ScalaFacade` over the Java door, for Scala 3. Its calls take strings.',
    unsureWord: '`Outcome.NOT_SURE`',
    facts: '`decide` returns the door\'s `TypedResult`. `value()` holds the answer, and `facts()` holds this call\'s run facts as a `Map`.',
    errors: 'A failed call throws a `Door.NativeFailure`, as in Java. Its `failure` names the kind, the message and the retry flag.',
    settings: '`Door(settingsJson)` takes the settings as JSON. `"record"` writes a recording to a folder. `"replay"` answers from that recording with no connection.',
    install: [
      ['io.github.botassembly:thinkthen-jvm:VERSION:scala', 'The Scala JAR. Add the Java coordinate beside it.'],
      ['thinkthen-jvm-VERSION-TARGET.tar.gz', 'The door, Kotlin and Scala JARs, from each release.'],
      C_ARCHIVE,
    ],
    particular: [
      '`ScalaFacade` takes an `ExecutionContext`. `decideAsync` gives a `future`, and `close()` waits for the call.',
      'The run needs the Scala library on the class path, such as `$SCALA_HOME/lib/scala.jar`.',
    ],
  },
  {
    slug: 'csharp', name: 'C#', deckHeading: null,
    backends: 'env',
    lang: 'csharp', tab: 'C#',
    blurb: 'An `Engine` over the C library, for .NET 8. Each call returns its answer and its run facts.',
    unsureWord: '`Outcome.NotSure`',
    facts: '`Decide` returns a `TypedResult`. `.Value` holds the answer, and `.Facts` holds this call\'s run facts as a `JsonElement`.',
    errors: 'A failed call throws a `Failure`. Its `Kind` names one of six `FailureKind` values, and `Retryable` says whether a retry may help.',
    settings: '`Engine.Open(settingsJson)` takes the settings as JSON. `"record"` writes a recording to a folder. `"replay"` answers from that recording with no connection.',
    install: [
      ['dotnet add package Botassembly.ThinkThen', 'The NuGet package.'],
      ['thinkthen-csharp-VERSION-TARGET.tar.gz', 'The package file, from each release. Its folder serves as a local feed.'],
      C_ARCHIVE,
    ],
    particular: [
      'The package finds the C library by name. `LD_LIBRARY_PATH` names its folder.',
      '`Trace.Assert` runs in a Release build. `Debug.Assert` does not.',
    ],
  },
  {
    slug: 'go', name: 'Go', deckHeading: null,
    backends: 'env',
    lang: 'go', tab: 'Go',
    blurb: 'A cgo package over the C library. Each call that sends takes a `context.Context` and returns a `Result` and an `error`.',
    unsureWord: '`thinkthen.Unsure`',
    facts: '`Decide` returns a `Result`. `.Value` holds the answer, and `.Facts` holds this call\'s run facts as `json.RawMessage`.',
    errors: 'A failed native call returns an `*Error`. Its `Kind` runs from `KindUsage` to `KindDefect`, and it carries `Retryable` and `Message`. `ErrEmbeddedNUL` and allocation failures come back as plain errors.',
    settings: '`NewWith(settingsJSON)` takes the settings as JSON. `"record"` writes a recording to a folder. `"replay"` answers from that recording with no connection.',
    install: [
      ['go get github.com/botassembly/thinkthen/libraries/go', null],
      ['thinkthen-go-VERSION-TARGET.tar.gz', 'The module source, from each release.'],
      C_ARCHIVE,
    ],
    particular: [
      'The package finds the C library through `pkg-config`. `PKG_CONFIG_PATH` names the C archive\'s `lib/pkgconfig`.',
      'A deadline on the context reaches the call as its deadline.',
    ],
  },
  {
    slug: 'swift', name: 'Swift', deckHeading: null,
    backends: 'env',
    lang: 'swift', tab: 'Swift',
    blurb: 'A SwiftPM package over the C library, for Swift 6. Each call returns its answer and its run facts.',
    unsureWord: '`.unsure`',
    facts: '`decide` returns a `CallResult`. `.value` holds the answer, and `.facts` holds this call\'s run facts as JSON text.',
    errors: 'A failed call throws a `DoorFailure`. Its `kind` names usage, backend, deadline, local, cancelled or defect.',
    settings: '`Engine(settingsJSON:)` takes the settings as JSON. `"record"` writes a recording to a folder. `"replay"` answers from that recording with no connection.',
    install: [
      ['thinkthen-swift-VERSION-TARGET.tar.gz', 'The package source, from each release.'],
      C_ARCHIVE,
    ],
    particular: [
      'Add the package with `.package(path:)`. Its product is `ThinkThen`.',
      'The linker needs the C library\'s folder, through `-Xlinker -L` and `-Xlinker -rpath`.',
      'Call `close()` once, after every call on the engine ends.',
    ],
  },
  {
    slug: 'zig', name: 'Zig', deckHeading: null,
    backends: 'env',
    lang: 'zig', tab: 'Zig',
    blurb: 'A Zig 0.15.2 module over the C library. Each call returns `.ok` or `.failed`.',
    unsureWord: '`.unsure`',
    facts: 'An `.ok` holds a `CallResult`. `.value` holds the answer, and `.facts` holds this call\'s run facts as parsed JSON. Free it with `deinit`.',
    errors: 'A `.failed` holds one of six `kind` values and a message. Free it with `engine.freeFailure`.',
    settings: '`Engine.initWithSettings` takes an allocator and the settings as JSON. `"record"` writes a recording to a folder. `"replay"` answers from that recording with no connection.',
    install: [
      ['thinkthen-zig-VERSION-TARGET.tar.gz', 'The module source, from each release.'],
      C_ARCHIVE,
    ],
    particular: [
      '`linkNative` in the build script links the C library. `-Dnative` names the unpacked C archive.',
      '`std.debug.assert` checks only in Debug and ReleaseSafe builds.',
    ],
  },
  {
    slug: 'php', name: 'PHP', deckHeading: null,
    backends: 'env',
    lang: 'php', tab: 'PHP',
    blurb: 'One `ThinkThen` class over the C library, through PHP 8.3 FFI. Each call returns its answer and its run facts.',
    unsureWord: '`ThinkThen::UNSURE`',
    facts: '`decide` returns an array. Its `value` holds the answer, and its `facts` holds this call\'s run facts as an array.',
    errors: 'A failed call throws a `ThinkThenFailure`. Its `kind` names usage, backend, deadline, local, cancelled or defect, and it carries `retryable` and the message.',
    settings: '`new ThinkThen($library, $settingsJson)` takes the settings as JSON. `"record"` writes a recording to a folder. `"replay"` answers from that recording with no connection.',
    install: [
      ['composer require botassembly/thinkthen', null],
      ['thinkthen-php-VERSION-TARGET.tar.gz', 'The package source, from each release.'],
      C_ARCHIVE,
    ],
    particular: [
      'PHP calls the C library through FFI. `php` takes `-d ffi.enable=1`.',
      '`new ThinkThen` takes the C library\'s absolute path.',
      '`outcome` holds 1, 0 or 2. `ThinkThen::YES`, `NO` and `UNSURE` name them.',
      '`assert` runs only with `-d zend.assertions=1`.',
    ],
  },
  {
    slug: 'dart', name: 'Dart', deckHeading: null,
    backends: 'env',
    lang: 'dart', tab: 'Dart',
    blurb: 'One `Door` class over the C library, through Dart FFI. Each call returns its answer and its run facts.',
    unsureWord: '`Outcome.notSure`',
    facts: '`decide` returns a record. `.value` holds the answer, and `.facts` holds this call\'s run facts as a `Map`.',
    errors: 'A failed call throws a `DoorFailure`. Its `kind` names one of six `ErrorKind` values. It also carries the message and `retryable`.',
    settings: '`create(settingsJson)` takes the settings as JSON. `"record"` writes a recording to a folder. `"replay"` answers from that recording with no connection.',
    install: [
      ['dart pub add thinkthen_dart', null],
      ['thinkthen-dart-VERSION-TARGET.tar.gz', 'The package source, from each release.'],
      C_ARCHIVE,
    ],
    particular: [
      '`Door` takes the C library\'s path. `create` makes an engine, and each call takes that engine first. `engineFree` closes it.',
      '`assert` runs only under `dart run --enable-asserts`.',
      'A Flutter app on Linux adds `thinkthen_dart` and passes the library path to `Door`.',
    ],
  },
  {
    slug: 'duckdb', name: 'DuckDB', deckHeading: 'DuckDB',
    lang: 'sql', tab: 'SQL',
    blurb: 'Ask a question in WHERE, SELECT, or ORDER BY.',
    unsureWord: '`NULL`',
    facts: '`thinkthen_usage()` gives the totals for the process.',
    errors: 'A failure is an error whose text starts `thinkthen <kind>: `. It never reads as `NULL`.',
    settings: '`SET thinkthen_record` writes a recording to a folder. `SET thinkthen_replay` answers from that recording with no connection.',
    install: [['duckdb -unsigned', 'The `-unsigned` flag lets DuckDB load a local extension file. The query loads that file first.']],
    particular: ['DuckDB hands the extension up to 2,048 rows at a time. One call judges those rows together, and `SET thinkthen_max_requests` caps that call.'],
  },
  {
    slug: 'sqlite', name: 'SQLite', deckHeading: 'SQLite',
    lang: 'sql', tab: 'SQL',
    blurb: 'Judge a whole table in one call, then join the answers back by key.',
    unsureWord: '`NULL`',
    facts: '`thinkthen_usage()` gives the totals for the process.',
    errors: 'A failure is an error whose text starts `thinkthen <kind>: `. The message follows, and the text ends with `(retryable: yes)` or `(retryable: no)`.',
    settings: '`thinkthen_configure` takes the settings as JSON. `"record"` writes a recording to a folder. `"replay"` answers from that recording with no connection.',
    install: [['.load ./thinkthen', null]],
    particular: [
      'Only a query you type can call its functions. A view or trigger in an untrusted schema cannot call them to spend requests or read files.',
    ],
  },
  {
    slug: 'postgresql', name: 'PostgreSQL', deckHeading: 'PostgreSQL',
    lang: 'sql', tab: 'SQL',
    blurb: 'One extension. Ask questions in any query.',
    unsureWord: '`NULL`',
    facts: '`thinkthen_usage()` gives the totals for the process.',
    errors: 'A failed call raises its named error. `thinkthen_try_details` returns a failure as `jsonb` instead.',
    settings: '`SET thinkthen.record` writes a recording to a folder. `SET thinkthen.replay` answers from that recording with no connection.',
    install: [['CREATE EXTENSION thinkthen;', null]],
    particular: [
      'A question file carries a band. The not-sure rows come back NULL, and a person reads them.',
      'pg_cancel_backend and statement_timeout stop a call. A request already sent still completes and is billed.',
    ],
  },
];

// The bindings: every language and database ThinkThen works with. Bash is
// the command line, the one CLI, and not a binding. The talk's slides
// name the same 24. Each binding links to
// /install/<slug>/. A binding with no entry in SURFACES has no page yet, and
// scripts/check-links.mjs allows exactly those paths.
const DATABASE_SLUGS = new Set(['duckdb', 'postgresql', 'sqlite']);
export const BINDINGS = [
  ['Ada', 'ada'], ['C', 'c'], ['C#', 'csharp'], ['C++', 'cpp'],
  ['COBOL', 'cobol'], ['Dart', 'dart'], ['DuckDB', 'duckdb'], ['Go', 'go'],
  ['Java', 'java'], ['Kotlin', 'kotlin'], ['Objective-C', 'objective-c'],
  ['pandas', 'pandas'], ['PHP', 'php'], ['Polars', 'polars'],
  ['PostgreSQL', 'postgresql'], ['Python', 'python'], ['R', 'r'],
  ['Ruby', 'ruby'], ['Rust', 'rust'], ['Scala', 'scala'],
  ['SQLite', 'sqlite'], ['Swift', 'swift'], ['TypeScript', 'typescript'],
  ['Zig', 'zig'],
].map(([name, slug]) => ({ name, slug, database: DATABASE_SLUGS.has(slug), route: `/install/${slug}/` }));

// The install paths of the bindings with no page yet.
export const BINDING_PATHS_WITHOUT_PAGES = BINDINGS
  .filter((b) => !SURFACES.some((s) => s.slug === b.slug))
  .map((b) => b.route);

// The counts the home page and the functions index give.
export const COUNTS = {
  functions: `${CODE_FUNCTIONS.length} functions`,
  cli: '1 CLI',
  bindings: `${BINDINGS.length} bindings`,
};

for (const s of SURFACES) {
  if (s.slug !== 'shell' && !BINDINGS.some((b) => b.slug === s.slug)) throw new Error(`catalog: the surface ${s.slug} is not in BINDINGS`);
  for (const field of s.slug === 'shell' ? ['facts'] : ['facts', 'errors', 'settings']) {
    if (!s[field]) throw new Error(`catalog: the surface ${s.slug} has no ${field}`);
  }
  if (s.backends && !['env', 'builder', 'cli', 'python'].includes(s.backends)) throw new Error(`catalog: the surface ${s.slug} has the backends route ${s.backends}`);
}

// The tabs on the home page sample, and on every code block that has variants.
export const TABS = ['Bash', 'Python', 'TypeScript', 'Ruby', 'R', 'Rust', 'SQL'];

// Which surface each tab draws from.
export const TAB_SURFACE = {
  Bash: 'shell', Python: 'python', TypeScript: 'typescript',
  Ruby: 'ruby', R: 'r', Rust: 'rust', SQL: 'duckdb',
};

// A function page groups its language tabs, so a reader finds one of 24
// quickly on a phone or a desktop. Every surface sits in one group. A tab
// shows when its sample file exists, or when the function names it in
// `cannot`.
export const TAB_GROUPS = [
  { name: 'Command line', slugs: ['shell'] },
  { name: 'Python and data', slugs: ['python', 'pandas', 'polars', 'r'] },
  { name: 'Web and scripting', slugs: ['typescript', 'ruby', 'php', 'dart'] },
  { name: 'JVM and .NET', slugs: ['java', 'kotlin', 'scala', 'csharp'] },
  { name: 'Systems', slugs: ['c', 'cpp', 'objective-c', 'rust', 'go', 'swift', 'zig', 'ada', 'cobol'] },
  { name: 'Databases', slugs: ['duckdb', 'sqlite', 'postgresql'] },
];

for (const s of SURFACES) {
  const homes = TAB_GROUPS.filter((g) => g.slugs.includes(s.slug));
  if (homes.length !== 1) throw new Error(`catalog: the surface ${s.slug} sits in ${homes.length} tab groups`);
}
for (const g of TAB_GROUPS) {
  for (const slug of g.slugs) {
    if (!SURFACES.some((s) => s.slug === slug)) throw new Error(`catalog: the tab group ${g.name} names ${slug}, which is not a surface`);
  }
}
for (const f of FUNCTIONS) {
  for (const slug of Object.keys(f.cannot || {})) {
    if (!SURFACES.some((s) => s.slug === slug)) throw new Error(`catalog: ${f.name} names ${slug} in cannot, which is not a surface`);
  }
}

// Captions for the tutorial's own examples, keyed by script name.
export const TUTORIAL_SEE = {
  '1-band': "\"I want to send this back.\" could mean an exchange or money back. It lands inside the band 0.2:0.8. decide prints null and exits 3.",
};

// The captions for the Caching and replay page's scripts.
export const CACHING_SEE = {
  '1-replay': 'The answers come from the folder. Nothing is sent, and no key is set.',
  '2-batch-one': 'One review a request reads the same saved answers and keeps the same two reviews.',
  '3-unused': 'The folder holds one answer the four reviews never asked for.',
};

// The Functional patterns page's caption for functions/filter/1-lines.
export const FUNCTIONAL_PIPE_SEE = 'filter keeps the lines whose answer is yes, in order.';

// The tutorial's caption for functions/decide/1-lines in its stream step.
export const TUTORIAL_STREAM_SEE = "Each answer sits beside its message. The send-back line is the null from step 3.";

// Captions for the Settings page's examples, keyed by script name.
export const SETTINGS_SEE = {
  '1-environment': 'THINKTHEN_BASE_URL names the address. The plan shows the request going there.',
  '2-flag': 'The same variable is set, and --url names another address. The flag wins.',
};

// The Backends section under /install/backends/, in side-list order. Each
// page runs the scripts in examples/install/backends/<slug>/, and `see`
// says what to look for in each.
export const BACKEND_PAGES = [
  { slug: '', title: 'Backends', label: 'Overview', group: null },
  { slug: 'typesafe', title: 'TypeSafe Jev', label: 'TypeSafe Jev', group: 'Built in' },
  { slug: 'liquid', title: 'Liquid d1', label: 'Liquid d1', group: 'Built in' },
  { slug: 'ollama', title: 'Ollama', label: 'Ollama', group: 'Built in' },
  { slug: 'system-one', title: 'Any System One server', label: 'Any System One server', group: 'Your own' },
  { slug: 'other-servers', title: 'Servers without System One', label: 'Servers without System One', group: 'Your own' },
  { slug: 'openai', title: 'OpenAI Decisions API', label: 'OpenAI Decisions API', group: 'Announced' },
].map((p) => ({ ...p, route: `/install/backends/${p.slug ? `${p.slug}/` : ''}` }));

// The three built-in backends each language page shows, in this order. The
// key variables repeat the "Named backends" table in
// specification/backends.md so the pages can name them, and the check
// below fails the build when they differ. Ollama's address is the second
// port its recordings sit at, as the Ollama page explains.
export const BACKEND_ROUTES = [
  { name: 'typesafe', keys: ['TYPESAFE_API_KEY'] },
  { name: 'liquid', keys: ['LIQUIDAI_API_KEY', 'LIQUID_API_KEY'] },
  { name: 'ollama', keys: ['OLLAMA_API_KEY'], address: 'http://localhost:11535/v1' },
].map((r) => {
  const page = BACKEND_PAGES.find((p) => p.slug === r.name);
  if (!page) throw new Error(`catalog: BACKEND_ROUTES names ${r.name}, which has no BACKEND_PAGES entry`);
  const row = backend(r.name);
  if (row.keys.join(', ') !== r.keys.join(', ')) {
    throw new Error(`catalog: BACKEND_ROUTES gives ${r.name} the key variables ${r.keys.join(', ')}, and specification/backends.md gives ${row.keys.join(', ')}`);
  }
  return { ...r, title: page.title, route: page.route, base: r.address ?? row.base };
});

// What each language page says about OpenAI's Decisions API. It states the
// announcement and nothing more.
export const OPENAI_ANNOUNCED = 'OpenAI announced its Decisions API on 2026-09-29 in its <a href="https://openai.com/index/devday-2026-recap/">DevDay 2026 recap</a>. OpenAI has published no address, schema or price, so ThinkThen cannot call it. The <a href="/install/backends/openai/">OpenAI Decisions API</a> page says what ThinkThen knows.';

export const BACKENDS_SEE = {
  typesafe: {
    '1-one': 'Jev answers yes. Sending the item back asks for a refund.',
    '2-lines': 'One request carries all three lines. The thanks is the only no.',
    '3-check-plan': "The check would post to TypeSafe's address with Jev's pinned model. The plan sends nothing.",
  },
  liquid: {
    '1-status': 'status names the address, the model, and the variable the key comes from. It sends nothing.',
    '2-one': 'd1 answers no and exits 1. Jev answers yes to the same line.',
    '3-lines': 'One request carries all three lines. d1 says yes to the broken order and no to the send-back line.',
    '4-check-plan': "The check would post to Liquid's address with d1:free. The plan sends nothing.",
  },
  ollama: {
    '1-check-plan': 'The plan names the second port and nimble. The first line says descriptions travel as text.',
    '2-one': 'The nimble model answers yes, as Jev does.',
    '3-lines': 'One request carries all three lines. The nimble model gives the same three answers as Jev.',
    '4-tev1': 'The smaller tev1 model also answers yes.',
  },
  'system-one': {
    '1-environment': 'THINKTHEN_BASE_URL names the server. The address came from the environment, and the key will come from THINKTHEN_API_KEY.',
    '2-entry': 'The entry local-d1 brings its own address, model, and key variable.',
    '3-bad-rate': 'A rate of 0 is out of range. The command names the field, exits 5, and sends nothing.',
    '4-check-plan': "The check would post its four fixed questions to the server's systemone address.",
  },
  'other-servers': {
    '1-request': 'The plan prints the exact body a server receives at BASE/systemone.',
  },
};

// Captions for the Configuration page's examples, keyed by script name.
export const CONFIGURATION_SEE = {
  '1-default': 'With no XDG variable, all three paths sit under HOME.',
  '2-xdg': 'Absolute XDG variables move the configuration file, the cache, and the usage totals.',
  '3-relative': 'A relative XDG_CONFIG_HOME does not count. The path falls back to HOME.',
  '4-file': 'The file names liquid. THINKTHEN_BACKEND outranks the file and names ollama.',
};

// The /how-tos/ index shows one section per group, in this order. Each
// section lists its pages in HOWTOS order. A group with no pages shows
// nothing.
export const HOWTO_GROUPS = ['Business teams', 'Data science', 'Ops/Security'];

// The how-tos. Each page runs the scripts in examples/how-tos/<slug>/, and
// `see` says what to look for in each. `group` names its HOWTO_GROUPS
// section.
export const HOWTOS = [
  {
    slug: 'triage-a-support-inbox', group: 'Business teams', title: 'Triage a support inbox', reader: 'for support teams',
    goal: 'Two commands in a pipe keep the messages that need a reply and label each by kind and urgency.',
    said: 'Keep the messages that need a reply, and label each by kind and urgency. `filter` keeps them. `annotate` labels them.',
    functions: ['filter', 'annotate'],
    see: { '1-inbox': 'Three messages need a reply, each beside its kind and its urgency from 0 to 2. The order needed tonight sits near 2, Immediate. The thank-you note drops out.' },
  },
  {
    slug: 'join-two-tables-by-meaning', group: 'Data science', title: 'Join two tables by meaning', reader: 'for data analysts',
    goal: 'filter joins two tables on meaning when no key and no shared word links them.',
    said: 'Match each ticket to the incident it describes, even when the words differ. The loop pairs every ticket with every incident. `filter` keeps the pairs that match.',
    functions: ['filter'],
    see: { '1-join': 'Of the four pairs, the two that match come back: the card failure with the payment gateway, and the late export with the export queue.' },
  },
  {
    slug: 'code-open-ended-survey-answers', group: 'Data science', title: 'Sort survey answers by mood and problem', reader: 'for survey and market researchers',
    goal: 'score finds the unhappy answers and tag names what went wrong in each.',
    said: 'Place every answer between unhappy and happy, then name what went wrong. `score` places them. `tag` names the problem.',
    functions: ['score', 'tag'],
    see: { '1-answers': 'Three answers are unhappy. Each prints beside its tag: price, bugs, and speed. The happy answer drops out.' },
  },
  {
    slug: 'screen-studies-for-a-review', group: 'Data science', title: 'Screen studies for a review', reader: 'for researchers',
    goal: 'A band sorts the clear studies in or out and hands a person the ones too thin to judge.',
    said: 'Sort the clear studies in or out, and hand a person the ones that give too little to judge. `decide` with a band does both.',
    functions: ['decide'],
    see: { '1-studies': 'The survey with a result is true and the opinion essay is false. The bare title gives too little to judge. Inside the band 0.1:0.9, it is not sure. It prints null, and a person reads it.' },
  },
  {
    slug: 'group-alerts-into-incidents', group: 'Ops/Security', title: 'Group alerts into incidents', reader: 'for on-call engineers',
    goal: 'find picks the open incident a new alert belongs to, or none, and decide confirms the match.',
    said: 'Tell whether a new alert belongs to an open incident. `find` picks the incident, or none. `decide` confirms the match.',
    functions: ['find', 'decide'],
    see: {
      '1-match': 'find picks INC-1 for the card failure, and decide confirms the match with true.',
      '2-none': 'No open incident covers a full disk. find prints nothing, and decide never runs.',
    },
  },
  {
    slug: 'rank-the-inbound-leads', group: 'Business teams', title: 'Rank the inbound leads', reader: 'for sales teams',
    goal: 'Three functions in one pipe drop the noise, order the leads, and route each to a team.',
    said: 'Drop the noise, put the buyer ready to pay first, and send each to the right sales team. `filter`, `rank`, and `choose` do it in one pipeline.',
    functions: ['filter', 'rank', 'choose'],
    see: { '1-leads': 'The unsubscribe and the compliment on the talk drop out. Neither asks to buy. The team of six buying today comes first and goes to smb. The 200 seats next quarter go to enterprise.' },
  },
  {
    slug: 'check-an-expense-against-the-policy', group: 'Business teams', title: 'Check an expense against the policy', reader: 'for finance staff',
    goal: 'find pulls the policy rule that covers an expense, and decide says whether the expense fits it.',
    said: 'Check an expense against your policy. `find` pulls the rule. `decide` says whether the expense fits.',
    functions: ['find', 'decide'],
    see: { '1-expense': 'The rule find pulled prints beside the answer. The $60 dinner fits the $75 meal rule.' },
  },
  {
    slug: 'split-a-scanned-packet-into-documents', group: 'Business teams', title: 'Split a scanned packet into documents', reader: 'for back-office staff',
    goal: 'choose names each page and decide marks where a new document starts.',
    said: 'Split a stack of scanned pages into documents. `choose` says what kind each page is. `decide` marks where a new document starts.',
    functions: ['choose', 'decide'],
    see: {
      '1-kinds': 'Three invoice pages and one notice.',
      '2-gaps': 'awk pairs each page with the one before it. The two pages of Invoice 7 stay together. A new document starts at Invoice 8 and at the notice.',
    },
  },
  {
    slug: 'screen-a-post-before-it-goes-up', group: 'Business teams', title: 'Screen a post before it goes up', reader: 'for community moderators',
    goal: 'One narrow decide question per rule judges a post against each rule on its own.',
    said: 'Judge a post against each rule on its own. `decide` asks one narrow question per rule.',
    functions: ['decide'],
    see: {
      '1-insult': 'The post calls the author an idiot. decide says true and exits 0.',
      '2-spam': 'The post is not spam. decide says false and exits 1.',
    },
  },
];

// The Bash techniques and recipes under /how-tos/bash/. Each page runs the
// scripts in examples/how-tos/bash/<slug>/. A technique teaches one shell
// form. A recipe does one job with the tools you already have.
export const TECHNIQUES = [
  {
    slug: 'if', title: 'Branch with if', label: 'if',
    goal: '`if` reads the exit code of `decide --quiet` directly.',
    said: '`decide --quiet` prints nothing. Its exit code is the answer. The function `asks_for_refund` names what that code means, and `if` reads it directly.',
    see: { '1-if': 'The ticket asks for a refund. The if branch picks the refunds queue.' },
  },
  {
    slug: 'case', title: 'Route with case', label: 'case',
    goal: '`case` routes on the bare label `choose --raw` prints.',
    said: '`choose --raw` prints the bare label. `case` sends each label to its own queue.',
    see: { '1-case': 'The password message belongs to account. The queue is identity.' },
  },
  {
    slug: 'not-sure', title: 'Handle not sure', label: 'not sure',
    goal: 'A script reads three exit codes and sends not sure to a person.',
    said: '`refund_code=$?` names the exit code of `decide`. `case` reads its three values: 0 for yes, 1 for no, and 3 for not sure.',
    see: { '1-route': 'The refund goes to refunds and the thanks gets a reply. The send-back line lands in the band 0.2:0.8 and goes to a person.' },
  },
  {
    slug: 'threshold-band', title: 'Set a cut or a band', label: 'cut or band',
    goal: 'One number is a cut, and two numbers make a band with a not-sure middle.',
    said: 'One number is a cut. Two numbers are a band. An answer inside the band is not sure, and it prints null.',
    see: {
      '1-cut': 'At a cut of 0.5, the send-back line counts as a refund.',
      '2-band': 'With a band from 0.2 to 0.8, the same line is not sure, and it goes to a person.',
    },
  },
  {
    slug: 'while-read', title: 'Loop over lines', label: 'while read',
    goal: '`while read` acts on the answer for each line.',
    said: '`while read` hands each line to `decide` on its own. The loop acts on each answer.',
    see: { '1-loop': 'The two complaints open a case. The thanks and the question about blue do not.' },
  },
  {
    slug: 'pipeline', title: 'Chain questions with pipes', label: 'pipes',
    goal: 'Two filters in a pipe keep the records that pass both questions.',
    said: 'Two filters in a row keep the records that pass both questions.',
    see: { '1-and': 'The first filter keeps what is about the item. The second keeps the complaints among them. The question about blue is not a complaint. It drops out.' },
  },
  {
    slug: 'batches', title: 'Ask many files at once', label: 'xargs',
    goal: '`xargs -P` asks one question of many files in parallel.',
    said: 'A function asks one question of one file. `xargs -P 4` runs it on four files at a time.',
    see: { '1-xargs': 'Two tickets ask for money back: T-1 and T-3.' },
  },
  {
    slug: 'ci-gate', title: 'Fail a build', label: 'CI gate',
    goal: 'A filter in a check script fails the build when a line matches.',
    said: '`filter` finds the lines that hedge. The script exits 1 when it finds any, and the build fails.',
    see: {
      '1-pass': 'No line hedges. The check prints nothing and passes.',
      '2-fail': 'One line hedges. The check prints it and exits 1.',
    },
  },
  {
    slug: 'long-lived-loop', title: 'Keep one process for a step loop', label: 'one process',
    goal: 'A coproc sends each step to one choose process and reads its answer before the next step.',
    said: '`coproc` holds one `choose` process open. `--batch 1` prints each answer while the input stays open. The example replays a recording. It needs no key and no network. Use a library binding when each call must cost less time.',
    see: { '1-loop': 'Three changing action lists produce three answers from one process.' },
  },
  {
    slug: 'judge-paragraphs', title: 'Judge one paragraph at a time', label: 'paragraphs',
    goal: 'awk splits a document into paragraphs, jq makes JSON records, and filter judges each paragraph.',
    said: '`awk -v RS=` splits on blank lines. `jq` gives each paragraph a `text` field, and `filter --field /text` sends only that text. Use a sentence splitter for sentences or a parser for code functions.',
    see: { '1-paragraphs': 'The two complaint paragraphs return as JSON records.' },
  },
  {
    slug: 'agent-tool-guard', title: 'Guard a coding agent tool call', label: 'tool guard',
    goal: 'Map a bounded decide answer to one coding agent host hook contract.',
    said: 'This Claude Code `PreToolUse` hook reads a proposed Bash command as text. It runs none of the proposals. A yes gives `allow`, a no gives `deny`, and not sure gives `ask`. A failed call also gives `deny`. The hook answers in JSON. In ThinkThen, exit 2 means a usage or input error. In a hook, exit 2 blocks the tool call.',
    source: ['Claude Code hooks reference, checked 2026-09-28', 'https://code.claude.com/docs/en/hooks#pretooluse-decision-control'],
    see: { '1-guard': 'One recorded proposal is allowed, one asks a person, and one is denied.' },
  },
];

export const RECIPES = [
  {
    slug: 'label-a-json-file', title: 'Label a JSON file and keep its ids', label: 'Label a JSON file',
    goal: 'annotate labels a JSON array and keeps every other field, and a second run costs nothing.',
    said: 'Label every ticket in a JSON array by kind and urgency. `--field /body` sends only the body. The id and the date stay in each record. Run it again, and the answers come from the answer cache with no request sent.',
    see: {
      '1-label': 'Each ticket keeps its id and date, and gains a kind and an urgency from 0 to 2. The double bill in September is billing.',
    },
  },
  {
    slug: 'review-a-diff-by-what-it-does', title: 'Review a diff by what it does', label: 'Review a diff',
    goal: 'decide separates the hunks of a diff that change behavior from those that do not.',
    said: '`jq` cuts a unified diff into hunks. `decide` asks of each hunk whether it changes what the code does. The file name and the hunk header stay in each record.',
    see: { '1-diff': 'Two hunks change what the code does: the refund limit and the rounded tax. The comment and the rename do not.' },
  },
  {
    slug: 'lint-prose-for-hedging', title: 'Lint prose for hedging', label: 'Lint prose',
    goal: 'filter finds the hedging lines in a draft and keeps their line numbers.',
    said: '`jq` numbers the lines. `filter` keeps the lines that hedge, and each comes back as it went in, line number and all.',
    see: { '1-lint': 'The two hedging lines come back with their line numbers.' },
  },
  {
    slug: 'fill-a-form-by-selection', title: 'Fill a form by selection', label: 'Fill a form',
    goal: 'annotate fills a form with picks from your own lists, and no field holds model-written text.',
    said: 'Every field is a pick from a list you wrote, or true or false. No character in the form comes from a model.',
    see: { '1-form': 'Each request gets a plan, a topic, and whether to call back, all from the lists in the form.' },
  },
  {
    slug: 'set-aside-bad-records', title: 'Set bad records aside first', label: 'Set aside bad records',
    goal: 'Split malformed and non-text records before a record run, then judge only valid inputs.',
    said: '`jq` reads each raw line and keeps only JSON objects with a string `body`. It writes every other line to an aside file before `decide` sees the good records. A record run otherwise stops at the first bad record.',
    see: { '1-split': 'The malformed line and the numeric body stay aside. decide judges the three text records.' },
  },
];

// The Bash section in side-list order. Its first page is the section's index.
export const BASH_FIRST = { slug: '', title: 'Bash techniques', label: 'Bash techniques', route: '/how-tos/bash/', group: null };
export const BASH = [
  BASH_FIRST,
  ...TECHNIQUES.map((t) => ({ ...t, group: 'Techniques', route: `/how-tos/bash/${t.slug}/` })),
  ...RECIPES.map((r) => ({ ...r, group: 'Recipes', route: `/how-tos/bash/${r.slug}/` })),
];
