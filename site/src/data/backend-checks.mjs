// The live `thinkthen check` runs the Backends pages state. A live check
// cannot replay in the smoke run, so each page shows `check --plan` and
// states the live result from here. Each entry copies a check run on record.
// `source` is the URL of the ThinkThen record that holds the run, or 'site'
// for a run a site ticket made, whose build record holds the transcript.
// `said` replaces the usual sentence when the check printed no report.

const RECORDS = 'https://github.com/botassembly/thinkthen/blob/main/sdlc';

export const CHECKS = {
  llamacpp: [
    {
      model: 'local (Clef-Flash Q4_K_M)', date: '2026-10-05', exit: 0, critical: 0, warning: 0,
      source: `${RECORDS}/records/0421-local-runtime-backends.md`,
      said: 'On 2026-10-05, llama.cpp v0.6.0 with Clef-Flash Q4_K_M and alias local passed four rich text probes and all ten functions on M5 through the Linux CLI, exit 0, with no warnings. No image request ran.',
    },
  ],
  mlx: [
    {
      model: 'strands-decider-2B-hobson-v19', date: '2026-10-05', exit: 0, critical: 0, warning: 3,
      source: `${RECORDS}/records/0421-local-runtime-backends.md`,
      said: 'On 2026-10-05, the pinned Strands MLX setup passed all ten text functions on M5 through the Linux CLI, exit 0, with three expected description warnings. No image request ran.',
    },
  ],
  typesafe: [
    // The run passed no --model. specification/check.md at b50e0425c
    // resolves the model from the option, then the configuration file,
    // then jev-1.13.0, so the run asked jev-1.13.0.
    {
      model: 'jev-1.13.0', date: '2026-09-26', commit: 'b50e0425c', exit: 0, critical: 0, warning: 0,
      source: `${RECORDS}/records/0160-build-the-answer-contract-holds.md`,
    },
  ],
  liquid: [
    {
      model: 'd1:free', date: '2026-09-30', commit: '74b82a8e1', exit: 0, critical: 0, warning: 0,
      source: `${RECORDS}/issues/closed/2026-09-29-systemone-adapter-sends-null-criteria-liquid-d1-refuses.md`,
    },
  ],
  openrouter: [
    {
      model: 'typesafe/jev-1.13', date: '2026-10-04', commit: '4349b91b3', exit: 0, critical: 0, warning: 0,
      source: `${RECORDS}/records/0399-backend-paths.md`,
    },
    ...['cloudflare/clef', 'cloudflare/clef-flash'].map((model) => ({
      model, date: '2026-10-05', exit: 0, critical: 0, warning: 0,
      source: `${RECORDS}/records/0400-provider-setups-and-width.md`,
      said: `On 2026-10-05, experiment 0034 asked ${model} through OpenRouter and passed its 15-request text check with no critical findings or warnings. Its image controls showed no observable image use; the experimental exchanges establish no released image support or accuracy.`,
    })),
  ],
  perplexity: [
    {
      model: 'pplx-decider-v1-27b', date: '2026-10-04', commit: '4349b91b3', exit: 0, critical: 0, warning: 0,
      source: `${RECORDS}/records/0399-backend-paths.md`,
    },
  ],
  ollama: [
    { model: 'nimble', date: '2026-09-30', commit: 'e8f2804fe', exit: 0, critical: 0, warning: 3, source: 'site' },
    { model: 'tev1', date: '2026-09-30', commit: 'e8f2804fe', exit: 0, critical: 0, warning: 3, source: 'site' },
  ],
};

// One sentence for each check of a backend, or one saying none is on record.
// `link` is the record's URL, or null for a site run.
export function checkLines(name) {
  const list = CHECKS[name];
  if (!list) throw new Error(`backend checks: no entry for ${name}`);
  return list.map((c) => {
    if (!c.source) throw new Error(`backend checks: the ${name} check of ${c.date} names no source`);
    return {
      ...c,
      link: c.source === 'site' ? null : c.source,
      said: c.said ?? `On ${c.date}, thinkthen check on build ${c.commit} asked ${c.model} and exited ${c.exit}, with ${c.critical} critical and ${c.warning} warning findings.`,
    };
  });
}
