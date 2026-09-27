# Command wording and help fixes before 0.1

Status: closed on 2026-09-27. Ticket 0152 Part A fixed item 1; reviewed Part B at 4a90c9e6 fixed item 16. Ticket 0138 fixed items 10 to 15 and 17 and checked items 2 to 9 against main `3b6954d6` on 2026-09-26.

Ian's ruling, 2026-09-25, on item 6: `check` shows the model, the provider, the URL, and all its outputs. When no model name is given, it prints "unspecified". It shows the model the user asked for and the model each reply names. Option 2, an optional model in every request, is not ruled.

This issue merges nine issues. They were `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, `2026-09-25-command-wording-and-help-fixes-before-0-1.md`, and `2026-09-25-command-wording-and-help-fixes-before-0-1.md`. Each one fixes a sentence a user reads: a refusal, a help line, or a printed report. None changes what the engine answers. They share one goal. The words must match the behavior before the 0.1.0 release. All evidence below was checked against main at `a95474be` on 2026-09-25.

## 1. A huge deadline prints hundreds of digits

Fixed by ticket 0152 Part A, from branch `ticket/0152-two-wording-fixes`. A deadline past 20 characters now prints in exponent form, so `1e300` refuses in 92 characters. Left open by ticket 0138 before that.

What the user sees. `CallOptions::deadline_seconds` refuses a budget past 4,294,967,295 seconds. Its sentence formats the raw `f64` with `{value}` (`crates/thinkthen/src/public/options.rs:129`). Rust's `Display` for `f64` never uses an exponent. A deadline of `1e300` prints a 301-digit number. The milliseconds sentence at `options.rs:156` has the same shape. Every surface that passes a user's deadline through prints the same sentence. The Python test pins the long form: `libraries/python/tests/test_inputs.py:84` expects `f"UsageError a deadline of {10**300} seconds {budget}"`.

What the ruling says. ADR 0041 sets the cap and the refusal. It does not ask for the full digits.

The fix. Print the value in a short form when it passes the cap, such as `{value:e}` (`1e300`), or name only the cap. Keep NaN and infinity as they read now. Update the Python test and any engine test that pins the long form.

Done when: a `1e300` deadline refuses with a sentence under 120 characters, and the Python and engine tests pin that sentence.

## 2. The help lists admin commands before the ten functions

Fixed by ticket 0126 in commit `36929f63`, landed 2026-09-25 at `5f9eea8f` from branch `ticket/0126-wording-help-and-doc-claims`.

## 3. `relate --either` with a question file names the wrong cause

Fixed by ticket 0123, landed 2026-09-25 at `6d33e99f` from branch `ticket/0123-relate-fits-the-backend`.

## 4. relate help lists `--jobs`, and relate refuses it

Fixed by ticket 0123, landed 2026-09-25 at `6d33e99f` from branch `ticket/0123-relate-fits-the-backend`.

## 5. The engine's WidthActive message still says width

Fixed by ticket 0126 in commit `36929f63`, landed 2026-09-25 at `5f9eea8f` from branch `ticket/0126-wording-help-and-doc-claims`.

## 6. check prints the model it sent, not the model that answered

Fixed by ticket 0126 in commit `36929f63`, landed 2026-09-25 at `5f9eea8f` from branch `ticket/0126-wording-help-and-doc-claims`.

## 7. recognize `--dry-run` help promises the request and shows no size

Fixed by ticket 0126 in commit `36929f63`, landed 2026-09-25 at `5f9eea8f` from branch `ticket/0126-wording-help-and-doc-claims`.

## 8. recognize help carries no cost sentence

Ticket 0123 fixed the relate half. Fixed by ticket 0126 in commit `36929f63`, landed 2026-09-25 at `5f9eea8f` from branch `ticket/0126-wording-help-and-doc-claims`.

## 9. `--jobs N` opens one connection per request in flight

Fixed by ticket 0126 in commit `36929f63`, landed 2026-09-25 at `5f9eea8f` from branch `ticket/0126-wording-help-and-doc-claims`.

## Items 10 to 17: found by experiment 218, wave 2

Each was checked at main `20e9b8d4` on 2026-09-25 with a fake key against a loopback backend. Seat: a person typing, unless the item says otherwise. Each harms a user by sending them the wrong way. None costs money on its own.

## 10. diff help never says what it leaves unchecked

Fixed by ticket 0138, from branch `ticket/0138-wording-and-failures`.

What the user sees. `thinkthen diff --help` never says that diff pairs answers by record id and answer name only. It never mentions the digest warning. The warning fires only when both lines carry `meta.question_sha256`. Two bare runs of two different questions pair and print changes with no warning:

    thinkthen decide 'Is it late?' --jsonl --field /t --url URL < recs.jsonl > a.jsonl
    thinkthen decide 'Is it lost?' --jsonl --field /t --url URL < recs.jsonl > b.jsonl
    thinkthen diff a.jsonl b.jsonl        # exit 0, standard error empty

What the spec says. `specification/diff.md` states both rules under "Warnings". The help is what a stranger reads.

The fix. Add two sentences to the diff help: diff pairs by record id and answer name, and it compares question digests only when both runs saved `--details`.

Done when: the diff help names both rules, and a help test pins them.

## 11. Status 502, 503, 504, and 529 after the retries print no next step

Fixed by ticket 0138, from branch `ticket/0138-wording-and-failures`.

What the user sees. After the allowed attempts, status 500 prints `thinkthen: the backend answered with status 500: the backend failed after the allowed attempts; try again later or change --max-retries`. Status 503 prints `thinkthen: the backend answered with status 503` and nothing more. `check` prints the same bare line on each probe row. The loopback backend's `/arm/503/v1` shows both.

What the spec says. `specification/backends.md` lists a phrase for 400, 401, 402, 403, 404, 422, 429, and 500. The other retried statuses have none, and `crates/thinkthen/src/cli/failure/status.rs` matches the page.

The fix. Give every retried status the 500 phrase, or one shared phrase: the backend kept failing after the allowed attempts; try again later or change `--max-retries`.

Done when: 502, 503, 504, and 529 each print a next step, and the status table test pins them.

## 12. `annotate @set.json` reads as a missing file

Fixed by ticket 0138, from branch `ticket/0138-wording-and-failures`.

What the user sees. `decide`, `choose`, `score`, `tag`, and `relate` take a question file as `@FILE`. The SQL surfaces take `'@set.json'` for annotate. The command's `annotate` takes a plain path. `thinkthen annotate @set.json` exits 5 with `thinkthen: the question set could not be opened: No such file or directory (os error 2)`, while `set.json` sits in the folder.

The fix. Accept `@FILE` in annotate as a synonym, or refuse a leading `@` with a sentence that says annotate takes the path without `@`.

Done when: `annotate @set.json` answers, or refuses with that sentence, and a command test pins the choice.

## 13. "a question file holds no key `version`" reads as a missing key

Fixed by ticket 0138, from branch `ticket/0138-wording-and-failures`.

What the user sees. A single-question file with `"version":1` exits 5 with `thinkthen: a question file holds no key `version``. The file holds that key. The sentence means the grammar has no such key. Question sets, `recognize` files, and `relate` files require `version`, so a user who copies one shape into another meets this line. Every surface prints the same engine sentence. `crates/thinkthen/src/core/question_file.rs:142` holds it.

The fix. Word it as a refusal of an unknown key: a question file takes no key `version`. Say where `version` belongs when the key is `version`.

Done when: the sentence names the key as not accepted, and the question-file tests pin it.

## 14. A choose file in `thinkthen_warm` names `decide_many`

Fixed by ticket 0138, from branch `ticket/0138-wording-and-failures`.

What the user sees. In DuckDB, `SELECT thinkthen_warm('@c.json', t)` with a choose file fails with `thinkthen usage: decide_many does not take a choose question`. The SQL user called `thinkthen_warm`. `decide_many` is a Rust and Python name. Ticket 0129 kept the sentence, and `databases/duckdb/tools/verbs_suite.py:156` pins it. Seat: a program that keeps the result.

The fix. Say that `thinkthen_warm` takes only a decide question. Keep zero sends.

Done when: the SQL warm refusal names `thinkthen_warm`, on every database surface.

## 15. A read-only default cache is called the recording folder

Fixed by ticket 0138, from branch `ticket/0138-wording-and-failures`.

What the user sees. With no `--cache`, `--record`, or `THINKTHEN_CACHE`, and a read-only `~/.cache`, a run exits 5 with `thinkthen: the recording folder could not be read or written; check its permissions and free space`. The user named no recording folder. Ticket 0124 split the backend mismatch sentence the same way, into the default cache and a named folder. This sentence at `crates/thinkthen/src/cli/failure/recording.rs:35` was left.

The fix. Use the default cache wording when the folder is the platform default, and name `--no-cache` and `THINKTHEN_CACHE` as the ways around it.

Done when: the default cache gets its own sentence, and a command test pins both.

## 16. "unresolved" still names the not-sure answer

Left open by ticket 0138. Its printed line and pages sit in files tickets 0134, 0135, and 0137 had open.

What the user sees. `thinkthen audit` prints `agreement ... N right, N wrong, N unresolved, N tied` (`crates/thinkthen/src/cli/audit.rs:178`). The threshold and band transforms emit `"unresolved"` as a key and as a value (`transforms/band/band.jq:31`). The public Rust docs say a pick is `None` "when unresolved". Demo pages 01, 13, 16, 25, and 41 use the word in prose. 55 files in pages, transforms, and source carry it.

What the vocabulary says. The product vocabulary names the answer "not sure" and lists "unresolved" among the words not to use. Its table on the words for numbers overrides older lines. The help text itself is clean: the vocabulary check over every `--help` found no banned word.

The fix. Say "not sure" in printed lines, docs, and page prose. The JSON keys are a contract, so rename them only with a ruling, or record that `unresolved` stays as a key name and say so in the vocabulary.

Done when: printed lines and page prose say "not sure", and the key name is either renamed or recorded as kept.

## 17. `recognize --kind PER` blames the kind count

Fixed by ticket 0138, from branch `ticket/0138-wording-and-failures`.

What the user sees. `thinkthen recognize --kind PER < text.txt` exits 2 with `thinkthen: recognize takes 1 to 20 distinct, nonblank kinds`. The user gave one kind, and it is not blank. `--kind` takes `KIND=DESCRIPTION`, and a bare kind goes as a positional argument. `--kind PER --kind ORG` prints the same line. `choose --option` and `tag --label` each name their missing `=` (`--option is LABEL=DESCRIPTION, and this one holds no `=``).

The fix. Give `--kind` the same sentence: `--kind is KIND=DESCRIPTION, and this one holds no `=`; give a bare kind without --kind`.

Done when: `--kind PER` refuses with that sentence at exit 2, and a refusal row pins it.

## Already fixed

- `thinkthen --version` prints the version. Ian's 2026-09-24 ruling asked for it. `crates/thinkthen/src/cli/args.rs:23-25` adds the flag, and `crates/thinkthen/src/cli/mod.rs:46-47` prints `CARGO_PKG_VERSION`. The crate and the Python library both carry `0.0.1`, which the ruling allows until launch. The command order part of that issue stays open as item 2.
- Connection reuse across a batch, the code answer that issue 9 proposed, already exists. The shared pool in `crates/thinkthen/src/engine/http.rs:54-59` reuses connections between requests. The missing sentence stays open as item 9.
