# 0017: Every question option has two homes

Branch `ticket/0017-two-homes`. Built 2026-09-19.

## What landed

`decide` takes `--true TEXT` and `--false TEXT`. `choose` takes `--option LABEL=DESCRIPTION`, which may repeat. All three verbs read a question file as `@FILE`. Every structural setting now has one home on the command line and one in a file, under the same word, and the command line wins over the file, which wins over the default.

A request that names no new option is byte for byte the request of yesterday, so every recording committed before this ticket still replays.

`specification/question-file.md` is new. It holds the grammar, one table with a row for every setting, the precedence, the `from` object, and the canonical form the question digest is taken over. Every `--details` row carries `meta.question_sha256`.

Two how-tos turned green: 40, say what yes and no mean, and 41, tune a question file and use the same file in the gate.

## The shape in the core

`question_file.rs` reads text and returns typed values. `question_file/resolve.rs` holds one pure function: a parsed file and the typed overrides go in, and the resolved question with the source of each setting comes out. `digest.rs` holds the canonical form and the SHA-256 over it, and it took the hexadecimal writer out of `recording.rs`, so one writer serves the recording key and the question digest.

The binary's `asked.rs` is the only place that opens a file. It strips the `@`, reads the bytes, and hands the text inward.

### The three constraints the coordinator added

1. **The parser takes text and never a path, and precedence is one pure function.** `QuestionFile::parse(&str)` takes a string slice. `resolve(verb, text, file, typed)` is callable with `file` as `None`, with `typed` as `Typed::default()`, and with the question text from either side, so a caller with no command line reaches the same question a shell user reaches. Every message names the key and its source, as `the question file's \`threshold\`: a single cut is above zero and at most one`, and no message in the core can name a path because the core never saw one. The unit tests in `question_file/resolve/tests.rs` drive the function with no command line at all.
2. **The canonical encoding is written out.** `specification/question-file.md` gives seven numbered rules: the key order per verb, absent keys, no insignificant white space, JSON escaping and nothing beyond it, the shortest round-tripping number, options as a map to a description or `null` with levels as a list, and the threshold riding with the question. Three digests are pinned in `digest.rs`, one per verb, and the page prints the same three. There was no earlier canonical encoder to reuse: the recording key digests the raw request bytes the adapter already wrote, which is a different document, so `digest.rs` writes the one encoder and `recording.rs` now borrows its `hex`.
3. **The adapter's name, address, and model stay where they are.** Nothing moved. `sdlc/scripts/policy.py` gained `check_seam`, which refuses `noul`, `criteria`, `systemone`, and the vendor's host name outside the adapter's module, the fixtures, and the tests. Six files hold an explicit allowance with the words each may keep, and the comment above `SEAM_ALLOWED` says a later ticket moves them. A listed file that stops needing its allowance fails the check, so the list cannot rot.

## Red then green

- The two new core files passed 500 non-blank lines. Each module's tests moved into a file of its own, which the seam check then had to learn to skip.
- `thiserror` treats a field named `source` as an error's source, so the field that names where a value came from is `origin`.
- The `--field` message reads `--field \`$.body\`: ...` and `spec/decide.md` pins it. A second helper writes the option name with no colon, so every command-line message is the sentence it always was.
- The pinned digests were written as placeholders, measured, and then cross-checked against an independent Python SHA-256 over the same compact JSON. The two agreed, which is the first evidence that the rule is followable in another language.

## The live checks

The cases are few and they are made up, and every trusted answer was committed before any call went out.

**Check 1, the two texts.** Forty labeled cases, asked twice. Both arms answered 40 of 40 correctly. The texts moved the probability toward the trusted answer on 22 cases, away on 2, and not at all on 16, with a mean absolute move of 0.0188 and a largest of 0.19. No answer crossed the half. **What it changed:** nothing in the design. The endpoint accepts `criteria.true` and `criteria.false`, and on easy cases the texts sharpen the probability without moving an answer. How-to 40 then had to find a question where the texts do move an answer, and it did.

**Check 2, a description under each option.** Sixty labeled picks over five labels, asked twice. The bare arm picked 58 of 60 and the described arm 59 of 60. Two rows changed: one wrong pick became right, and one exact tie became a wrong pick. None went from right to wrong. The mean winning probability rose from 0.9433 to 0.9880. **What it changed:** nothing in the design. The option is built as the ticket asked, and `choose.md` keeps its advice that the labels do the work.

**Check 3, the evidence shape.** Forty labeled cases whose evidence is built from two pointers, sent as the string the tool flattens it into and as a real JSON object. Both arms answered 40 of 40 correctly, no answer differed, the probability moved by 0.0045 on average and by 0.05 at most, and the object arm cost 13,954 input tokens against the string arm's 13,714. **What it changed:** the tool keeps sending the string. The ticket says to switch when the object form is as good or better, and on these forty cases both arms are at the ceiling, so the check separated nothing and the only measured difference is the cost. `specification/records.md` records the numbers. **Ian can overturn this.** A measurement on cases the model finds hard is the lever.

**Check 4, the token budget, was not run.** A request that tests a 32,000 token ceiling costs more than 32,000 input tokens, and after the other three checks and the two recordings the budget for this ticket held about 19,000. `probes/token-budget/` holds the job, written and ready: a state of about 20,000 tokens with three questions of about 4,500 each, which keeps the state plus the longest question under 32,000 while the whole request passes it. An accepted request settles it for the larger number and a 422 settles it for the smaller. **The lever is a raised token cap.** Nothing in this ticket depends on the answer, and the tool already reports the 422 with a fixed phrase.

