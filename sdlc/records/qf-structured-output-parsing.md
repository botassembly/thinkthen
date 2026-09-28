# Quick Fix: parse annotate streams and detailed answers

Status: candidate for fresh independent review. Branch: `ticket/qf-structured-output-parsing`, based on `origin/main` at `8cb674ae`. Scope: experiment 284 register 67 and 84. No runtime, schema, or site source changed.

## Findings on current main

Register 67 is an actual guidance gap. `crates/thinkthen/src/cli/annotate/aggregation.rs::finish` writes a bare object by appending answers to object records, but wraps non-object stream records in `RecordValue`. `crates/thinkthen/src/core/records.rs::AnnotatedRecord::serialize` preserves original object members before adding named answers. Existing `crates/thinkthen/tests/backend/record_values.rs::annotate_wraps_non_objects_in_record_mode_and_still_enriches_objects` pins line, JSONL scalar and array, CSV, and TSV shapes. `specification/records.md` already says every CSV or TSV cell is a string. Since an original object may have `input` or `value` members, bare output cannot be classified universally by testing those keys. The missing page guidance is an explicit `--details` carrier and a parser recipe.

Register 84 is also a guidance gap. `crates/thinkthen/src/core/result.rs::AnnotateResult` serializes `schema`, `input`, `value`, `answers`, and `meta`. Detailed success entries carry `value`, `question`, `answer`, `threshold`, and `request`; failures carry `question`, `failure`, and `request`. `crates/thinkthen/tests/backend/annotate/partial_failure.rs::bare_and_detailed_rows_distinguish_failed_from_not_sure` pins a successful `null` beside a failed entry. `specification/result.schema.json` defines the same detailed outer carrier and answer union. Dynamic question names stay nested in `value` and `answers`. Repeated question names are refused while reading the set, and a question name that collides with an original object member is refused for that record. The readable option names in `question` omit descriptions that `core/question_set/resolved.rs` includes in the canonical set digest. `specification/question-file.md` already states the canonical rule. The page needed to connect these facts to parsing and identity.

## Change and retained behavior

`specification/annotate.md` now gives one `jq` line for a `--jsonl --details` run and says to select the actual framing for line, CSV, or TSV input. It produces one row per completed input, preserves original `input`, and turns dynamic question keys into a `results` list. Each result has an explicit answered or failed status; a successful `null` remains distinct from failure. The page names CSV string cells, variable answer types, and the ambiguity of bare rows with user-owned `input` or `value` keys. `specification/result.md` explains the outer detailed carrier, nested dynamic names, answer union, and the separate meanings of `meta.questions_sha256` and per-answer `request`. It links to the recipe and existing canonical rules. `specification/records.md` already covers CSV types precisely and did not need a duplicate edit. All output contracts, examples, and pending record/batch contracts remain unchanged.

## Proof, limits, and closure recommendation

A temporary literal `jq` table compared the documented filter with independently written expected JSONL for an original object containing `input`, `value`, and `meta`; a `null` input with both a successful `null` and a failed question named `value`; and CSV-like string cells plus a list answer. `diff -u` passed. The table is disposable, not a persistent test suite. Existing source and tests cited above establish the producer shapes; no provider or runtime suite was run for a page-only change. The recipe applies to output produced by a command invoked with `--details`; it does not try to sniff an arbitrary bare JSON object or normalize a row a failed run never emitted.

Focused checks passed: `python3 sdlc/scripts/pages` reported 1 coming and 21 green; `python3 sdlc/scripts/tickets` reported 0 evidence failures; `git diff --check` was clean. The changed specification pages and this record are not executable demo pages with word limits. The full changed prose was read alongside the current schema, serializer, canonical digest code, and cited existing tests.

Recommend closing 67 and 84 after fresh review and landing because both are documentation defects and the claimed pages now give a supported parsing route. The register and plan remain for the coordinator to update. No site or reference-generation work was claimed here; the site was not audited or changed.

## What the build taught us

An `input`/`value` key test is unsafe on bare annotate rows because those names can belong to the original record. The producer's `--details` switch supplies the needed context before parsing, and `answers | to_entries` removes any need to guess question names. A successful `null` is only distinguishable from a failure by the detailed entry's `failure` member or an equivalent explicit status. Readable option names help humans, while the canonical question-set digest identifies the resolved descriptions and settings; the request digest identifies the exchange instead. CSV cell spelling survives as a string even when it resembles a JSON value.


## Accepted review and landing

Fresh independent review accepted `5d56d86b`. The reviewer checked the actual detailed annotate producer and current schema and independently ran a tiny literal jq table covering reserved-looking input keys, null input and answer, dynamic question names, failure markers, CSV string cells and a list answer. The recipe deliberately uses the fixed `--details` carrier instead of guessing ordinary object wrappers. Resolved question-set serialization includes descriptions, while readable label output uses names; per-answer request identity is the prepared request/recording digest.

The coordinator merged later main queue notes; reviewed product prose is unchanged. Pages, tickets and diff checks passed. Registers 67 and 84 close as page defects. No runtime, schema, site, full-suite or provider result is implied.
