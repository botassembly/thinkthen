Status: open for item 4. Filed 2026-09-26 by the marketing lead from a fresh architect review. Items 1, 2 and 3 are done: ticket 0161 landed on 2026-09-26 (`sdlc/records/0161-build-annotate-reads-what-it-names.md`). Item 5 is done: ticket 0158 landed on 2026-09-26 (`sdlc/records/0158-build-a-cache-keeps-no-failed-question.md`). Item 4 stays open as `2026-09-26-a-mixed-model-cache-fails-every-annotate-record.md`.

# Architect review 03: `annotate`

A fresh reviewer tested `annotate` as an architect who adds typed answer fields to records. The review ran against main `9d652bed` and the release binary. It rated the topic fair and found 0 severity 1, 5 severity 2 and 7 severity 3 issues. The full detail sits in the architect review report 273, 03. In the commands below, `./run.sh` is the reviewer's wrapper around the release binary. Work file names refer to the report's local work folder, which stays unpushed.

Severity 1 means a wrong answer, data loss, a security problem or a hang. Severity 2 means a broken guarantee or a misleading document. Severity 3 means a sharp edge or a missing feature an integrator needs.

## 1. The spec's sample question set fails on the spec's own examples (severity 2)

Evidence. `specification/annotate.md` defines `triage.json` with `unresolved` reading `"on": "/body"`. The reviewer ran it against its three examples:

```text
$ ./run.sh annotate triage.json --dry-run < issue.txt                         # "thinkthen annotate triage.json < issue.txt"
thinkthen: the input is not valid JSON: the JSON at line 1 column 1 is not one
exit 2
$ ./run.sh annotate triage.json --jsonl --field /body --dry-run < issues.jsonl
thinkthen: the input is not valid JSON: the JSON at line 1 column 1 is not one
exit 2
$ ./run.sh annotate triage.json --lines --dry-run < issue.txt                  # the {"input":…,"value":…} example
thinkthen: the input is not valid JSON: the JSON at line 1 column 1 is not one
exit 2
```

Only `--jsonl` with no `--field` works. The `spec/` folder has no executable `annotate` page, so no gate catches this.

What an integrator hits. They copy the headline example and it fails. The error blames the input, not the question.

Direction. Make the examples match the set. Either give the text examples a set with no `on`, or change the input to JSON objects. Add an executable page that runs every example on the page.

## 2. `on` re-parses the selected text as JSON, so what gets selected depends on the data (severity 2)

Evidence. `crates/thinkthen/src/cli/annotate.rs:299-301` turns the selected evidence into text with `as_text()` and parses it again as a JSON document.

```text
$ printf '%s\n' '{"id":1,"body":"{\"body\":\"inner text\",\"secret\":\"s\"}"}' | ./run.sh annotate onbody.json --jsonl --field /body --dry-run
… "request":{"state":"inner text", …}                    exit 0
$ printf '%s\n' '{"id":1,"body":"plain text"}' | ./run.sh annotate onbody.json --jsonl --field /body --dry-run
thinkthen: the input is not valid JSON: the JSON at line 1 column 1 is not one   exit 2
$ printf '%s\n' '{"body":"line-json"}' | ./run.sh annotate onbody.json --lines --dry-run
… "request":{"state":"line-json", …}                     exit 0
```

`records.md` says "`--field` with `--lines` is a usage error. A text line has no members." `on` treats a line as having members whenever the line happens to be JSON.

What an integrator hits. The same set and flags pass some records and stop on others, depending on whether a string or line happens to hold JSON text. A body that happens to hold JSON sends only part of itself to the model. The error does not name the question or `on`.

Direction. Apply `on` to the selected JSON value, not to its re-serialized text. Refuse `on` on a text selection with a message that names the question and says text has no members.

## 3. `--dry-run` under a profile prints a request that will never be sent (severity 2)

Evidence. `cli/annotate/plan.rs:427-437` runs `facade::split` only as a check, then prints `plans.first()`, which is the unsplit group.

```text
$ ./run.sh annotate big.json --dry-run --profile prof.json < issue.txt | python3 -c '…len(d["request"]["questions"])'
questions in printed request: 1500        # the profile sets max_questions 100
```

Live, `mid.json` (250 questions) with `--profile prof100.json`: `--dry-run` shows one 250-question request. The live run sent 3 requests (`meta.requests` has 3 digests, `requests_sent` 3). `annotate.md` says dry-run "prints the first request". How-to 16 says "`--dry-run` shows the exact request".

What an integrator hits. A privacy or cost reviewer approves a request shape the tool will not send. The chunk count, and so the cost, is invisible.

Direction. Print the first real chunk, plus the chunk count per group.

## 4. After a model version change, editing one question fails every record under a cache (severity 2)

Evidence. `engine/facade/annotate.rs:130-131` and `:218-244` refuse a record whose chunks report different models. That covers a mix of cached and live chunks. The reviewer simulated it with unchanged groups from `fake-1` entries and the edited group from a `fake-2` entry:

```text
$ ./run.sh annotate triage-changed.json --jsonl --replay rec-mixmodel < issues.jsonl
thinkthen: the backend returned different model versions for one record; pin --model and rerun with --record or --cache
thinkthen: stopped at record 1; 0 records finished, 0 records from a recording
exit 4
```

What an integrator hits. The default model is the moving alias `jev-latest`. After any vendor release, the promised incremental edit ("a split group reuses only chunks whose exact bytes … remain unchanged") stops every record. The message blames "the backend" and suggests `--cache`, but the cache caused the stop. Pinning `--model` changes every digest and re-asks everything.

Direction. Treat a cached group from another model version as a cache miss for that record, or allow mixed versions and list each group's model in `--details`. Fix the message. The root cause, that nothing ties a cache entry to the answering version, is carried in the architect review 08 file.

## 5. A cached partial failure never retries (severity 2)

Five reviews found this. The architect review 08 file carries it. Review 03's evidence: a write-capable `--cache` folder holding a partial reply gave `{…,"kind":{"failed":{"kind":"backend","cause":"missing_probability"}},…}` and exit 6 with no request attempted.

## Severity 3 titles

- A name collision stops the run, even when no merge would happen (`cli/annotate.rs:237`; under `--details` the run still exits 2).
- A missing pointer stops the whole run.
- The row shape and field types vary within one stream, so `jq 'select(.kind == "bug")'` from the spec silently drops rows where `kind` failed.
- Failure isolation depends on how questions are grouped: the same malformed answer gives exit 6 in a shared group and exit 4 alone.
- No request ceiling protects `annotate` at the built-in address. A 2,600-question set (467,469 bytes) passed `--dry-run` and failed live with `400 (max_tokens_exceeded)`. Carried in the architect review 06 file.
- The libraries refuse any set that uses `on`. Already filed in `2026-09-25-public-library-api-gaps.md` item 5.
- Document mode sniffs JSON, so the output shape of a free-text job depends on the text's content.
