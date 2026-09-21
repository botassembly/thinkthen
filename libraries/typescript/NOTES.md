# Notes

Write here as you go. A note written later is a guess. Newest entry last.

## 2026-09-21 — the surface lands

- **Tried:** `npx napi build --release --platform --cargo-cwd ./addon --js loader.cjs --dts loader.d.ts .` with the addon on `napi` 2 (`napi8`, no tokio feature: the engine is blocking and `Task::compute` runs on a libuv worker). First build failed inside the repository workspace (the addon was not a member), then on dependency paths one level short; the addon got its own empty `[workspace]` table and `../../../` paths, so nine surfaces never race one members array.
- **Saw:** clean build, `index.linux-x64-gnu.node`, loader exports `CancelHandle, usage, resetUsage, call`.
- **Means:** one native door (`call(op, spec, payload, cancel, deadlineSec)`) carries all ten calls; the wrapper (`index.js`) owns every shape, the envelope carries failures as data (`{err:{kind,retryable,message}}`) because Node-API errors cannot hold fields, and the wrapper raises `ThinkThenError` with `kind` and `retryable`.

- **Tried:** `ENGINE_NULL=1 node --test tests/*.test.mjs`.
- **Saw:** first run failed on three tests; every failure traced to my own test evidence — the stand-in's keyword rule is case-sensitive and `"Maybe later"` contains no lowercase `maybe` (0.03, not 0.55), and `decide_many` with a plain string takes the default cut, where 0.55 reads `true` and `null` is impossible. Fixed the tests, not the engine. `19 pass, 0 fail, 6 skipped` offline.
- **Means:** the wrapper maps the contract faithfully; `null` is unsure, failures never read as values, and the six kinds arrive with the retry signal.

- **Tried:** the wire suites with the stub on 8212 at 300 ms (`ENGINE_WIDTH=32 THEN_TS_WIRE_URL=... node --test tests/bulk.test.mjs`).
- **Saw:** 5 pass. Bulk of 64 crossed once (requests 64, max in flight ≤ 32). The AbortSignal: cancelled kind, return well under 1.5 s, stub's request count identical after a 1.2 s settle — nothing served after the return. The deadline: its own kind, message names the limit, retryable true, stub frozen. Event-loop drift over 30 10 ms ticks during a 96-record run stayed under 250 ms beside a matching idle baseline. Twenty sequential decides: connections ≤ 3.
- **Means:** brief item 7 for TypeScript is proven: the blocking engine rides a libuv worker, the loop stays free, and an AbortSignal stops a batch and its bill.

- **Tried:** `./check.sh` (build, offline suites, wire suites when a stub answers, the dead-address child, and `tsc --noEmit --strict` over the sample itself).
- **Saw:** `typescript surface: all checks green`; `tsc` exits 0 on the first pass.
- **Means:** a wrong branch is a compile error, which is this surface's stated goal.

- **Tried:** the conformance slice (`tests/conformance.test.mjs`), all twenty cases against the null backend.
- **Saw:** 20 of 20 pass on values (cancel runs on the wire in bulk.test.mjs). Three named divergences, none shimmed silently: the file names the digest `question_sha256` where the contract names it `digest` (the values agree); case 19 expects judgment objects where the ruled shape answers `decide`'s bare value per record; case 13's `nearest_level` is not reachable on this surface because the audit trail is decide-shaped today and pick 6 parks the nearest level in `details` — the contract already returns `Scored{value, nearest}`, so the open point is where details carries it. Case 17's `cache_answers: 1` cannot be verified: the stand-in holds no disk cache and reports 0.
- **Means:** the file needs a small reconciliation pass to the contract's field names; the values already agree.

## Findings, reported rather than hidden

