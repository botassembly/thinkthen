// The surface's slice of the one conformance file, run offline against
// the null backend. The file's expect blocks carry the 205-era spelling
// (answer/unsure/details.question_sha256) while the contract carries its
// own field names, so this runner compares at the value level where the
// shapes overlap and records every divergence by name; nothing is shimmed
// silently. Cases needing the wire (cancel mid-batch) run in bulk.test.mjs
// instead and are skipped here with a note.

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

import * as tt from '../index.mjs';

const here = dirname(fileURLToPath(import.meta.url));
// THEN_CONF overrides the file (the check's can-fail probe corrupts a
// copy); the skip decisions still come from the real table, whose reader
// is the contract's one implementation.
const held = process.env.THEN_CONF ?? join(here, '../../../conformance/conformance.json');
const file = JSON.parse(readFileSync(held, 'utf8'));

const outcomes = [];

function note(id, text) {
  // The gate's counting reads `skip <id>: why` lines, so the covered
  // cases reach the gate totals and not only this test's own summary
  // (surfaces-review-4: covered cases never reached the totals).
  outcomes.push(`${id}: ${text}`);
  console.log(`skip ${id}: ${text}`);
}

// The one shared reader's decision for this case on this surface:
// 'RUN', 'SKIP<TAB>why', or 'DIVERGE<TAB>why'. The private matcher that
// used to live here disagreed with the other eight runners; the third
// review replaced all of them with conformance/skiptable.py.
function centralSkip(one) {
  const args = [
    'python3',
    join(here, '../../../conformance/skiptable.py'),
    'lookup',
    'typescript',
    one.id,
  ];
  for (const facet of ['verb', 'form']) {
    if (one[facet] !== undefined) {
      const value = Array.isArray(one[facet]) ? one[facet].join(',') : String(one[facet]);
      args.push(`--${facet}`, value);
    }
  }
  // The kind facet is derived: an error case names its expected kind.
  const wanted = one.expect?.error?.kind;
  if (wanted !== undefined) args.push('--kind', String(wanted));
  // The record facet is derived: a case that carries a null record names
  // it, because the table's record selectors key on that shape.
  if (one.record !== undefined) {
    args.push('--record', String(one.record));
  } else if ((one.records ?? []).some((record) => record === null)) {
    args.push('--record', 'null');
  }
  // The boolean facets are derived the same way: a case marked `none`
  // carries a null-bearing shape the table selects on, and a case whose
  // expectation is an error names that. Without these the reader cannot
  // see why `25-find-none-fits` is table-covered, and the case runs red
  // (review-4, item 17 — the tip was red on exactly this).
  if (one.none !== undefined) {
    args.push('--none', String(one.none === true));
  }
  if (one.expect?.error !== undefined) {
    args.push('--error', 'true');
  }
  // The wire facet, by the rule every runner uses: an engine address in
  // the environment means the engine answers from the wire.
  if (process.env.ENGINE_BASE_URL || process.env.THINKTHEN_BASE_URL) {
    args.push('--wire', 'true');
  }
  const asked = spawnSync(args[0], args.slice(1), { encoding: 'utf8' });
  assert.equal(asked.status, 0, `skiptable: ${asked.stderr}`);
  return asked.stdout.trim();
}

async function runCase(held) {
  const { verb, question, evidence, records, expect } = held;
  const central = centralSkip(held);
  if (central !== 'RUN') {
    note(held.id, central.replace('\t', ' - '));
    return 'covered';
  }
  const spec = JSON.parse(JSON.stringify(question));
  if (Array.isArray(spec.threshold)) spec.threshold = spec.threshold.join(':');
  const text = evidence ?? '';
  const list = records ?? [];

  if (expect?.error) {
    const wanted = expect.error.kind;
    // A spent deadline is legal everywhere (settled 2026-09-21): the
    // case's budget_ms of zero reaches the engine, which returns the
    // deadline kind naming the budget without sending. Every other kind
    // runs with no call options.
    const call = wanted === 'deadline' ? { deadlineMs: held.budget_ms ?? 0 } : undefined;
    await assert.rejects(
      () => runVerb(held, spec, text, list, call),
      (raised) => raised instanceof tt.ThinkThenError && raised.kind === wanted,
    );
    return;
  }
  await runVerb(held, spec, text, list);
}

