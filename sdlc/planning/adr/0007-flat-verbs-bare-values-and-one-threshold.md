# ADR 0007: Flat verbs, bare values, and one threshold option

- Status: Accepted
- Date: 2026-09-19

Ian ruled. He rewrote the command line in four design notes, read the review in `../flat-verbs-review.md`, and accepted every point in it. Changing a line below takes a new ADR.

## Context

ADR 0003 fixed the grammar `thinkthen decide VERB`. Ticket 0003 built `decide if` on it. Every answer was a full result object, a pass mark was optional and symmetric, and an exit code carried the answer only under `--status`. Ian found that surface heavy for the common case: ask a question, get an answer, branch on it. His notes proposed flat verbs, bare JSON values, and one threshold option. The review compared the notes with the twelve demos and the measurements, and it departed from the notes on fifteen points.

## Decision

1. **Verbs are flat.** `decide`, `choose`, `score`, `filter`, `rank`, `segment`, `annotate`, `report`, and `config` sit at the top level. The `decide` family is gone.
2. **The question comes first on every judging verb.** The grammar is verb, question, then what the verb needs. No verb has a hidden default question.
3. **Standard output holds a bare JSON value.** `--details` prints the full result object. One internal result model feeds both views, and the view never changes the request or the answer.
4. **`decide` is always a shell test.** Exit 0 is yes, 1 is no, and 3 is unresolved. `--status` is gone. `--quiet` suppresses standard output.
5. **One option sets the rule.** `--threshold T` is a single cut, and `--threshold LOW:HIGH` adds an unresolved band. The default is 0.5. Boundaries are inclusive.
6. **`rank` orders by the probability of yes.** It takes no rubric. `score` ships with its measured weakness stated in its help, and no other verb depends on it.
7. **`filter` and `choose` take a single cut only.** An exact tie in `choose` is unresolved.
8. **The saved-question verb is `annotate`.** An object record gains one top-level field per question. The saved file holds questions and nothing else.
9. **A configuration file holds profiles and run settings.** `--profile NAME` selects a profile. The backend flags of ADR 0004 stay as advanced options.
10. **Every file the tool reads or writes is JSON.**
11. **Seven verbs from the notes stay out of version one:** `match`, `reduce`, `patch`, `state`, `assign`, `cover`, and `select`. `specification/roadmap.md` lists each with what would bring it in.
12. **The local `chat-logprobs` adapter moves ahead of the record verbs in the plan.**

## The surface in full

### Commands

```text
thinkthen decide   QUESTION            [--threshold T|LOW:HIGH] [--quiet]
thinkthen choose   QUESTION OPTION...  [--threshold T] [--raw] [--quiet]
thinkthen score    QUESTION LEVEL...
thinkthen filter   QUESTION            [--threshold T]
thinkthen rank     QUESTION            [--top N]
thinkthen segment  QUESTION            [--threshold T] [--units lines|paragraphs] [--window N]
thinkthen annotate FILE
thinkthen report
thinkthen config   path | show | check
```

Options may sit before or after the operands, and `--` ends option parsing. An unknown option, a repeated single-value option, and an option that cannot act in the chosen mode are usage errors at exit 2. `choose` takes 2 to 255 options. `score` takes 2 to 10 levels, lowest first, and the score runs from 0 to the number of levels minus one.

Everyday options: `--threshold`, `--details`, `--quiet`, `--raw`, `--input FILE`, `--lines`, `--jsonl`, `--field POINTER`, `--top N`, `--dry-run`, `--profile NAME`.

Advanced options, shown only in the long help: `--url`, `--adapter`, `--model`, `--key-env`, `--timeout`, `--max-retries`, `--record DIR`, `--replay DIR`, `--config FILE`.

### The threshold

Let p be the probability of yes.

