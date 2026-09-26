// The Beatles Bench pages under /learn/beatles-bench/, in the order of the
// talk. Each page is a short article: the slide, the idea, one command, the
// same command at another bar, and the lesson.
//
// Every step runs in a folder of a Beatles Bench checkout and answers from a
// saved recording. scripts/beatles-replay.mjs runs each one and writes what it
// printed to beatles/runs.json. The page shows that file, so no output here is
// typed by hand. Prose marks code with backticks.

import RUNS from './beatles/runs.json' with { type: 'json' };

export const REPO = 'https://github.com/botassembly/beatles-bench';
const tree = (name) => `${REPO}/tree/main/functions/${name}`;

const ABBEY = 'The text is the title of a song by the Beatles. It appears on the album Abbey Road.';
const LEAD = 'The text is the title of a song by the Beatles. Who sings the lead vocal on it?';
const SINGERS = `--option 'john=John Lennon' --option 'paul=Paul McCartney' \\
    --option 'george=George Harrison' --option 'ringo=Ringo Starr'`;
const SONG_YES = `jq -c '{song: .input, answer: .value, yes: .answer.probability}'`;

const RUN = 'results/runs/2026-09-25-thinkthen-jev';

export const GROUPS = [
  ['Start', ['strings', 'jev']],
  ['The ten functions', ['decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate']],
  ['Tune your bar', ['audit', 'diff']],
  ['What Jev knows', ['what-jev-knows', 'blind-spots', 'rad']],
  ['Backends', ['backends']],
];