async function runVerb(held, spec, text, list, call) {
  const { verb, expect } = held;
  switch (verb) {
    case 'decide': {
      const answer = await tt.decide(spec, text, call);
      assert.equal(answer, expect.answer ?? null);
      return;
    }
    case 'decide_many': {
      const answers = await tt.decide_many(spec, list);
      const wanted = (expect.judgments ?? expect.answers ?? []).map((held) =>
        held.unsure ? null : held === true || held.answer === true ? true : false,
      );
      // The file expects judgment objects; the ruled shape is decide's own
      // answer per record, so the comparison reads either spelling.
      if (!expect.judgments && !expect.answers) {
        throw new Error(`case ${held.id} carries no comparable expectations`);
      }
      // A mismatch fails the case outright (review-4, item 17): these
      // arms used to `return note(...)`, which counted as a pass in the
      // totals while printing only a line.
      if (wanted.length !== answers.length) {
        throw new Error(`case ${held.id}: expected ${wanted.length} judgments, the surface answers ${answers.length}`);
      }
      for (let at = 0; at < answers.length; at += 1) {
        if (wanted[at] !== null && answers[at] !== wanted[at]) {
          throw new Error(`case ${held.id}: record ${at}: expected ${wanted[at]}, got ${answers[at]}`);
        }
      }
      // The ruled record row (go-ahead item 4): this host's own pair, the
      // record and the value it carries, in input order.
      if (expect.rows) {
        const rows = list.map((record, at) => ({ input: record, value: answers[at] }));
        assert.deepEqual(rows, expect.rows, 'the ruled {input, value} rows');
      }
      console.log(`  ${held.id}: passes on values; the file expects judgment objects where the surface answers bare values`);
      return;
    }
    case 'filter': {
      const kept = await tt.filter(spec, list);
      const wanted = (expect.indexes ?? []).map((index) => list[index]);
      assert.deepEqual(kept, wanted);
      if (expect.rows) {
        const rows = kept.map((record) => ({ input: record, value: true }));
        assert.deepEqual(rows, expect.rows, 'the ruled {input, value} rows');
      }
      return;
    }
    case 'choose': {
      const pick = await tt.choose(spec, text);
      if (expect.answer === null) {
        assert.equal(pick, null);
      } else {
        assert.equal(pick, expect.details?.pick ?? expect.answer);
      }
      return;
    }
    case 'score': {
      const value = await tt.score(spec, text);
      assert.ok(Math.abs(value - expect.answer) < 1e-9, `score ${value} vs ${expect.answer}`);
      const nearest = expect.details?.nearest_level;
      if (nearest !== undefined) {
        // The nearest level rides the audit trail (ADR 0017 pick 6), so
        // the case's nearest_level is checkable here now.
        const audit = await tt.details(spec, text);
        assert.equal(audit.nearest, nearest, `nearest level ${audit.nearest} vs ${nearest}`);
      }
      return;
    }
    case 'tag': {
      const labels = await tt.tag(spec, text);
      const wanted = expect.labels ?? expect.answer ?? [];
      assert.deepEqual(labels, wanted);
      return;
    }
    case 'rank': {
      const ranked = await tt.rank(spec, list);
      assert.deepEqual(
        ranked.map((one) => one.index),
        expect.ranking ?? [],
        `expected order ${JSON.stringify(expect.ranking)}, got ${JSON.stringify(ranked.map((one) => one.index))}`,
      );
      return;
    }
    case 'find': {
      const found = await tt.find(spec, list);
      assert.equal(found.index, expect.answer, `expected unit ${expect.answer}, got ${found.index}`);
      return;
    }
    case 'annotate': {
      const set = JSON.stringify({ version: 1, questions: held.set });
      if (expect.rows) {
        // The multi-record form: one answer object a record, in input
        // order, each field the bare answer (a score reads its position).
        const wantedRows = expect.rows.map((row) => row.value);
        const rows = await tt.annotate(set, list);
        assert.equal(rows.length, wantedRows.length, 'one answer a record');
        rows.forEach((got, at) => {
          const want = wantedRows[at];
          assert.deepEqual(Object.keys(got).sort(), Object.keys(want).sort(), `row ${at} fields`);
          for (const [name, value] of Object.entries(want)) {
            if (typeof value === 'number') {
              const field = got[name];
              const read = field !== null && typeof field === 'object' && 'position' in field ? field.position : field;
              assert.ok(
                typeof read === 'number' && Math.abs(read - value) < 1e-9,
                `row ${at} ${name}: ${JSON.stringify(field)} vs ${value}`,
              );
            } else {
              assert.deepEqual(got[name], value, `row ${at} ${name}`);
            }
          }
        });
        return;
      }
      const rows = await tt.annotate(set, [text]);
      const wanted = {};
      for (const [name, field] of Object.entries(expect.fields ?? expect.answers ?? {})) {
        if (field !== null && typeof field === 'object' && 'failed' in field) {
          // The ruled marker (0054), in this host's own spelling.
          wanted[name] = { failed: field.failed };
        } else {
          wanted[name] = field === null || typeof field !== 'object' ? field : field.answer;
        }
      }
      const got = rows[0];
      const wantedKeys = Object.keys(wanted);
      for (const key of wantedKeys) {
        // Review-4, item 17: a missing field or a wrong value fails the
        // case; a note line never reaches the totals.
        if (!(key in got)) throw new Error(`case ${held.id}: field ${key} missing from the row`);
        if (wanted[key] !== null && JSON.stringify(got[key]) !== JSON.stringify(wanted[key])) {
          throw new Error(`case ${held.id}: field ${key}: expected ${JSON.stringify(wanted[key])}, got ${JSON.stringify(got[key])}`);
        }
      }
      return;
    }
    case 'details': {
      const audit = await tt.details(spec, text);
      // The audit's identity fields and the two 0053/0054 additions; the
      // recorded probability is not compared because the null backend's
      // own rule cannot reproduce case 73's recorded number.
      assert.equal(audit.model, expect.details.model);
      assert.equal(audit.digest, expect.details.question_sha256);
      if (expect.details.requests !== undefined) {
        assert.deepEqual(audit.requests, expect.details.requests);
      }
      if (expect.details.failed_questions !== undefined) {
        assert.equal(audit.failed_questions, expect.details.failed_questions);
      }
      console.log(`  ${held.id}: passes on values; the file names the digest question_sha256 and the contract names it digest`);
      return;
    }
    case 'usage':
    case 'cancel': {
      // These cases carry expect.error (handled above) or a skip-table
      // entry; a case reaching this arm was silently passing before the
      // third review, so now it fails loudly instead.
      throw new Error(`case ${held.id} reached a bare ${verb} arm: it needs an expectation or a table entry`);
    }
    case 'recognize': {
      const options = {};
      if (spec.kinds) options.kinds = spec.kinds;
      if (Array.isArray(spec.relations) && spec.relations.length) {
        options.relations = {};
        for (const rule of spec.relations) options.relations[rule.name] = [rule.source, rule.target];
      }
      if (typeof spec.threshold === 'number') options.threshold = spec.threshold;
      if (typeof spec.relation_threshold === 'number') options.relationThreshold = spec.relation_threshold;
      const source = held.text ?? text;
      const found = await tt.recognize(source, options);
      const wantedNames = expect.entities ?? [];
      assert.equal(
        found.entities.length,
        wantedNames.length,
        `expected ${wantedNames.length} names, got ${found.entities.length}`,
      );
      for (let at = 0; at < wantedNames.length; at += 1) {
        const got = found.entities[at];
        assert.equal(got.text, wantedNames[at].text);
        assert.equal(got.kind, wantedNames[at].kind);
        assert.equal(source.slice(got.start, got.end), got.text, 'the slice is the name');
        assert.ok(Math.abs(got.strength - wantedNames[at].strength) < 1e-9, `strength ${got.strength}`);
      }
      const wantedRelations = expect.relations ?? [];
      assert.equal(
        found.relations.length,
        wantedRelations.length,
        `expected ${wantedRelations.length} relations, got ${found.relations.length}`,
      );
      for (let at = 0; at < wantedRelations.length; at += 1) {
        const got = found.relations[at];
        assert.deepEqual(
          { name: got.name, source: got.source, target: got.target, probability: got.probability },
          wantedRelations[at],
        );
      }
      return;
    }
    case 'relate': {
      const options = {};
      const names = [];
      const either = [];
      for (const rule of spec.relations ?? []) {
        if (typeof rule === 'string') names.push(rule);
        else if (rule.either) either.push(rule.name);
        else names.push(rule.name);
      }
      for (const rule of spec.either ?? []) either.push(typeof rule === 'string' ? rule : rule.name);
      if (names.length) options.relations = names;
      if (either.length) options.either = either;
      if (typeof spec.threshold === 'number') options.threshold = spec.threshold;
      const edges = await tt.relate(held.records ?? list, options);
      assert.deepEqual(
        edges.map((edge) => ({
          name: edge.name,
          source: edge.source,
          target: edge.target,
          probability: edge.probability,
        })),
        expect.edges ?? [],
      );
      return;
    }
    default:
      // Review-4, item 17: an unknown verb counted as a pass here; the
      // runner must refuse what it cannot assert.
      throw new Error(`case ${held.id}: the runner has no arm for verb ${verb}`);
  }
}