| Form | Accepted values | Yes | No | Unresolved |
| --- | --- | --- | --- | --- |
| none given | | p ≥ 0.5 | p < 0.5 | never |
| `--threshold T` | 0 < T ≤ 1 | p ≥ T | p < T | never |
| `--threshold LOW:HIGH` | 0 ≤ LOW < HIGH ≤ 1 | p ≥ HIGH | p ≤ LOW | LOW < p < HIGH |

A value is a decimal fraction. A percent, a reversed band, and a number that is not finite are usage errors before any request. Under a single cut, "no" means the answer did not reach the mark. It does not mean the model is sure of no. The help says so.

`choose` and `filter` accept the single form only. On `choose` the cut applies to the winning option's probability. With no threshold `choose` returns the winning label. An exact tie for first place is unresolved with or without a threshold. On `segment` a gap is a boundary when p reaches the cut. `score`, `rank`, `annotate`, `report`, and `config` take no `--threshold`. A question inside an `annotate` file carries its own.

### Output

| Command | Default standard output |
| --- | --- |
| `decide` | `true`, `false`, or `null` |
| `choose` | a JSON string, or `null`. `--raw` prints the label without quotation marks and prints nothing for `null` |
| `score` | a JSON number |
| `filter` | each kept record, byte for byte as it arrived, in input order |
| `rank` | each record as it arrived, most likely yes first. Ties keep input order. `--top N` limits what prints and saves no requests |
| `segment` | one JSON object per segment with `start_line`, `end_line`, `start_unit`, `end_unit`, and `text` |
| `annotate` | one JSON object per record |
| `report` | one JSON object |

`--details` prints this object in place of the bare value:

```json
{"schema":"thinkthen.result/1","value":true,"question":{"verb":"decide","text":"Does this ask for a refund?"},"answer":{"kind":"yes_no","probability":0.92},"threshold":0.5,"meta":{"profile":"jev","url":"https://api.typesafe.ai/v1/systemone","adapter":"systemone","model":"jev-1.13.0","usage":{"input_tokens":312,"output_tokens":48},"replayed":false}}
```

`threshold` is a number for a single cut, the string `"LOW:HIGH"` for a band, and `null` when none applies. A value copied from a result works on the command line and in a saved file. `meta.profile` is `null` for an ad-hoc backend. In record mode the object also carries `input`, the original record. `input` holds the whole record, including parts that were never sent. On `filter` and `rank`, `--details` prints these objects for the same records in the same order.

### Exit codes

| Code | Meaning |
| --- | --- |
| 0 | The command finished. On single-input `decide`, the answer is yes |
| 1 | Single-input `decide` only: the answer is no |
| 2 | Usage error. Nothing was sent |
| 3 | Single-input `decide` and `choose` only: the answer is unresolved |
| 4 | The backend failed or sent a reply the adapter refused |
| 5 | A local failure: a file, a recording, the configuration |
| 70 | A defect in the tool |

Codes 6, 7, and 8 stay reserved. In record mode the exit code reports the run, and no record's answer sets it. A valid answer on standard output can accompany exit 1 or 3. The help warns about `set -e` and `pipefail`.

### Records

The default input is one text document on standard input. `--input FILE` reads a file instead. `--lines` makes each line a text record. `--jsonl` makes each line a JSON record. `--field POINTER` is a JSON Pointer (RFC 6901) naming the part of each record the model sees. The rest of the record stays on the machine. `--field` without `--jsonl` reads the whole input as one JSON value. `--field` with `--lines` is a usage error. A pointer that finds nothing is an input error at exit 2 for that record, before any request for it.

`decide`, `choose`, and `score` accept `--lines` and `--jsonl` and then print one value per record in input order. `filter` and `rank` require one of the two. `annotate` reads one document by default and records under either flag. Each record is its own request, and records never share model context.

A run stops at the first failed record. Rows already printed stay printed. An empty record stream succeeds with no output and no request. An empty document is a usage error. A rerun with `--record DIR --replay DIR` on one folder answers the finished records from disk and pays only for the rest.

### The `annotate` file

