---
flow: build
priority: 49
opens: crates/thinkthen-core crates/thinkthen probes demos/15-find-the-line specification sdlc/planning/adr sdlc/planning sdlc/ratchet.json
---

# 0040: Find the answering unit

Status: in progress

## Outcome

`thinkthen find` reads one bounded set of text lines or JSON records, sends the set in one request, and prints the unit that best answers the question. `--none` lets the answer be that no unit fits. A staged live check must meet fixed reach and accuracy gates before implementation begins.

## Evidence and decisions

ADR 0014 measured twenty made-up documents of 11 to 14 lines. The one-request pick found 16 of 16 answering lines; `rank --top 1` found 15. `--none` found all four blank documents and falsely refused none of the sixteen answerable ones. ADR 0015 accepted `find`, but both ADRs require a comparison on documents of 100 to 250 lines before implementation. A later Banking77 run proves that 77 options reach the hosted backend. It does not test long documents, unit mapping, or `none`.

This ticket settles the remaining behavior. Ian can overturn it cheaply before implementation:

1. Without `--none`, `find` takes 2 to 255 units. With `--none`, it takes 2 to 254 units, because the sentinel is the 255th choice and the existing core ceiling stays unchanged. Empty input succeeds with no output and no request. One unit and either overflow are usage errors before a key or connection. The messages are ``find` takes 2 to 255 units` and ``find --none` takes 2 to 254 units`.
2. The whole original input may contain at most 16 MiB across all units. Count bytes incrementally as they are read, including record terminators, before building aggregate JSON, reading a key, or sending. Reuse the value of core `MAX_RECORD_BYTES` as the one size authority rather than copying the literal. Exactly 16 MiB is accepted. One byte more exits 2 with the fixed safe message ``find` reads at most 16 MiB across all units` and sends nothing.
3. Lines are the default. `--lines` is an explicit spelling of that default. `--jsonl` and `--field` follow `records.md`. CSV and TSV are refused before input or a request because the settled `find` surface has two unit framings. `--jobs` is refused because the whole run is one request.
4. The core assigns `u001` onward in input order. Request evidence is one compact JSON array of `{"id":"u001","evidence":"..."}` objects. Each evidence value is the exact text `Reading::evidence` produced. The wire choice options are those ids, with `none` last when requested. User text is never interpreted as an id or JSON structure.
5. Bare output is the selected line or JSONL record under the same preservation rules as `filter`, followed by one line feed. A `none` result prints nothing and exits 3. Any real-unit result exits 0. Without `--none`, `find` never exits 3.
6. Equal top probabilities among real units select the first input unit. Any top tie involving `none` is unresolved. A strict `none` lead is unresolved. No threshold enters.
7. `--details` has a dedicated result. `question` is `{"verb":"find","text":"...","none":BOOL}`. `value` is the selected text string or JSON value, or `null`. `answer` has `kind: "find"`, `pick` holding the first leading `uNNN` or `none`, ordered `probabilities` for every wire choice, and `confidence` only when reported. `threshold` is `null`; normal `meta` follows. Generated ids and unit count do not enter `question_sha256`; the question text and `none` policy do. The request digest still covers the whole encoded request.
8. `--dry-run` reads and bounds the complete set, prints its exact one-request plan, and reads no key. Record, replay, and cache use the existing immutable entries and digest lock. No request byte or behavior of an existing command changes.

ADR 0030 records these clarifications to the settled `find`, result, records, and channels contracts. It records the measured reach after the probe. Correct `specification/README.md` from seven version-one commands to eight.

## Phase one: measure the actual request

Write the fixtures, trusted answers, analysis, fake-binary self-test, and exact request constructor before a paid call. Hash the cases before the first call. Every invocation uses `--max-retries 0` and one shared cache/recording folder. The fake binary requires the exact mode, cache, retry setting, request count, stop rule, and safe summary.

### Feasibility

