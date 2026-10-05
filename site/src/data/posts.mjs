// Blog posts written with front matter in src/articles/. A post marked
// `draft: true` builds only under `npm run dev`, or when THINKTHEN_DRAFTS=1 is
// set for a build. A normal build leaves it out, so a draft cannot publish by
// accident.

import { postLineProblems } from '../lib/post-line.mjs';

const files = import.meta.glob('../articles/*.md', { eager: true });

export const SHOW_DRAFTS = import.meta.env.DEV || process.env.THINKTHEN_DRAFTS === '1';

const articles = Object.values(files).filter((f) => f.frontmatter?.slug);
for (const article of articles) {
  const problems = postLineProblems(article.frontmatter);
  if (problems.length) throw new Error(`${article.frontmatter.slug}: ${problems.join(', ')}`);
}

export const POSTS = articles
  .filter((f) => SHOW_DRAFTS || !f.frontmatter.draft)
  .map((f) => ({ ...f.frontmatter, Content: f.Content, href: `/blog/${f.frontmatter.slug}/` }));
