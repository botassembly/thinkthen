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

The packaging rehearsal found it: the binding treated the last argument as call options only (`signal`, `deadlineMs`) and built the question from the first argument alone, so the rebuilt deck's samples could not run as drawn. Before, from `sdlc/records/surfaces-notes/NOTES-packaging.md`:

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

**Vocabulary sweep, the words for numbers** (the product vocabulary page), over `index.js`, `index.d.ts`, `README.md`, and `tests/`:

```
restricted: one line, index.d.ts:102, "(never call it confidence)" — the restriction
  itself; no number of ours is named confidence, and the entity field (the
  interim `number`, renamed to `strength` on 2026-09-21) never carried it
banned: no hits (certainty, likelihood, cutoff, gray zone)
"score" as a probability name: no hits; `score` appears only as the verb and the
  position's name
not-sure synonyms: "unknown answer"/"unknown field" are defect messages about an
  unrecognized door value, and "maybe" is the null backend's evidence keyword in
  test data; none names the unsure value, which is `null` on this surface
```

**Unchecked, honestly:** the cancel, deadline, backend, local, and defect kinds cannot fire for these two functions offline — the replay engine ignores call options and the wire is not their path — so only the usage kind is proven here; the door carries the options and the six-kind mapping is the shared `invoke` path proven for the other verbs. The question-file form for `recognize` (`@names.json`) is not wired on this surface yet. `source_kind`/`target_kind` ride the edge only when the rule names kinds, which the recordings refuse, so they are carried but not exercised.

**Counts:** `tests/recognize.test.mjs` 9 pass, 0 fail; the whole offline set `44 tests, 38 pass, 0 fail, 6 skipped` (the skips are wire tests without a stub); `libraries/typescript/check.sh` green end to end; `scripts/check_surfaces.sh` green for this surface (`typescript surface: all checks green`) — the tree-wide exit is 1 today only for sibling lanes still landing (python's build, R's conformance arms), the transient state the supervisor predicted.

## 2026-09-21 — the name number is settled: `strength`

Ian settled the open item in `sdlc/issues/2026-09-21-one-rule-for-every-number-the-tool-prints.md` ("The comparison came back"): the number on a recognized name is the field `strength` — ours, computed, defined once in the manual, with its parts under details. The relation number stays `probability`; the vendor's `confidence` passes through under details only. The contract, stand-in, conformance file, and validator already carried the rename (commits 78204cb, 14f418e); this surface followed: the typed declaration in `index.d.ts` (doc comment now states the settled rule), the wrapper conversion in `index.js` (`strength: held.strength`), and the conformance slice's assertion in `tests/conformance.test.mjs`. Rerun: `./check.sh` with the stub on 8212 — `typescript surface: all checks green`, the deck's recognize call and the conformance slice included.

## 2026-09-21 — the rulings wave (languages lane)

Ruling 4: `reset_usage` is removed — the addon's `#[napi]` export, `index.js`'s wrapper and export entry, the `index.d.ts` declaration, and the tests (`verbs.test.mjs`, `conformance.test.mjs`, `types.test.ts`). The generated name lists were regenerated from `functions.toml` (14 functions). Grep proof:

```
$ grep -rn "reset_usage\|resetUsage" addon/src index.js index.d.ts index.mjs loader.cjs loader.d.ts tests
(no matches; the napi build regenerated loader.cjs and loader.d.ts, and the commit carries them)
```

Ruling 1 aftermath: the wrapper's spec build and its messages use `source`/`target`; `tests/conformance.test.mjs` reads the re-keyed rules.

Ruling 2: `the_defect_kind_maps_into_the_failure_envelope` constructs a contract `Error` with kind `defect` and asserts the envelope carries `"kind":"defect"`, `retryable: false`, and the message. The envelope moved into a named `failure_envelope` so the mapping has one home. Runs in `check.sh` (`cd addon && cargo test --quiet --lib`): 1 passed.

## 2026-09-21 — the shapes from lane B item 2 (languages lane)