```json
{
  "version": 1,
  "questions": {
    "unresolved": {"decide": "Does this report a failure that is still unresolved?", "threshold": "0.1:0.9", "on": "/body"},
    "kind": {"choose": "Which kind of request is this?", "options": {"bug": "Reports broken behavior.", "feature": "Asks for new behavior.", "other": "Neither fits."}, "threshold": 0.8},
    "impact": {"score": "How much disruption does this report?", "levels": ["None.", "Work continues with a workaround.", "Work is blocked."]}
  }
}
```

A question has exactly one of `decide`, `choose`, or `score`, and its value is the question text. `options` is a list of labels or a map from label to description. `levels` is a list, lowest first. `threshold` follows the command-line rule for its verb. `on` is a JSON Pointer inside the evidence that `--field` selected, and it can never reach outside that evidence. Questions with the same `on` share one request. A question name uses lowercase letters, digits, and underscores. An unknown key anywhere in the file is an error. `annotate --dry-run` checks the file and sends nothing.

An object record gains one top-level field per question. A name already present in the record is an input error for that record before any request. Any other record, a text document included, yields an object of the named answers alone. An unresolved answer is `null`. A backend failure is never `null`. `--details` prints `input`, `value` holding the named answers, `answers` holding the result for each name, and `meta`.

### `report`

`report` reads `--details` rows on standard input and calls no model. It prints counts of yes, no, and unresolved for each question, label counts for each `choose`, and a summary for each `score`. Given a pointer to a labeled truth in `input`, it also prints how each threshold would have scored against those labels. The exact options are Draft until a demo drives them.

### Configuration

The file is `$XDG_CONFIG_HOME/thinkthen/config.json`, or `~/.config/thinkthen/config.json` when that variable is unset or empty. `--config FILE` or `THINKTHEN_CONFIG` names another file. A missing default file is fine.

```json
{
  "version": 1,
  "profile": "local",
  "profiles": {
    "local": {"url": "http://127.0.0.1:8080/v1/chat/completions", "adapter": "chat-logprobs", "model": "some-local-model", "key_env": null}
  },
  "timeout_seconds": 30,
  "max_retries": 3,
  "jobs": 4
}
```

A profile is complete: `url`, `adapter`, and `model` are required, and `key_env` names the key's environment variable or is `null`. The file never holds a key. The built-in profile `jev` always exists, and a file profile named `jev` replaces it. Selection order is the `--profile` flag, `THINKTHEN_PROFILE`, the file's `profile`, then `jev`. The ad-hoc flags keep the rules of ADR 0004 and ADR 0006. The five `THINKTHEN_*` backend variables of ADR 0004 are gone. The file never holds a threshold, a context, an output path, or an output format. `config path` prints the path in use. `config show` prints the effective settings. `config check` validates the file. None of the three sends a request, and the tool never writes the file.

## What this replaces

- ADR 0003: the grammar `thinkthen decide VERB`. The rest of ADR 0003 stands.
- ADR 0004: the name `--backend`, now `--profile`, and the five environment variables. Profiles, pure adapters, and the key rules stand.
- ADR 0006: `--status`, the `unassessed` outcome, and the name `--plan`, now `--dry-run`. The plan document keeps its six always-present fields, with `backend` renamed `profile`.
- The symmetric `--min-prob P`. It equals `--threshold (1−P):P`.
- The Markdown question file proposed in `demos/FINDINGS.md`.
- Stream options from the draft `records.md`: `--on`, `--id`, `--emit`, `--unknown`, `--on-error`, `--max-requests`, and `--jobs` as a flag.

## Consequences

The specification and the demos are rewritten to this surface before any code changes, as ADR 0005 requires. The executable pages in `spec/` and the fixtures keep describing the landed code until a ticket changes both together. One rework ticket reshapes the landed `decide if`. The adapter, the HTTP edge, the failure table, recording, and replay stay as they are. The wire format does not change, because the landed adapter already sends the question as written.
