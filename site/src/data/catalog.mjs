// The names, the order, and the one line for every function and surface.
//
// The one line for each function is the help text's first line, copied from
// `products/thinkthen/vocabulary.md`. The option lists come from
// `repos/thinkthen/specification/`. Nothing here is an example: examples live
// in `src/data/examples/` and are pulled by `scripts/pull-examples.mjs`.

export const TAGLINE = 'ThinkThen: code that knows what you mean';

// Set to false the day the site is linked from anywhere.
export const NOINDEX = true;

export const KEY_VARIABLE = 'THINKTHEN_API_KEY';

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

const COMMON_OPTIONS = [
  ['--details', 'Prints the whole result in place of the bare value: the probabilities, the question, and the run.'],
  ['--input FILE', 'Reads the evidence from a file instead of standard input.'],
  ['--dry-run', 'Prints the plan and sends nothing. It needs no key.'],
  ['--lines, --jsonl, --csv, --tsv', 'Says how a stream of records is framed. Pick one.'],
  ['--field POINTER', 'Names the part of each record to judge, as a JSON Pointer. It may repeat.'],
  ['--jobs N', 'How many requests run at once over a stream of records, from 1 to 32. The default is 4. Outside record mode every function refuses it, and annotate alone takes it on one document.'],
];

export const FUNCTIONS = [
  {
    name: 'decide',
    primitive: 'Yes or no',
    line: 'Answer one yes or no question about the evidence.',
    takes: 'One question and one piece of evidence.',
    gives: 'true, false, or null.',
    requests: 'One request for one piece of evidence. One request for each record in a stream.',
    args: 'QUESTION or @FILE',
    options: [
      ['--threshold T or LOW:HIGH', 'The bar the probability of yes must reach. One number is a cut. Two numbers are a band, and the middle is not sure. The default is 0.5.'],
      ['--true TEXT', 'What a yes means, in the words the model reads.'],
      ['--false TEXT', 'What a no means.'],
      ['--quiet', 'Prints nothing. The exit code carries the answer.'],
      ...COMMON_OPTIONS,
    ],
    exits: [[0, 'yes'], [1, 'no'], [3, 'not sure'], [2, 'usage or input error'], [4, 'the backend failed'], [5, 'a local failure'], [70, 'a defect in the tool']],
    unsure: 'The probability landed inside the band. The tool prints null and exits 3.',
    howtos: ['find-the-clause-then-check-it', 'screen-studies-for-a-review', 'join-two-tables-by-meaning', 'group-alerts-into-incidents', 'screen-a-post-before-it-goes-up', 'check-an-expense-against-the-policy', 'split-a-scanned-packet-into-documents'],
  },
  {
    name: 'choose',
    primitive: 'Pick one',
    line: 'Pick one option from your list.',
    takes: 'One question, one piece of evidence, and 2 to 255 options.',
    gives: 'One of your options, or null.',
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
    exits: [[0, 'an option came back'], [3, 'not sure'], [2, 'usage or input error'], [4, 'the backend failed'], [5, 'a local failure'], [70, 'a defect in the tool']],
    unsure: 'No option reached the threshold. The tool prints null and exits 3. choose never exits 1.',
    howtos: ['join-two-tables-by-meaning', 'rank-the-inbound-leads', 'check-an-expense-against-the-policy', 'split-a-scanned-packet-into-documents'],
  },
  {
    name: 'tag',
    primitive: 'Yes or no, per label',
    line: 'Name every label that fits.',
    takes: 'One question, one piece of evidence, and 1 to 20 labels.',
    gives: 'The labels that fit, as a list.',
    requests: 'Every label rides in one request. One request for one piece of evidence, whatever the label count.',
    args: 'QUESTION or @FILE, then LABEL...',
    options: [
      ['--threshold T', 'The bar every label must reach on its own. The default is 0.5. A band is refused.'],
      ['--label LABEL=DESCRIPTION', 'A label and what it means. It may repeat, and it replaces the positional labels.'],
      ...COMMON_OPTIONS,
    ],
    exits: [[0, 'the run finished'], [2, 'usage or input error'], [4, 'the backend failed'], [5, 'a local failure'], [70, 'a defect in the tool']],
    unsure: 'A label under the bar is left out. An empty list is a good answer and exits 0.',
    howtos: ['code-open-ended-survey-answers', 'screen-a-post-before-it-goes-up'],
  },
  {
    name: 'score',
    primitive: 'Place on a scale',
    line: 'Place the evidence on a scale you name.',
    takes: 'One question, one piece of evidence, and 2 to 10 levels, least first.',
    gives: 'A number along your levels. The first level is 0.',
    requests: 'One request for one piece of evidence. One request for each record in a stream.',
    args: 'QUESTION or @FILE, then LEVEL...',
    options: [
      ...COMMON_OPTIONS,
    ],
    exits: [[0, 'the run finished'], [2, 'usage or input error'], [4, 'the backend failed'], [5, 'a local failure'], [70, 'a defect in the tool']],
    unsure: 'score has no threshold and no not-sure answer. It always lands somewhere on the scale. It orders a queue a person reads, and it is not a gate.',
    howtos: ['code-open-ended-survey-answers'],
  },
  {
    name: 'filter',
    primitive: 'Yes or no, per record',
    line: 'Keep the records where the answer is yes.',
    takes: 'One yes-or-no question and many records.',
    gives: 'The records that pass, byte for byte, in the order they went in.',
    requests: 'One request for each record.',
    args: 'QUESTION or @FILE. One of --lines, --jsonl, --csv, or --tsv is required.',
    options: [
      ['--threshold T', 'The bar a record must reach. The default is 0.5. A band is a usage error.'],
      ['--true TEXT', 'What a yes means, in the words the model reads.'],
      ['--false TEXT', 'What a no means.'],
      ...COMMON_OPTIONS,
    ],
    exits: [[0, 'the run finished'], [2, 'usage or input error'], [4, 'the backend failed'], [5, 'a local failure'], [70, 'a defect in the tool']],
    unsure: 'No record sets the exit code. A record under the bar is dropped.',
    howtos: ['triage-a-support-inbox', 'rank-the-inbound-leads', 'build-a-morning-reading-list'],
  },
  {
    name: 'rank',
    primitive: 'Yes or no, per record',
    line: 'Sort records by how likely the answer is yes.',
    takes: 'One yes-or-no question and many records.',
    gives: 'Every record again, most likely first.',
    requests: 'One request for each record. --top trims the printed list and saves nothing.',
    args: 'QUESTION or @FILE. One of --lines, --jsonl, --csv, or --tsv is required.',
    options: [
      ['--top N', 'Prints the first N records of the order. Every record is still judged.'],
      ['--true TEXT', 'What a yes means, in the words the model reads.'],
      ['--false TEXT', 'What a no means.'],
      ...COMMON_OPTIONS,
    ],
    exits: [[0, 'the run finished'], [2, 'usage or input error'], [4, 'the backend failed'], [5, 'a local failure'], [70, 'a defect in the tool']],
    unsure: 'rank takes no threshold, so nothing is dropped and nothing is not sure. The sort happens on this machine.',
    howtos: ['screen-studies-for-a-review', 'rank-the-inbound-leads', 'build-a-morning-reading-list'],
  },
  {
    name: 'find',
    primitive: 'Pick one, over the lines of the evidence',
    line: 'Pick the one line that best answers a question.',
    takes: 'A question and 2 to 255 lines or records. 2 to 254 with --none.',
    gives: 'The one line that fits best.',
    requests: 'One request for the whole document, however many lines it holds.',
    args: 'QUESTION',
    options: [
      ['--none', 'Lets it answer that nothing fits. It then prints nothing and exits 3.'],
      ['--lines, --jsonl', 'How the lines are framed. --lines is the default. CSV and TSV are refused.'],
      ['--field POINTER', 'Names the part of each record to read.'],
      ['--details', 'Prints the whole result, with a probability for every line.'],
      ['--input FILE', 'Reads the evidence from a file instead of standard input.'],
      ['--dry-run', 'Prints the plan and sends nothing.'],
    ],
    exits: [[0, 'a line came back'], [3, 'nothing fits, under --none'], [2, 'usage or input error'], [4, 'the backend failed'], [5, 'a local failure'], [70, 'a defect in the tool']],
    unsure: 'Without --none, find must pick a line, and it will pick a wrong one. --none is how it says nothing fits. A document past the backend’s token limit is refused, and the command exits 4.',
    howtos: ['find-the-clause-then-check-it', 'group-alerts-into-incidents', 'check-an-expense-against-the-policy'],
  },
  {
    name: 'annotate',
    primitive: 'All three, many at once',
    line: 'Fill out a form for every record.',
    takes: 'A question set and your records.',
    gives: 'Every record back with one field per question.',
    requests: 'One request for each record, for each distinct place the questions read. Records never share a request.',
    args: 'FILE, the saved question set',
    options: [
      ['--profile FILE', 'Applies local backend limits and names the calibration profile in use.'],
      ...COMMON_OPTIONS,
    ],
    exits: [[0, 'every question was answered'], [6, 'the run finished with failed questions'], [2, 'usage or input error'], [4, 'the backend failed'], [5, 'the question set could not be read'], [70, 'a defect in the tool']],
    unsure: 'Each question carries its own threshold, so a question set mixes cuts and bands. A question the backend could not answer is marked failed, counted, and never turned into null. The questions and the evidence together can pass the backend’s token limit for one request. The backend refuses it, and the command exits 4.',
    howtos: ['triage-a-support-inbox'],
  },
  {
    name: 'recognize',
    primitive: 'Pick one, per word',
    line: 'Find every name in the evidence and say what kind it is.',
    takes: 'The evidence and the kinds of name you allow.',
    gives: 'Each name, its kind, where it sits, and a strength.',
    requests: 'The specification does not yet carry recognize, so the request count is not settled.',
    args: 'Not settled. The command has no recognize yet.',
    status: 'planned',
    options: [],
    exits: [],
    unsure: 'The model only picks from options, so a name that is not in the evidence cannot come back. Three thresholds are yours to set. The number on a name is its strength, which is ours and computed, and it claims nothing about chance. Relations are beta.',
    howtos: [],
  },
  {
    name: 'relate',
    primitive: 'Pick one, per pair of records',
    line: 'Say how the records relate to each other.',
    takes: 'A set of records and the relations you allow.',
    gives: 'One edge for each related pair, with a probability.',
    requests: 'The specification does not yet carry relate, so the request count is not settled.',
    args: 'Not settled. The command has no relate yet.',
    status: 'planned',
    options: [],
    exits: [],
    unsure: 'A relation has a direction, or it is marked as reading the same both ways. The number on an edge is a probability. relate is beta.',
    howtos: [],
  },
  {
    name: 'question-file',
    title: '@question',
    primitive: 'Not a function',
    line: 'A saved question every function accepts.',
    takes: 'One JSON file holding exactly one question.',
    gives: 'Nothing. A question file gives no answer. It carries a question.',
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
    notAFunction: true,
  },
];

