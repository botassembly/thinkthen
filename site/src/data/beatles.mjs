// The Beatles Bench pages under /learn/beatles-bench/, in the order of the
// talk. Each page is a short article: the slide, the idea, one example, the
// same example at another bar, and the lesson. `goal` says what the page
// must communicate. `lesson` says what the runs showed, and `takeaway` is the
// one line a reader keeps. `files` names the question files the page shows,
// from the page's bench folder.
//
// The examples live in examples/beatles/<slug>/. Each runs in the Beatles
// Bench folder examples/beatles/folders.json names for its page, and answers
// from a saved recording. `see` gives the caption for each example and
// `headings` the heading above it. `source` links the bench record behind a
// number in the prose. Prose marks code with backticks.

export const REPO = 'https://github.com/botassembly/beatles-bench';
const RESULTS = { text: "the bench's results", href: `${REPO}#results` };
const FULL = { text: "the bench's full results", href: `${REPO}/blob/main/reports/results.md` };
const tree = (name) => `${REPO}/tree/main/functions/${name}`;

export const GROUPS = [
  ['Start', ['strings', 'jev']],
  ['The ten functions', ['decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate']],
  ['Tune your bar', ['audit', 'diff']],
  ['What Jev knows', ['what-jev-knows', 'blind-spots', 'rad']],
  ['Backends', ['backends']],
];

