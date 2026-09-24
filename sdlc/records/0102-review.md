ACCEPT

# Review of ThinkThen 0102: redact recognized names in Debug

Reviewer: fresh read-only Claude session. Branch `ticket/0102-redact-recognized-names-in-debug` at 85c2edf7, fix commit c2327121. Every claim below comes from a command run in this review.

## 1. Completeness

I scanned every `derive(Debug)` struct and enum in `crates/thinkthen/src` outside test files (173 types) and read the field types of each.

- The recognize path is complete. `Token`, `RecognizedName`, `TokenInput`, and `Record` now withhold their text. `Recognized`, `Detailed`, `StrengthInputs`, and `RelationEdge<RecognizedName>` reach only redacted fields. `TokenAnswer`, `DryRun`, `From`, and `Aggregate` hold numbers, static strings, or digests. `Running` has no `Debug`. `Held` is private to `records.rs` and is reached only through `Record`.
- Record-bearing types elsewhere are now safe too. `Sending`, `AnnotatedRecord`, `AnnotateResult`, `RecordValue`, `DecisionResult`, and `FindResult` hold a `Record`, and each one now prints `Record(<withheld>)`.
- No production code formats any of these types. The `{:?}` hits are all inside tests. Engine error strings (`ReplayMiss`, `Entry`) carry cache file names.

Gaps outside this ticket's outcome. Neither one blocks this ticket:

- `core/adapters/systemone/request.rs:24` `Request { state: Json, .. }` derives `Debug`, and `state` is the evidence (`plan.evidence().as_json()`). `encode_raw` builds it and serializes it right away. Only the module's own tests use it, so no log or error reaches it today. No secrecy test covers it. File it as an issue: either redact `state` in `Debug` or drop the derive.
- `choose --options` builds `Question::Choose` labels from the record (`cli/asking.rs` `Asks::of`). `Question` and `Labels` print those labels under `{:?}`, and `Sending.question` holds one. Question text prints on purpose. Options taken from the record are evidence, though. This is the same class of gap. It is also unreachable today.

## 2. Proof

In a scratch copy I restored `derive(Debug)` and removed the manual impl for each type, one at a time. Then I ran `cargo test --lib no_recognize_debug_line`.

- Baseline: passed.
- `Token`, `RecognizedName`, `TokenInput`, `Record`: each failed at `cli/failure/tests.rs:130`, the absence check.

The scratch copy is deleted.

## 3. Usefulness

I printed the rendered output. `RecognizedName` keeps kind, start, end, and strength. `TokenInput` keeps both probabilities. `Token` keeps its character start and end, and it shows the byte length of its text. `RelationEdge` keeps relation and probability.

Nits:
- `Token` hides `byte_start` and `byte_end` behind `..`. Those are positions and not secret. `core/recognize.rs:255` slices by them, so showing them would help debugging.
- `Record(<withheld>)` gives neither the byte length nor text versus JSON. The other redactions show the length.

## 4. Size

`lint` printed `ratchet: crates 44424/44424`. The ceiling equals the measured total. 89 lines for four impls and one test is proportionate.

The commit message says a helper would save no lines. That holds for this commit alone. For the repo it is slightly off. Each `.field("x", &format_args!("<{} bytes withheld>", len))` takes 4 lines after rustfmt. A `Withheld(usize)` newtype with a `Debug` impl would cost about 7 lines. It would turn each site into one line, and six sites exist (`http.rs:168`, `recording.rs:83`, `cli/schedule.rs:38`, and the three new ones). That saves about 11 lines net. This is optional follow-up and does not block.

## 5. Package-rung issue

The issue is real and fairly stated. I reproduced it in the scratch copy:

- I ran `cargo package` over an existing 454820-byte crate that would now be shorter. The file stayed 454820 bytes, and `gzip -t` reported "trailing garbage ignored".
- `tar -tzf` exited 2.
- After deleting the crate, the rerun wrote 454729 bytes and `gzip -t` passed. This is cargo 1.93.1.

The proposed fix is sound: remove the crate before `cargo package` and check the rung twice. The issue could also say this is cargo's own behavior. That belongs in the repo's dependency notes, not in an upstream report.

## 6. Merge

`origin/main` moved during this review. 0091 has landed (18c0dc10, ceiling 44778). `git merge-tree` shows exactly one conflict, in `sdlc/ratchet.json`. The merged tree measures 44867 non-blank lines (44778 + 89). Resolve the conflict to 44867. The merged tree is newer than the reviewed commit, so it needs its own lint run before landing.

## 7. Ladder at 85c2edf7

I ran the ladder in the worktree with `THINKTHEN_API_KEY`, `THINKTHEN_BASE_URL`, and `THINKTHEN_URL` unset. The one-minute load stayed between 4.1 and 6.9.

- `install` exit 0.
- `lint` exit 0 (`ratchet: crates 44424/44424`).
- `test` exit 0: 737 passed, 0 failed.
- `spec` exit 0: `demos: 21 green, 0 red`.
- `sdlc/scripts/live` never ran.

The worktree is clean at 85c2edf7.
