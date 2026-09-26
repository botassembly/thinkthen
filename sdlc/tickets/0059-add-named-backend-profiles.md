---
flow: build
priority: 40
opens: crates/thinkthen conformance specification profiles sdlc/planning sdlc/ratchet.json
---

# 0059: Add named backend profiles and local size preflight

Status: landed

## Outcome

An explicitly selected JSON profile gives a backend a stable public name and enforceable byte and question limits. Every request is checked locally before it leaves. A saved question can name the profile its threshold was calibrated under; running it under another profile prints a warning and puts the same fact in detailed metadata.

## Current facts and decisions

The command currently knows only an address and model. It can send a request far beyond a backend's limit, pay for the refusal, and report only status 400. The second compatible backend accepted the same wire format but allowed only 512 tokens and 16 questions. Its probabilities also showed that one backend's tuned threshold does not carry safely to another. Ian approved named JSON profiles, one threshold per question, and a warning visible to both people and programs.

ADR 0010 removed the old configuration profiles because they repeated address, model, adapter, and key settings. ADR 0032 will supersede that narrow decision. This profile describes enforceable backend behavior and calibration identity. It does not select an adapter, address, model, key variable, cache, retry, timeout, or width.

`--profile FILE` reads one UTF-8 JSON object with this closed shape:

```json
{"schema":"thinkthen.backend-profile/1","name":"jev","max_evidence_bytes":120000,"max_request_bytes":250000,"max_questions":64}
```

Correction, 2026-09-25: no record gives a source for this example's byte numbers, and a 161,252-byte relate request was refused. Ticket 0123 measured the hosted cap and set a built-in relation ceiling of 96,000 request bytes. See `sdlc/issues/2026-09-25-recognize-and-relate-scale-and-shape.md`, item 1.

The name uses lowercase letters, digits, hyphens, and underscores and is nonempty. The three positive integer limits are optional independently; at least one is required. Bytes mean UTF-8 evidence bytes and exact encoded request bytes. The tool does not estimate vendor tokens. A backend whose only known limit is in tokens needs a tokenizer or a verified byte ceiling before a profile can enforce that limit. The repository ships documented example profiles only when evidence supports their byte values; it invents no Jev ceiling from token measurements.

Single question files and question sets may hold one top-level `profile: NAME`. Every threshold in a set shares that calibration identity. A named question nested under `questions` cannot carry its own profile, and the parser refuses one. The top-level name enters the question or question-set canonical digest and changes no request byte. A mismatch exists only when both the saved question and the run name profiles and the names differ. A missing name on either side produces no warning. Typed questions carry no calibration name. Command-line threshold overrides do not erase the saved calibration identity.

The fixed warning is `thinkthen: warning: threshold calibrated for profile NAME is running under profile NAME`. Profile names use the restricted grammar, so the sentence needs no raw-value escaping. Detailed results add `meta.profile_warning` only on mismatch, with `calibrated` and `running` string fields. Bare output keeps its current shape and carries the warning on standard error. The warning occurs once per run at the ordered boundary for the first successful logical result, before `filter` decides whether to print that record. A successful filter run warns even when it prints no records. A refusal on the first logical record prints no warning, even if a later parallel worker completed successfully. A later refusal follows the warning from an earlier successful result and never prints another. A successful dry run warns immediately before its plan.

The three limits count exact production values. Evidence bytes are `plan.evidence().as_str().as_bytes().len()` after field extraction and JSON normalization. Request bytes are the length of the exact adapter-encoded body. Questions are the adapter-expanded wire questions, including one per `tag` label.

An explicitly selected profile applies to dry runs, recording replays, cache hits, and live calls. The engine constructs and encodes a complete logical request, checks it, and only then may look in a recording, read a key, or touch the network. `annotate` constructs and checks every group for one record before any group starts. Its dry run checks every group before printing the existing first-group plan.

Limit refusals use these fixed forms at exit 2:

- `thinkthen: profile NAME allows at most LIMIT evidence bytes; this request has ACTUAL`
- `thinkthen: profile NAME allows at most LIMIT request bytes; this request has ACTUAL`
- `thinkthen: profile NAME allows at most LIMIT questions; this request has ACTUAL`

They name counts and never repeat evidence or encoded request bytes.

Ian can overturn the file shape, name grammar, byte units, mismatch rule, and metadata shape.

## Scope

Write ADR 0032, parse the explicit profile at the command edge, carry its limits and name into the private engine, and preflight every encoded logical request. Add `profile` to both question grammars and canonical digests. Render the mismatch warning and metadata. Document byte units, the absence of token estimation, and how a custom URL or model is still selected through the existing settings.

Excluded: a default profile, profile discovery, XDG configuration, tokenizers, token estimates, windowing or splitting, address or model fields in a profile, another adapter, raw backend bodies, changed request bytes, cache behavior, record-return changes, and paid calls.

## Acceptance

