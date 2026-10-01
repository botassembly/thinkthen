// The Beatles Bench pages under /learn/beatles-bench/, in the order of the
// talk "Jev, ThinkThen, and Beatles Bench". Each page is a short article: the
// slide, the idea, one example, the same example at another bar, and the
// lesson. A page with no example shows the slide and the idea. `goal` says
// what the page must communicate. `lesson` says what the runs showed, and
// `takeaway` is the one line a reader keeps. `files` maps each file the page
// shows to its caption. `blocks` holds code a reader types and the site does
// not run, such as a clone.
//
// The examples live in examples/beatles/<slug>/. Each runs in the Beatles
// Bench folder examples/beatles/folders.json names for its page, and answers
// from a saved recording. A page with no bench folder runs from its own
// files/, and its recordings come from the talk's deck or a recorded run. `see` gives the
// caption for each example and `headings` the heading above it. `slideNote`
// names the build or run behind a slide whose printed output differs from what
// the current thinkthen prints in the page's example. `source`
// links the bench record behind a number in the prose. Prose marks code with
// backticks and a link as [text](/route/).
//
// Links into the bench use paths on bench main. The bench history may be
// squashed at launch, and a pinned commit would then stop resolving
// (sdlc/issues/closed/2026-09-26-the-beatles-bench-section-keeps-its-own-copy.md).

import { BINDINGS, COUNTS } from './catalog.mjs';

export const REPO = 'https://github.com/botassembly/beatles-bench';
const tree = (name) => `${REPO}/tree/main/examples/${name}`;
// The bench folder that names a slide's numbers and their sources.
const record = (name) => ({ text: `the bench's record for this slide`, href: tree(name) });

export const GROUPS = [
  ['Start', ['strings', 'jev', 'runs-in']],
  ['The ten functions', ['decide', 'choose', 'tag', 'score', 'score-bands', 'filter', 'rank', 'find', 'recognize', 'relate', 'question-file', 'annotate']],
  ['Scripts', ['bash']],
  ['Bindings', ['languages', 'data']],
  ['What Jev knows', ['blind-spots', 'rad']],
  ['Tune your bar', ['audit', 'diff']],
  ['Agents', ['use-cases', 'shell-uses', 'data-uses', 'retrieval']],
  ['The field', ['competitors', 'bench-field', 'backends']],
];

// The choose and Jev slides show four faces cropped from one photo.
const FACES = "The four faces: United Press International photo of the Beatles in New York, 7 February 1964, via the Library of Congress and Wikimedia Commons. Public domain in the US.";

