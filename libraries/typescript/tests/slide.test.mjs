// The slide sample, exactly as drawn in
// repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md,
// run against the stand-in engine. The deck's import names the installed
// package (`import * as tt from "thinkthen"`); the packaging rehearsal
// installs it under that name. Here the same surface loads from the
// package root beside this test.
//
// Two captures were added so the test can assert values the deck leaves
// bare (`const first =`, `const second =`); every argument, call, and
// shape is the deck's own.
//
// One finding, recorded: the second call's comment says `null`, the real
// backend's answer. The stand-in's keyword rule maps that text to 0.03,
// under the band, so the sample's answer under the stand-in is `false`.
// The sample runs; the comment does not reproduce against the stand-in.

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { chdir, cwd } from 'node:process';
import { dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

import * as tt from '../index.mjs';

test('the slide sample runs as drawn', async () => {
  // The sample names `form.json` bare, so it runs from the folder holding
  // it, exactly as the deck's Bash example did.
  const was = cwd();
  chdir(dirname(fileURLToPath(import.meta.url)));
  try {
  const text = 'I want a refund for order 9';
  const reviews = [
    'I want a refund for order 9',
    'Just saying hi',
    'Please refund my broken mug',
  ];
  const tickets = [
    'I want a refund for order 9',
    'Maybe refund it later',
    'Just saying hi',
  ];
  const signal = AbortSignal.timeout(60_000);

  // --- the sample, as drawn -------------------------------------------

  const first = await tt.decide("Does the customer ask for a refund?",
    text);  // true

  const refund = tt.question({
    decide: "Does the customer ask for a refund?",
    threshold: [0.2, 0.8],
  });
  const second = await tt.decide(refund,
    "I was charged twice. Can you fix this?");  // null

  const complaints = await tt.filter("Is this a complaint?",
    reviews);
  const rows = await tt.annotate("form.json", tickets,
    { signal });

  // ---------------------------------------------------------------------

  assert.equal(first, true, 'the first call matches its comment');
  // The finding: the stand-in answers false; the comment's null is the
  // real backend's answer. See this file's header.
  assert.equal(second, false, 'the stand-in maps the second text to 0.03, under the band');
  assert.deepEqual(complaints, [
    'I want a refund for order 9',
    'Please refund my broken mug',
  ]);
  assert.equal(rows.length, 3);
  // wants_refund follows the stub rule; the choose under the stand-in
  // weighs its options' own text, the slide's three options name no
  // keyword, the uniform spread clears no cut, and the field reads null —
  // the real backend's answer is billing; urgency is the specification's
  // weighted position.
  assert.equal(rows[0].wants_refund, true);
  assert.ok(rows[0].team === null || ['billing', 'shipping', 'account'].includes(rows[0].team));
  assert.equal(typeof rows[0].urgency, 'number');
  assert.equal(rows[2].wants_refund, false);
  } finally {
    chdir(was);
  }
});
