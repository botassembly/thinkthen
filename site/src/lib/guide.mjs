// Guides use the site's Markdown front matter; only publication fields differ.
export function guideProblems(guide) {
  const problems = [];
  for (const field of ['title', 'blurb', 'goal']) {
    if (typeof guide[field] !== 'string' || !guide[field].trim()) problems.push(`guide ${field} is required`);
  }
  if (!/^[a-z][a-z0-9-]*$/.test(guide.slug || '')) problems.push('guide slug is invalid');
  if (typeof guide.draft !== 'boolean') problems.push('guide draft must be explicit');
  for (const field of ['prerequisites', 'limits']) {
    if (!Array.isArray(guide[field]) || !guide[field].length || guide[field].some(value => typeof value !== 'string' || !value.trim())) {
      problems.push(`guide ${field} must be a nonempty list`);
    }
  }
  for (const field of ['video', 'blog']) {
    const value = guide[field];
    if (guide.draft && value === undefined) continue;
    if (field === 'blog' && typeof value === 'string' && /^\/blog\/[a-z0-9-]+\/$/.test(value)) continue;
    try {
      const url = new URL(value);
      if (url.protocol !== 'https:' || url.username || url.password ||
          /^(?:localhost|example\.(?:com|net|org))$|\.(?:example|invalid|test)$/.test(url.hostname)) throw Error();
    } catch { problems.push(`guide ${field} requires an actual HTTPS link${field === 'blog' ? ' or canonical blog path' : ''}`); }
  }
  return problems;
}
