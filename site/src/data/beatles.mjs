// The Beatles Bench pages under /learn/beatles-bench/, in the order of the
// talk. Each page is a short article: the slide, the idea, one example, the
// same example at another bar, and the lesson.
//
// The examples live in examples/beatles/<slug>/. Each runs in the Beatles
// Bench folder examples/beatles/folders.json names for its page, and answers
// from a saved recording. `see` gives the caption for each example and
// `headings` the heading above it. Prose marks code with backticks.

export const REPO = 'https://github.com/botassembly/beatles-bench';
const tree = (name) => `${REPO}/tree/main/functions/${name}`;

export const GROUPS = [
  ['Start', ['strings', 'jev']],
  ['The ten functions', ['decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate']],
  ['Tune your bar', ['audit', 'diff']],
  ['What Jev knows', ['what-jev-knows', 'blind-spots', 'rad']],
  ['Backends', ['backends']],
];

const ARTICLES = {
  strings: {
    title: "Code sees strings, not meaning.",
    label: "Strings and meaning",
    idea: [
      "To code, \"Octopus's Garden\" is 16 characters. You know it is a Ringo song on Abbey Road. `grep` matches letters, and none of these titles holds the words Abbey Road.",
      "ThinkThen asks Jev what the words mean. Jev returns the probability of yes, and your threshold turns it into an answer.",
    ],
    credit: "Ringo Starr photo: UPI, 1964, public domain in the US, via Wikimedia Commons.",
    see: {
      '1-grep': "`grep` finds nothing and exits 1.",
      '2-decide': "At the default bar of 0.5, Octopus's Garden is yes and the others are no.",
      '3-band': "The band 0.3:0.7 turns Penny Lane into not sure.",
    },
    headings: { '3-band': "Change the bar" },
    lesson: "Penny Lane falls inside the band, so it goes to a person as not sure. Octopus's Garden is the only one of the three on Abbey Road.",
    link: tree('filter'),
  },

  jev: {
    title: "Jev set the standard.",
    label: "Jev",
    idea: [
      "Jev is the model behind ThinkThen, a System One model from TypeSafe. It reads a text and a question and returns a probability for every answer. It writes no text, and you train nothing.",
      "A typical answer takes about a third of a second. Roger Bannister's mile set a standard for runners. Jev sets one for code.",
    ],
    credit: "Roger Bannister photo: 6 May 1954, public domain in the US, via Wikimedia Commons.",
    see: {
      '1-choose': "Jev gives Ringo 0.84.",
      '2-bar': "A bar of 0.9 asks for more than 0.84. The answer is not sure.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "Ringo is right at 0.84. The bar decides whether 0.84 is enough.",
    link: tree('choose'),
  },

  decide: {
    title: "decide answers yes, no, or not sure.",
    idea: [
      "`decide` is an if statement that reads. Ask one yes or no question about a text. Jev returns the probability of yes. Your threshold turns it into yes, no, or not sure.",
    ],
    see: {
      '1-lines': "At the default bar of 0.5, three songs are love songs.",
      '2-details': "The details put Yesterday at 0.56.",
      '3-band': "Inside the band 0.3:0.7, Yesterday is not sure.",
    },
    headings: { '3-band': "Change the bar" },
    lesson: "Yesterday sits at 0.56. The single bar calls it yes. The band calls it not sure. The bench has no answer key for this question. You decide how sure a yes must be.",
    link: tree('decide'),
  },

  choose: {
    title: "choose selects one option.",
    idea: [
      "`choose` is a switch statement that reads. You list the options, and Jev puts a probability on each. The top option is the pick.",
    ],
    see: {
      '1-lines': "Every song gets a pick.",
      '2-bar': "Under a bar of 0.5, She Loves You is not sure.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "John and Paul sing She Loves You together. Jev's pick of John does not reach 0.5, so the bar turns it into not sure. The four sure picks stay the same, and all four are right.",
    link: tree('choose'),
  },

  tag: {
    title: "tag applies labels to data.",
    idea: [
      "`tag` names every label that fits. Each label gets its own probability. A label counts when it clears the bar, and the default bar is 0.5.",
    ],
    see: {
      '1-lines': "At 0.5, Octopus's Garden gets three labels.",
      '2-bar': "At 0.7, two songs lose psychedelic.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "Penny Lane and Octopus's Garden are psychedelic at 0.5 and lose the label at 0.7. The bench has no answer key for tag. The right bar depends on what you do with the labels.",
    link: tree('tag'),
  },

  score: {
    title: "score rates using your scale.",
    idea: [
      "`score` places a text on a scale you name, lowest level first. It returns a number along that scale. Here the scale runs from about 0 minutes to about 9 minutes.",
    ],
    see: {
      '1-lines': "Each song gets a length in minutes.",
      '2-cut': "Your code keeps the songs at 5 minutes or more.",
    },
    headings: { '2-cut': "Set the bar" },
    lesson: "`score` has no threshold. The score is a position on your scale, and your code draws the line. A Day in the Life runs 5:38. Jev scores it 4.87. A bar of 5 leaves it out.",
    link: tree('score'),
  },

  filter: {
    title: "filter keeps what clears your bar.",
    idea: [
      "`filter` is a grep that reads. It keeps each record where the answer is yes and passes it through unchanged.",
    ],
    see: {
      '1-bar': "At 0.7, seven songs stay.",
      '2-bar': "At 0.9, five songs stay.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "Six of the seven songs at 0.7 are on Abbey Road. A Day in the Life came out on Sgt. Pepper's Lonely Hearts Club Band. At 0.9, Octopus's Garden and Something drop out. A Day in the Life stays. A higher bar cannot remove a miss Jev is sure of.",
    link: tree('filter'),
  },

  rank: {
    title: "rank sorts by your criteria.",
    idea: [
      "`rank` is a sort that reads. It asks one yes or no question of each record and sorts the records by the probability of yes.",
    ],
    see: {
      '1-lines': "All twelve songs, likeliest hit first.",
      '2-top': "`--top 5` keeps the first five.",
    },
    headings: { '2-top': "Set the cut" },
    lesson: "`rank` orders every record and drops none. You choose where the list ends. `--top` cuts the same order and asks nothing new.",
    link: tree('rank'),
  },

  find: {
    title: "find picks one from many.",
    idea: [
      "`find` reads every line together and picks the one that answers the question. Each line sees the others. An answer can depend on the whole set.",
    ],
    see: {
      '1-find': "`find` picks Love Me Do.",
      '2-details': "Each line has a probability. u005 is the fifth line.",
    },
    headings: { '2-details': "Read the number" },
    lesson: "Love Me Do came out in October 1962, and the pick is right at 0.63. Please Please Me comes next at 0.27. `find` always names a line. Your code decides whether 0.63 is enough.",
    link: tree('find'),
  },

  annotate: {
    title: "annotate fills in a form.",
    idea: [
      "`annotate` answers a saved set of questions about each record. The set is a file, and each question in it carries its own threshold. This one asks for the lead singer, the first album, and the year, each at a bar of 0.8.",
    ],
    see: {
      '1-card': "At 0.8, Octopus's Garden leaves the album and the year not sure.",
      '2-bar': "At 0.5, every field fills.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "At 0.5, Octopus's Garden gains White Album and 1968. Both are wrong. The song came out on Abbey Road in 1969. The lower bar let the weak guesses through.",
    link: tree('annotate'),
  },

  recognize: {
    title: "recognize labels things it finds.",
    idea: [
      "`recognize` finds every name in a text and gives each one a kind from your list. Each name carries a strength.",
    ],
    see: {
      '1-low': "At 0.01, all five names come back.",
      '2-high': "At 0.98, three names are left.",
    },
    headings: { '2-high': "Change the bar" },
    lesson: "All five names are right. The bar of 0.98 drops Abbey Road Studios and the album Abbey Road. Their strengths did not change.",
    link: tree('recognize'),
  },

  relate: {
    title: "relate links names into a graph.",
    idea: [
      "`relate` takes a set of names and the relations you care about, and finds each link. Here the names are songs, singers, and albums. The file holds two relations: sung by and appears on. Each line below is one edge with its probability.",
    ],
    see: {
      '1-low': "At 0.5, fourteen edges.",
      '2-high': "At 0.8, twelve edges.",
    },
    headings: { '2-high': "Change the bar" },
    lesson: "Octopus's Garden first came out on Abbey Road. Jev links it to Revolver at 0.73, and the bar of 0.8 removes that wrong edge. The same bar removes Yesterday on Help! at 0.56, and that edge is right.",
    link: tree('relate'),
  },

  audit: {
    title: "audit finds your bar.",
    idea: [
      "You already know the right answer for some of your records. `audit` grades saved answers against those answers at every bar and suggests the bar that gets the most right. It sends no request.",
      "Here Jev was asked of 70 songs whether each is on Abbey Road.",
    ],
    see: {
      '1-default': "At the default bar of 0.5, 50 are right. audit suggests 0.78.",
      '2-bar': "At 0.78, 66 are right.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "At 0.5, Jev says yes to 20 songs from other albums. At 0.78, four remain, and no Abbey Road song is lost. audit found that bar from answers you already had.",
    link: tree('audit'),
  },

  diff: {
    title: "diff shows what changed.",
    idea: [
      "Ask the same question twice and `diff` lists every answer that changed. With an answer key, it says whether each change fixed a mistake or made one.",
      "Here the first run asks about 70 song titles from memory. The second gives Jev each song's catalog entry too. The two `jq` lines give both runs the song title as a shared id. The warning says the two runs asked different questions, and they did.",
    ],
    see: {
      '1-bar': "At 0.5, 20 answers changed, and every one became right.",
      '2-band': "Inside the band 0.2:0.8, 48 changed.",
    },
    headings: { '2-band': "Change the bar" },
    lesson: "The band reads the middle as not sure. From memory, only 20 answers clear it and are right. With context, 68 do, and none went wrong. Both runs kept their probabilities. The band changed how you read them.",
    link: tree('diff'),
  },

  "what-jev-knows": {
    title: "Jev knows more than search.",
    label: "What Jev knows",
    idea: [
      "Search matches words. Jev knows facts. On the bench's Beatles questions, a random guess gets 31% right and vector search gets 38%. Jev gets 68% from memory. A large chat model gets 96%.",
      "No song title below holds the name Paul McCartney. Search has nothing to match.",
    ],
    see: {
      '1-choose': "Jev picks d, I've Just Seen a Face, at 0.63.",
      '2-bar': "A bar of 0.7 turns the pick into not sure.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "Paul sings I've Just Seen a Face alone. The pick is right. Jev knew it from memory at 0.63. A bar of 0.7 asks for more than Jev knows.",
    link: REPO,
  },

  "blind-spots": {
    title: "Jev has blind spots.",
    label: "Blind spots",
    idea: [
      "Jev's memory fades on obscure facts. It gets 74% right on the songs most viewed on Wikipedia and 51% on the least viewed. Tricky wording drops it from 67% to 48%. Two hops are hard. Asked whether two songs came out in the same month, Jev knew both dates 27 times. It chained them right 14 times.",
      "Nothin' Shakin' is an obscure song, and George sings it.",
    ],
    see: {
      '1-choose': "Jev leans to John at 0.38 and gives George 0.13.",
      '2-bar': "Under a bar of 0.5, the wrong pick becomes not sure.",
    },
    headings: { '2-bar': "Change the bar" },
    lesson: "With no bar, the answer is John, and John is wrong. A bar of 0.5 sends this question to a person. A low probability is Jev saying it does not know.",
    link: REPO,
  },

  rad: {
    title: "Retrieval-augmented decisions (RAD)",
    label: "RAD",
    idea: [
      "Give Jev the facts in the text. Look up the record, put it in front of the question, and ask. We call it retrieval-augmented decisions.",
      "On 196 questions Jev mostly missed, it got 39% right from memory and 95% with the song catalog in the text. Context costs input tokens. Jev charges $0.042 per million input tokens, and output is free.",
    ],
    see: {
      '1-memory': "From memory, Jev is sure A Day in the Life is on Abbey Road.",
      '2-context': "With the catalog entry, the answer is no at 0.04.",
    },
    headings: { '2-context': "Add context" },
    lesson: "No sensible bar fixes a miss at 0.94. Context does. With the entry, the answer turns right. With context, probabilities sit near 0 or 1, and the choice of bar matters less.",
    link: tree('decide'),
  },

  backends: {
    title: "Bring your own backend.",
    label: "Your own backend",
    idea: [
      "ThinkThen sends each question to Jev by default. Any server with the same interface can answer instead. Name it with `--url` or `THINKTHEN_BASE_URL`.",
      "`thinkthen check` proves a server works over four requests. With `--dry-run`, it prints the requests and sends nothing.",
    ],
    see: {
      '1-check': "The check names the address and the model it would ask.",
    },
    lesson: "A bar tuned on one model does not carry to another. Run `audit` again on your labeled records before you trust a new backend.",
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