- The profile parser accepts the exact closed schema and rejects bad JSON, unknown or missing required fields, unsafe names, zero or non-integer limits, and a profile with no limit. A failure names the profile file at exit 5 without repeating its bytes.
- Evidence at the exact production byte count passes; one byte over exits 2 with the fixed evidence line. Exact encoded request bytes and expanded wire-question count behave the same at and one unit past their limits, with their fixed lines. `tag` counts one wire question per label.
- Preflight covers ordinary single calls, every record framing, grouped `annotate`, and `find`, before replay, cache, key, or network access. A counted listener proves over-limit live cases send zero requests. Replay and cache fixtures prove over-limit requests do not read a stored answer. A streamed run may have completed earlier records before a later record exceeds a limit, and its stopped line remains exact.
- `annotate` checks every group for a record before starting any group. Its dry run checks all groups before printing the first plan. Every other dry run performs the same preflight, needs no key, sends nothing, and prints no plan when over limit.
- A saved top-level question profile equal to the selected run profile produces no warning. A mismatch prints the fixed warning once at the ordered boundary for the first successful logical result, before output selection, and adds `meta.profile_warning` to every affected detailed result. If either name is absent, there is no warning and no metadata field. A filter run that rejects every record still warns once. A refusal on the first logical record prints no warning even when a later worker completed; a later streamed refusal follows the warning already printed for an earlier successful result.
- An unprofiled saved question retains its current question digest. Adding calibration changes only the question or question-set digest and mismatch metadata. Selecting a run profile never changes request bytes or the recording digest. Existing recordings replay byte for byte. One text in still prints one bare answer.
- Shared conformance cases use the production profile parser and adapter encoder. They cover each limit at and one unit over, expanded `tag`, grouped `annotate`, equal profiles, differing profiles, and either profile side absent. Compiled CLI tests pin help, warning order, stopped-run wording, every framing, secrecy, and counted-listener behavior. The four repository gates and `git diff --check` pass without a key or outside network access.

## Dependencies

Tickets 0055 and 0058, ADRs 0010 and 0017, the A2 build queue entry, `sdlc/planning/go-ahead-for-the-build-team-2026-09-21.md`, and the issues about the second backend, refused oversized requests, and pre-binding shape changes.

## Complexity

- Contract: 2
- State and timing: 2
- Reach: 1
- Proof: 2
- Cost of error: 1
- Total: 8
- Minimum level floor: none
- Final level: 3
- Reasons: the profile and warning are new public contracts, preflight must cover every request path without changing bytes or partial-send ordering, and incorrect limits can either bill a request that should stay local or refuse valid work.
- Selected model: `gpt-5.6-sol` with medium reasoning after keeping profile discovery, tokenization, configuration, and backend selection outside this ticket.

Re-score if the work adds automatic discovery, token counting, another adapter, or a profile-selected address or model.

## Review

Independent design review rejected the first draft because question-set calibration could be read as one profile per nested question, the three counts and refusal lines were not exact, replay and cache ordering was unstated, warning order was incomplete, digest compatibility and conformance cases were too broad, and the scores exceeded the rubric's range. The repair fixes one top-level calibration identity, exact production counts and messages, preflight before stored or live answers, compatibility invariants, named shared cases, and the level-3 score. Re-review found that a streamed run can warn before a later record fails preflight. The second repair defines the warning at the first successful logical preflight and distinguishes an early refusal from a later streamed refusal. The same reviewer accepted the final design and its Sol Medium route.

Implementation uses one production profile parser and the adapter's existing encoder. `PreparedRequest` checks exact evidence bytes, encoded bytes, and expanded question count before replay, cache, key lookup, or network access. `annotate` prepares every request for a record before starting its first group. Shared cases cross the production question grammar, parser, and encoder. Compiled tests cover exact boundaries, all four record framings, expanded tags, grouped live and dry runs, replay, cache, four-worker warning emission, request identity, safe failures, and zero-request refusals.

The pre-change compiled contract treated `--profile` as removed. The new boundary tests became green only after the explicit file option, parser, and preflight path landed. Focused unit, conformance, and compiled boundary tests pass without a key or network access. The implementation adds no default, discovery, tokenizer, backend selection, adapter, or paid call.

The Rust ceiling rises from 26,545 to 28,112 nonblank lines. The shared parser and conformance cases, compiled boundary tests, explicit CLI wiring, canonical digest coverage, ordered warning proof, and request preparation account for the growth. The final 78 lines pin both filtered-result warning cases at the compiled boundary and place warning timing tests in their own module. Duplication was checked in the existing question parser, adapter encoder, prepared request, recording, scheduler, output boundary, secrecy sweep, and result metadata paths. The implementation reuses each of them. The files that crossed the 500-line boundary were split by their existing responsibilities rather than exempted.

All four repository rungs pass with the key and base-address variables unset. The test rung passes 184 library tests, 231 backend tests, every compiled edge suite, two documentation tests, transform checks, and local live-script cases. The specification rung passes 27 page checks, 7 transform checks, every committed replay, and all 19 green how-tos. `git diff --check` passes. No outside network or paid call ran.

Independent code review rejected the first implementation because worker threads printed the mismatch warning. A later parallel success could therefore warn even when logical record one failed preflight and the run printed no result. The repair carries the notice with a completed row and prints it only from the scheduler's ordered output boundary. Ranked output waits until it emits a held row. Annotate uses the same boundary, and dry runs propagate warning write failures. One deterministic test proves a later ordinary row can complete behind a refused first row without output or warning. Another proves annotate refuses its first row before later work starts and prints no warning. The repair also brings the annotate and roadmap pages level and extends compiled and Debug secrecy proof across all three profile failures and all eight command families.

Re-review found that an all-rejected `filter` run could hide the mismatch because its successful judgments had no printable rows. The final repair emits at the first ordered successful judgment before output selection. Compiled tests prove an all-rejected run warns once with empty standard output, a rejected first row followed by a kept row warns once, and a first-record preflight refusal still suppresses a later worker's warning and output. The same reviewer accepted the final implementation.

Code re-review found that the first repair tied a streaming warning to printed output. That suppressed the calibration warning when `filter` successfully judged every record but rejected them all. The final repair emits at the first successful logical-result callback before output selection. Compiled cases cover an all-rejected filter run and a rejected first record followed by a printed record. The parallel first-record refusal cases remain green.
