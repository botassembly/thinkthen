// One function's examples, split the way its pages show them: the opening
// run and the files it reads, then the further runs, the bigger library
// samples and the files only those read. The function page and the
// function's reference page both read this, so the two never differ.

import { SURFACES, TAB_GROUPS } from './catalog.mjs';
import { captioned, files, sample, moreSamples } from './samples.mjs';

export function examplesOf(fn) {
  const page = `functions/${fn.name}`;
  const all = captioned(page, fn.see, true);
  const inputs = files(page);
  const first = all[0];
  // The bigger library samples, in tab order.
  const order = TAB_GROUPS.flatMap((g) => g.slugs);
  const more = moreSamples(fn.name, fn.moreSee, order)
    .map((m) => ({ ...m, name: SURFACES.find((s) => s.slug === m.surface).name }));
  // A file shows next to the samples that read it: under the opening tabs
  // when an opening sample names it, and under More examples otherwise.
  const opening = [first?.command ?? '', ...order.map((slug) => sample(fn.name, slug)?.code ?? '')].join('\n');
  return {
    first,
    openingInputs: inputs.filter((file) => opening.includes(file.name)),
    extra: all.slice(1),
    more,
    moreInputs: inputs.filter((file) => !opening.includes(file.name)),
  };
}
