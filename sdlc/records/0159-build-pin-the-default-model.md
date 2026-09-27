# 0159: Build the pinned default model

Status: built 2026-09-26, awaiting code review. Owner: Claude.

Branch `ticket/0159-pin-the-default-model`, built in lane 2. The ticket is `sdlc/tickets/0159-pin-the-default-model.md`, accepted at `5b4fdf87`. The build merged `origin/main` `19ca8302` first, after tickets 0150 and 0158 landed. Ian can overturn every decision the ticket lists. No live call ran. Every test, plant and rung ran with `THINKTHEN_API_KEY` unset, against recordings and loopback backends.

## Result

- `DEFAULT_MODEL` is `jev-1.13.0`. `status` prints `model jev-1.13.0` with `model_source built_in`, and `--help` shows `[default: jev-1.13.0]`. `--model jev-latest` and a file's `"model":"jev-latest"` still send the alias.
- `sdlc/scripts/rekey-model FROM TO FOLDER...` is new, with its fixture of two recorder-written entries and its `--self-test` in `lint`. `sdlc/scripts/README.md` lists it.
- The script re-keyed 251 entries, and every reply named `jev-1.13.0`. The table below counts them. `conformance/cases.json` takes 56 new request digests in 93 places and the demo entry's new path. `databases/postgresql/fixtures/runner-excuse.json`, `specification/fixtures/check/requests.jsonl`, `check.md`'s copy of those bodies, `demos/27`'s page and the conformance mutation take the new bodies and digests.
- Each `probes/0N/job.sh` names `--model jev-latest` on every command. The `spec` rung replayed all nine probes, every committed row matched, and no recording byte changed.
- `audit --write` writes a single file's `model` member beside a written bar, with the one model every results line names. It keeps the model and prints the ticket's two sentences when the lines name more than one model or a line names none. A question set gets no model.
- The two-version stop prints the ticket's two sentences, and the library prints its own.
- Pages: `settings.md`, `backends.md`, `question-file.md`, `relate.md`, `check.md`, `recognize.md`, `channels.md`, the seven "Backend options" rows, `recording.md`, `audit.md`, `result.md`, demo 41 and `CHANGELOG.md`.
- `sdlc/issues/2026-09-26-site-recordings-rekey-for-the-default-model-pin.md` hands the site's 247 entries to the marketing lead with the exact command.

| Re-keyed folder | Entries |
| --- | --- |
| `crates/thinkthen/tests/fixtures/recognize-225/C01` to `C40` | 40, one each |
| `demos/01-refund-gate/recording` | 2 |
| `demos/02-route-a-ticket/recording` | 6 |
| `demos/03-grep-for-meaning/recording` | 5 |
| `demos/06-top-search-hits/recording` | 12 |
| `demos/12-keep-going/recording` | 4 |
| `demos/14-grade-a-batch/recording` | 24 |
| `demos/16-triage-pipeline/recording` | 6 |
| `demos/17-rate-and-sort/recording` | 4 |
| `demos/19-no-or-could-not-ask/recording` | 3 |
| `demos/21-options-from-the-record/recording` | 3 |
| `demos/27-test-with-no-network/recording` | 1 |
| `demos/39-screen-a-message/recording` | 1 |
| `demos/40-what-yes-and-no-mean/recording` | 6 |
| `demos/41-tune-a-question-file/recording` | 48 |
| `demos/43-lint-a-change/recording` | 5 |
| `demos/44-recognize-names/recording` | 1 |
| `transforms/rows/recording` | 80 |

A check over all 251 renames found that each new file equals its old bytes with the request's model value swapped, and nothing else.

## Tests

- The literal pins the ticket lists moved to `jev-1.13.0`, and the unit tests under `src` read `DEFAULT_MODEL`.
- `tests/audit_model.rs`, `audit_writes_the_model_it_tuned_on`, runs six rows through the compiled binary over replayed `transforms/rows` results. Each row pins the file's bytes and standard error: one version, two versions, a typed alias, a line with no model, a failed and an unlabeled line, and no bar. It went red before the change, because the file gained no model.
- `tests/backend/annotate/cache_versions.rs`, `a_cache_that_mixes_versions_stops_the_record`, keeps one `Listener::serving` up across two runs over one `--cache` folder. It pins exit 4, the whole sentence, no row, and three requests. It went red before the change on the old sentence.
- `rekey-model --self-test` in `lint` re-keys the fixture and plants six refusals.
- `audit_write.rs` and `audit_sets.rs` now pin the model member their writes add. The two existing two-version tests pin the new sentences.

The four questions, for the two new tests:

- **What behavior does it protect?** A tuned file records the version its bar was tuned on, and a two-version stop names the cache as the likely cause.
- **What credible regression fails it?** Plants (c) to (g) and (l) below.
- **Why does no existing test catch it?** Nothing read audit's model, and the two-version tests used live chunks only.
- **Does it need a test-only hook?** No. The question file, the result lines, the loopback and the cache folder are the real boundaries.

## Plants

Each plant was applied in the working tree, run, and restored from a copy. `git status` was clean after each restore.