1. **The slide's second comment does not reproduce under the stand-in.** The comment says `null`, the real backend's answer for "I was charged twice. Can you fix this?" under a band. The stand-in's keyword rule maps that text to 0.03, under the band, so the sample answers `false`. The sample itself runs as drawn, every call and shape unchanged. The fixes are three, none mine to pick alone: a recording-replay mode in the stand-in, deck evidence that names a stub keyword, or accepting that band comments quote the real backend.
2. **The null `choose` weighs its options' own text, not the evidence.** The slide's three team options name no keyword, the uniform spread clears no cut, and the `team` field reads `null` where the deck's real answer is `billing`. A stand-in fidelity gap, named here so no one mistakes it for a binding bug.
3. **The goals page's call options are superseded.** `sdlc/planning/libraries/javascript.md` shows `{ field, threshold }` options and streaming async iterables for record verbs; ADR 0017 section 4 rules `{signal, deadlineMs}` and pick 8 rules containers that cross once. The page needs the hand-off edit; this surface follows the ADR.
4. **`annotate` over objects has no TypeScript ruling.** Python's `on="body"` names the evidence field; the slides pass strings here. This surface takes strings and throws a usage error naming the gap. A pick for the parent.
5. **`filterBytes` and the streaming forms are not built.** The goals page names them from 205's round two; pick 8's cross-once containers come first, and the buffer door is a later ticket if the maintainability table wants it.

## Removals and footprint

`npm install` twice (devDependencies: `@napi-rs/cli`, `typescript`), removed with `rm -rf node_modules`. The stub ran on 8212 and is down. No key, no paid call, nothing published, no sudo, nothing outside this folder touched.

## 2026-09-21 — the last-object shape fix

The packaging rehearsal found it: the binding treated the last argument as call options only (`signal`, `deadlineMs`) and built the question from the first argument alone, so the rebuilt deck's samples could not run as drawn. Before, from `NOTES-packaging.md`:

```
choose {options}-last  -> ERROR defect | a choose reply carried no choice
choose question-object -> "the refund team"
tag {labels}-last      -> ERROR defect | a tag reply carried no labels
tag question-object    -> []
rank {top,signal}-last -> 3 rows returned for { top: 2 }   (top ignored)
rank question-object   -> 3 rows
```

After the fix: the last object carries the question's inputs and the call's options together, per the one-shape ruling and ADR pick 2. `options` (choose), `labels` (tag), and `top` (rank, a slice of the ordered result, the same shape Ruby and Python ship) build the answer; `signal` and `deadlineMs` ride the call (the existing typed spellings; the deck shows `signal` only); any other key is a usage error naming the key. A question value or spec still carries its own inputs, and combining a question value with `options`/`labels` is refused as ambiguous. `top` is validated before any request. A bare `choose`/`tag` question string with no inputs in the last object is a usage error rather than an engine defect.

The deck's TypeScript section, as of today, runs as drawn (`tests/slide.test.mjs`):

```
urgent.length === 5, urgent[0] = "I want a refund for order 9"
team  = null   (stand-in: uniform option spread; the comment's "billing" is the real backend's)
topics = []    (stand-in: no keywordless label holds; the comment's three labels are the real backend's)
```

The two comment mismatches are the stand-in's rule, the same class as finding 2 above, and are asserted with comments saying so. New tests: the ruled shape for choose/tag/rank (question string plus the last object), `top` with a question value, `top` slicing, and five usage-error tests (unknown key, cross-verb key, bare choose, bare tag, non-positive `top`, question-value conflict).

Counts: offline `29 pass, 0 fail, 6 skipped`; the surface's own `check.sh` green (build, offline, wire on 8212, dead address, types); `scripts/check_surfaces.sh` green end to end, `all landed checks green`. The rc guard: no installer ran; `grep -c deno ~/.zshrc` = 0.

## 2026-09-21 — recognize and relate land on this surface

The two functions whose results have no fixed size, over the stand-in's recordings, offline. The addon gained two ops on the one `call` door (`recognize`, `relate`); `relate` enters through the contract's `relate_checked`, so the 255-record limit is inherited at the door. The wrapper builds the spec from the last object — `kinds` and `relations` (and the two bars) for `recognize`, `relations`, `either`, and `threshold` for `relate` — beside `signal` and `deadlineMs`, and shapes the answers back: entities with the offset conversion, relations and edges passed through with `source`, `target`, and `probability`.

**The deck's call with the recorded rules runs as written** (`tests/recognize.test.mjs`), and every entity's slice is the name:

