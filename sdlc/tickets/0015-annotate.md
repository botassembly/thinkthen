---
flow: build
priority: 50
opens: crates specification spec demos sdlc/planning sdlc/ratchet.json
---

# 0015: `annotate`

Status: landed

## Outcome

`thinkthen annotate QUESTIONS` asks several named questions about one document or each record. Questions that see the same evidence share one request, and one complete JSON object returns per record in input order. How-tos 39 and 14 are green. The command behaves like the five existing commands at the shell, file, recording, failure, and concurrency boundaries.

## Current facts

The core already holds the three question types, thresholds, pointers, multi-question plans, request encoding, reply decoding, and question digests. The binary already holds record framing, one shared connection, retries, recordings, replay, cache, and a bounded ordered scheduler. `annotate` adds the question-set grammar, grouping, several requests per record, aggregate results, and the command surface. ADR 0027 records the four replacements to settled text that this work requires.

The first live probe packed forty clear yes-or-no questions with no changed answer across 120 cases and 20.8 times fewer billed input tokens than separate requests. The ticket's authorized 29-request job then recorded both how-tos and measured mixed and borderline sets. One packed decision, choice, and score kept the separate values while input fell from 915 tokens to 371. Among six deliberately borderline decisions, packing changed one `false` to unresolved and moved probabilities by at most 0.04. The packed form resolved three of three correctly; the separate form resolved four of four correctly. Neither probe found a lower question-count ceiling. No probe should be repeated.

## Contract

- A set has `version: 1`, optional top-level `threshold`, and a nonempty ordered `questions` object. A name matches `[a-z0-9_]+`. Each entry uses the existing `decide`, `choose`, or `score` file grammar, except `model` is refused. The top-level threshold supplies only a missing `decide` threshold. Unknown keys, duplicate keys, bad shapes, pointer-key clashes, and an empty set are exit 5 with the full key path.
- `--model` applies to the run. The set holds no backend, rules, output path, or framing. `--field` first selects the record evidence; each question's `on` pointers act inside it.
- Questions with the same normalized `on` pointer list share one request in file order. The queue orders groups by record, then by the first question in each group. Records never share a request. `--jobs` bounds all requests in flight, including the groups of one document, and at most `jobs` records may have been read without a complete row being printed.
- Object input keeps its fields and appends bare answers in question order. Other input returns the answer object alone. An existing field name fails that record at exit 2 before any request for it. An unresolved answer is `null`.
- `--details` holds `schema`, original `input`, `value` as the named bare answers, `answers` as the complete per-question results with request digests, and aggregate `meta`. Usage appears only when every reply reports it. `replayed` is true only when every reply was replayed. A record whose replies report different model versions fails. The diagnostic names both only when each is at most 64 bytes of ASCII letters, digits, `.`, `_`, `-`, or `/`, and each either equals the requested model or matches `jev-` followed by one to three decimal parts of one to four digits each. The requested model must pass the same 64-byte printable grammar before equality permits a reply name. Otherwise the diagnostic says that the backend returned different model versions without printing either value. It says to pin `--model` and rerun with `--record` or `--cache`.
- `questions_sha256` is the lowercase hexadecimal SHA-256 of the resolved set's canonical UTF-8 JSON bytes. The object keys are `version`, then `questions`. `version` is the number `1`. `questions` is a list in file order. Each member has keys `name`, `question`, then `on`. `question` is the existing canonical question object from `question-file.md`, including the effective threshold. `on` is always a list: absent `on` becomes `[""]`, one pointer becomes a one-member list, and a list retains its order. Strings, numbers, absent question keys, and whitespace follow the seven existing canonical-question rules. Path, model, address, formatting, and other runtime settings are absent.
- This canonical set is one worked and pinned example: `{"version":1,"questions":[{"name":"refund","question":{"verb":"decide","text":"Does this ask for a refund?","threshold":0.5},"on":[""]}]}`. Its digest is `4318689ccd64c08b788ea48c5f72b8dca279cf3d482173ed280f3fe243158b62`.
- `--dry-run` validates with no key, connection, or recording. With evidence it prints the first group's request plus the complete question-to-pointer map. With no evidence it succeeds and prints nothing.
- `annotate` accepts the established input, framing, field, details, backend, retry, jobs, record, replay, and cache options. It refuses command-level `--threshold`, `--quiet`, and `--raw` with an actionable sentence. A stray second path points to `--input`; swapped question and evidence files name the mistake.
- Standard output contains compact JSON rows alone. Diagnostics use standard error. Earlier complete rows remain after a later failure, and no partial row prints. Once any request failure is observed, no queued request starts. Requests already in flight finish and any complete response is recorded because it was paid for. The run waits for earlier work, prints complete rows in order, and reports the earliest failed `(record, group)` in queue order, independent of completion order. A closed pipe stops reading and scheduling quietly; requests already in flight may finish and be recorded.
- Short help starts with runnable stdin and `--input` examples. Advanced options stay in long help.