test('the conformance slice runs against the null backend', async () => {
  assert.ok(file.cases.length >= 84, `the conformance file must carry its cases: ${file.cases.length}`);
  const results = {};
  for (const one of file.cases) {
    try {
      const covered = await runCase(one);
      results[one.id] = covered === 'covered' ? 'covered' : 'pass';
    } catch (raised) {
      results[one.id] = `FAIL ${raised.message}`;
    }
  }
  console.log('conformance slice:');
  for (const [id, one] of Object.entries(results)) console.log(`  ${id}: ${one}`);
  if (outcomes.length) {
    console.log('table-covered outcomes:');
    for (const one of outcomes) console.log(`  - ${one}`);
  }
  const passed = Object.values(results).filter((one) => one === 'pass').length;
  const covered = Object.values(results).filter((one) => one === 'covered').length;
  const failed = Object.entries(results).filter(([, one]) => one.startsWith('FAIL'));
  console.log(`conformance: ${passed} passed, ${covered} table-covered, ${failed.length} failed`);
  // Every case is asserted or carries the table's own reason; nothing is
  // silently swallowed. A FAIL anywhere ends the run nonzero, which the
  // gate's TAP counting reads.
  assert.deepStrictEqual(
    failed.map(([id]) => id),
    [],
    'no conformance case may fail',
  );
  assert.equal(passed + covered, file.cases.length, 'every case was either run or covered');
});
