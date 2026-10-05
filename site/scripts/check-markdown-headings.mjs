// Independent pre-move annotate heading inventory, checked on real export surfaces.
import fs from 'node:fs';
import path from 'node:path';
const headings = ['Flat fields', 'The wrapper', 'A name clash', '--details keeps the record under input', 'A missing pointer and exit 7', 'Failed questions and exit 6'];
try {
  for (const file of ['functions/annotate.md', 'llms-full.txt']) {
    const text = fs.readFileSync(path.join(process.cwd(), 'dist', file), 'utf8');
    for (const heading of headings) {
      if (!text.split('\n').includes(`#### ${heading}`)) throw new Error(`missing preserved h4 heading: ${heading} in ${file}`);
    }
    if (!text.split('\n').includes('### Edge cases')) throw new Error(`missing Edge cases parent in ${file}`);
  }
  console.log('Markdown preservation: six h4 headings and their parent in both exports');
} catch (e) { console.error(e.message); process.exitCode = 1; }
