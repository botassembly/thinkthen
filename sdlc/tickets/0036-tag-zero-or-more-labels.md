---
flow: build
priority: 50
opens: crates specification spec demos sdlc/planning sdlc/ratchet.json
---

# 0036: Tag zero or more labels

Status: landed

## Outcome

`thinkthen tag QUESTION LABEL...` returns every applicable label as one JSON array, and a `tag` entry in an `annotate` question set returns the same value. One record makes one request containing one yes-or-no question per label. How-to 39 demonstrates the command with described labels.

## Current facts

The core already validates ordered labels, represents yes-or-no questions and probabilities, groups several questions over one record, and preserves request and output order. `annotate` already owns the global request scheduler and grouped result assembly. The saved twenty-post probe sent twenty labels in one request. Descriptions in `criteria.true` improved exact-set accuracy from 10 of 20 to 16 of 20 and reduced false positives from ten to three. Adding or reordering labels changed three of four hundred answers near the cut. No packing, order, or description probe should be repeated.

Ian accepted `tag`, a single cut with default 0.5, JSON-array output alone, and CSV/TSV input as the next separate ticket. ADR 0029 records the public contract. ADR 0028 is amended around TSV before this ticket lands, and every mistaken generic-delimiter reference is corrected.

## Contract

- `tag QUESTION LABEL...` takes 1 to 20 labels. Repeated `--label LABEL=DESCRIPTION` is the described form. Mixing the two forms is exit 2. Labels are unique, nonblank, printable text in their given order.
- `@FILE` accepts `tag` and `labels`; `labels` is a list or an ordered map from label to description. A command-line label list replaces the file list whole. A question-set entry accepts the same `tag` shape. No per-record label pointer enters in this ticket.
- Each label becomes one yes-or-no question. Its instructions are `QUESTION`, two line feeds, then `Determine whether the label JSON_STRING applies to this item.` `JSON_STRING` is the label rendered as a JSON string, including its quotation marks, so quotes and backslashes cannot change the sentence boundary. Its optional description becomes `criteria.true`; no false criterion is invented. All label questions ride in one request in label order.
- One single threshold applies to every label. The default is 0.5. A band is exit 2. A label is selected when its yes probability clears the existing single-cut rule.
- The bare value is one compact JSON array in label order. `[]` is a successful answer and exits 0. `tag` has neither `--raw` nor `--quiet`. Record mode prints one array per input record in input order and exits 0 after a successful run.
- `--details` writes `question` as `{"verb":"tag","text":"Which topics?","labels":["billing","urgent"]}`, `answer` as `{"kind":"tag","probabilities":{"billing":0.91,"urgent":0.22}}`, and `value` as `["billing"]`, followed by the numeric threshold and established metadata. A tag answer inside `annotate` uses the same nested shape and the request digest shared by its group.
- The canonical tag question has keys `verb`, `text`, `labels`, and `threshold`, in that order. `labels` is an ordered map from each name to its description or `null`. Model and pointers are absent. The canonical example `{"verb":"tag","text":"Which topics?","labels":{"billing":null,"urgent":"The item needs prompt attention."},"threshold":0.5}` has digest `00b00cf7e1d55b2bb16356f583da7d2dab8fb538f459d859f817392b59efdedf`. The annotate set digest nests this form and carries pointers through its existing rule. Changing or reordering a label changes the request and cache key. The documentation warns that it re-asks the whole set and can move answers near the cut.
- `tag` accepts the established input, framing, field, details, backend, retry, jobs, record, replay, cache, and dry-run options. The request scheduler, failure order, closed-pipe behavior, secrecy rules, and recording rules stay shared rather than copied into a tag-specific path.
- Short help leads with the described-label form and shows the bare form as the quick form. The root README and reference tables describe the four single-judgment shapes as yes/no, one of many, any of many, and a position on a scale.

## Pages and evidence

How-to 39 is the executable `tag` page for screening one message for several hazards. It teaches descriptions first, shows `[]`, and retains the three measured probabilities with `--details`. Its one authorized response is saved in the recording folder; no other product probe was repeated.

## Acceptance

- Core tests pin 1 and 20 labels, label order, descriptions, threshold equality, empty selection, the complete detailed result, the canonical form and digest, and tag aggregation inside `annotate` beside decide, choose, and score.
- CLI tests cover command-line labels replacing file labels. They reject zero and twenty-one labels, duplicates, blank and control labels, mixed bare and described forms, bands, `--raw`, `--quiet`, malformed question files, and invalid question-set entries before any request.
- Local-listener and replay tests pin exact request bytes, including a label with quotes and backslashes, one request per record, stable output under jobs 1, 4, and 32, request and output order, cache behavior, failure order, closed pipes, and secrecy on success and failure. A dry run pins the complete expanded request and proves it needs no key or connection.
- Existing requests and digests remain byte-for-byte unchanged. Every committed recording still replays. How-to 39 passes with the key unset and no network.
- The four repository rungs and `git diff --check` pass locally. Commit, push, and the remote check belong to the coordinator's landing step.

## Excluded and following order

Excluded: CSV/TSV parsing, per-record labels, raw tag output, `find`, transforms, library APIs, and changes to the paid-call ledger. The next ticket adds CSV and TSV input with JSONL output alone.

## Complexity

- Contract: 2
- State and timing: 1
- Reach: 1
- Proof: 2
- Cost of error: 1
- Total: 7
- Minimum level floor: none
- Final level: 3
- Reasons: the new grammar and result type cross the pure core and CLI, while the existing grouped scheduler contains the concurrency risk. The total selects level 3 without a floor.
- Selected model: `gpt-5.6-sol` with medium reasoning.

## Review

- Design review: accepted after one remediation pass by a separate Sol Medium agent. The review corrected the canonical digest boundary, pinned the public result shape, made generated label text safe for quotes and backslashes, kept accepted ADR 0028 as an amendment, and corrected the complexity score. It independently recomputed the pinned digest.
- Code review: the first review rejected generic label diagnostics and missing Tag-specific boundary coverage. The remediation pins safe noun-specific messages and the compiled matrix for expansion, malformed replies, scheduling, failure order, closed pipes, and secrecy. The same independent reviewer accepted the remediated code before the live run and accepted the branch after it was rebased onto current main.
