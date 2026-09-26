// The names, the order, and the one line for every function and surface.
//
// The one line for each function is the first line of its help text. The
// option lists come from specification/. The examples live in examples/, and
// scripts/smoke.mjs runs them.

export const TAGLINE = 'ThinkThen: code that knows what you mean';

// Set to false the day the site is linked from anywhere.
export const NOINDEX = true;

export const KEY_VARIABLE = 'THINKTHEN_API_KEY';

export const REPO = 'https://github.com/botassembly/thinkthen';

// The four outcomes. One color each, everywhere a number or an answer shows.
export const OUTCOMES = [
  { key: 'yes', code: 0, label: 'yes', prints: 'true' },
  { key: 'no', code: 1, label: 'no', prints: 'false' },
  { key: 'unsure', code: 3, label: 'not sure', prints: 'null' },
  { key: 'broken', code: '2, 4, 5, 6, 70', label: 'broken', prints: 'nothing' },
];

export function outcomeOf(exit) {
  if (exit === 0) return 'yes';
  if (exit === 1) return 'no';
  if (exit === 3) return 'unsure';
  return 'broken';
}

// The backend and cache options every shipped function takes, from
// specification/backends.md and specification/recording.md.
const BACKEND_OPTIONS = [
  ['--model NAME', 'The model the request carries. Name a version to pin a run. The default is jev-latest.'],
  ['--url BASE', 'The backend base address. It outranks THINKTHEN_BASE_URL.'],
  ['--timeout SECONDS', 'How long one attempt may take. The default is 30.'],
  ['--max-retries N', 'How many times a retried status is sent again. A transport failure is never sent again. The default is 2.'],
  ['--record DIR', 'Calls the backend and saves each exchange in DIR.'],
  ['--replay DIR', 'Answers from DIR alone, with no key and no network.'],
  ['--cache DIR', 'Answers from DIR when it can and saves new exchanges there.'],
  ['--no-cache', 'Turns off the saved answers for one run.'],
  ['--profile FILE', 'Applies local backend limits and names the calibration profile in use.'],
];

