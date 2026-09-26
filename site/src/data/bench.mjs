// The Beatles Bench section: its pages in reading order and their groups.
//
// scripts/pull-bench.mjs writes bench/pages.json from the bench's own list.
// One page, every-language, is written here. The build fails when a group
// names a page that does not exist, or a page belongs to no group.

import pulled from './bench/pages.json';

export const PIN = pulled.pin;
export const REPO = 'https://github.com/botassembly/beatles-bench';

const HERE = [
  { slug: 'every-language', title: 'One question, three answers', route: '/beatles-bench/every-language/' },
];

export const GROUPS = [
  ['Start', ['', 'run-it-for-free', 'the-data']],
  ['The ten functions', ['decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate']],
  ['Tune your bar', ['audit', 'diff']],
  ['Your language', ['every-language']],
  ['What Jev knows', ['what-jev-knows', 'blind-spots', 'rad']],
];

// The name in the side list. A command page shows its command.
const COMMANDS = new Set(GROUPS[1][1].concat(GROUPS[2][1]));
const bySlug = new Map([...pulled.pages, ...HERE].map((p) => [p.slug, p]));

export const PAGES = GROUPS.flatMap(([group, slugs]) => slugs.map((slug) => {
  const page = bySlug.get(slug);
  if (!page) throw new Error(`bench: the group ${group} names ${slug || 'the front page'}, and no page has it`);
  return { ...page, group, label: COMMANDS.has(slug) ? slug : page.title, command: COMMANDS.has(slug) };
}));

for (const slug of bySlug.keys()) {
  if (!PAGES.some((p) => p.slug === slug)) throw new Error(`bench: ${slug} belongs to no group`);
}

// The worked example for a function page, or null.
export const benchPageFor = (name) => PAGES.find((p) => p.command && p.slug === name) || null;