## The spend

The ledger moved from 251,466 to 382,326 input tokens, a delta of 130,860. The eight jobs printed 127,295 between them, and the committed recordings and answer files hold 125,745. `sdlc/scripts/live` counts every `*.json` newer than its stamp anywhere outside `target/`, so the ledger's figure is the larger one and it is the one `sdlc/live-tokens` carries.

One run was wasted. The first pass of check 2 stopped at case P-55, where `choose` returned an exact tie and the job treated anything but 0 as a stop. That pass had already paid 19,490 tokens. Every probe job now records and replays one folder, so a case the folder holds is answered from it and only a new case is sent.

## Choices made where the pages were silent

Each of these is Ian's to overturn.

- **A command that names the wrong verb for the file is exit 2, not 5.** The file is a good question file. The line to fix is the one the user typed, so it is a usage error.
- **A verb with no options and no levels in either home names neither home.** The message is `\`choose\` takes 2 to 255 options`, at exit 2. Naming the file would blame it for a list the command line could equally have supplied.
- **A blank description is no description.** `--option late=` and a map entry of `"late": "   "` both ask the same question `late` alone asks, and they reach the same digest.
- **The digest is taken over a canonical form and not over the printed `question` field.** The printed field shows a pick's options as a bare list of names, which how-to 21 pins, and the digest has to separate two runs whose descriptions differ.
- **The threshold rides inside the digest.** A cut tuned on labeled cases belongs to the question it was tuned for.
- **The tool keeps sending the evidence as a string**, as check 3 above explains.
- **`probes/token-budget/` carries no leading number**, so the `spec` rung's replay check passes it by. It posts three questions in one request, which the tool never does, so there is no exchange to record.

**How-to 40 shares how-to 20's scenario on purpose.** ADR 0016 rule 6 lets no two pages share a scenario unless one is the sequel of the other. 40 is written so that 20 can be deleted with nothing lost, which is the coordinator's instruction, and ADR 0018 on main rules that 20 leaves once 40 is green. The rule is met the moment 20 goes, and 40 links nowhere that 20's departure breaks. Deleting 20 is not this ticket's work.

## One issue opened

`sdlc/issues/2026-09-19-compare-cannot-see-a-question-change-that-only-the-digest-shows.md`. `transforms/compare/compare.jq` reads a question change from `question.text`, which the two texts never reach, so it reported `changed.question` as false for two runs that asked two different questions. How-to 41 names this in its traps section, and the fix is to read `meta.question_sha256`.

## The review

A second agent with fresh context read the ticket, the diff against `origin/main`, and the "What reviewers keep finding" section of `AGENTS.md`. Its verdict was **not ready**, with four must-change findings and four suggestions. It checked the three pinned digests by computing them outside the crate, the seven canonical rules against `digest.rs`, both how-tos against their recordings, the purity of the core, and key handling in the four probe jobs. It did not run the rungs.

1. **`resolve(Verb::Score, ...)` accepted a rule.** `question_sha256` could then digest a `score` canonical form carrying a `threshold`, which rule 2 of `question-file.md` forbids, and `Sources::serialize` dropped it from `from` without a word. The command line was safe only because `judge.rs` refused first. **Fixed.** `threshold_of` now refuses it as `RuleOnScore`, the binary's duplicate `Failure::RuleOnScore` and its table row are deleted, and `asked.rs` forwards the typed rule so the core writes the one sentence. A test pins the file form and the typed form.
2. **The secrecy case was vacuous.** `run()` calls `.env_clear()`, so no key was ever set and the assertion could not fail. **Fixed.** `no_message_from_either_home_ever_carries_the_key_or_the_evidence` sets a real key and sends real evidence over all twenty grammar refusals plus the unopenable file, the wrong verb, both `--option` refusals, and a good file against a closed port. It reads both channels for the key, the evidence, and `bearer`.
3. **How-to 40 shares how-to 20's scenario.** The reviewer asked for Ian's ruling. **Resolved without him.** ADR 0018, already on main, cuts the how-to list to twenty and says 20 leaves once 40 is green, so the ruling exists. 40's closing link was repointed away from 20 so nothing breaks when it goes.
4. **Commit `c648f47` raised the ceiling without saying so.** True. The history is pushed, and a rewrite would cost more than it returns, so the rationale is here instead: that commit added the question-file grammar, the precedence function, and the digest. Every later ceiling change says what grew.
5. **`check_seam` stopped at the first `#[cfg(test)]`.** A `#[cfg(test)] use ...` near the top would have silenced the rest of a file. **Fixed.** It now ends a file's code only at a `#[cfg(test)]` immediately followed by `mod tests`, and it skips `*/tests.rs`.
6. **`Labels::described` was widened to `pub` with no caller outside the core.** **Fixed.** Narrowed back to `pub(crate)`.
7. **A clap sentence was checked with `contains`.** **Fixed.** The test pins the whole four-line sentence. The two cases it named as untested, a control character in a `score` level from a file and a `threshold` in a `score` file, are now both tested.
8. **`sent()` swallowed a listener miss into `None`,** so a pair that both reached no listener would have compared equal. **Fixed.** It asserts a request arrived.

Splitting out of the reviewer's work: the test binary passed the 500-line ceiling once findings 2 and 7 landed, so `tests/question_file.rs` became `tests/question_file/` with a shared harness and three pages, the way `tests/backend/` already is. The ceiling rose to the measured total in the same commit.

**Live check 4 was not run.** It was designed and its cases were sized, and the budget left after the first three checks did not cover it. Today's behavior stands unchanged, which is what the ticket says to do when a check does not settle.