The current package gains an internal library target and a thin binary entrypoint. Parsing, grouping, digesting, and result assembly stay pure. Request execution, scheduling, record/replay, and cache stay independent of argument parsing and printing. This ticket does not merge the two crates or promise the later public Rust API.

## Pages and evidence

- How-to 39 screens one message for several hazards. How-to 14 grades assistant replies with three decisions, one choice, and one score. Both are green over committed recordings. Page 08 is deleted into 39. Page 07 stays until page 16 is green.
- `specification/annotate.md` carries the measured 20.8-times figure over 120 clear cases, the 371-versus-915 mixed-question measurement, and the borderline result.
- Explain that the cache key covers a whole group: changing one question re-asks the group for every record, and a near-cut answer can move when neighboring questions change. Recommend retaining probabilities, holding the group fixed, or using a distinct narrower `on` where the record permits it.
- The authorized job made 29 requests and wrote 29 recordings. Their usage totals 10,104 input tokens and 1,533 output tokens. Every recording passed the schema, endpoint, request-response key, model, usage, digest-link, and credential-marker checks.

## Acceptance

- Data-backed parser tests cover every grammar refusal and exact path. Property tests cover parse/canonical round trips and digest equivalence.
- Local-listener tests prove grouping, disclosure boundaries, request order and count, jobs 1/4/32 on one document and record streams, the read-ahead bound, output order, stop behavior, recording of already-billed completions, collisions before a request, mixed replay/live aggregation, model mismatch, dry-run, replay without a key, broken pipes, and secrecy on every route. One listener makes two groups fail in reverse completion order and pins the queue-first diagnostic. The secrecy sweep gives model fields control characters, oversized text, an oversized all-digit `jev-` value, the evidence marker, and the key marker; one hostile reply equals the requested model. None reaches diagnostics or `Debug` output.
- A cold CLI pass with an empty environment checks help, likely argument mistakes, stdout/stderr separation, exit codes, deterministic output, and recovery messages.
- Existing request bytes and pinned single-question digests stay unchanged. Every committed recording replays.
- How-tos 39 and 14 pass with the key unset and no network. The four repository rungs pass, an independent reviewer accepts the diff, main is clean and pushed, and GitHub is green.

## Excluded and following order

Excluded: `tag`, CSV/TSV, `find`, rules, templating, a public library API, request IDs, and special plain-line output. Do not add hooks for them.

After this ticket: `tag`; CSV and TSV input plus the CSV transform; the correction pass; duplicate-cache coalescing; `find`; page 16 and transforms; a separate one-crate merge; release preparation. Tickets are written when work begins.

## Complexity

- Contract: 2
- State and timing: 2
- Reach: 1
- Proof: 2
- Cost of error: 1
- Total: 8
- Minimum level floor: level 3, because one record fans out into concurrent paid requests and combines their answers into a new public result.
- Final level: 3
- Reasons: the public grammar, request grouping, disclosure boundary, ordered concurrency, aggregate metadata, recordings, and failure behavior form one command contract. Splitting them would leave an unusable or misleading partial command.
- Selected model: `gpt-5.6-sol` with medium reasoning.

## Review

- Design review: accepted after three passes by a separate Sol Medium agent. The review corrected the settled-decision record, canonical digest, queue and failure rules, hostile model diagnostics, and complexity score. It independently recomputed the pinned digest.
- Code review: accepted after two remediation passes. The first review replaced record workers with one global request queue, normalized root evidence before grouping, required checked usage totals, widened the concurrency and secrecy matrix, and corrected the paid launch. The second review made a closed output pipe outrank later in-flight failures while still recording paid completions, redacted duplicate paths from `Debug` while keeping exact paths in the user message, and required a real generated resolved-source round trip. The same reviewer accepted the final regressions and local gates before the paid run.
