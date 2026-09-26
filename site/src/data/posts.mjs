// Blog posts written with front matter in src/articles/. A post marked
// `draft: true` builds only under `npm run dev`, or when THINKTHEN_DRAFTS=1 is
// set for a build. A normal build leaves it out, so a draft cannot publish by
// accident.

const files = import.meta.glob('../articles/*.md', { eager: true });

export const SHOW_DRAFTS = import.meta.env.DEV || process.env.THINKTHEN_DRAFTS === '1';

export const POSTS = Object.values(files)
  .filter((f) => f.frontmatter?.slug)
  .filter((f) => SHOW_DRAFTS || !f.frontmatter.draft)
  .map((f) => ({ ...f.frontmatter, Content: f.Content, href: `/blog/${f.frontmatter.slug}/` }));