// The choose, relate, and Jev slides show four faces cropped from one photo.
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
      "Jev is the model behind ThinkThen, a System One model from TypeSafe. It reads a text and a question and returns a probability for every answer. It writes no text, and you train nothing.",
      "Roger Bannister's mile set a standard for runners. On Beatles Bench, Jev's median answer took 0.21 seconds, and a thousand answers cost 0.015 dollars.",
    ],
    credit: `Roger Bannister photo: 6 May 1954, public domain in the US, via Wikimedia Commons. ${FACES}`,
    source: RESULTS,
    see: {
      '1-choose': "Jev gives Ringo 0.84.",
      '2-bar': "A bar of 0.9 asks for more than 0.84. The answer is not sure.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "Ringo is right at 0.84. A bar of 0.9 asks for more than Jev gives.",
    takeaway: "Jev answers with a probability, and your bar decides what counts.",
    link: tree('choose'),
  },

  decide: {
    title: "decide answers yes, no, or not sure.",
    goal: "A band turns the close calls of decide into not sure.",
    idea: [
      "`decide` is an if statement that reads. Ask one yes or no question about a text. Jev returns the probability of yes. Your threshold turns it into yes, no, or not sure.",
    ],
    see: {
      '1-lines': "At the default bar of 0.5, three songs are love songs.",
      '2-details': "The details put Yesterday at 0.56.",
      '3-band': "Inside the band 0.3:0.7, Yesterday is not sure.",
    },
    headings: { '3-band': "Change the bar" },
    lesson: "Yesterday sits at 0.56. The single bar calls it yes. The band calls it not sure. The bench has no answer key for this question.",
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
    title: "score rates using your scale.",
    goal: "score gives a position on your scale, and your code draws the line.",
    idea: [
      "`score` places a text on a scale you name, lowest level first. It returns a number along that scale. Here the scale runs from about 0 minutes to about 9 minutes.",
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
    goal: "Without `--none`, find names one line, and its probability says how much to trust the pick.",
    idea: [
      "`find` reads every line together and picks the one that answers the question. Each line sees the others. An answer can depend on the whole set.",
    ],
    see: {
      '1-find': "`find` picks Love Me Do.",
      '2-details': "Each line has a probability. u005 is the fifth line.",
    },
    headings: { '2-details': "Read the number" },
    lesson: "Love Me Do came out first, and the pick is right at 0.63. Please Please Me comes next at 0.27.",
    takeaway: "Without `--none`, find names a line. Read its probability before you trust it.",
    link: tree('find'),
  },

  annotate: {
    title: "annotate fills in a form.",
    goal: "Each question in an annotate form carries its own bar, and a low bar lets weak guesses through.",
    idea: [
      "`annotate` answers a saved set of questions about each record. Each question in the set carries its own threshold. This set asks for the lead singer, the first album, and the year, each at 0.8.",
    ],
    files: ["annotate-cold-card.json"],
    see: {
      '1-card': "At 0.8, Octopus's Garden leaves the album and the year not sure.",
      '2-bar': "`jq` sets every bar to 0.5. At 0.5, every field fills.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "At 0.5, Octopus's Garden gains White Album and 1968. Both are wrong. The song came out on Abbey Road in 1969.",
    takeaway: "A lower bar fills every field, and it lets weak guesses through.",
    link: tree('annotate'),
  },

  recognize: {
    title: "recognize labels things it finds.",
    goal: "The recognize bar picks which names you keep and never changes a strength.",
    idea: [
      "`recognize` finds every name in a text and gives each one a kind from your list. Each name carries a strength.",
    ],
    see: {
      '1-low': "At 0.01, all five names come back.",
      '2-high': "At 0.98, three names are left.",
    },
    headings: { '2-high': "Change the bar" },
    lesson: "All five names are right. The bar of 0.98 drops Abbey Road Studios and the album Abbey Road. Their strengths did not change.",
    takeaway: "The bar picks which names you keep. It never changes a strength.",
    link: tree('recognize'),
  },

  relate: {
    title: "relate links names into a graph.",
    credit: FACES,
    goal: "relate returns each link with a probability, and a higher bar removes wrong and right links alike.",
    idea: [
      "`relate` takes a set of names and the relations you care about, and finds each link. Here the names are songs, singers, and albums. The file names two relations: sung by and appears on. Each output line is one edge with its probability.",
    ],
    files: ["relate.json"],
    see: {
      '1-low': "At 0.5, fourteen edges.",
      '2-high': "At 0.8, twelve edges.",
    },
    headings: { '2-high': "Change the bar" },
    lesson: "Octopus's Garden first came out on Abbey Road. Jev links it to Revolver at 0.73, and the bar of 0.8 removes that wrong edge. The same bar removes Yesterday on Help! at 0.56, and that edge is right.",
    takeaway: "A higher bar removes wrong edges and right ones alike.",
    link: tree('relate'),
  },

  audit: {
    title: "audit finds your bar.",
    goal: "audit grades saved answers against answers you already know and suggests the bar that gets the most right.",
    idea: [
      "You already know the right answer for some of your records. `audit` grades saved answers against those answers at every bar and suggests the bar that gets the most right. It sends no request.",
      "Here Jev was asked of 70 songs whether each is on Abbey Road.",
    ],
    see: {
      '1-default': "At the default bar of 0.5, 50 are right. audit suggests 0.78.",
      '2-bar': "At 0.78, 66 are right.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "At 0.5, Jev says yes to 20 songs from other albums. At 0.78, four remain, and no Abbey Road song is lost.",
    takeaway: "The answers you already know find your bar, at no cost.",
    link: tree('audit'),
  },

  diff: {
    title: "diff shows what changed.",
    goal: "diff counts the answers that changed between two runs, and how many turned right or wrong.",
    idea: [
      "Ask the same question twice and `diff` lists every answer that changed. With an answer key, it says whether each change fixed a mistake or made one.",
      "Here the first run asks about 70 song titles from memory. The second gives Jev each song's catalog entry too. The `jq` lines key both runs by song title. The warning is right: the two runs asked different questions.",
    ],
    see: {
      '1-bar': "At 0.5, 20 answers changed, and every one became right.",
      '2-band': "Inside the band 0.2:0.8, 48 changed.",
    },
    headings: { '2-band': "Change the bar" },
    lesson: "The band reads the middle as not sure. From memory, only 20 answers clear it and are right. With context, 68 do, and none went wrong.",
    takeaway: "diff counts what a change fixed and what it broke.",
    link: tree('diff'),
  },

  "what-jev-knows": {
    title: "Jev knows more than search.",
    label: "What Jev knows",
    goal: "Jev knows facts that no word in the text gives away, and search has nothing to match.",
    idea: [
      "Search matches words. Jev knows facts. On the bench's 1,313 Beatles questions, Jev gets about two in three right from memory. The best vector search gets 38%, and a random guess gets 31%. GLM-5.3 Flash, a large chat model, gets 96%.",
      "No song title below holds the name Paul McCartney. Search has nothing to match.",
    ],
    source: RESULTS,
    see: {
      '1-choose': "Jev picks d, I've Just Seen a Face, at 0.6.",
      '2-bar': "A bar of 0.7 turns the pick into not sure.",
      '3-table': "The table shows the first measure for each function, best first. The best is `recognize` at 0.96. The worst is `tag` at 0.29, and its top pick scores 0.75.",
    },
    headings: { '2-bar': "Change the bar", '3-table': "Score every function" },
    lesson: "Paul sings I've Just Seen a Face alone. The pick is right. Jev knew it from memory at 0.6. A bar of 0.7 asks for more than Jev knows. Strict scores understate Jev where a question has more than one right answer. A song can have two lead singers. On `tag`, Jev names the whole set on 0.29 of songs, and its top pick is a true lead on 0.75.",
    takeaway: "Jev answers from what it knows. The words in the text need not match.",
    link: REPO,
  },

  "blind-spots": {
    title: "Jev has blind spots.",
    label: "Blind spots",
    goal: "Jev misses obscure facts, and a low probability is how it says so.",
    idea: [
      "Jev's memory fades on obscure facts. It gets 73% of the questions about the most viewed quarter of songs on Wikipedia right, and 49% about the least viewed. Tricky wording trips it. It gets 70% of the plain control questions and 50% of the word traps. Two hops are hard. On 26 questions, Jev knew the month a song came out and the month of an event. It still missed 13 when asked whether the two fell in the same month.",
      "Nothin' Shakin' is an obscure song, and George sings it. The slide comes from an earlier run and puts John at 0.38. The example below puts John at 0.34.",
    ],
    source: FULL,
    see: {
      '1-choose': "Jev leans to John at 0.34. Ringo is close at 0.32. George gets 0.14.",
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
      "Give Jev the facts in the text. Look up the record, put it in front of the question, and ask. We call it retrieval-augmented decisions.",
      "The bench drew 196 questions, mostly from Jev's misses. From memory, Jev got 68 right. With the song catalog in the text, it got 184. Context costs input tokens. Here the call reads 291 input tokens from memory and 368 with the entry.",
    ],
    source: RESULTS,
    see: {
      '1-memory': "From memory, Jev is sure A Day in the Life is on Abbey Road.",
      '2-context': "With the catalog entry, the answer is no at 0.04.",
    },
    headings: { '2-context': "Add context" },
    lesson: "No sensible bar fixes a miss at 0.94. With the catalog entry, the answer turns right at 0.04.",
    takeaway: "When memory fails, put the facts in the text.",
    link: tree('decide'),
  },

  backends: {
    title: "Bring your own backend.",
    label: "Your own backend",
    goal: "Any server with Jev's interface can answer, and a bar must be tuned again on the new model.",
    idea: [
      "Any server with the same interface as Jev can answer. Name it with `--url`.",
      "`thinkthen check` sends four fixed requests to check that a server works. With `--dry-run`, it prints its plan and sends nothing.",
    ],
    see: {
      '1-check': "The check names the address and the model it would ask.",
    },
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
    command: !a.label,
    route: `/learn/beatles-bench/${slug}/`,
    slide: `/learn/beatles-bench/${slug}.webp`,
  };
}))];

for (const slug of Object.keys(ARTICLES)) {
  if (!PAGES.some((p) => p.slug === slug)) throw new Error(`beatles: ${slug} belongs to no group`);
}

// The Beatles page for a function, or null.
export const beatlesPageFor = (name) => PAGES.find((p) => p.command && p.slug === name) || null;
