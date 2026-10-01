// The live `thinkthen check` runs the Backends pages state. A live check
// cannot replay in the smoke run, so each page shows `check --plan` and
// states the live result from here. Each entry is copied from a check's own
// report, and `record` links where that report is kept.

import { REPO } from './repo.mjs';

export const CHECKS = {
  typesafe: [
    {
      model: 'jev-1.13.0', date: '2026-09-26', commit: 'b50e0425c', exit: 0, critical: 0, warning: 0,
      record: `${REPO}/blob/main/sdlc/records/0160-build-the-answer-contract-holds.md`,
    },
  ],
  liquid: [
    {
      model: 'd1:free', date: '2026-09-30', commit: '09ebcc5ce', exit: 0, critical: 0, warning: 0,
      record: `${REPO}/blob/main/sdlc/planning/cleanup-2026-09-30.md`,
    },
  ],
  ollama: [],
};

// One sentence for each check of a backend, or one saying none is on record.
export function checkLines(name) {
  const list = CHECKS[name];
  if (!list) throw new Error(`backend checks: no entry for ${name}`);
  return list.map((c) => ({
    ...c,
    said: `On ${c.date}, thinkthen check on build ${c.commit} asked ${c.model} and exited ${c.exit}, with ${c.critical} critical and ${c.warning} warning findings.`,
  }));
}
