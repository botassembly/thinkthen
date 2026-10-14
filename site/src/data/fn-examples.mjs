// One function's Bash examples, split the way its page shows them: the
// opening run and the files it reads, then the further runs and the files
// only those read. The language samples sit on the language pages.

import { captioned, files } from './samples.mjs';

export function examplesOf(fn) {
  const page = `functions/${fn.name}`;
  const all = captioned(page, fn.see, true);
  const inputs = files(page);
  const first = all[0];
  const extra = all.slice(1);
  const opening = first?.command ?? '';
  const further = extra.map((run) => run.command).join('\n');
  return {
    first,
    openingInputs: inputs.filter((file) => opening.includes(file.name)),
    extra,
    moreInputs: inputs.filter((file) => !opening.includes(file.name) && further.includes(file.name)),
  };
}
