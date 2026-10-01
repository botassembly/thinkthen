// The Reference section: the pages its side list shows, in order. The
// overview at /reference/ comes first. Each later page keeps its captions
// here, keyed by script name, so a page reads its words from one place.

export const REFERENCE_PAGES = [
  { slug: '', title: 'Reference', label: 'Overview', group: null },
  { slug: 'answers', title: 'How answers work', label: 'How answers work', group: 'Answers' },
].map((p) => ({ ...p, route: p.slug ? `/reference/${p.slug}/` : '/reference/' }));

export const referencePage = (slug) => {
  const page = REFERENCE_PAGES.find((p) => p.slug === slug);
  if (!page) throw new Error(`reference: no page ${slug}`);
  return page;
};
