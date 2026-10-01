// The live `thinkthen check` runs the Backends pages state. A live check
// cannot replay in the smoke run, so each page shows `check --plan` and
// states the live result from here. Each entry is copied from a check run for
// these pages. `said` replaces the usual sentence when the check printed no
// report. The site ticket's build record holds each transcript.

export const CHECKS = {
  typesafe: [],
  liquid: [
    {
      model: 'd1:free', date: '2026-09-30', commit: 'e8f2804fe',
      said: 'On 2026-09-30, thinkthen check on build e8f2804fe sent its four requests to d1:free and printed no report within five minutes. The check was stopped.',
    },
  ],
  ollama: [
    { model: 'nimble', date: '2026-09-30', commit: 'e8f2804fe', exit: 0, critical: 0, warning: 3 },
    { model: 'tev1', date: '2026-09-30', commit: 'e8f2804fe', exit: 0, critical: 0, warning: 3 },
  ],
};

// One sentence for each check of a backend, or one saying none is on record.
export function checkLines(name) {
  const list = CHECKS[name];
  if (!list) throw new Error(`backend checks: no entry for ${name}`);
  return list.map((c) => ({
    ...c,
    said: c.said ?? `On ${c.date}, thinkthen check on build ${c.commit} asked ${c.model} and exited ${c.exit}, with ${c.critical} critical and ${c.warning} warning findings.`,
  }));
}