The three shapes `e44d492` landed in `contract/` and `standin/`, carried
into this surface and proven offline.

**`Details.requests` and `failed_questions` (0053, 0054).** `tt.details`
now carries both beside the trail it already had:

```
$ ENGINE_NULL=1 node -e "require('./index.js').details('Is this a complaint?', 'I demand a refund today').then((d) => console.log(JSON.stringify(d)))"
{"probability":0.97,"value":true,"model":"jev-latest","digest":"0b3e8345...","sends":1,"requests":["d3476c41..."],"failed_questions":0}
```

**The failed marker, this host's spelling: a plain object.** The wrapper's
`annotatedField` now returns `{ failed: { kind, cause } }` for the door's
`{"failed":...}` field, with the key order fixed in the wrapper because
the engine's own JSON map sorts its keys. `index.d.ts` names the type
(`FailedField`, `FailureCause`) and widens `AnnotatedField`:

```
$ ENGINE_NULL=1 node -e "...annotate(set, ['order 4471: charged twice, please refund'])..."
{"refund":true,"topic":{"failed":{"kind":"backend","cause":"missing_answer"}}}
```

**The record row (go-ahead item 4).** The conformance slice builds the
ruled `{ input, value }` objects from the surface's own outputs and checks
them where the cases carry rows (`05`, `06`, `19`); no function was added.

**Conformance, offline: 73 and 74 run green.**

```
#   73-details-carries-requests: pass
#   74-annotate-preserves-good-answers: pass
# pass 1  fail 0
```

The two standing divergences (the find none arm, the per-subject relate
case) are unchanged; case 73 checks the audit's identity fields because
the null backend's own rule cannot reproduce its recorded probability.

**The fast-backend cancel.** `tests/cancel_fast.test.mjs` starts a
two-million-record null batch, aborts its `AbortSignal` one second in, and
requires the rejection within 1.5 s; the batch runs about 9.7 s deaf:

```
$ ENGINE_NULL=1 node --test tests/cancel_fast.test.mjs
ok 1 - a fast backend hears an AbortSignal within a tick   (1458 ms)
```

**The examples file.** `examples.json` holds all ten functions keyed by
name, each an awaited call and the answer the null backend gives;
`tests/examples.test.mjs` evaluates every call with `tt` in scope and
`check.sh`'s offline suite runs it:

```
ok 20 - every function example answers as the file says
```

**Full check, `./check.sh`:** build, the addon unit test, the offline
suites (46 tests: 40 pass, 6 skipped wire tests), the dead-address child,
and `tsc` — all green, exit 0.


## 2026-09-21 — the settle wave

Wired to the contract's settlements (`1fe8173`): `deadlineMs` of zero or
less is legal and spent immediately (case 27 unskipped and passing on the
null backend; the runner threads `budget_ms` into the call); `nearest`
rides in `details` (the level on a score question, null elsewhere); a
built question plus `levels` refuses naming both, like `options` and
`labels` already did. `rank` and `find` already returned the ruled pair.
The addon's deadline filter now clamps at zero instead of dropping it.
`node --test`: 45 pass, 0 fail; `tsc --strict` green; the name check
green.

## 2026-09-22 — the review fix wave: the listener leak, the checked deadline, the connector

The TypeScript lane of the surfaces branch review: group 4's abort-listener
leak and group 2's deadline crash, plus the phase-1 adoption (connector,
checked deadline, panic guard).

**The listener leak (group 4).** `callOptions` added one `abort` listener
to the caller's signal per call and never removed it; `{ once: true }`
only removes it when the abort fires. A server sharing one shutdown signal
leaked one listener per call. Reproduced against HEAD's `index.js`: the
first test in `tests/abort_listener.test.mjs` fails on call 1. The wrapper
now hands `invoke` a `release` that removes the listener in a `finally`,
and the deadline check runs before the listener is added, so a refusal
cannot leave one behind either.