const COMMON_OPTIONS = [
  ['--details', 'Prints the whole result in place of the bare value: the probabilities, the question, and the run.'],
  ['--input FILE', 'Reads the evidence from a file instead of standard input.'],
  ['--dry-run', 'Prints the plan and sends nothing. It needs no key.'],
  ['--lines, --jsonl, --csv, --tsv', 'Says how a stream of records is framed. Pick one.'],
  ['--field POINTER', 'Names the part of each record to judge, as a JSON Pointer. It may repeat.'],
  ['--jobs N', 'How many requests run at once, from 1 to 32. The default is 4. It works on a stream of records. annotate also takes it on one document.'],
  ...BACKEND_OPTIONS,
];

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
    primitive: 'Yes or no',
    line: 'Answer one yes or no question about the evidence.',
    takes: 'one question and one piece of evidence',
    gives: 'true, false, or null',
    toPerson: true,
    requests: 'One request for one piece of evidence. One request for each record in a stream.',
    args: 'QUESTION or @FILE',
    options: [
      ['--threshold T or LOW:HIGH', 'The bar the probability of yes must reach. One number is a cut. Two numbers are a band, and the middle is not sure. The default is 0.5.'],
      ['--true TEXT', 'What a yes means, in the words the model reads.'],
      ['--false TEXT', 'What a no means.'],
      ['--quiet', 'Prints nothing. The exit code carries the answer.'],
      ...COMMON_OPTIONS,
    ],
    exits: [[0, 'yes'], [1, 'no'], [3, 'not sure'], ...COMMON_EXITS],
    unsure: 'The probability landed inside the band. The tool prints null and exits 3.',
    howtos: ['screen-studies-for-a-review', 'group-alerts-into-incidents', 'screen-a-post-before-it-goes-up', 'check-an-expense-against-the-policy', 'split-a-scanned-packet-into-documents'],
    see: {
      '1-lines': "The refund request answers true and the thank-you note false. \"I want to send this back.\" could mean an exchange or money back, so it lands inside the band as null.",
      '2-case': "Sending it back could mean an exchange or money back. decide exits 3, and the case sends it to a person. The script then exits 0.",
      '3-one': "The ticket asks for a refund, so decide prints true and exits 0.",
      '4-details': "The details put yes at 0.99 for this ticket.",
      '5-means': "A cancellation is not money back under these words, so decide says false and exits 1.",
    },
  },
  {
    name: 'choose',
    primitive: 'Pick one',
    line: 'Pick one option from your list.',
    takes: 'one question, one piece of evidence, and 2 to 255 options',
    gives: 'one of your options, or null',
    toPerson: true,
    requests: 'One request for one piece of evidence. One request for each record in a stream.',
    args: 'QUESTION or @FILE, then OPTION...',
    options: [
      ['--threshold T', 'The bar the winning option must reach. One number only. A band is a usage error. There is no default.'],
      ['--option LABEL=DESCRIPTION', 'One option and what it means. It may repeat, and it replaces the positional options.'],
      ['--options POINTER', 'Takes the options from each record. It needs --jsonl.'],
      ['--raw', 'Prints the label without quotation marks.'],
      ['--quiet', 'Prints nothing. The exit code carries the run.'],
      ...COMMON_OPTIONS,
    ],
    exits: [[0, 'an option came back'], [3, 'not sure'], ...COMMON_EXITS],
    unsure: 'No option reached the threshold. The tool prints null and exits 3. choose never exits 1.',
    howtos: ['rank-the-inbound-leads', 'split-a-scanned-packet-into-documents'],
    see: {
      '1-lines': "Each of the first three messages names one team: billing, shipping, and account. The fourth names a parcel and a login, so no team reaches 0.9 and it comes back null.",
    },
  },
  {
    name: 'tag',
    primitive: 'Yes or no, per label',
    line: 'Name every label that fits.',
    takes: 'one question, one piece of evidence, and 1 to 20 labels',
    gives: 'the labels that fit, as a list',
    requests: 'Every label rides in one request. One request for one piece of evidence, whatever the label count.',
    args: 'QUESTION or @FILE, then LABEL...',
    options: [
      ['--threshold T', 'The bar every label must reach on its own. The default is 0.5. A band is refused.'],
      ['--label LABEL=DESCRIPTION', 'A label and what it means. It may repeat, and it replaces the positional labels.'],
      ...COMMON_OPTIONS,
    ],
    exits: [[0, 'the run finished'], ...COMMON_EXITS],
    unsure: 'A label under the bar is left out. An empty list is a good answer and exits 0.',
    howtos: ['code-open-ended-survey-answers'],
    see: {
      '1-one': "The message praises the dashboard, reports a crash, and names a double charge, so it gets praise, bug, and billing.",
      '2-details': "praise, bug, and billing each clear 0.5.",
      '3-labels': "Each line gets the one label its description fits.",
    },
  },
  {
    name: 'score',
    primitive: 'Place on a scale',
    line: 'Place the evidence on a scale you name.',
    takes: 'one question, one piece of evidence, and 2 to 10 levels, least first',
    gives: 'a number along your levels. The first level is 0',
    requests: 'One request for one piece of evidence. One request for each record in a stream.',
    args: 'QUESTION or @FILE, then LEVEL...',
    options: [
      ...COMMON_OPTIONS,
    ],
    exits: [[0, 'the run finished'], ...COMMON_EXITS],
    unsure: 'score has no threshold and no not-sure answer. It always lands somewhere on the scale. It orders a queue a person reads. Do not use it as a gate.',
    howtos: ['code-open-ended-survey-answers'],
    see: {
      '1-one': "The outage scores 2.0, and 2 is Immediate on this scale.",
      '2-details': "The details put all of the weight on Immediate.",
      '3-lines': "The address change lands near 0, the Friday deadline near 1, and the login outage at 2.",
    },
  },
  {
    name: 'filter',
    primitive: 'Yes or no, per record',
    line: 'Keep the records where the answer is yes.',
    takes: 'one yes-or-no question and many records',
    gives: 'the records that pass, byte for byte, in the order they went in',
    requests: 'One request for each record.',
    args: 'QUESTION or @FILE. One of --lines, --jsonl, --csv, or --tsv is required.',
    options: [
      ['--threshold T', 'The bar a record must reach. The default is 0.5. A band is a usage error.'],
      ['--true TEXT', 'What a yes means, in the words the model reads.'],
      ['--false TEXT', 'What a no means.'],
      ...COMMON_OPTIONS,
    ],
    exits: [[0, 'the run finished'], ...COMMON_EXITS],
    unsure: 'No record sets the exit code. A record under the bar is dropped.',
    howtos: ['triage-a-support-inbox', 'join-two-tables-by-meaning', 'rank-the-inbound-leads'],
    see: {
      '1-lines': "filter keeps the two complaints: the broken zipper and the snapped strap.",
      '2-means': "The zipper, the color, and the strap are about the item, so they stay. The early delivery drops out.",
      '3-csv': "Only R-2 complains, and it comes back as one JSON object, id and all.",
    },
  },
  {
    name: 'rank',
    primitive: 'Yes or no, per record',
    line: 'Sort records by how likely the answer is yes.',
    takes: 'one yes-or-no question and many records',
    gives: 'every record again, most likely first',
    requests: 'One request for each record. --top trims the printed list and saves nothing.',
    args: 'QUESTION or @FILE. One of --lines, --jsonl, --csv, or --tsv is required.',
    options: [
      ['--top N', 'Prints the first N records of the order. Every record is still judged.'],
      ['--true TEXT', 'What a yes means, in the words the model reads.'],
      ['--false TEXT', 'What a no means.'],
      ...COMMON_OPTIONS,
    ],
    exits: [[0, 'the run finished'], ...COMMON_EXITS],
    unsure: 'rank takes no threshold, so nothing is dropped and nothing is not sure. The sort happens on this machine.',
    howtos: ['rank-the-inbound-leads'],
    see: {
      '1-lines': "The outage comes first and the quote due today second. The bill due in a month comes third, and the newsletter that needs no reply comes last.",
      '2-jsonl': "The checkout crash stops customers from buying, so B-2 comes back first, id and all.",
    },
  },
  {
    name: 'find',
    primitive: 'Pick one, over the lines of the evidence',
    line: 'Pick the one line that best answers a question.',
    takes: 'a question and 2 to 255 lines or records, or 2 to 254 with --none',
    gives: 'the one line that fits best',
    requests: 'It sends one request for the whole document.',
    args: 'QUESTION',
    options: [
      ['--none', 'Lets it answer that nothing fits. It then prints nothing and exits 3.'],
      ['--lines, --jsonl', 'How the lines are framed. --lines is the default. CSV and TSV are refused.'],
      ['--field POINTER', 'Names the part of each record to read.'],
      ['--details', 'Prints the whole result, with a probability for every line.'],
      ['--input FILE', 'Reads the evidence from a file instead of standard input.'],
      ['--dry-run', 'Prints the plan and sends nothing.'],
      ...BACKEND_OPTIONS,
    ],
    exits: [[0, 'a line came back'], [3, 'nothing fits, under --none'], ...COMMON_EXITS],
    unsure: 'Without --none, find must pick a line, and it will pick a wrong one. --none is how it says nothing fits. A document past the backend’s token limit is refused, and the command exits 4.',
    howtos: ['group-alerts-into-incidents', 'check-an-expense-against-the-policy'],
    see: {
      '1-find': "find prints the line with the 30 day refund deadline.",
      '2-details': "The second line holds all of the weight.",
      '3-none': "No line says how to cancel, so find prints nothing and exits 3.",
      '4-jsonl': "Q1 holds the reset steps, and the whole record comes back.",
    },
  },
  {
    name: 'annotate',
    primitive: 'Every kind of answer, many at once',
    line: 'Fill out a form for every record.',
    takes: 'a saved set of questions and your JSON',
    gives: 'the same JSON with one field added per question. Nested fields ride through unchanged',
    requests: 'It sends one request for each record and each part the questions read.',
    toPerson: true,
    args: 'FILE, the saved question set',
    options: [
      ...COMMON_OPTIONS,
    ],
    exits: [[0, 'every question was answered'], [6, 'the run finished with failed questions'], ...COMMON_EXITS.map(([code, what]) => [code, code === 5 ? 'the question set could not be read' : what])],
    unsure: 'Each question carries its own threshold, so a question set mixes cuts and bands. A question the backend could not answer is marked failed, counted, and never turned into null. The questions and the evidence together can pass the backend’s token limit for one request. The backend refuses it, and the command exits 4.',
    howtos: ['triage-a-support-inbox'],
    see: {
      '0-json': "One JSON document goes in. The same document comes back with three answers added: steps, area, and impact. The nested report rides through unchanged.",
      '1-jsonl': "Each report keeps its id and lands in its own area: login, billing, and export. The two with steps say true, and the blue button blocks nothing.",
    },
  },
  {
    name: 'recognize',
    primitive: 'Pick one, per word',
    line: 'Find every name in the evidence and say what kind it is.',
    takes: 'the evidence and the kinds of name you allow',
    gives: 'each name, its kind, where it sits, and a strength',
    requests: 'It asks one question about every word, and one more about its kind when you allow two or more kinds. --dry-run prints the exact requests for the first record.',
    args: 'KIND..., or one @FILE question file',
    options: [
      ['--kind KIND=DESCRIPTION', 'One kind and what it means.'],
      ['--threshold T', 'Keeps names whose strength reaches this cut. The default is 0.5.'],
      ['--relation NAME=SOURCE:TARGET', 'Also links the names it finds. This part is in beta.'],
      ['--relation-threshold T', 'Keeps relation edges whose probability reaches this cut. The default is 0.5.'],
      ...COMMON_OPTIONS,
    ],
    exits: [[0, 'the run finished'], ...COMMON_EXITS],
    unsure: 'The model only picks from options, so a name that is not in the evidence cannot come back. The number on a name is its strength. We compute it, and it claims nothing about chance. Your threshold decides which names you keep.',
    howtos: [],
  },
  {
    name: 'relate',
    primitive: 'Yes or no, or a direction, per pair of records',
    line: 'Find records that clash, repeat, or rely on each other.',
    lede: 'Find the rules in a policy that contradict each other. You give it a set of records and the relations you allow. You get back one edge for each related pair, with a probability. An edge is a link between two records. Here <code>contradicts</code> links two rules that cannot both be followed.',
    takes: 'a set of records and the relations you allow',
    gives: 'one edge for each related pair, with a probability',
    requests: 'It reads the whole set at once, up to 255 records. --dry-run prints every request it would send.',
    args: 'RELATION... as NAME=SOURCE_KIND:TARGET_KIND or a bare NAME, or one @FILE',
    options: [
      ['--either', 'Treats every relation as reading the same both ways.'],
      ['--threshold T', 'Keeps edges whose probability reaches this cut. The default is 0.5.'],
      ['--kind-field POINTER', 'Reads each record\'s kind from this pointer.'],
      ...COMMON_OPTIONS,
    ],
    exits: [[0, 'the run finished'], [6, 'the run finished with failed questions'], ...COMMON_EXITS],
    unsure: 'A relation has a direction, or it reads the same both ways. The number on an edge is a probability. Your threshold decides which edges you keep.',
    howtos: [],
  },
  {
    name: 'question-file',
    title: '@question',
    primitive: 'Not a function',
    line: 'A saved question every function accepts.',
    lede: 'Save one question in a JSON file. Pass it as <code>@FILE</code> to decide, choose, tag, score, filter, or rank. The hook, the test, and the pipeline then ask the same question.',
    requests: 'None of its own. The function that reads it sends the requests.',
    args: '@FILE in place of the question words, on decide, choose, tag, score, filter, and rank.',
    options: [
      ['the verb key', 'One of decide, choose, tag, or score. It names the verb and carries the question text.'],
      ['true, false', 'What a yes and a no mean, for a decide question.'],
      ['options, labels', 'A list, or a map from label to what it means.'],
      ['levels', 'The scale, lowest first.'],
      ['threshold', 'A cut, or a band written LOW:HIGH.'],
      ['on', 'One JSON Pointer, or a list of them.'],
      ['model, profile', 'The model to ask and the calibration profile to apply.'],
    ],
    exits: [[5, 'the file could not be read, is not one JSON object, or breaks a rule'], [2, 'the command names the wrong verb for the file']],
    unsure: 'A band in the file sends the answers it is not sure of to a person. Tune it once, and the hook, the test, and the pipeline all run the same question.',
    howtos: [],
    see: {
      '1-yes': "The saved question answers true for a plain request for money back.",
      '2-unsure': "Sending it back could mean an exchange or money back, so the saved question prints null and exits 3.",
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


export const SURFACES = [
  {
    slug: 'shell', name: 'Bash', deckHeading: null,
    lang: 'bash', tab: 'Bash',
    blurb: 'Pipe text in, read the answer out, and branch on the exit code.',
    unsureWord: 'null, and exit code 3',
    install: [
      ['curl -fsSL https://thinkthen.dev/install.sh | sh', 'Download script.'],
      ['brew install botassembly/thinkthen/thinkthen', 'Homebrew, an option on a Mac.'],
    ],
    particular: [
      'Standard input carries the evidence. Standard output carries the answer and nothing else.',
      'On one piece of evidence, the exit code is the answer. `if` and `case` read it directly.',
      '`--jobs` sets how many requests run at once.',
      '`--dry-run` prints the plan and needs no key.',
    ],
  },
  {
    slug: 'python', name: 'Python', deckHeading: 'Python',
    lang: 'python', tab: 'Python',
    blurb: 'Pass a string or a list, and get `True`, `False`, or `None` back. Build a question once and reuse it.',
    unsureWord: 'None',
    install: [['pip install thinkthen', null], ['uv add thinkthen', null]],
    particular: [
      'A list goes in and a list comes out. The list crosses into the engine once.',
      '`tt.question()` builds a question that carries its own threshold. Reuse it wherever you ask.',
    ],
  },
  {
    slug: 'polars', name: 'Polars', deckHeading: 'Polars',
    lang: 'python', tab: 'Python',
    blurb: 'A Polars frame goes in, and it comes back with one new column for each question.',
    unsureWord: 'None',
    install: [['pip install thinkthen[polars]', null]],
    particular: [
      'Rust reads the column where it sits. There is no copy and no Python loop.',
      '`on=` names the column the questions read.',
    ],
  },
  {
    slug: 'typescript', name: 'TypeScript', deckHeading: 'TypeScript',
    lang: 'ts', tab: 'TypeScript',
    blurb: 'Ten async functions. Pass one options object and await the answer.',
    unsureWord: 'null',
    install: [['npm install thinkthen', null], ['pnpm add thinkthen', null], ['bun add thinkthen', null]],
    particular: [
      'An AbortSignal cancels a batch and stops its bill.',
      'Every call returns a promise. An array crosses once.',
    ],
  },
  {
    slug: 'ruby', name: 'Ruby', deckHeading: 'Ruby',
    lang: 'ruby', tab: 'Ruby',
    blurb: 'Ten module methods. Any Enumerable goes in.',
    unsureWord: 'nil',
    install: [['gem install thinkthen', null]],
    particular: ['Any Enumerable crosses to the engine once.'],
  },
  {
    slug: 'r', name: 'R', deckHeading: 'R',
    lang: 'r', tab: 'R',
    blurb: 'Ten `tt_` functions that work inside dplyr.',
    unsureWord: 'NA',
    install: [['install.packages("thinkthen")', null]],
    particular: [
      'A column goes in and a column comes out.',
      'dplyr\'s `filter()` drops NA rows. A not-sure answer leaves the pipeline on its own.',
    ],
  },
  {
    slug: 'rust', name: 'Rust', deckHeading: 'Rust',
    lang: 'rust', tab: 'Rust',
    blurb: 'Call the engine directly. The compiler makes you handle not sure.',
    unsureWord: 'Answer::Unsure',
    install: [['cargo add thinkthen', null]],
    particular: [
      'Calls block. No async runtime comes with it.',
    ],
  },
  {
    slug: 'c', name: 'C', deckHeading: 'C',
    lang: 'c', tab: 'Rust',
    blurb: 'One header and one library. Bind ThinkThen to any language.',
    unsureWord: 'an outcome of THINKTHEN_UNSURE',
    install: [['thinkthen.h + libthinkthen', 'One archive per platform, with the header, both libraries, and a .pc file.']],
    particular: [
      'Every call returns 0 or an error kind.',
      'The answer lands in a struct: the outcome and its probability.',
    ],
  },
  {
    slug: 'duckdb', name: 'DuckDB', deckHeading: 'DuckDB',
    lang: 'sql', tab: 'SQL',
    blurb: 'Ask a question in WHERE, SELECT, or ORDER BY.',
    unsureWord: 'NULL',
    install: [['duckdb -unsigned', 'DuckDB loads the extension unsigned. The query loads the extension file first.']],
    particular: ['A whole column chunk crosses at once.'],
  },
  {
    slug: 'sqlite', name: 'SQLite', deckHeading: 'SQLite',
    lang: 'sql', tab: 'SQL',
    blurb: 'One warm pass answers the whole table. Every later query reads the saved answers.',
    unsureWord: 'NULL',
    install: [['.load ./thinkthen', null]],
    particular: [
      'SQLite calls a function one row at a time. `thinkthen_warm` answers the whole table in one pass first.',
    ],
  },
  {
    slug: 'postgresql', name: 'PostgreSQL', deckHeading: 'PostgreSQL',
    lang: 'sql', tab: 'SQL',
    blurb: 'One extension. Ask questions in any query.',
    unsureWord: 'NULL',
    install: [['CREATE EXTENSION thinkthen;', null]],
    particular: [
      'A question file carries a band. The not-sure rows come back NULL, and a person reads them.',
      'pg_cancel_backend and statement_timeout stop a query and its bill.',
    ],
  },
];

// The tabs on the home page sample, and on every code block that has variants.
export const TABS = ['Bash', 'Python', 'TypeScript', 'Ruby', 'R', 'Rust', 'SQL'];

// Which surface each tab draws from.
export const TAB_SURFACE = {
  Bash: 'shell', Python: 'python', TypeScript: 'typescript',
  Ruby: 'ruby', R: 'r', Rust: 'rust', SQL: 'duckdb',
};

// The business how-tos. Each page runs the scripts in
// examples/how-tos/<slug>/, and `see` says what to look for in each.
export const HOWTOS = [
  {
    slug: 'triage-a-support-inbox', title: 'Triage a support inbox', reader: 'for support teams',
    said: 'Keep the messages that need a reply, and label each by kind and urgency. `filter` keeps them. `annotate` labels them.',
    functions: ['filter', 'annotate'],
    see: { '1-inbox': 'Three messages need a reply, each beside its kind and its urgency from 0 to 2. The order needed tonight sits near 2, Immediate. The thank-you note drops out.' },
  },
  {
    slug: 'screen-studies-for-a-review', title: 'Screen studies for a review', reader: 'for researchers',
    said: 'Sort the clear studies in or out, and hand a person the ones that give too little to judge. `decide` with a band does both.',
    functions: ['decide'],
    see: { '1-studies': 'The survey with a result is true and the opinion essay is false. The bare title gives too little to judge, so it comes back null for a person.' },
  },
  {
    slug: 'code-open-ended-survey-answers', title: 'Sort survey answers by mood and problem', reader: 'for survey and market researchers',
    said: 'Place every answer between unhappy and happy, then name what went wrong. `score` places them. `tag` names the problem.',
    functions: ['score', 'tag'],
    see: { '1-answers': 'Three answers are unhappy. Each prints beside its tag: price, bugs, and speed. The happy answer drops out.' },
  },
  {
    slug: 'join-two-tables-by-meaning', title: 'Join two tables by meaning', reader: 'for data analysts',
    said: 'Match each ticket to the incident it describes, even when the words differ. The loop pairs every ticket with every incident. `filter` keeps the pairs that match.',
    functions: ['filter'],
    see: { '1-join': 'Of the four pairs, the two that match come back: the card failure with the payment gateway, and the late export with the export queue.' },
  },
  {
    slug: 'group-alerts-into-incidents', title: 'Group alerts into incidents', reader: 'for on-call engineers',
    said: 'Tell whether a new alert belongs to an open incident. `find` picks the incident, or none. `decide` confirms the match.',
    functions: ['find', 'decide'],
    see: {
      '1-match': 'find picks INC-1 for the card failure, and decide confirms the match with true.',
      '2-none': 'No open incident covers a full disk. find prints nothing, so decide never runs.',
    },
  },
  {
    slug: 'rank-the-inbound-leads', title: 'Rank the inbound leads', reader: 'for sales teams',
    said: 'Drop the noise, put the buyer ready to pay first, and send each to the right sales team. `filter`, `rank`, and `choose` do it in one pipeline.',
    functions: ['filter', 'rank', 'choose'],
    see: { '1-leads': 'The unsubscribe and the compliment on the talk drop out. Neither asks to buy. The team of six buying today comes first and goes to smb. The 200 seats next quarter go to enterprise.' },
  },
  {
    slug: 'screen-a-post-before-it-goes-up', title: 'Screen a post before it goes up', reader: 'for community moderators',
    said: 'Judge a post against each rule on its own. `decide` asks one narrow question per rule.',
    functions: ['decide'],
    see: {
      '1-insult': 'The post calls the author an idiot, so decide says true and exits 0.',
      '2-spam': 'The post is not spam, so decide says false and exits 1.',
    },
  },
  {
    slug: 'check-an-expense-against-the-policy', title: 'Check an expense against the policy', reader: 'for finance staff',
    said: 'Check an expense against your policy. `find` pulls the rule. `decide` says whether the expense fits.',
    functions: ['find', 'decide'],
    see: { '1-expense': 'The rule find pulled prints beside the answer. The $60 dinner fits the $75 meal rule.' },
  },
  {
    slug: 'split-a-scanned-packet-into-documents', title: 'Split a scanned packet into documents', reader: 'for back-office staff',
    said: 'Split a stack of scanned pages into documents. `choose` says what kind each page is. `decide` marks where a new document starts.',
    functions: ['choose', 'decide'],
    see: {
      '1-kinds': 'Three invoice pages and one notice.',
      '2-gaps': 'awk pairs each page with the one before it. The two pages of Invoice 7 stay together. A new document starts at Invoice 8 and at the notice.',
    },
  },
];

// The Bash techniques and recipes under /how-tos/bash/. Each page runs the
// scripts in examples/how-tos/bash/<slug>/. A technique teaches one shell
// form. A recipe does one job with the tools you already have.
export const TECHNIQUES = [
  {
    slug: 'if', title: 'Branch with if', label: 'if',
    said: '`decide --quiet` prints nothing. Its exit code is the answer, so `if` reads it directly.',
    see: { '1-if': 'The ticket asks for a refund, so the if branch picks the refunds queue.' },
  },
  {
    slug: 'case', title: 'Route with case', label: 'case',
    said: '`choose --raw` prints the bare label. `case` sends each label to its own queue.',
    see: { '1-case': 'The password message belongs to account, so the queue is identity.' },
  },
  {
    slug: 'not-sure', title: 'Handle not sure', label: 'not sure',
    said: '`case $?` reads the three exit codes of `decide`: 0 for yes, 1 for no, and 3 for not sure.',
    see: { '1-route': 'The refund goes to refunds and the thanks gets a reply. The send-back line lands in the band and goes to a person.' },
  },
  {
    slug: 'threshold-band', title: 'Set a cut or a band', label: 'cut or band',
    said: 'One number is a cut. Two numbers are a band, and the middle comes back as null.',
    see: {
      '1-cut': 'At a cut of 0.5, the send-back line counts as a refund.',
      '2-band': 'With a band from 0.2 to 0.8, the same line comes back null for a person.',
    },
  },
  {
    slug: 'while-read', title: 'Loop over lines', label: 'while read',
    said: '`while read` hands each line to `decide` on its own. The loop acts on each answer.',
    see: { '1-loop': 'The two complaints open a case. The thanks and the question about blue do not.' },
  },
  {
    slug: 'pipeline', title: 'Chain questions with pipes', label: 'pipes',
    said: 'Two filters in a row keep the records that pass both questions.',
    see: { '1-and': 'The first filter keeps what is about the item. The second keeps the complaints among them. The question about blue is not a complaint, so it drops out.' },
  },
  {
    slug: 'batches', title: 'Ask many files at once', label: 'xargs',
    said: 'A function asks one question of one file. `xargs -P 4` runs it on four files at a time.',
    see: { '1-xargs': 'Two tickets ask for money back: T-1 and T-3.' },
  },
  {
    slug: 'ci-gate', title: 'Fail a build', label: 'CI gate',
    said: '`filter` finds the lines that hedge. The script exits 1 when it finds any, and the build fails.',
    see: {
      '1-pass': 'No line hedges, so the check prints nothing and passes.',
      '2-fail': 'One line hedges. The check prints it and exits 1.',
    },
  },
];

export const RECIPES = [
  {
    slug: 'label-a-json-file', title: 'Label a JSON file and keep its ids', label: 'Label a JSON file',
    said: 'Label every ticket in a JSON array by kind and urgency. `--field /body` sends only the body, so the id and the date ride through. Run it again and the saved answers come back at no further cost.',
    see: {
      '1-label': 'Each ticket keeps its id and date, and gains a kind and an urgency from 0 to 2. The double bill in September is billing.',
      '2-again': 'The same command again, with the details of the first ticket. The answers come from the saved answers, and no request is sent.',
    },
  },
  {
    slug: 'review-a-diff-by-what-it-does', title: 'Review a diff by what it does', label: 'Review a diff',
    said: '`jq` cuts a unified diff into hunks. `decide` asks of each hunk whether it changes what the code does, and the file and hunk header ride through.',
    see: { '1-diff': 'Two hunks change what the code does: the refund limit and the rounded tax. The comment and the rename do not.' },
  },
  {
    slug: 'lint-prose-for-hedging', title: 'Lint prose for hedging', label: 'Lint prose',
    said: '`jq` numbers the lines. `filter` keeps the lines that hedge, and each comes back as it went in, line number and all.',
    see: { '1-lint': 'The two hedging lines come back with their line numbers.' },
  },
  {
    slug: 'fill-a-form-by-selection', title: 'Fill a form by selection', label: 'Fill a form',
    said: 'Every field is a pick from a list you wrote, or true or false. No character in the form comes from a model.',
    see: { '1-form': 'Each request gets a plan, a topic, and whether to call back, all from the lists in the form.' },
  },
];

// The Bash section in side-list order. Its first page is the section's index.
export const BASH_FIRST = { slug: '', title: 'Bash techniques', label: 'Bash techniques', route: '/how-tos/bash/', group: null };
export const BASH = [
  BASH_FIRST,
  ...TECHNIQUES.map((t) => ({ ...t, group: 'Techniques', route: `/how-tos/bash/${t.slug}/` })),
  ...RECIPES.map((r) => ({ ...r, group: 'Recipes', route: `/how-tos/bash/${r.slug}/` })),
];

// Measured facts. Every one names the run it came from.
export const FACTS = {
  cost: {
    headline: 'A thousand answers cost about a penny.',
    rows: [
      ['1.2 cents for 1,000 short records', 'thinkthen sdlc/issues/closed/2026-09-20-live-probe-findings-packing-tagging-status-and-cost.md'],
      ['3.6 cents to ask one question of 3,000 lines of a novel', 'the same page, "The cost of a real file"'],
      ['about 290 input tokens a short record, with about 256 of them fixed', 'the same page: 290.4 on average, from 277 to 301, over 3,000 records'],
      ['$0.042 per million input tokens', 'thinkthen sdlc/records/0011-the-live-probe.md'],
      ['Jev 1.13 answered every case', 'the same findings page'],
    ],
  },
  size: {
    headline: 'How big the evidence can be.',
    rows: [
      ['About 32,000 tokens of evidence in one request', 'thinkthen specification/records.md, "Evidence in one request"'],
      ['About 64,000 tokens in one whole request, the questions included', 'the same table, confirmed by one live request of 33,663 input tokens on 2026-09-19'],
      ['find reads 2 to 255 lines, or 2 to 254 with --none', 'thinkthen specification/find.md'],
      ['The evidence crosses whole and is never split', 'thinkthen sdlc/issues/2026-09-25-docs-how-tos-and-spec-claims-owed.md'],
      ['Evidence over the limit is refused with exit code 4', 'thinkthen specification/backends.md'],
    ],
  },
};