Send eight exact JSON-framed find requests: one answerable and one blank document at 100, 175, and 250 units; 255 answerable units without `none`; and 254 blank units with `none`. These pin both exact upper boundaries while keeping every request at no more than 255 choices. Stop on any refusal or unrelated failure.

The old `find --none` run used 11,183 input tokens for 239 units, about 46.79 tokens per unit including its request overhead. The eight feasibility requests contain 1,559 unit appearances, for an estimate of about 72,947 input tokens. The launch is `sdlc/scripts/live --max-tokens 120000 probes/find-0040/run feasibility`, about 64 percent headroom. That 120,000-token reservation is a durable ledger charge even when actual use is lower, so do not enlarge it.

All eight calls must be accepted before the comparison begins. Otherwise record the result and stop for a design revision. The authorized command ran once after the probe landed. All eight calls were accepted, with 70,265 billed input tokens across 1,559 unit appearances, 8 live answers, no replay, and no failure. The backend accepted both exact ceilings: 255 units without `none` and 254 with it. The 120,000-token reservation moved the live ledger from 19,500,118 to 19,620,118 charged tokens.

### Comparison

Preregister two answerable documents and one blank document at each of 100, 175, and 250 units. Run each of the nine documents through find with and without `none`, for 18 find calls. Run `rank --top 1` only over the six answerable documents, for 1,050 rank calls. Reuse feasibility recordings when a request is identical. The blank documents measure `none`; buying 525 rank judgments over them would not answer the comparison gate. This is a reach gate, not a marketing accuracy claim.

Run and analyze the 18 find calls first. Every call must be accepted. On the six answerable documents, each find arm must hit at least 5 of 6 and must not trail rank after the rank arm runs. The `--none` arm must find all 3 of 3 blank documents and falsely refuse 0 of 6 answerable documents. Any miss of those gates stops implementation and returns the evidence for a design revision.

The old rank arm used 69,143 input tokens for 239 calls, about 289.3 per call. Its 1,050 calls therefore suggest about 303,765 tokens. Feasibility measured 45.07055805003207 billed input tokens per unit appearance, below the preregistered old rate of 11,183/239. Using the larger old rate for 3,150 find unit appearances produces a combined estimate of 451,157.32217573223. Fifteen percent headroom rounded to the next thousand fixes the comparison reservation at 519,000. The committed next command is `sdlc/scripts/live --max-tokens 519000 probes/find-0040/run comparison`. The comparison has not run. Its reservation becomes a durable ledger charge when it runs and is never raised merely for convenience.

Report hits by size, `none` hits and false refusals, request counts, input tokens, largest accepted size, and whether a floor on the winning probability would have caught a wrong pick without losing a correct one. Keep every arm, including failures. Retain no credential or response outside the normal recording. Do not buy an arm already answered by an identical recording.

## Phase two: build the command

Begin only after both probe stages meet every gate above. Add one pure `find` module to `thinkthen-core`. It accepts validated question text, ordered evidence, the model, and the `none` policy; validates the set, assigns ids, builds the aggregate evidence and internal choice plan, and maps the decoded distribution to a selected index or none. It knows no Clap type, file, environment variable, writer, `Failure`, or exit code. It exposes only the typed construction and mapping needed by a later engine.

The binary owns argument parsing, incremental aggregate-size enforcement, original bytes or records, backend and recording setup, output, diagnostics, and exit codes. Reuse `Reading` and pointers, `MAX_RECORD_BYTES`, `Backend`, the existing ask/cache path, adapter choice encoding and decoding, `Meta`, and write helpers. Do not use the record scheduler or rank sorter: `find` sends one aggregate request. Do not add a dependency or a second HTTP, recording, or JSON implementation.

Update `find.md`, `result.md`, `records.md`, `channels.md`, the specification index, help, both active plans, and the ratchet. Rewrite how-to 15 around its actual recordings and executable assertions. The plan still puts page 16 and transforms next.

## Acceptance