**The deadline crash (group 2).** The addon computed
`Instant::now() + Duration::from_secs_f64(seconds.max(0.0))` unchecked.
Reproduced against HEAD's addon: `deadlineMs: Number.MAX_VALUE` panicked
at `core/src/time.rs:965` and took the process down — `fatal runtime
error: failed to initiate panic, error 5, aborting`, SIGABRT. The door now
takes milliseconds and converts them through the contract's
`deadline_from_millis`: zero is a spent deadline, minus one means no
deadline, and a NaN, any other negative, or an oversized budget comes back
as a `usage` rejection in the ordinary failure envelope. The unit tests
and `tests/deadline_bounds.test.mjs` pin it.

**The connector and the panic guard (phase 1).** The addon's engine is
`OnceLock<Arc<dyn tt::Engine>>` built by `StandinConnector` through the
contract's `Connector`; `CallTask::run` takes `&dyn tt::Engine`. A
`guarded` helper catches panics around the engine call and `usage()`, so a
panic cannot cross into the host process; its unit test pins the defect
envelope.

**Case 74's opt-in.** The stand-in's synthesized partial failure now fires
only under `ENGINE_SYNTHETIC_PARTIAL` (phase 1), read when the door builds
the engine on the first call, so `tests/verbs.test.mjs` sets it at the
top. Without that, the marker test read `true`.

Commands and output (offline with `ENGINE_NULL=1`; the wire run against
the stub on 8212 at 300 ms):

```
$ (cd addon && cargo test --quiet --lib)
test result: ok. 3 passed (the two new ones: a panic becomes the defect
kind; a hostile budget is a usage error not a panic)

$ ENGINE_NULL=1 node --test tests/abort_listener.test.mjs tests/deadline_bounds.test.mjs
# pass 6, # fail 0
(against HEAD's index.js the first listener test fails; against HEAD's
addon the deadline test aborts with SIGABRT)

$ ./check.sh
offline: 56 pass, 0 fail, 6 skipped; wire suites: 5 pass; types green
typescript surface: all checks green
```

## 2026-09-22 — wave 3: the second review's TypeScript items

**The deadline rule (item 10).** `deadlineMs: null` (and leaving the key
out) is now this host's one spelling of no deadline, and every negative —
including the contract's `-1` sentinel — rejects with the usage kind.
Before: `Number(null)` read as `0`, so an explicit `null` was a *spent*
deadline, and `-1` silently meant "no deadline", which a computed
`end - Date.now()` can land on by chance. The wrapper refuses the negative
before the door, and the door refuses it too, so a native caller cannot
bypass the rule. Pre-fix, two tests fail:

```
not ok 2 - a hostile budget rejects with the usage kind      (deadlineMs -1)
not ok 3 - null and an absent budget both mean no deadline   (null read as spent)
```

**Shape parity with Python (item 6).** Two shapes aligned, ten recorded
with reasons in `DIVERGENCES.md` (new): the audit trail's key is `answer`
(was `value`; the contract's own field name and every other surface's),
and `annotate` rows are the set's fields only (the `record` key is gone;
the input record is `records[index]`, the shape Python, Ruby, C, and the
conformance rows use). The alignment let three dead checks live: the
conformance runner's annotate cases were failing on a set without the
`version` the core's parser requires and were being *recorded as
divergences* rather than run — with the version carried (as the Python
runner does) cases 15, 74, 82, 83, and 84 pass, and the score cases now
assert `details.nearest` against the case's `nearest_level` instead of
noting it as unreachable. Pre-fix, the example and details tests fail:

```
not ok 5 - every function example answers as the file says  (the record key)
not ok 15 - details: probability, answer, model, digest, sends  (value vs answer)
```

**The shared panic guard (item 7).** `guarded` now calls the contract's
`catch_panic("the Node door", ...)` — one implementation for every
surface instead of the seventh copy — and the message names the boundary
and the panic's own words. The zero-listener proof is unchanged and
green: 2,000 calls on one shared `AbortSignal` leave zero listeners.

