// Match the article draft switch; public lists always exclude incomplete lessons.
import { guideProblems } from '../lib/guide.mjs';
const files = import.meta.glob('../guides/*.md', { eager: true });
const lessons = Object.values(files).map(file => ({ ...file.frontmatter, Content: file.Content }));
const slugs = new Set();
for (const lesson of lessons) {
  const problems = guideProblems(lesson);
  if (slugs.has(lesson.slug)) problems.push('duplicate guide slug');
  slugs.add(lesson.slug);
  if (problems.length) throw Error(`${lesson.slug}: ${problems.join(', ')}`);
}
export const PUBLISHED_GUIDES = lessons.filter(lesson => !lesson.draft);
export const GUIDES = (import.meta.env.DEV || process.env.THINKTHEN_DRAFTS === '1') ? lessons : PUBLISHED_GUIDES;