| Plant | Result |
| --- | --- |
| (a) `DEFAULT_MODEL` back to `jev-latest` | Red. 27 tests failed: `decide_edge`, `choose_and_score_edge`, `status`, the `check` tests, `streaming`, `recordings`, the `recognize-225` replay, `relate::ceiling`, and the three in-process conformance runners, which replay the shared cases' pinned bodies. `spec/decide.md` failed 4 blocks and `spec/check.md` 1. `backend.rs`, `resolve/tests.rs` and `plan_document.rs` stayed green by design |
| (b) Skip `demos/01-refund-gate/recording` in the re-key | Red. The page's first block fails, and the command exits 5 naming the missing entry |
| (c) Write the model when lines disagree | Red at the "two versions" row |
| (d) Keep a model the file already names | Red at the "an alias typed" row |
| (e) Write the model with no bar | Red at the "no bar" row |
| (f) Skip the version check for cached chunks | Red. The second run exits 0 and prints a row |
| (g) Restore the old sentence | Red. The sentence differs |
| (h) Re-serialize the entry as JSON | Red. The second entry differs from the recorder's bytes, the first changed more than its model value, and the existing-target plant moved |
| (i) Skip the old-name check | Red. The misnamed entry moved with exit 0 |
| (j) Write each entry before checking the next | Red. The misnamed-entry plant left a written file |
| (k) Follow a symlink | Red. The refusal sentence differs |
| (k) Overwrite a target | Red. `git mv` fails with a trace in place of the refusal |
| (l) Skip lines without `meta.model` | Red at the "a line with no model" row |
| (m) Skip the Git check | Red. The untracked entry fails in `git mv`, and the edited entry moves |

## Budgets

Nonblank lines, net against `origin/main`.

| Item | Budget | Measured |
| --- | --- | --- |
| `core/adapters/systemone.rs` | 1 changed line | 1 |
| `cli/failure.rs` and `public/error.rs` | at most 6 | 0 |
| `cli/audit` | at most 45 | 34 (35 in `write.rs`, −1 in `audit.rs`) |
| `core/plan_document.rs` | at most 4 | −3 |
| `tests/audit_model.rs` | at most 120 | 76 |
| The new annotate test | at most 70 | 65, and one `mod` line |
| `sdlc/scripts/rekey-model` | at most 160 | 160 |
| `lint` and `scripts/README.md` | at most 6 | 3 |
| Pages under `specification/`, `demos/`, `CHANGELOG.md` | at most 30 | 10 |
| `sdlc/ratchet.json` | at most 265 above main | 173, from 70,015 to 70,188 |

No dependency was added. The ratchet's growth is the two new tests (141), `write.rs` (35), the shared replay helper in `tests/support/measure.rs` (9), the `audit_sets.rs` expectation (4), the `find_edge.rs` model line (2), and the new `mod` line (1). Moving the replay helper out of `audit_write.rs` removed 13 lines there, the plan document and tag request tests lost 5, and `audit.rs` lost 1. The commit `d5d6bb6e` says the `audit_sets.rs` expectation shrank. It grew by 4 lines, as this table says.

## Ladder

| Rung | Result |
| --- | --- |
| `install` | exit 0 |
| `lint`, with `THINKTHEN_PRIVATE_NAMES` set | exit 0. Its log holds one bare "Killed" line, as expected |
| `test` | exit 0 |
| `spec` | exit 0, demos 21 green, all nine probes reproduced their rows |
| `surfaces` | exit 0: every landed surface passes, with its not-run cases reported, plus the release smoke |

The `test` rung failed twice before it passed. Plant (a) left `target/tmp/read-only-replay` read-only, so the next run could not reuse it. The builder removed that build folder. Demo 41 then broke ADR 0016's page limits, and a fix commit fit the page to 120 lines and 899 words.

## Deviations

- The build's `git grep` found six hits the ticket's lists missed or placed wrongly. The ticket now names them under its list: `cache_partial.rs:160` stays; the `check.rs` replies and `check.md`'s reply lines move, because the conformance arms echo the model sent; `request.rs:453` and `facade_tests.rs:35` move to the default constant; `find_edge.rs` replays `probes/find-0040` and names `--model jev-latest`; and `relate/ceiling.rs` takes a new dry-run digest.
- The question-set row is not repeated in `tests/audit_model.rs`. `audit_write.rs`, `inserts_when_absent`, already pins a set's whole bytes after a write, and it shows no model member. The new test's header points there.
- The cache test's replies name `fake-1` and `fake-2`, as the ticket says. Those names are not safe to print, so the test pins the sentence without names. The named sentence is pinned by `different_safe_model_versions_fail_one_record_and_name_both`.
- Plant (d) kept a model the file already names, the alias a user typed. The results carry no requested model, so that is the alias audit could write in place of `meta.model`.
- Demo 41 shows the written member inside step 1's block, because a separate step would pass ADR 0016's 120-line and 900-word limits.
- `recording.md` carries the re-key sentence as the last bullet of "An entry".

## Left for landing and later

- The lander closes ruling 8 and the pin half of ruling 2 in the 2026-09-22 vendor issue, the message half of the mixed-model cache issue, finding 2.1 of architect review 05, and the default half of item 1 of architect review 08, as the ticket's "Closes" says. It also answers question 1 of the 0.1 backlog.
- The site issue stays open for the marketing lead. A Quick Fix removes `rekey-model`, its fixture and its `lint` line when it closes.
- The ticket's deferred gaps stand.