- Red-green core tests pin 2 and 255 units without `none`, 2 and 254 with it, and each one-unit and overflow refusal; ids and order; JSON escaping of hostile unit text; exact aggregate evidence; `none`; real-unit and `none` ties; the question digest; and reply-to-index mapping. Generated cases cover construction and mapping across both valid ranges.
- Compiled listener tests pin the exact request and one-request count for lines and JSONL with a pointer; chosen-unit preservation; ordered details; `none` output and exit 3; empty input; and no request for every preflight refusal. Exact 16 MiB original input succeeds; one byte more earns the fixed message, exit 2, and zero requests without constructing the aggregate request.
- Focused tests cover invalid JSON, invalid UTF-8, an individually oversized unit, missing pointers, input files and directories, dry-run with no key or connection, record/replay/cache with no second request, safe recording failures, a closed output pipe, backend failures, and every secrecy route including `Debug` text.
- CSV, TSV, and `--jobs` have exact safe diagnostics and send zero requests. Help leads with the fact that every unit leaves in one request, says that units see each other, and states both count ceilings and the aggregate byte ceiling.
- How-to 15 is green from committed recordings for one selected line and one `none` result. Its blocks prove one request, exact output, exit 3, replay without a key, and whole-set disclosure. Data fixtures are suitable for the later cross-language conformance suite.
- Existing request bytes, digests, judgments, recordings, record framings, scheduler behavior, and successful output remain unchanged. All committed recordings replay.
- The four repository rungs and `git diff --check` pass. The record carries both live commands, the derivation and durable charge of each reservation, ledger states, billed usage, gate results, red-green evidence, independent review, and exact size growth.

## Excluded and following order

Excluded: more than one selected unit, a threshold, two-pass search beyond 255 choices, CSV/TSV, question files, packing separate find jobs, embeddings, a public engine interface, and the one-crate move. Next are page 16 and the transforms, then the ADR 0017 rewrite and one-crate work.

## Complexity

- Contract: 2
- State and timing: 1
- Reach: 2
- Proof: 2
- Cost of error: 1
- Total: 8
- Minimum level floor: none
- Final level: 3
- Reasons: one new command reuses existing transport and recording machinery, but it widens the public result schema and depends on staged paid evidence at the option-count boundary.
- Selected model: `gpt-5.6-sol` with medium reasoning.

## Review

- Design review: accepted after two rejections. The first draft conditionally exceeded the existing 255-choice cap, lacked a product-quality gate on the paid results, did not bound aggregate input, and overreserved both live stages. The first rewrite fixed those four points. The second review removed 525 rank calls over blank documents because they do not answer the hit-rate comparison. The same reviewer accepted the narrowed rank arm, reservations, gates, core boundary, and proof plan.
- Phase-one implementation: complete locally. The fixed case table and constructor are hashed before use. The real compiled dry-run path produces the planned compact JSON state and 255 ordered choices at both upper boundaries. A fake binary proves eight feasibility calls, 18 find comparison calls, 1,050 rank requests, one cache, zero retries, safe early stop, the quality gate before rank, derived summaries, and secrecy. The public command and paid calls remain blocked.
- Code review: accepted after one rejection. The runner could publish a stale or partial summary when analysis failed; it trusted `choose`'s value; rank hid all but one row with `--top`; summaries did not separate logical, live, replayed, and billed work; failures had no retained safe row; and the fake accepted loose command shapes. The remediation writes summaries to scratch and publishes only successful analysis, derives the settled tie behavior from ordered probabilities, analyzes every rank row locally, reports traffic and hits by size, records safe stage/case/arm/exit failures, requires the exact two command shapes, and proves six feasibility replays plus both tie classes. The same reviewer accepted the remediated probe and its full local test run before any paid call.
- Feasibility evidence: eight recordings passed schema, endpoint, request-answer key, model, usage, digest-link, permission, and credential-marker checks. They report model `jev-1.13.0`, 70,265 input tokens, and 15,774 output tokens. A keyless local replay reproduced all eight answers, reported eight replays and zero live requests, and opened no paid path. The saved summary and rows contain only fixed case ids, counts, policies, selected ids, probabilities, usage, and replay facts. The comparison and public command remain blocked.
