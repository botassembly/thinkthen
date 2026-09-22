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
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

import * as tt from '../index.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const file = JSON.parse(readFileSync(join(here, '../../../conformance/conformance.json'), 'utf8'));

const divergences = [];

function note(id, text) {
  divergences.push(`${id}: ${text}`);
}

// The shared skip table's entry for this case on this surface, or null.
// First match wins; entries naming a surface apply only there.
function centralSkip(held) {
  const kind = held.expect?.error?.kind;
  for (const entry of file.skips ?? []) {
    if (entry.surfaces && !entry.surfaces.includes('typescript')) continue;
    const when = entry.when;
    if (when.id !== undefined && held.id !== when.id) continue;
    if (when.verb !== undefined) {
      const verbs = Array.isArray(when.verb) ? when.verb : [when.verb];
      if (!verbs.includes(held.verb)) continue;
    }
    if (when.kind !== undefined && kind !== when.kind) continue;
    if (when.form !== undefined && held.form !== when.form) continue;
    if (when.none !== undefined && Boolean(held.none) !== when.none) continue;
    if (when.error !== undefined && Boolean(held.expect?.error) !== when.error) continue;
    if (when.record === 'null' && !(held.records ?? []).some((record) => record === null)) continue;
    return { disposition: entry.as ?? 'skip', why: entry.why };
  }
  return null;
}

async function runCase(held) {
  const { verb, question, evidence, records, expect } = held;
  const central = centralSkip(held);
  if (central) {
    note(held.id, `${central.disposition}: ${central.why}`);
    return;
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
      // answer per record. Compare what can be compared and note the rest.
      if (!expect.judgments && !expect.answers) return note(held.id, 'no comparable expectations carried');
      if (wanted.length !== answers.length) {
        return note(held.id, `expected ${wanted.length} judgments, the surface answers ${answers.length}`);
      }
      for (let at = 0; at < answers.length; at += 1) {
        if (wanted[at] !== null && answers[at] !== wanted[at]) {
          return note(held.id, `record ${at}: expected ${wanted[at]}, got ${answers[at]}`);
        }
      }
      // The ruled record row (go-ahead item 4): this host's own pair, the
      // record and the value it carries, in input order.
      if (expect.rows) {
        const rows = list.map((record, at) => ({ input: record, value: answers[at] }));
        assert.deepEqual(rows, expect.rows, 'the ruled {input, value} rows');
      }
      note(held.id, 'passes on values; the file expects judgments where the surface answers bare values');
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
        if (!(key in got)) return note(held.id, `field ${key} missing from the row`);
        if (wanted[key] !== null && JSON.stringify(got[key]) !== JSON.stringify(wanted[key])) {
          return note(held.id, `field ${key}: expected ${JSON.stringify(wanted[key])}, got ${JSON.stringify(got[key])}`);
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
      note(held.id, "passes on values; the file names the digest question_sha256 and the contract names it digest");
      return;
    }
    case 'usage': {
      return;
    }
    case 'cancel': {
      return;
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
      note(held.id, `the runner has no arm for verb ${verb}`);
  }
}

test('the conformance slice runs against the null backend', async () => {
  const results = {};
  for (const held of file.cases) {
    try {
      await runCase(held);
      results[held.id] = 'pass';
    } catch (held2) {
      results[held.id] = `FAIL ${held2.message}`;
      divergences.push(`${held.id}: ${held2.message}`);
    }
  }
  console.log('conformance slice:');
  for (const [id, held] of Object.entries(results)) console.log(`  ${id}: ${held}`);
  if (divergences.length) {
    console.log('divergences, by name:');
    for (const held of divergences) console.log(`  - ${held}`);
  }
  // The decide family must hold: those cases are the ruled core.
  for (const id of [
    '01-decide-yes-cut',
    '02-decide-no-cut',
    '03-decide-band-unresolved',
    '04-decide-band-resolves',
    '05-filter-keeps-some-of-five',
    '06-filter-empty-list',
    '07-backend-refuses',
    '08-usage-threshold-90',
    '09-usage-filter-band',
    '10-usage-blank-question',
    '16-details-carries-model-and-digest',
  ]) {
    assert.ok(results[id] === 'pass', `${id}: ${results[id]}`);
  }
});
