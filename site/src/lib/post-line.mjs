// Index summaries have one bounded source field, independent of descriptions.
export function postLineProblems(post) {
  if (typeof post.line !== 'string' || !post.line.trim()) return ['post-line-missing'];
  const problems = [];
  if (/[\r\n]/.test(post.line)) problems.push('post-line-newline');
  if ([...post.line].length > 100) problems.push('post-line-long');
  return problems;
}