**The gate's fixture, compile-time now.** The stand-in's synthesized
partial failure is armed by the `synthetic-partial` cargo feature
(phase 1), so the old `ENGINE_SYNTHETIC_PARTIAL` opt-in in
`tests/verbs.test.mjs` was dead. `addon/Cargo.toml` declares the feature,
`npm run build:synthetic` builds the gate's copy with it (napi's
`--features`), `check.sh` uses that script, and the test no longer sets
the dead variable. `npm run build`, the packaging path, never carries the
fixture.

**Commands and output (final gate).**

```
$ ./check.sh            # exit 0
offline: 62 tests, 56 pass, 0 fail, 6 skipped (wire, dead address)
addon unit tests: 3 passed (the panic test now asserts the contract's
boundary message)
dead address: 1 pass; types: tsc --noEmit clean
typescript surface: all checks green
```

## 2026-09-23: the third review's TypeScript findings (refs surfaces-review-3)

**Gate honesty: the conformance test that could not fail.** The old runner
caught every case's failure into a results map and then asserted only eleven
named ids; `usage` and `cancel` arms returned silently, and skips were decided
by a private matcher the review found disagreeing with the other eight. The
rewrite:

- `conformance/skiptable.py` is the one skip authority: each case resolves
  through `lookup typescript <id> --verb ... --kind ... --form ... --record ...`
  (the kind facet derived from `expect.error.kind`, the record facet derived
  from a null record in the list — both derivations the private matcher did
  ad hoc).
- Every RUN case is a hard assertion; every covered case prints the table's
  own reason; the final count asserts `passed + covered === 84` and any FAIL
  ends node nonzero.
- The rewrite itself exposed two swallowed cases: `81-decide-many-null` (the
  private matcher's null-record rule never fired) and
  `26-local-missing-question-file` (the kind facet was never passed) — both
  now resolved by the shared reader with its recorded reasons.
- The reviewer's probe, reproduced and then made standing: with case 13's
  answer corrupted to 7.0, `THEN_CONF=<copy> node --test` →
  `13-score-levels: FAIL score 1.05 vs 7`, exit 1, one TAP `not ok` the gate
  counts. check.sh now runs this corruption as its own step:
  `ok: the corrupted expectation failed as it must`.

**Item 21, one deadline spelling.** `options.deadlineMs` must be a real
number: `true`, `"5"`, and `[]` refuse as usage (before, `Number([])` was
zero — a silent spent deadline); `-1` is the no-deadline sentinel beside
`null` and an absent key; every other negative refuses; zero stays spent.
Probe (`ENGINE_NULL=1 node`): `true`/`'5'`/`[]`/`-2`/`NaN` → usage;
`-1` and `null` → `true`; `0` → `deadline`. The old bounds test that
asserted "-1 refuses" was rewritten to the decided rule.

**Item 22, score with a question string.** A bare string fell through to
`specOf`, which spells it a decide question, and the score call failed as a
defect. `specFrom` now takes the score branch: the string plus
`{ levels }` in the last object, the same shape choose takes its options.
Probe: `tt.score('How strong is the claim?', 'maybe later',
{levels:['low','mid','high']})` → `1.05`; without levels → usage naming the
shape.

**Build hygiene.** `build-addon.sh` carries the `$HOME` → `/build` remap the
Python wheel build has, and every build path goes through it (the gate's
synthetic build, the packaging build). `prepack` runs the clean build, so
`npm pack` can never ship the fixture-armed test binary: the packed
`index.linux-x64-gnu.node` greps zero `/home/ian` strings (the on-disk
binary went 138 → 0 after the remap rebuild). The addon gained the
conventional `build.rs` (`napi_build::setup()`) with `napi-build` as a
build-dependency, so a plain `cargo build` of the addon works. One inert
string remains in every build: the fixture text rides inside
`standin`'s `include_str!` of the conformance file, dead code the compiler
already warns about — recorded for the standin owner, outside this folder.

`./check.sh`: exit 0 — build, shim tests, 61 offline ok with conformance at
`77 passed, 7 table-covered, 0 failed`, the standing can-fail probe, wire
suites (skipped, no stub), the dead-address refusal, and the typed sample.