```
entities: [['Maria Chen','person',0,10,true],['Northwind Freight','organization',18,35,true],['Chicago','place',39,46,true]]
relations: [{"name":"works_for","probability":1,"source":1,"target":2}]
edges: [{"name":"caused_by","probability":0.59,"source":1,"target":2},{"name":"caused_by","probability":0.94,"source":1,"target":4},{"name":"caused_by","probability":0.94,"source":2,"target":4},{"name":"caused_by","probability":0.84,"source":3,"target":4}]
```

**Offsets convert once, code points to UTF-16 units.** The contract counts code points; JavaScript indexes UTF-16 units. The offset case (`Le café 😀 Maria Chen arrived.`):

```
offset: 11 21 "Maria Chen" true
```

The raw contract offsets are 10 and 20 (code points); the emoji is two UTF-16 units, so the surface answers 11 and 21 and `text.slice(start, end)` is the name. One conversion per boundary in `utf16Index`.

**Finding 1, the deck's call as written.** The TypeScript section asks `located_in` on the Maria Chen sentence; the C01 recording covers `works_for` and `based_in`. The call cannot run as written until the deck's rule name changes or the recording gains the rule:

```
located_in: usage | no recorded answer for the rule located_in on this text; the recording covers works_for, based_in
```

The recorded-rule form is the passing test above; the deck form is pinned in its own test asserting exactly this usage error, so the divergence stays visible. The Python and Rust sections of the same deck page use `located_in` too — every lane hits this; the deck owner or the recordings should reconcile it.

**Finding 2, a conformance-data collision.** Cases `71-relate-R03-persubject-10` and `72-relate-R04-pairs-10` carry the identical ten records and pin different answers (five subject-object edges against ten pair edges). The stand-in's replay holds both rows and serves the pairs form (its code prefers a row whose form is not `per-subject`), so the per-subject expectation cannot be served by a replay keyed on input. The runner records this as a named divergence, not a bare failure, and it is reported upward for the build team to reconcile (drop one case, or give the per-subject form its own input). The database slice diverges on the same case independently (`71 of 72 cases ok, 1 diverged`).

**The 255-record limit, at the boundary:**

```
255: usage | no recorded answer for the records "x...
256: usage | relate takes at most 255 records and 256 came
```

255 reaches the engine; 256 refuses at the contract's door before any work.

**The any-kind end** is the one-character string `"*"` in every call and test (`located_in: ['*', 'place']` runs; a relation end naming a kind outside the asked kinds is a usage error from the contract's one parser).

**The conformance slice now runs both families**: all 41 recognize cases and 3 of 4 relate cases pass offline with every entity's slice checked and every probability compared; the fourth is finding 2.

**Vocabulary sweep, the words for numbers** (`repos/mktg/products/thinkthen/vocabulary.md`), over `index.js`, `index.d.ts`, `README.md`, and `tests/`:

```
restricted: one line, index.d.ts:102, "(never call it confidence)" — the restriction
  itself; no number of ours is named confidence, and the entity field is `number`
banned: no hits (certainty, likelihood, cutoff, gray zone)
"score" as a probability name: no hits; `score` appears only as the verb and the
  position's name
not-sure synonyms: "unknown answer"/"unknown field" are defect messages about an
  unrecognized door value, and "maybe" is the null backend's evidence keyword in
  test data; none names the unsure value, which is `null` on this surface
```

**Unchecked, honestly:** the cancel, deadline, backend, local, and defect kinds cannot fire for these two functions offline — the replay engine ignores call options and the wire is not their path — so only the usage kind is proven here; the door carries the options and the six-kind mapping is the shared `invoke` path proven for the other verbs. The question-file form for `recognize` (`@names.json`) is not wired on this surface yet. `source_kind`/`target_kind` ride the edge only when the rule names kinds, which the recordings refuse, so they are carried but not exercised.

**Counts:** `tests/recognize.test.mjs` 9 pass, 0 fail; the whole offline set `44 tests, 38 pass, 0 fail, 6 skipped` (the skips are wire tests without a stub); `libraries/typescript/check.sh` green end to end; `scripts/check_surfaces.sh` green for this surface (`typescript surface: all checks green`) — the tree-wide exit is 1 today only for sibling lanes still landing (python's build, R's conformance arms), the transient state the supervisor predicted.