const ARTICLES = {
  strings: {
    title: "Code sees strings, not meaning.",
    label: "Strings and meaning",
    goal: "A pattern matches letters, and decide answers a question about what the words mean.",
    idea: [
      "To code, \"Octopus's Garden\" is a string of letters. You know it is a Ringo song on Abbey Road. `grep` matches letters, and none of these titles holds the words Abbey Road.",
      "ThinkThen asks Jev what the words mean. Jev returns the probability of yes, and your threshold turns it into an answer.",
    ],
    credit: "Ringo Starr photo: UPI, 1964, public domain in the US, via Wikimedia Commons.",
    see: {
      '1-grep': "`grep` finds nothing and exits 1.",
      '2-decide': "At the default bar of 0.5, Octopus's Garden is yes and the others are no.",
      '3-band': "The band 0.3:0.7 turns Penny Lane into not sure.",
    },
    headings: { '3-band': "Change the bar" },
    lesson: "Octopus's Garden is the only one of the three on Abbey Road. Penny Lane falls inside the band and goes to a person as not sure.",
    takeaway: "A question about meaning finds what no pattern can match.",
    link: tree('filter'),
  },

  jev: {
    title: "Jev set the standard.",
    label: "Jev",
    goal: "Jev reads a text and a question and returns a probability for every answer.",
    idea: [
      "Jev is the model behind ThinkThen. TypeSafe makes it. It reads a text and a question and returns a probability for every answer. It writes no text, and you train nothing.",
      "Roger Bannister's mile set a standard for runners. Jev answers in about 60 milliseconds. That is the median time at Jev's server, measured on 2026-09-26. On Beatles Bench, a thousand answers cost 0.016 dollars.",
    ],
    credit: `Roger Bannister photo: 6 May 1954, public domain in the US, via Wikimedia Commons. ${FACES}`,
    source: record('jev'),
    see: {
      '1-choose': "At a bar of 0.8, Jev picks Ringo.",
      '2-bar': "A bar of 0.9 asks for more. The answer is not sure.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "Ringo is right. Jev's probability for Ringo clears 0.8 and falls short of 0.9. A bar of 0.9 asks for more than Jev gives.",
    takeaway: "Jev answers with a probability, and your bar decides what counts.",
    link: tree('choose'),
  },

  "runs-in": {
    title: `${COUNTS.functions}, ${COUNTS.cli}, ${COUNTS.bindings}.`,
    label: "Functions and bindings",
    goal: "ThinkThen is one command with ten functions, and its bindings bring the same functions to languages and databases.",
    idea: [
      "ThinkThen is one command-line tool, `thinkthen`. It runs from the command line in [Bash](/install/shell/). It has ten functions, such as `decide`, `filter`, and `rank`. It also has six tools, such as `audit` and `diff`.",
      `Bindings bring the same ten functions to ${BINDINGS.length} programming languages and databases: ${BINDINGS.map((b, i) => `${i === BINDINGS.length - 1 ? 'and ' : ''}[${b.name}](${b.route})`).join(', ')}.`,
      "To set one up, start at [Install](/install/).",
    ],
    see: {
      '1-help': "The first ten names are the functions. `help` and the six tools follow.",
    },
    lesson: "Each function asks Jev a question. `audit` and `diff` work on answers you already saved. They grade the answers and compare two runs.",
    takeaway: "One command, ten functions, and a binding for your language.",
    link: REPO,
  },

  decide: {
    title: "decide answers yes, no, or not sure.",
    goal: "A band turns the close calls of decide into not sure.",
    idea: [
      "`decide` is an if statement that reads. Ask one yes or no question about a text. Jev returns the probability of yes. Your threshold turns it into yes, no, or not sure.",
    ],
    see: {
      '1-lines': "At the default bar of 0.5, three songs are love songs.",
      '2-bar': "At a bar of 0.6, Yesterday turns to no.",
      '3-band': "Inside the band 0.3:0.7, Yesterday is not sure.",
    },
    headings: { '2-bar': "Change the bar", '3-band': "Set a band" },
    lesson: "Yesterday sits between 0.5 and 0.6. A bar of 0.5 calls it yes, and a bar of 0.6 calls it no. The band calls it not sure. The bench has no answer key for this question.",
    takeaway: "You decide how sure a yes must be.",
    link: tree('decide'),
  },

  choose: {
    title: "choose selects one option.",
    credit: FACES,
    goal: "A bar on choose turns a weak pick into not sure and leaves the strong picks alone.",
    idea: [
      "`choose` is a switch statement that reads. You list the options, and Jev puts a probability on each. The top option is the pick.",
    ],
    see: {
      '1-lines': "Every song gets a pick.",
      '2-bar': "Under a bar of 0.5, She Loves You is not sure.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "John and Paul sing She Loves You together. Jev's pick of John does not reach 0.5. The bar turns it into not sure. The four sure picks stay the same, and all four are right.",
    takeaway: "A bar keeps the sure picks and hands a person the weak one.",
    link: tree('choose'),
  },

  tag: {
    title: "tag applies labels to data.",
    goal: "Each tag label clears the bar on its own, and a higher bar keeps fewer labels.",
    idea: [
      "`tag` names every label that fits. Each label gets its own probability. A label counts when it clears the bar, and the default bar is 0.5.",
    ],
    see: {
      '1-lines': "At 0.5, Octopus's Garden gets three labels.",
      '2-bar': "At 0.7, two songs lose psychedelic.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "Penny Lane and Octopus's Garden are psychedelic at 0.5 and lose the label at 0.7. The bench has no answer key for tag.",
    takeaway: "A higher bar keeps fewer labels.",
    link: tree('tag'),
  },

  score: {
    title: "score can rate on a linear scale you define.",
    goal: "score gives a position on your scale, and your code draws the line.",
    idea: [
      "`score` places a text on a scale you name, lowest level first. It returns a number along that scale. Here the scale runs from about 0 minutes to about 9 minutes, in even steps.",
    ],
    see: {
      '1-lines': "Each song gets a length in minutes.",
      '2-cut': "Your code keeps the songs at 5 minutes or more.",
    },
    headings: { '2-cut': "Set the bar" },
    lesson: "`score` has no threshold. A Day in the Life runs over five minutes. Jev scores it 4.868686868687, and a bar of 5 leaves it out.",
    takeaway: "A score is a position. Your code draws the line.",
    link: tree('score'),
  },

  "score-bands": {
    title: "score can rate on a non-linear scale you define.",
    label: "score bands",
    command: true,
    goal: "Each level of a score scale can carry its own range, so the steps can be uneven.",
    idea: [
      "The steps of a scale need not be even. Here `score` places Beatles songs on five levels of length, from very short to very long. Each level carries its name and its range. The ranges grow wider as the songs grow longer.",
      "The value runs from 0 for very short to 4 for very long.",
    ],
    files: { 'question.json': "question.json, the question and its five levels" },
    see: {
      '1-bands': "Every song with a known length goes in. `jq` keeps three of them.",
    },
    lesson: "Her Majesty is the shortest Beatles song. It scores 0.23, very short. Revolution 9 is the longest. It scores 3.31, nearest long, one level short of the truth. Yesterday runs 2:05, right on the line between short and average. It scores 1.73, nearest average.",
    takeaway: "Give each level a range, and the steps can be as uneven as your data.",
    link: REPO,
  },

  filter: {
    title: "filter keeps what clears your bar.",
    goal: "A higher bar on filter drops weak yeses and keeps a sure mistake.",
    idea: [
      "`filter` is a grep that reads. It keeps each record where the answer is yes and passes it through unchanged.",
    ],
    see: {
      '1-bar': "At 0.7, seven songs stay.",
      '2-bar': "At 0.9, five songs stay.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "Six of the seven songs at 0.7 are on Abbey Road. A Day in the Life came out on Sgt. Pepper's Lonely Hearts Club Band. At 0.9, Octopus's Garden and Something drop out. A Day in the Life stays.",
    takeaway: "A higher bar cannot remove a mistake Jev is sure of.",
    link: tree('filter'),
  },

  rank: {
    title: "rank sorts by your criteria.",
    goal: "rank orders every record and drops none, and you choose where the list ends.",
    idea: [
      "`rank` is a sort that reads. It asks one yes or no question of each record and sorts the records by the probability of yes.",
    ],
    see: {
      '1-lines': "All twelve songs, likeliest hit first.",
      '2-top': "`--top 5` keeps the first five.",
    },
    headings: { '2-top': "Set the cut" },
    lesson: "Both runs give the same order. `--top 5` prints the first five and asks nothing new.",
    takeaway: "rank drops nothing. You choose where the list ends.",
    link: tree('rank'),
  },

  find: {
    title: "find picks one from many.",
    goal: "Without `--none`, find always names one line.",
    idea: [
      "`find` reads every line together and picks the one that answers the question. Each line sees the others. An answer can depend on the whole set.",
    ],
    see: {
      '1-find': "`find` picks Love Me Do.",
    },
    lesson: "Love Me Do came out first, and the pick is right.",
    takeaway: "Without `--none`, find always names a line. Check its picks on records you know before you trust it.",
    link: tree('find'),
  },

  annotate: {
    title: "annotate answers a question set.",
    goal: "annotate asks a set of questions about each record in one run, and each question carries its own bar.",
    idea: [
      "`annotate` answers many questions at once. The question set holds four questions you have already seen. choose asks who sings the lead vocal. tag asks which labels fit. Two decide questions ask whether the song is on Abbey Road and whether it is one of the biggest hits.",
      "One run asks all four about each song. `--field /title` sends only the title. Each record comes back with one field for each question.",
    ],
    files: { 'questions.json': "questions.json, the question set" },
    see: {
      '1-card': "Every question uses the default bar of 0.5.",
      '2-band': "`jq` sets the band 0.2:0.8 on the two decide questions. Three answers turn not sure.",
    },
    headings: { '2-band': "Set a band" },
    lesson: "At the default bar, every field fills. Paul sings Yesterday, and Ringo sings Octopus's Garden. Octopus's Garden is on Abbey Road, and Yesterday is not. Under the band, Yesterday's Abbey Road answer and both of Octopus's Garden's decide answers fall inside it. A person checks them.",
    takeaway: "One question set fills every field of a record, and each question keeps its own bar.",
    link: REPO,
  },

  recognize: {
    title: "recognize labels things it finds.",
    goal: "recognize works in three steps: it finds the names, labels each one, and relates the pairs your rules allow.",
    idea: [
      "`recognize` finds every name in a text and gives each one a kind from your list. It works in three steps.",
      "First it finds the names. It splits the text into pieces and asks one question about each piece. Does the piece begin a name, sit inside one, end one, stand alone as a name, or sit outside every name? Then it labels each name with one of your kinds. Last it relates the names. Each rule names a relation, a subject kind, and an object kind. `recognize` asks one yes or no question for each pair a rule allows.",
    ],
    see: {
      '1-find': "`--plan` shows the find step's request: 28 pieces in one request.",
      '2-label': "The label step gives each name one of the four kinds.",
      '3-relate': "The relate step keeps the edges at 0.5 or more.",
    },
    headings: { '1-find': "Step 1: find", '2-label': "Step 2: label", '3-relate': "Step 3: relate" },
    lesson: "All five names and their kinds are right. The sentence says Ringo Starr wrote Octopus's Garden, and that edge reads 0.99. The rules also let `recognize` ask whether Octopus's Garden was recorded at Sardinia. That answer falls under 0.5. No edge comes out for it.",
    takeaway: "Find the names, label them, then link them.",
    link: tree('recognize'),
  },

  relate: {
    title: "relate asks what Jev knows about the names.",
    goal: "relate asks Jev about the names your rules allow, a menu gives each song at most one album, and a higher bar does not remove a confident wrong edge.",
    slideNote: "The slide shows the talk's run, which asked one yes or no question for each pair. The example below marks the song-to-album rule single, so it asks one menu for each song.",
    idea: [
      "`relate` gets a list of names and their kinds, and no text. Jev answers from what it knows. `recognize --relation` links names by what a text states.",
      "Here four people, five songs, and three albums go in. The rules file names two relations. `sings` runs from a person to a song, and it asks one yes or no question for each pair. One person sings the lead on many songs. `on_album` runs from a song to an album. A song first appears on one album, so the rule says `\"single\": true`. It asks one menu for each song: which listed album, or none of these. Every answer at 0.5 or more comes out as an edge.",
    ],
    files: {
      'names.jsonl': "names.jsonl, the names in",
      'rules.json': "rules.json, the two rules",
    },
    see: {
      '1-edges': "Nine edges come out at the default bar of 0.5. Each song gets one album.",
      '2-bar': "At 0.8, eight edges stay.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "One of the nine edges is wrong. Jev puts Octopus's Garden on Revolver at 0.91. It first appeared on Abbey Road. No edge says who sings Taxman. George sings it, and Jev's answer fell under the bar. At 0.8, Yesterday on Help! drops out at 0.62, and it is right. The wrong edge stays.",
    takeaway: "A menu gives each song one album. A bar of 0.8 keeps the confident wrong answer.",
    link: tree('relate'),
  },

  "question-file": {
    title: "Store and reuse questions.",
    label: "Question files",
    goal: "A question file says what yes and no mean, when to answer not sure, and which fields to read, and decide runs it on one song.",
    idea: [
      "A [question file](/functions/question-file/) is a small recipe. `decide` holds the question. `true` and `false` say what yes and no mean. `threshold` sets the band 0.2:0.8. An answer of 0.8 or more is yes, and an answer under 0.2 is no. `on` picks the title and the album from each record. The request sends only those two fields.",
      "Here a band plays only songs the Beatles wrote. The file asks whether a Beatle wrote a song. `decide @original.json` runs the recipe on one song.",
    ],
    files: {
      'original.json': "original.json, the question file",
    },
    see: {
      '1-something': "`decide` prints `true` for Something and exits 0.",
    },
    source: { text: "the bench's songwriter list", href: `${REPO}/blob/main/data/songs.tsv` },
    lesson: "George Harrison wrote Something. The answer is right. The exit code carries the answer too: 0 for yes, 1 for no, and 3 for not sure. The next page puts that code to work in a script.",
    takeaway: "Write the question once in a file, and every run asks it the same way.",
    link: REPO,
  },

  bash: {
    title: "Bash runs on judgment.",
    label: "Bash scripts",
    goal: "A Bash script reads the exit code of decide, and each answer picks the next step.",
    idea: [
      "The script reads one song per line on standard input and asks whether a Beatle wrote each one. It passes the question and its flags on the command line. `--true` and `--false` say what yes and no mean. `--threshold 0.2:0.8` sets the band. A probability of 0.8 or more is yes, and one under 0.2 is no. Between them is not sure. The two `--field` pointers send the title and the album as one JSON object. Without them, `decide` sends the whole line as text. The previous page keeps the same question in a file. The script plays a yes, skips a no, and hands a not sure to a person.",
      "`decide --quiet` prints nothing. Its exit code is the answer: 0 for yes, 1 for no, and 3 for not sure. The function `is_original` names what the code means. `code=$?` names the code, and `case` picks the step. Any other code is a failure, and the script stops. [Handle not sure](/how-tos/bash/not-sure/) teaches the same form.",
    ],
    files: {
      'setlist.sh': "setlist.sh, the script",
      'setlist.jsonl': "setlist.jsonl, the songs, one JSON record per line",
    },
    see: {
      '1-setlist': "The run feeds setlist.jsonl to the script. Two songs play, one is skipped, and one goes to a person.",
    },
    source: { text: "the bench's songwriter list", href: `${REPO}/blob/main/data/songs.tsv` },
    lesson: "Two of the three sure answers are right. Jev says a Beatle wrote Words of Love. Buddy Holly wrote it. Mr. Moonlight falls inside the band, and a person checks it. Roy Lee Johnson wrote it.",
    takeaway: "The exit code is the answer, and an ordinary `case` acts on it.",
    link: REPO,
  },

  languages: {
    title: "Scripting and systems languages",
    label: "Languages",
    goal: "Each language binding asks the same question the command asks.",
    idea: [
      "On the slide, Python and C ask Jev the same `decide` question: does Ringo Starr sing the lead vocal on Octopus's Garden? Each stores the answer in `is_ringo` and asserts it. Both get yes.",
      "On the slide, Python heads the scripting languages and C heads the systems languages.",
      "Each binding calls the same engine as the command. To set one up, start at [Install](/install/).",
    ],
    takeaway: "Pick your language. The question stays the same.",
    link: REPO,
  },

  data: {
    title: "Database tables and data frames",
    label: "Tables and frames",
    goal: "One question adds an answer column to a database table or a data frame.",
    idea: [
      "One SQL query adds an answer column to a table of songs. It asks one `decide` question of each title and names the column `on_abbey`. The question and the band 0.3:0.7 sit inside the query as one JSON string.",
      "SQLite runs this query and prints 1, 0, or NULL. The slide draws each as a mark for yes, no, or not sure. Penny Lane and Hey Jude fall inside the band. Both come back not sure. Jev says yes to A Day in the Life. That answer is wrong. The song first came out on Sgt. Pepper's Lonely Hearts Club Band.",
      "DuckDB and PostgreSQL ask questions inside SQL too. Polars, pandas, and R take a column and give a column back.",
      "To set one up, start at [Install](/install/).",
    ],
    takeaway: "Ask the question where your data already lives.",
    link: REPO,
  },

  audit: {
    title: "audit grades a run.",
    goal: "audit grades saved answers against answers you already know, at any bar, and sends no request.",
    idea: [
      "You already know the right answer for some of your records. `audit` grades saved answers against those answers at any bar. It sends no request.",
      "Here Jev was asked whether each of 10 songs is on Abbey Road, from the title alone. The slide's no-context column shows these answers. The slide reads each answer at the band 0.2:0.8. A red cross marks a wrong answer, and an amber ? marks a not-sure answer. The first example below runs `audit` on the same 10 songs. The [diff page](/learn/beatles-bench/diff/) asks about them again with context.",
    ],
    see: {
      '1-band': "At the band 0.2:0.8, 3 are right, 2 are wrong, and 5 are not sure.",
      '2-default': "At the default bar of 0.5, 5 are right. Jev says yes wrongly 5 times.",
      '3-bar': "At 0.78, 8 are right. audit suggests 0.78 for the whole recorded run.",
    },
    headings: { '2-default': "Try the default bar", '3-bar': "Change the bar" },
    lesson: "From the title alone, half the answers fall inside the band. At 0.5, Jev says yes to 5 songs from other albums. At 0.78, only A Day in the Life and The Long and Winding Road remain wrong, and no Abbey Road song is lost.",
    takeaway: "The answers you already know grade any bar, at no cost.",
    link: tree('audit'),
  },

  diff: {
    title: "diff shows what changed.",
    goal: "diff prints only the answers that changed between two runs, and says whether each change fixed a mistake.",
    idea: [
      "Ask the same question twice, and `diff` prints only the answers that changed. A summary comes last. With an answer key, each change says whether it fixed a mistake, made one, or settled a not-sure answer.",
      "Here the page reads two saved runs for the 10 songs on the [audit page](/learn/beatles-bench/audit/). The first run asked from the title alone. The second run gave Jev each song's catalog entry too. The slide's last column shows the answers with context. The warning is right. The two runs asked different questions.",
    ],
    see: {
      '1-band': "At the band 0.2:0.8, 7 of the 10 answers changed, and all 10 end right.",
      '2-bar': "At a bar of 0.5, 5 changed. Each was a wrong yes that turned right.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "`gained` marks a wrong answer that turned right. `resolved` marks a not-sure answer that settled, right or wrong. `lost` counts right answers that turned wrong, and none did. At the band 0.2:0.8, Come Together, Here Comes the Sun, and Octopus's Garden kept their answers. `diff` leaves them out.",
    takeaway: "diff shows what a change fixed and what it broke.",
    link: tree('audit'),
  },

  "blind-spots": {
    title: "Jev has blind spots.",
    label: "Blind spots",
    goal: "Jev misses obscure facts, and a low probability is how it says so.",
    idea: [
      "Jev's memory fades on obscure facts. It gets 72% of the questions about the most viewed quarter of songs on Wikipedia right, and 52% about the least viewed. Tricky wording trips it. It gets 67% of the plain control questions and 48% of the word traps. Two hops are hard. One kind of question asks whether a song came out the same month as another event. On 30 of them, Jev knew both facts on their own. It got the chained question right on only 13.",
      "Nothin' Shakin' is an obscure song, and George sings it.",
    ],
    slideNote: "The slide shows the bench's Jev run of 2026-09-30. There Jev picks Ringo for Nothin' Shakin'. The examples below replay the bench's recording of 2026-09-26, where Jev picks John.",
    source: record('catches'),
    see: {
      '1-choose': "With no bar, Jev picks `john`.",
      '2-bar': "Under a bar of 0.5, the wrong pick becomes not sure.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "With no bar, the answer is John, and John is wrong. A bar of 0.5 sends this question to a person.",
    takeaway: "A low probability is Jev saying it does not know.",
    link: REPO,
  },

  rad: {
    title: "Retrieval-augmented decisions (RAD)",
    label: "RAD",
    goal: "Putting the facts in the text fixes a sure miss that no bar can fix.",
    idea: [
      "Giving Jev context improves accuracy. Look up the record, put it in front of the question, and ask. We call it retrieval-augmented decisions. ThinkThen sends this context as `state`.",
      "The song catalog covers 1,075 of the bench's questions. From memory, Jev gets 68% of them right. With the catalog in the text, the bench's report estimates about 97%. Context makes each request longer, and a longer request costs more.",
    ],
    source: record('open-book'),
    see: {
      '1-memory': "From memory, with the band 0.1:0.9, Jev still says yes. A Day in the Life is not on Abbey Road.",
      '2-context': "With the catalog entry, the same band gives no.",
    },
    headings: { '2-context': "Add context" },
    lesson: "No sensible bar fixes a miss this sure. With the catalog entry, the answer turns right.",
    takeaway: "When memory fails, put the facts in the text.",
    link: tree('decide'),
  },

  "use-cases": {
    title: "Agent harnesses",
    label: "Agent harnesses",
    goal: "An agent harness could ask Jev nine small questions, and each answer picks the harness's next step.",
    idea: [
      "These are ideas for Jev inside an agent harness. Many come from [a post by Diogo Almeida](https://x.com/completeskeptic/status/2101894250401271876). Diogo works at TypeSafe, the maker of Jev. Each card names a job, the question Jev would answer, what the harness does with the answer, and the function that asks.",
      "Permission asks \"Should this command run?\" If Jev is not sure, the harness asks a person. Tool choice ranks the tools for a step, and the harness loads only the top few. Context asks what should happen to a chunk. `choose` picks load, summarize, or hide.",
      "Model choice asks \"Is this step easy?\" An easy step goes to a smaller model. Parallel work asks whether tasks can run at once, and the harness splits them. Data safety asks \"Could this touch secrets?\" Such a task runs on an approved model.",
      "Off track asks whether the agent is going in circles. The harness stops it and asks a person. Done check asks whether the task is done as asked. The harness stops, or sends the work back. Evaluation asks which of two answers is better, and the winning prompt wins. A Jev pick comes back with a probability. You can check it against cases a person labeled.",
    ],
    credit: "Many ideas from Diogo Almeida.",
    takeaway: "Each decision is one small question with a probability.",
    link: REPO,
  },

  "shell-uses": {
    title: "Ops/Security",
    label: "Ops and security",
    goal: "Each answer drives a shell script, as nine ops and security jobs show.",
    idea: [
      "Each card names a job, the question ThinkThen asks, what the script does with the answer, and the function that asks.",
      "`decide` answers in its exit code: 0 for yes and 1 for no. With a band such as `--threshold 0.1:0.9`, not sure exits 3. Alert triage uses all three. The script pages a person, logs the alert, or queues it. Flaky test retries a flaky failure and fails the build on any other.",
      "`choose` prints its pick, and failure routing runs that kind's runbook. `filter` keeps the lines where the answer is yes. Grep by meaning keeps the log lines about an outage, whatever words they use. Sample sheet keeps the complete rows and stops before hours of compute.",
      "`score` places a login session on five levels, and the top two go to a person. `tag` labels each commit as a feature, a fix, or a breaking change, and the labels bump the version. `annotate` reads an incident email and hands its service, severity, and start time to `jq` as JSON. `rank` puts the error that hurts users most first.",
      "[Bash techniques](/how-tos/bash/) teach these forms with real runs.",
    ],
    takeaway: "An answer is an exit code or a line of output, and the shell does the rest.",
    link: REPO,
  },

  "data-uses": {
    title: "Data science",
    label: "Data science",
    goal: "One question asked of each row turns free text into answers a query can filter, group, and count.",
    idea: [
      "Each card names a job, the question asked of each row, what the query does with the answers, and the function that asks.",
      "Filter by meaning keeps the rows that are complaints. Category column picks a category for each row, and the query groups by it. Extract fields turns a receipt's free text into amount, date, and vendor columns.",
      "Find duplicates asks whether two rows name the same company and merges the matching pairs. Score and average rates how happy each review is and averages by product. Count topics tags each ticket with its topics and counts them.",
      "Check at load sends incomplete addresses to quarantine. Top N per group keeps the 3 leads in each account likeliest to buy. Second opinion lists the rows where the model and the existing label disagree.",
      "The [tables and frames page](/learn/beatles-bench/data/) shows a real query on Beatles songs. To set one up, start at [Install](/install/).",
    ],
    takeaway: "Ask your data a question, row by row, where it already lives.",
    link: REPO,
  },

  retrieval: {
    title: "Four ways to retrieve.",
    label: "Retrieval",
    goal: "The four ways combine to find records in a large collection at an affordable cost.",
    idea: [
      "Keyword search matches shared words. TF-IDF and BM25 work this way. Semantic search matches similar meaning. Embeddings work this way. Each text becomes a list of numbers, and close lists mean close meaning. Hybrid search blends the two scores.",
      "Classification asks a question about each item. Should it stay? How well does it fit? Which tags apply? A language model reads the item and answers. Each answer carries a probability.",
      "The strategies combine. A cheap search narrows a large collection. Classification then judges the shortlist.",
      "Classification can also stack on itself. Ask a coarse question first. Then ask finer questions of the fewer items that pass.",
      "Cost grows with the number of items judged. Keyword and semantic search look items up in a prebuilt index. Classification sends every item it judges to a language model. Narrowing first keeps the cost affordable.",
    ],
    takeaway: "Combine them: search narrows the set, and classification judges what is left.",
    link: REPO,
  },

  competitors: {
    title: "Two weeks later, the field arrived.",
    label: "The field arrives",
    goal: "Within two weeks of Jev's launch, other decision models arrived, and ThinkThen speaks to the ones that share Jev's interface.",
    idea: [
      "TypeSafe launched Jev on 2026-09-15. Two weeks later, the field had moved on four fronts.",
      "Liquid released d1 on 2026-09-29. Liquid calls it the first decision foundation model, and Liquid AI is a spin-off from MIT. d1 speaks the System One API, so ThinkThen runs on it. See [Liquid d1](/install/backends/liquid/).",
      "Ollama 0.35 shipped on 2026-09-30. It serves the nimble and tev1 models on your own machine through the same API. See [Ollama](/install/backends/ollama/).",
      "On 2026-09-30, Benchmark Heaven's JevBench board ranked 106 systems. Three days earlier it listed 62 open models. Cygnet tops its composite score, and Jev is third.",
      "OpenAI announced a Decisions API at DevDay on 2026-09-29. Its model, Luna, answers predefined questions over text or images. See [OpenAI Decisions API](/install/backends/openai/).",
      "The slide's last line names the aim: ThinkThen is one SDK for every decision API.",
    ],
    source: [
      { text: "Ollama's announcement", href: 'https://ollama.com/blog/ollama-now-supports-jev-style-decision-models' },
      { text: "the JevBench board", href: 'https://www.benchmarkheaven.com/jev-models/v1.5.4' },
      { text: "OpenAI's DevDay 2026 recap", href: 'https://openai.com/index/devday-2026-recap/' },
    ],
    takeaway: "Decision models now come from more than one maker, and one interface reaches them.",
    link: REPO,
  },

  "bench-field": {
    title: "Beatles Bench scores the field.",
    label: "The scores",
    goal: "Beatles Bench asks every system the same questions, Jev leads the decision models, and anyone can replay the answers.",
    idea: [
      "The bench asks every system the same 1,501 questions. The hard set holds 505 of them, such as word traps and two-step questions. The easy set holds the other 996.",
      "Jev leads the decision models. It gets 70.5% of all the questions right and 56.9% of the hard ones. Liquid d1 gets 67.6% and 52.9%. Nimble 9B gets 51.9% and 41.6%. Kev-4B gets 47.2% and 39.0%. Laya gets 35.8% and 29.9%.",
      "Jev and d1 sit close, so the bench compares them question by question. It counts the questions only one of them got right: Jev alone right on 191, d1 alone on 140. A gap that wide is unlikely to come from chance.",
      "GLM-5.3 Flash is a general language model, run with no reasoning. It gets 96.7% and 95.4%. The slide grays it out as the yardstick. It is not a decision model.",
      "OpenAI announced its Decisions API on 2026-09-29. The bench has not run it, so it has no row.",
      "The bench is open source. The repository holds the songs, the questions and their right answers, and a saved recording of every answer. The data is CC BY-SA 4.0.",
    ],
    source: { text: "the bench's five-model report", href: `${REPO}/blob/main/reports/models.md` },
    blocks: [
      { caption: "Get the bench.", code: `git clone ${REPO}\ncd beatles-bench` },
      { caption: "Replay the Jev run. It is free and needs no key.", code: "./run.sh" },
      { caption: "Rerun it all fresh, on Jev or on your own backend.", code: "export THINKTHEN_BASE_URL=https://your-server/v1\nexport THINKTHEN_API_KEY=...\n./run.sh my-rerun" },
    ],
    takeaway: "Every system answers the same questions, and you can replay every answer for free.",
    link: REPO,
  },

  backends: {
    title: "Bring your own backend.",
    label: "Your own backend",
    goal: "Any server with Jev's interface can answer, and a bar must be tuned again on the new model.",
    idea: [
      "Any server with [the same interface as Jev](/install/backends/system-one/) can answer. Name it with `--url`.",
      "`thinkthen check` sends four fixed requests to check that a server works. With `--plan`, it prints its plan and sends nothing.",
    ],
    see: {
      '1-check': "The check names the address and the model it would ask.",
    },
    slideNote: "The slide's console is a real run of thinkthen check on 2026-10-01, from thinkthen 55db6f59f. It checked ThinkThen's conformance server on the same machine, with no key.",
    source: { text: 'the check specification', href: 'https://github.com/botassembly/thinkthen/blob/main/specification/check.md' },
    lesson: "A bar tuned on one model does not carry to another. Run `audit` again on your labeled records before you trust a new backend.",
    takeaway: "A new model needs its own bar.",
    link: REPO,
  },
};

// The ordered list the side list and the pager follow. The section's first
// page comes first.
export const FIRST = { slug: '', title: 'Beatles Bench', label: 'Beatles Bench', route: '/learn/beatles-bench/', group: null };

export const PAGES = [FIRST, ...GROUPS.flatMap(([group, slugs]) => slugs.map((slug) => {
  const a = ARTICLES[slug];
  if (!a) throw new Error(`beatles: the group ${group} names ${slug}, and no article has it`);
  return {
    ...a,
    slug,
    group,
    label: a.label || slug,
    command: a.command ?? !a.label,
    route: `/learn/beatles-bench/${slug}/`,
    slide: `/learn/beatles-bench/${slug}.webp`,
  };
}))];

for (const slug of Object.keys(ARTICLES)) {
  if (!PAGES.some((p) => p.slug === slug)) throw new Error(`beatles: ${slug} belongs to no group`);
}

// The Beatles page for a function, or null.
export const beatlesPageFor = (name) => PAGES.find((p) => p.command && p.slug === name) || null;
