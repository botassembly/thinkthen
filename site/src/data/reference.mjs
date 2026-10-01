// The Reference section: the pages its side list shows, in order. The
// overview at /reference/ comes first. Each later page keeps its captions
// here, keyed by script name, so a page reads its words from one place.

export const REFERENCE_PAGES = [
  { slug: '', title: 'Reference', label: 'Overview', group: null },
  { slug: 'answers', title: 'How answers work', label: 'How answers work', group: 'Answers' },
  { slug: 'annotate', title: "annotate's edge cases", label: "annotate's edge cases", group: 'annotate' },
  { slug: 'question-sets', title: 'Question sets', label: 'Question sets', group: 'annotate' },
].map((p) => ({ ...p, route: p.slug ? `/reference/${p.slug}/` : '/reference/' }));

export const referencePage = (slug) => {
  const page = REFERENCE_PAGES.find((p) => p.slug === slug);
  if (!page) throw new Error(`reference: no page ${slug}`);
  return page;
};

// Captions for the annotate edge-case page, keyed by script name.
export const ANNOTATE_SEE = {
  '1-lines': 'A line is text, not an object. Its answers come back under value, and the line itself under input.',
  '2-clash': 'The record already holds steps. annotate refuses it at exit 2 and sends nothing for it.',
  '3-plan': 'The second record holds area. The plan refuses it at exit 2 and sends nothing.',
  '4-details': 'Under --details the record keeps its own steps under input. The answer named steps sits under value.',
  '5-continue': 'B-8 has no /body. It gets one error row in its place. B-7 and B-9 get their answers, and the run exits 7.',
};

// Captions for the question-set page, keyed by script name.
export const QUESTION_SETS_SEE = {
  '1-plan': 'The plan shows each question\'s pointers. urgent reads both fields as one object, keyed subject and body. team reads only the body.',
};