export const CODE_FUNCTIONS = FUNCTIONS.filter((f) => !f.notAFunction);

export const SURFACES = [
  {
    slug: 'shell', name: 'The shell', deckHeading: null, status: 'shipped',
    lang: 'bash', tab: 'Bash',
    blurb: 'The command. Ten functions, standard in, standard out, and an exit code your script branches on.',
    unsureWord: 'null, and exit code 3',
    install: [
      ['brew install genomoncology/thinkthen/thinkthen', 'Homebrew tap. Coming with 0.1.'],
      ['curl -fsSL https://thinkthen.dev/install.sh | sh', 'Download script. Coming with 0.1.'],
    ],
    particular: [
      'Standard in carries the evidence. Standard out carries the answer, and nothing else.',
      'The exit code is the answer for a single piece of evidence, so `if` and `case` read it directly.',
      'Requests run four wide by default and up to 32 with --jobs.',
      '--dry-run prints the plan and needs no key.',
    ],
  },
  {
    slug: 'python', name: 'Python', deckHeading: 'Python', status: 'planned',
    lang: 'python', tab: 'Python',
    blurb: 'The ten functions as plain Python functions. Build a question once and use it anywhere.',
    unsureWord: 'None',
    install: [['pip install thinkthen', null], ['uv add thinkthen', null]],
    particular: [
      'A list goes in and a list comes out. It crosses into the engine once and runs 32 wide.',
      'A question built with tt.question() carries its own threshold, so one object serves the hook, the test, and the pipeline.',
    ],
  },
  {
    slug: 'polars', name: 'Polars', deckHeading: 'Polars', status: 'planned',
    lang: 'python', tab: 'Python',
    blurb: 'A Polars column goes in and a column comes out.',
    unsureWord: 'None',
    install: [['pip install thinkthen[polars]', null]],
    particular: [
      'Rust reads the column where it sits. There is no copy and no Python loop.',
      'Hand over a whole frame with on= and it comes back with one new column for each question in the form.',
    ],
  },
  {
    slug: 'typescript', name: 'TypeScript', deckHeading: 'TypeScript', status: 'planned',
    lang: 'ts', tab: 'TypeScript',
    blurb: 'The ten functions, all async. Options ride in one object.',
    unsureWord: 'null',
    install: [['npm install thinkthen', null], ['pnpm add thinkthen', null], ['bun add thinkthen', null]],
    particular: [
      'An AbortSignal cancels a batch and stops its bill.',
      'Every call returns a promise. An array crosses once.',
    ],
  },
  {
    slug: 'ruby', name: 'Ruby', deckHeading: 'Ruby', status: 'planned',
    lang: 'ruby', tab: 'Ruby',
    blurb: 'The ten functions as module methods.',
    unsureWord: 'nil',
    install: [['gem install thinkthen', null]],
    particular: ['Any Enumerable crosses to the engine once.'],
  },
  {
    slug: 'r', name: 'R', deckHeading: 'R', status: 'planned',
    lang: 'r', tab: 'R',
    blurb: 'The ten functions with a tt_ prefix, inside dplyr.',
    unsureWord: 'NA',
    install: [['install.packages("thinkthen")', null]],
    particular: [
      'A column goes in and a column comes out.',
      'filter() drops the NA rows, so a not-sure answer leaves the pipeline on its own.',
    ],
  },
  {
    slug: 'rust', name: 'Rust', deckHeading: 'Rust', status: 'planned',
    lang: 'rust', tab: 'Rust',
    blurb: 'The engine itself, with no binding in between.',
    unsureWord: 'Answer::Unsure',
    install: [['cargo add thinkthen', null]],
    particular: [
      'Answer::Unsure is an arm the compiler makes you handle.',
      'Calls block. No async runtime comes with it.',
    ],
  },
  {
    slug: 'c', name: 'C', deckHeading: 'C', status: 'planned',
    lang: 'c', tab: 'Rust',
    blurb: 'The header and the library. The door to every other language.',
    unsureWord: 'an outcome of THINKTHEN_UNSURE',
    install: [['thinkthen.h + libthinkthen', 'One archive per platform, with the header, both libraries, and a .pc file.']],
    particular: [
      'Every call returns 0 or one of six error kinds.',
      'Arrays of pointers and lengths cross once and run 32 at a time.',
    ],
  },
  {
    slug: 'duckdb', name: 'DuckDB', deckHeading: 'DuckDB', status: 'planned',
    lang: 'sql', tab: 'SQL',
    blurb: 'The functions work in WHERE, SELECT, and ORDER BY.',
    unsureWord: 'NULL',
    install: [['INSTALL thinkthen FROM community; LOAD thinkthen;', null]],
    particular: ['A whole column chunk crosses at once.'],
  },
  {
    slug: 'sqlite', name: 'SQLite', deckHeading: 'SQLite', status: 'planned',
    lang: 'sql', tab: 'SQL',
    blurb: 'The same function names, with a warm pass in front of them.',
    unsureWord: 'NULL',
    install: [['.load ./thinkthen', null]],
    particular: [
      'SQLite asks row by row, so thinkthen_warm judges the table in one fast pass, 32 requests at a time.',
      'Every query after that reads the saved answers at no further cost.',
    ],
  },
  {
    slug: 'postgresql', name: 'PostgreSQL', deckHeading: 'PostgreSQL', status: 'planned',
    lang: 'sql', tab: 'SQL',
    blurb: 'An extension. annotate returns jsonb.',
    unsureWord: 'NULL',
    install: [['CREATE EXTENSION thinkthen;', null]],
    particular: [
      'A question file carries a band, so the not-sure rows are the ones a person should read.',
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

export const HOWTOS = [
  {
    slug: 'triage-a-support-inbox', title: 'Triage a support inbox', reader: 'for support teams',
    said: 'filter keeps the messages that need a reply, and annotate answers two more: what kind, and how urgent.',
    functions: ['filter', 'annotate'], input: 'inbox.txt', runs: ['01-inbox'],
  },
  {
    slug: 'find-the-clause-then-check-it', title: 'Find the clause, then check it', reader: 'for contract and procurement staff',
    said: 'find pulls the line that states the notice period, and decide judges it.',
    functions: ['find', 'decide'], input: 'terms.txt', runs: ['02-clause'],
  },
  {
    slug: 'screen-studies-for-a-review', title: 'Screen studies for a review', reader: 'for researchers',
    said: 'decide with a band puts the clear ones in or out and leaves a middle, and rank orders that middle for a person.',
    functions: ['decide', 'rank'], input: 'studies.txt', runs: ['03-studies'],
  },
  {
    slug: 'code-open-ended-survey-answers', title: 'Code open-ended survey answers', reader: 'for survey and market researchers',
    said: 'score places every answer on a scale you name, and tag says what is wrong in the unhappy ones.',
    functions: ['score', 'tag'], input: 'answers.txt', runs: ['04-answers'],
  },
  {
    slug: 'join-two-tables-by-meaning', title: 'Join two tables by meaning', reader: 'for data analysts',
    said: 'decide is the join condition and choose routes the ticket.',
    functions: ['decide', 'choose'], input: 'tickets.txt', runs: ['05-join', '05-team'],
  },
  {
    slug: 'group-alerts-into-incidents', title: 'Group alerts into incidents', reader: 'for on-call engineers',
    said: 'find --none matches the alert to an open incident or to nothing. decide runs only when there is a match.',
    functions: ['find', 'decide'], input: 'incidents-open.txt', runs: ['06-alert'],
  },
  {
    slug: 'rank-the-inbound-leads', title: 'Rank the inbound leads', reader: 'for sales teams',
    said: 'filter drops the noise, rank puts the ready buyers first, and choose routes them.',
    functions: ['filter', 'rank', 'choose'], input: 'leads.txt', runs: ['07-leads'],
  },
  {
    slug: 'screen-a-post-before-it-goes-up', title: 'Screen a post before it goes up', reader: 'for community moderators',
    said: 'decide with a band says yes, no, or null for a moderator, and tag names the rule.',
    functions: ['decide', 'tag'], input: 'post.txt', runs: ['08-gate', '08-why'],
  },
  {
    slug: 'check-an-expense-against-the-policy', title: 'Check an expense against the policy', reader: 'for finance staff',
    said: 'find pulls the rule that applies, decide says whether the expense fits it, and choose files it.',
    functions: ['find', 'decide', 'choose'], input: 'policy.txt', runs: ['09-expense', '09-category'],
  },
  {
    slug: 'build-a-morning-reading-list', title: 'Build a morning reading list', reader: 'for anyone',
    said: 'filter keeps the headlines on your topic, and rank puts the one to read first at the top.',
    functions: ['filter', 'rank'], input: 'feed.txt', runs: ['10-reading'],
  },
  {
    slug: 'split-a-scanned-packet-into-documents', title: 'Split a scanned packet into documents', reader: 'for back-office staff',
    said: 'choose names each page. decide checks each gap for a new document.',
    functions: ['choose', 'decide'], input: 'pages.txt', runs: ['11-kinds', '11-gaps'],
  },
];

// Measured facts. Every one names the run it came from.
export const FACTS = {
  cost: {
    headline: 'A thousand answers cost about a penny.',
    rows: [
      ['1.2 cents for 1,000 short records', 'thinkthen sdlc/issues/2026-09-20-live-probe-findings-packing-tagging-status-and-cost.md'],
      ['3.6 cents to ask one question of 3,000 lines of a novel', 'the same page, "The cost of a real file"'],
      ['about 290 input tokens a short record, with about 256 of them fixed', 'the same page: 290.4 on average, from 277 to 301, over 3,000 records'],
      ['$0.042 per million input tokens', 'thinkthen sdlc/records/0011-the-live-probe.md'],
      ['Jev 1.13 answered every case', 'the same findings page'],
    ],
  },
  size: {
    headline: 'How big the evidence can be.',
    rows: [
      ['32,000 tokens of evidence with one question', 'thinkthen specification/records.md, "Evidence in one request"'],
      ['64,000 tokens of evidence with all its questions', 'the same table, confirmed by one live request of 33,663 input tokens on 2026-09-19'],
      ['find reads 2 to 255 lines, or 2 to 254 with --none', 'thinkthen specification/find.md'],
      ['The evidence crosses whole and is never split', 'thinkthen sdlc/issues/2026-09-21-size-cost-and-other-backends-what-the-manual-and-the-tests-must-carry.md'],
      ['Evidence over the limit is refused with exit code 4', 'thinkthen specification/backends.md'],
    ],
  },
};