const ARTICLES = {
  strings: {
    title: 'Code sees strings, not meaning.',
    label: 'Strings and meaning',
    idea: [
      'To code, "Octopus\'s Garden" is 16 characters. You know it is a Ringo song on Abbey Road. `grep` matches letters, and none of these titles holds the words Abbey Road.',
      'ThinkThen asks Jev what the words mean. Jev returns the probability of yes, and your threshold turns it into an answer.',
    ],
    credit: 'Ringo Starr photo: UPI, 1964, public domain in the US, via Wikimedia Commons.',
    dir: 'examples/05-filter',
    steps: [
      { see: '`grep` finds nothing and exits 1.', command: `printf '%s\\n' "Octopus's Garden" 'Hey Jude' 'Penny Lane' | grep -i 'abbey road'` },
      { see: 'At the default bar of 0.5, Octopus\'s Garden is yes and the others are no.', command: `printf '%s\\n' "Octopus's Garden" 'Hey Jude' 'Penny Lane' |
  thinkthen decide '${ABBEY}' \\
    --lines --details --replay recording | ${SONG_YES}` },
      { heading: 'Change the bar', see: 'The band 0.3:0.7 turns Penny Lane into not sure.', command: `printf '%s\\n' "Octopus's Garden" 'Hey Jude' 'Penny Lane' |
  thinkthen decide '${ABBEY}' \\
    --lines --details --threshold 0.3:0.7 --replay recording | ${SONG_YES}` },
    ],
    lesson: 'Penny Lane sits at 0.43 in both runs. The band sends it to a person as not sure. Octopus\'s Garden is the only one of the three on Abbey Road.',
    link: tree('filter'),
  },

  jev: {
    title: 'Jev set the standard.',
    label: 'Jev',
    idea: [
      'Jev is the model behind ThinkThen, a System One model from TypeSafe. It reads a text and a question and returns a probability for every answer. It writes no text, and you train nothing.',
      'A typical answer takes about a third of a second. Roger Bannister\'s mile set a standard for runners. Jev sets one for code.',
    ],
    credit: 'Roger Bannister photo: 6 May 1954, public domain in the US, via Wikimedia Commons.',
    dir: 'examples/02-choose',
    steps: [
      { see: 'Jev gives Ringo 0.84.', command: `printf '%s' "Octopus's Garden" |
  thinkthen choose '${LEAD}' \\
    John Paul George Ringo 'John and Paul duet' --details --replay recording |
  jq -c '.answer.probabilities'` },
      { heading: 'Change the bar', see: 'A bar of 0.9 asks for more than 0.84. The answer is not sure.', command: `printf '%s' "Octopus's Garden" |
  thinkthen choose '${LEAD}' \\
    John Paul George Ringo 'John and Paul duet' --threshold 0.9 --replay recording` },
    ],
    lesson: 'Ringo is right, and Jev gives him 0.84 in both runs. The bar decides whether 0.84 is enough.',
    link: tree('choose'),
  },

  decide: {
    title: 'decide answers yes, no, or not sure.',
    idea: [
      '`decide` is an if statement that reads. Ask one yes or no question about a text. Jev returns the probability of yes. Your threshold turns it into yes, no, or not sure.',
    ],
    dir: 'examples/01-decide',
    steps: [
      { see: 'At the default bar of 0.5, three songs are love songs.', command: `printf '%s\\n' 'She Loves You' 'Michelle' 'Yesterday' 'Taxman' |
  thinkthen decide 'The text is the title of a song by the Beatles. It is a love song.' \\
    --lines --details --replay recording | ${SONG_YES}` },
      { heading: 'Change the bar', see: 'Inside the band 0.3:0.7, Yesterday is not sure.', command: `printf '%s\\n' 'She Loves You' 'Michelle' 'Yesterday' 'Taxman' |
  thinkthen decide 'The text is the title of a song by the Beatles. It is a love song.' \\
    --lines --details --threshold 0.3:0.7 --replay recording | ${SONG_YES}` },
    ],
    lesson: 'Yesterday sits at 0.56 in both runs. The single bar calls it yes. The band calls it not sure. The bench has no answer key for this question. You decide how sure a yes must be.',
    link: tree('decide'),
  },

  choose: {
    title: 'choose selects one option.',
    idea: [
      '`choose` is a switch statement that reads. You list the options, and Jev puts a probability on each. The top option is the pick.',
    ],
    dir: 'examples/02-choose',
    steps: [
      { see: 'Every song gets a pick.', command: `printf '%s\\n' 'Come Together' 'Yesterday' 'Something' "Octopus's Garden" 'She Loves You' |
  thinkthen choose '${LEAD}' \\
    John Paul George Ringo 'John and Paul duet' --lines --replay recording` },
      { heading: 'Change the bar', see: 'Under a bar of 0.5, She Loves You is not sure.', command: `printf '%s\\n' 'Come Together' 'Yesterday' 'Something' "Octopus's Garden" 'She Loves You' |
  thinkthen choose '${LEAD}' \\
    John Paul George Ringo 'John and Paul duet' --lines --threshold 0.5 --replay recording` },
    ],
    lesson: 'John and Paul sing She Loves You together. Jev leans to John at 0.33 and gives the duet 0.28. The bar turns that weak pick into not sure. The four sure picks stay the same, and all four are right.',
    link: tree('choose'),
  },

  tag: {
    title: 'tag applies labels to data.',
    idea: [
      '`tag` names every label that fits. Each label gets its own probability. A label counts when it clears the bar, and the default bar is 0.5.',
    ],
    dir: 'examples/03-tag',
    steps: [
      { see: 'At 0.5, Octopus\'s Garden gets three labels.', command: `printf '%s\\n' Michelle Yesterday 'Eleanor Rigby' 'Penny Lane' "Octopus's Garden" 'Lucy in the Sky with Diamonds' |
  thinkthen tag 'The text is the title of a song by the Beatles. Which of these describe it?' \\
    'love song' sad psychedelic 'about a place' 'about the sea' --lines --replay recording` },
      { heading: 'Change the bar', see: 'At 0.7, two songs lose psychedelic.', command: `printf '%s\\n' Michelle Yesterday 'Eleanor Rigby' 'Penny Lane' "Octopus's Garden" 'Lucy in the Sky with Diamonds' |
  thinkthen tag 'The text is the title of a song by the Beatles. Which of these describe it?' \\
    'love song' sad psychedelic 'about a place' 'about the sea' --lines --threshold 0.7 --replay recording` },
    ],
    lesson: 'Penny Lane is psychedelic at 0.56, and Octopus\'s Garden at 0.68. Both drop under 0.7. The bench has no answer key for tag. The right bar depends on what you do with the labels.',
    link: tree('tag'),
  },

  score: {
    title: 'score rates using your scale.',
    idea: [
      '`score` places a text on a scale you name, lowest level first. It returns a number along that scale. Here the scale runs from about 0 minutes to about 9 minutes.',
    ],
    dir: 'examples/04-score',
    steps: [
      { see: 'Each song gets a length in minutes.', command: `printf '%s\\n' 'Her Majesty' Yesterday "Octopus's Garden" Something 'Come Together' 'A Day in the Life' 'Hey Jude' 'Revolution 9' |
  thinkthen score 'The text is the title of a song by the Beatles. How long is the recording?' \\
    'about 0 minutes' 'about 1 minute' 'about 2 minutes' 'about 3 minutes' 'about 4 minutes' \\
    'about 5 minutes' 'about 6 minutes' 'about 7 minutes' 'about 8 minutes' 'about 9 minutes' \\
    --lines --replay recording` },
      { heading: 'Set the bar', see: 'Your code keeps the songs at 5 minutes or more.', command: `printf '%s\\n' 'Her Majesty' Yesterday "Octopus's Garden" Something 'Come Together' 'A Day in the Life' 'Hey Jude' 'Revolution 9' |
  thinkthen score 'The text is the title of a song by the Beatles. How long is the recording?' \\
    'about 0 minutes' 'about 1 minute' 'about 2 minutes' 'about 3 minutes' 'about 4 minutes' \\
    'about 5 minutes' 'about 6 minutes' 'about 7 minutes' 'about 8 minutes' 'about 9 minutes' \\
    --lines --replay recording | jq -c 'select(.value >= 5)'` },
    ],
    lesson: '`score` has no threshold. The score is a position on your scale, and your code draws the line. A Day in the Life runs 5:38. Jev scores it 4.87. A bar of 5 leaves it out.',
    link: tree('score'),
  },

  filter: {
    title: 'filter keeps what clears your bar.',
    idea: [
      '`filter` is a grep that reads. It keeps each record where the answer is yes and passes it through unchanged.',
    ],
    dir: 'examples/05-filter',
    steps: [
      { see: 'At 0.7, seven songs stay.', command: `printf '%s\\n' "Octopus's Garden" 'Yellow Submarine' 'Something' 'Here Comes the Sun' 'Yesterday' 'Come Together' \\
    'Hey Jude' 'Penny Lane' 'A Day in the Life' 'Her Majesty' 'Let It Be' "Maxwell's Silver Hammer" |
  thinkthen filter '${ABBEY}' \\
    --threshold 0.7 --lines --replay recording` },
      { heading: 'Change the bar', see: 'At 0.9, five songs stay.', command: `printf '%s\\n' "Octopus's Garden" 'Yellow Submarine' 'Something' 'Here Comes the Sun' 'Yesterday' 'Come Together' \\
    'Hey Jude' 'Penny Lane' 'A Day in the Life' 'Her Majesty' 'Let It Be' "Maxwell's Silver Hammer" |
  thinkthen filter '${ABBEY}' \\
    --threshold 0.9 --lines --replay recording` },
    ],
    lesson: 'Six of the seven songs at 0.7 are on Abbey Road. A Day in the Life came out on Sgt. Pepper\'s Lonely Hearts Club Band. At 0.9, Octopus\'s Garden and Something drop out. A Day in the Life stays. A higher bar cannot remove a miss Jev is sure of.',
    link: tree('filter'),
  },

  rank: {
    title: 'rank sorts by your criteria.',
    idea: [
      '`rank` is a sort that reads. It asks one yes or no question of each record and sorts the records by the probability of yes.',
    ],
    dir: 'examples/06-rank',
    steps: [
      { see: 'All twelve songs, likeliest hit first.', command: `printf '%s\\n' Something 'Hey Jude' Blackbird 'Help!' "Octopus's Garden" 'She Loves You' Piggies Yesterday \\
    'Penny Lane' 'Her Majesty' "Can't Buy Me Love" 'Good Night' |
  thinkthen rank "The text is the title of a song by the Beatles. It is one of the Beatles' biggest hits." \\
    --lines --replay recording` },
      { heading: 'Set the cut', see: '`--top 5` keeps the first five.', command: `printf '%s\\n' Something 'Hey Jude' Blackbird 'Help!' "Octopus's Garden" 'She Loves You' Piggies Yesterday \\
    'Penny Lane' 'Her Majesty' "Can't Buy Me Love" 'Good Night' |
  thinkthen rank "The text is the title of a song by the Beatles. It is one of the Beatles' biggest hits." \\
    --lines --top 5 --replay recording` },
    ],
    lesson: '`rank` orders every record and drops none. You choose where the list ends. `--top` cuts the same order and asks nothing new.',
    link: tree('rank'),
  },

  find: {
    title: 'find picks one from many.',
    idea: [
      '`find` reads every line together and picks the one that answers the question. Each line sees the others. An answer can depend on the whole set.',
    ],
    dir: 'examples/07-find',
    steps: [
      { see: '`find` picks Love Me Do.', command: `printf '%s\\n' 'She Loves You' 'I Saw Her Standing There' 'From Me to You' 'All My Loving' 'Love Me Do' \\
    'I Want to Hold Your Hand' 'Please Please Me' "Can't Buy Me Love" "A Hard Day's Night" 'I Feel Fine' |
  thinkthen find 'These are songs by the Beatles. Which one did they release first?' --replay recording` },
      { heading: 'Read the number', see: 'Each line has a probability. u005 is the fifth line.', command: `printf '%s\\n' 'She Loves You' 'I Saw Her Standing There' 'From Me to You' 'All My Loving' 'Love Me Do' \\
    'I Want to Hold Your Hand' 'Please Please Me' "Can't Buy Me Love" "A Hard Day's Night" 'I Feel Fine' |
  thinkthen find 'These are songs by the Beatles. Which one did they release first?' --details --replay recording |
  jq -c '.answer.probabilities'` },
    ],
    lesson: 'Love Me Do came out in October 1962, and the pick is right at 0.63. Please Please Me comes next at 0.27. `find` always names a line. Your code decides whether 0.63 is enough.',
    link: tree('find'),
  },

  annotate: {
    title: 'annotate fills in a form.',
    idea: [
      '`annotate` answers a saved set of questions about each record. The set is a file, and each question in it carries its own threshold. This one asks for the lead singer, the first album, and the year, each at a bar of 0.8.',
    ],
    dir: 'examples/08-annotate',
    steps: [
      { see: 'At 0.8, Octopus\'s Garden leaves the album and the year not sure.', command: `printf '%s\\n' Blackbird "Octopus's Garden" |
  thinkthen annotate annotate-cold-card.json --lines --replay recording` },
      { heading: 'Change the bar', see: 'At 0.5, every field fills.', command: `jq '.questions[].threshold = 0.5' annotate-cold-card.json > card-0.5.json
printf '%s\\n' Blackbird "Octopus's Garden" |
  thinkthen annotate card-0.5.json --lines --replay recording` },
    ],
    lesson: 'At 0.5, Octopus\'s Garden gains White Album and 1968. Both are wrong. The song came out on Abbey Road in 1969. Jev gave each guess 0.6 in both runs. The lower bar let them through.',
    link: tree('annotate'),
  },

  recognize: {
    title: 'recognize labels things it finds.',
    idea: [
      '`recognize` finds every name in a text and gives each one a kind from your list. Each name carries a strength.',
    ],
    dir: 'examples/09-recognize',
    steps: [
      { see: 'At 0.01, all five names come back.', command: `printf '%s' "Ringo Starr wrote Octopus's Garden on a boat off Sardinia, and the band recorded it at Abbey Road Studios for the album Abbey Road." |
  thinkthen recognize person song album place --threshold 0.01 --replay recording |
  jq -c '.entities[] | {name, kind, strength}'` },
      { heading: 'Change the bar', see: 'At 0.98, three names are left.', command: `printf '%s' "Ringo Starr wrote Octopus's Garden on a boat off Sardinia, and the band recorded it at Abbey Road Studios for the album Abbey Road." |
  thinkthen recognize person song album place --threshold 0.98 --replay recording |
  jq -c '.entities[] | {name, kind, strength}'` },
    ],
    lesson: 'All five names are right. The bar of 0.98 drops Abbey Road Studios and the album Abbey Road. Their strengths did not change.',
    link: tree('recognize'),
  },

  relate: {
    title: 'relate links names into a graph.',
    idea: [
      '`relate` takes a set of names and the relations you care about, and finds each link. Here the names are songs, singers, and albums. The file holds two relations: sung by and appears on. Each line below is one edge with its probability.',
    ],
    dir: 'examples/10-relate',
    steps: [
      { see: 'At 0.5, fourteen edges.', command: `jq -c '.records[]' relate-cold.jsonl |
  thinkthen relate @relate.json --jsonl --threshold 0.5 --replay recording |
  jq -c '[.relation, .source.name, .target.name, .probability]'` },
      { heading: 'Change the bar', see: 'At 0.8, twelve edges.', command: `jq -c '.records[]' relate-cold.jsonl |
  thinkthen relate @relate.json --jsonl --threshold 0.8 --replay recording |
  jq -c '[.relation, .source.name, .target.name, .probability]'` },
    ],
    lesson: 'Octopus\'s Garden first came out on Abbey Road. Jev links it to Revolver at 0.73, and the bar of 0.8 removes that wrong edge. The same bar removes Yesterday on Help! at 0.56, and that edge is right.',
    link: tree('relate'),
  },

  audit: {
    title: 'audit finds your bar.',
    idea: [
      'You already know the right answer for some of your records. `audit` grades saved answers against those answers at every bar and suggests the bar that gets the most right. It sends no request.',
      'Here Jev was asked of 70 songs whether each is on Abbey Road.',
    ],
    dir: 'examples/11-audit',
    steps: [
      { see: 'At the default bar of 0.5, 50 are right. audit suggests 0.78.', command: `thinkthen audit rows.jsonl key.jsonl --id /input |
  jq -c '{right, wrong_yes: .false_yes, missed_yes: .false_no, suggested_bar: .suggested.cut}'` },
      { heading: 'Change the bar', see: 'At 0.78, 66 are right.', command: `thinkthen audit rows.jsonl key.jsonl --id /input --threshold 0.78 |
  jq -c '{right, wrong_yes: .false_yes, missed_yes: .false_no, suggested_bar: .suggested.cut}'` },
    ],
    lesson: 'At 0.5, Jev says yes to 20 songs from other albums. At 0.78, four remain, and no Abbey Road song is lost. audit found that bar from answers you already had.',
    link: tree('audit'),
  },

  diff: {
    title: 'diff shows what changed.',
    idea: [
      'Ask the same question twice and `diff` lists every answer that changed. With an answer key, it says whether each change fixed a mistake or made one.',
      'Here the first run asks about 70 song titles from memory. The second gives Jev each song\'s catalog entry too. The two `jq` lines give both runs the song title as a shared id. The warning says the two runs asked different questions, and they did.',
    ],
    dir: 'examples/12-diff',
    steps: [
      { see: 'At 0.5, 20 answers changed, and every one became right.', command: `jq -c '.input = {id: (.input.input | split("\\nText: ") | last)}' ../11-audit/rows.jsonl > a.jsonl
jq -c '.input = {id: (.input.input | split("\\nText: ") | last)}' ../11-audit/rows-context.jsonl > b.jsonl
thinkthen diff a.jsonl b.jsonl --threshold 0.5 --key ../11-audit/key.jsonl |
  tail -1 | jq -c '.summary | {changed, right_a, right_b, lost}'` },
      { heading: 'Change the bar', see: 'Inside the band 0.2:0.8, 48 changed.', command: `thinkthen diff a.jsonl b.jsonl --threshold 0.2:0.8 --key ../11-audit/key.jsonl |
  tail -1 | jq -c '.summary | {changed, right_a, right_b, lost}'` },
    ],
    lesson: 'The band reads the middle as not sure. From memory, only 20 answers clear it and are right. With context, 68 do, and none went wrong. Both runs kept their probabilities. The band changed how you read them.',
    link: tree('diff'),
  },

  'what-jev-knows': {
    title: 'Jev knows more than search.',
    label: 'What Jev knows',
    idea: [
      'Search matches words. Jev knows facts. On the bench\'s Beatles questions, a random guess gets 31% right and vector search gets 38%. Jev gets 68% from memory. A large chat model gets 96%.',
      'No song title below holds the name Paul McCartney. Search has nothing to match.',
    ],
    dir: RUN,
    steps: [
      { see: 'Jev picks d, I\'ve Just Seen a Face, at 0.63.', command: `printf '%s' 'Paul McCartney' |
  thinkthen choose 'The text names a member of the Beatles. Which of these songs by the Beatles has this member as its only lead singer?' \\
    --option "a=I'm a Loser" --option 'b=Girl' --option 'c=Sweet Little Sixteen' --option "d=I've Just Seen a Face" \\
    --details --replay recording | jq -c '.answer.probabilities'` },
      { heading: 'Change the bar', see: 'A bar of 0.7 turns the pick into not sure.', command: `printf '%s' 'Paul McCartney' |
  thinkthen choose 'The text names a member of the Beatles. Which of these songs by the Beatles has this member as its only lead singer?' \\
    --option "a=I'm a Loser" --option 'b=Girl' --option 'c=Sweet Little Sixteen' --option "d=I've Just Seen a Face" \\
    --threshold 0.7 --replay recording` },
    ],
    lesson: 'Paul sings I\'ve Just Seen a Face alone. The pick is right. Jev knew it from memory at 0.63. A bar of 0.7 asks for more than Jev knows.',
    link: REPO,
  },

  'blind-spots': {
    title: 'Jev has blind spots.',
    label: 'Blind spots',
    idea: [
      'Jev\'s memory fades on obscure facts. It gets 74% right on the songs most viewed on Wikipedia and 51% on the least viewed. Tricky wording drops it from 67% to 48%. Two hops are hard. Asked whether two songs came out in the same month, Jev knew both dates 27 times. It chained them right 14 times.',
      'Nothin\' Shakin\' is an obscure song, and George sings it.',
    ],
    dir: RUN,
    steps: [
      { see: 'Jev leans to John at 0.38 and gives George 0.13.', command: `printf '%s' "Nothin' Shakin'" |
  thinkthen choose '${LEAD}' \\
    ${SINGERS} \\
    --details --replay recording | jq -c '.answer.probabilities'` },
      { heading: 'Change the bar', see: 'Under a bar of 0.5, the wrong pick becomes not sure.', command: `printf '%s' "Nothin' Shakin'" |
  thinkthen choose '${LEAD}' \\
    ${SINGERS} \\
    --threshold 0.5 --replay recording` },
    ],
    lesson: 'With no bar, the answer is John, and John is wrong. A bar of 0.5 sends this question to a person. A low probability is Jev saying it does not know.',
    link: REPO,
  },

  rad: {
    title: 'Retrieval-augmented decisions (RAD)',
    label: 'RAD',
    idea: [
      'Give Jev the facts in the text. Look up the record, put it in front of the question, and ask. We call it retrieval-augmented decisions.',
      'On 196 questions Jev mostly missed, it got 39% right from memory and 95% with the song catalog in the text. Context costs input tokens. Jev charges $0.042 per million input tokens, and output is free.',
    ],
    dir: 'examples/01-decide',
    steps: [
      { see: 'From memory, Jev is sure A Day in the Life is on Abbey Road.', command: `printf '%s' 'A Day in the Life' |
  thinkthen decide '${ABBEY}' \\
    --details --replay recording | jq -c '{answer: .value, yes: .answer.probability}'` },
      { heading: 'Add context', see: 'With the catalog entry, the answer is no at 0.04.', command: `printf '%s' "Catalog:
Sgt. Pepper's Lonely Hearts Club Band (1967-05-26)
A Day in the Life (lead: Lennon; written: Lennon–McCartney; 5:38; released 1967-05-26; first album: Sgt. Pepper's Lonely Hearts Club Band)
Text: A Day in the Life" |
  thinkthen decide 'The text gives a catalog entry and then names a song by the Beatles. It appears on the album Abbey Road.' \\
    --details --replay recording | jq -c '{answer: .value, yes: .answer.probability}'` },
    ],
    lesson: 'No bar fixes a miss at 0.94. Context does. With the entry, the answer turns right. With context, probabilities sit near 0 or 1, and the choice of bar matters less.',
    link: tree('decide'),
  },

  backends: {
    title: 'Bring your own backend.',
    label: 'Your own backend',
    idea: [
      'ThinkThen sends each question to Jev by default. Any server with the same interface can answer instead. Name it with `--url` or `THINKTHEN_BASE_URL`.',
      '`thinkthen check` proves a server works over four requests. With `--dry-run`, it prints the requests and sends nothing.',
    ],
    dir: 'examples/01-decide',
    steps: [
      { see: 'The check names the address and the model it would ask.', command: `thinkthen check --dry-run --url https://your-server/v1 | head -4` },
    ],
    lesson: 'A bar tuned on one model does not carry to another. Run `audit` again on your labeled records before you trust a new backend.',
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

// Every step, keyed for the replay script.
export const STEPS = PAGES.filter((p) => p.steps).flatMap((p) => p.steps.map((s, i) => ({ key: `${p.slug}/${i}`, dir: p.dir, command: s.command })));

// The recorded output of one step. A missing run fails the build.
export function runOf(slug, i) {
  const run = RUNS.runs?.[`${slug}/${i}`];
  if (!run) throw new Error(`beatles: no recorded run for ${slug} step ${i}. Run npm run beatles-replay -- --write.`);
  return run;
}
export const BENCH_COMMIT = RUNS.bench;

// The Beatles page for a function, or null.
export const beatlesPageFor = (name) => PAGES.find((p) => p.command && p.slug === name) || null;
