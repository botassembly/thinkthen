// The Reference section: the pages its side list shows, in order. The
// overview at /reference/ comes first. Each later page keeps its captions
// here, keyed by script name, so a page reads its words from one place.

export const REFERENCE_PAGES = [
  { slug: '', title: 'Reference', label: 'Overview', group: null },
  { slug: 'answers', title: 'How answers work', label: 'How answers work', group: 'Answers' },
  { slug: 'annotate', title: "annotate's edge cases", label: "annotate's edge cases", group: 'annotate' },
  { slug: 'question-sets', title: 'Question sets', label: 'Question sets', group: 'annotate' },
  { slug: 'recording', title: 'Recording and the answer cache', label: 'Recording and the answer cache', group: 'Saved answers' },
  { slug: 'audit', title: 'audit', label: 'audit', group: 'Tools', command: true },
  { slug: 'diff', title: 'diff', label: 'diff', group: 'Tools', command: true },
  { slug: 'check', title: 'check', label: 'check', group: 'Tools', command: true },
  { slug: 'transform', title: 'transform', label: 'transform', group: 'Tools', command: true },
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
  '5-continue': 'B-8 has no /body. It gets one error row in its place. B-7 and B-9 get their answers. The run exits 7, and the test line checks it.',
};

// Captions for the question-set page, keyed by script name.
export const QUESTION_SETS_SEE = {
  '1-plan': 'The plan shows each question\'s pointers. urgent reads both fields as one object, keyed subject and body. team reads only the body.',
};

// Captions for the recording page, keyed by script name. Its examples sit
// in examples/reference/answer-cache/, because check-samples skips any folder
// named recording.
export const RECORDING_SEE = {
  '1-plan': 'The plan ends with a summary line: one record, one request, and the size of what it would send.',
  '2-dry-run': 'The old flag is refused at exit 2, and the message names the new one.',
  '3-miss': 'The folder holds one answer, and this question is not it. The run exits 5 and names the missing key.',
  '4-facts': 'The answer came from a replay folder. The facts line counts one record, no request sent and no cache answer. Seconds change on every run. jq drops them.',
};

// The caption for the one run on the Reference page.
export const STATUS_SEE = {
  '1-json': 'The schema names version 2. No backend is named, so the key would come from THINKTHEN_API_KEY.',
};

// Captions for the four tool pages, keyed by page and script name.
export const TOOLS_SEE = {
  audit: {
    '1-counts': 'Ten songs at the band 0.2:0.8: 3 right, 2 wrong, 5 not sure, and no ties.',
  },
  diff: {
    '1-cuts': 'One saved run, read at 0.5 and then at the band 0.2:0.8. Five answers become not sure, and each is withdrawn. No request is sent.',
  },
  check: {
    '1-probes': 'The plan names the four probes in the order a live check sends them.',
  },
  transform: {
    '1-list': 'The ten transforms, in name order.',
    '2-show': 'counts reads a saved run and counts its answers. jq runs it. thinkthen does not.',
  },
};
