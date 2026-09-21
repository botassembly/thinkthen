// The slide sample, exactly as drawn in
// repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md,
// run against the stand-in engine. The deck's import names the installed
// package (`import * as tt from "thinkthen"`); the packaging rehearsal
// installs it under that name. Here the same surface loads from the
// package root beside this test.
//
// Two captures were added so the test can assert values the deck leaves
// bare (`const team =`, `const topics =`, `const firstUrgent =`); every
// argument, call, and shape is the deck's own.
//
// The findings, recorded: the stand-in weighs an option's, a label's, or a
// record's own text. The comment answers ("billing", the three labels) are
// the real backend's and do not reproduce against the stand-in: its choose
// spreads the option list uniformly and answers null, and its tag holds no
// label. The sample runs as drawn; the comments do not.

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { chdir, cwd } from 'node:process';
import { dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

import * as tt from '../index.mjs';

test('the slide sample runs as drawn', async () => {
  // Fixtures. The null rule is the stub's own: a record naming a refund
  // scores 0.97, "maybe" 0.55, anything else 0.03.
  const was = cwd();
  chdir(dirname(fileURLToPath(import.meta.url)));
  try {
  const text = 'I want a refund for order 9';
  const message = 'I was charged twice and want a refund for order 9';
  const inbox = [
    'Where is my order?',
    'I want a refund for order 9',
    'Hello team',
    'maybe escalate this one',
    'The refund never arrived',
    'Weekly summary attached',
  ];

  // --- the sample, as drawn -------------------------------------------

  // pick one option: "billing"
  const options = ["billing", "shipping", "account"];
  const team = await tt.choose("Which team owns it?", text, { options });

  // every label that fits: ["billing", "shipping", "urgent"]
  const labels = ["billing", "shipping", "urgent", "praise"];
  const topics = await tt.tag("Which topics?", message, { labels });

  // a batch you can cancel: abort() stops the requests
  const stop = new AbortController();
  const urgent = await tt.rank("Is this urgent?", inbox, {
    top: 5,
    signal: stop.signal,
  });
  const firstUrgent = urgent[0]; // console.log in the deck

  // ---------------------------------------------------------------------

  // The stand-in's choose weighs the options' own text; these three name
  // no keyword, so the spread is uniform and the answer is null. The
  // comment's "billing" is the real backend's answer — a finding for the
  // slide owner, the same class as the band-decide finding.
  assert.equal(team, null, 'the stand-in spreads equal options');
  // The stand-in's tag weighs the labels' own text; none carries a
  // keyword, so nothing holds. The comment's three labels are the real
  // backend's answer.
  assert.deepEqual(topics, [], 'the stand-in holds no keywordless label');
  assert.equal(urgent.length, 5, 'top: 5 holds the top five of six');
  assert.equal(firstUrgent.record, 'I want a refund for order 9');
  assert.ok(firstUrgent.probability >= urgent[1].probability);
  } finally {
    chdir(was);
  }
});
