// The menu includes the twenty-one published guides alongside existing references.
// One list supplies navigation and the build's canonical target inventory.
import { functions } from './function-surfaces.mjs';
const page = (label, slug) => ({ label, slug, route: slug ? `/${slug}/` : '/' });
const sdks = [['Python', 'python'], ['pandas', 'pandas'], ['Polars', 'polars'], ['TypeScript', 'typescript'], ['Go', 'go'], ['Java', 'java'], ['Kotlin', 'kotlin'], ['C#', 'csharp'], ['R', 'r'], ['Ruby', 'ruby'], ['Rust', 'rust'], ['Swift', 'swift'], ['Zig', 'zig'], ['C', 'c'], ['C++', 'cpp'], ['Shared SQL', 'sql'], ['DuckDB', 'duckdb'], ['SQLite', 'sqlite'], ['PostgreSQL', 'postgresql']];
export const menu = [
  { label: 'Home', items: [page('Home', '')] },
  { label: 'Start', items: [page('Quickstart', 'quickstart'), page('Installation', 'install'), page('Configuration', 'configuration')] },
  { label: 'Functions', items: functions.map((name) => page(name, `functions/${name}`)) },
  { label: 'Command line', items: [['Command guide', 'guide'], ['File types', 'file-types'], ['JSON Lines', 'json-lines'], ['Exit codes', 'exit-codes'], ['Toolboxes', 'toolboxes'], ['Agents', 'agents']].map(([label, slug]) => page(label, `command-line/${slug}`)) },
  { label: 'SDKs', items: sdks.map(([label, slug]) => page(label, `sdks/${slug}`)) },
  { label: 'How-tos', items: [
    page('Triage a support inbox', 'how-tos/triage-a-support-inbox'),
    page('Fail a CI job on a yes-or-no question', 'how-tos/fail-a-ci-job-on-a-yes-or-no-question'),
    page('Measure answers against your own labels', 'how-tos/measure-answers-against-your-own-labels'),
    page('Screen papers for a review', 'how-tos/screen-papers-for-a-review'),
    page('Find the same company in two lists', 'how-tos/find-the-same-company-in-two-lists'),
    page('Break a problem into decisions', 'how-tos/break-a-problem-into-decisions'),
  ] },
  { label: 'Reference', items: [['Troubleshooting', 'troubleshooting'], ['Options', 'options'], ['JSON output', 'json-output'], ['Release notes', 'release-notes']].map(([label, slug]) => page(label, `reference/${slug}`)) },
];
export const menuPages = menu.flatMap(({ items }) => items);
export const sidebar = menu.map(({ label, items }) => ({ label, items: items.map(({ label: name, route }) => ({ label: name, link: route })) }));
