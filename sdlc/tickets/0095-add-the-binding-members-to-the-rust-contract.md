---
flow: build
priority: 95
opens: sdlc/tickets/0095-add-the-binding-members-to-the-rust-contract.md sdlc/tickets/0084-freeze-the-public-rust-contract.md sdlc/planning/libraries/rust.md sdlc/planning/adr/0017-libraries-over-one-bound-core.md
---

# 0095: Add the binding members to the Rust contract

Status: revised after re-review; confirming. Owner: Claude.

## Outcome and authority

Ticket 0084 freezes the public Rust contract. This ticket adds the members that the nine surfaces need, so each surface ticket only converts host values. It changes design records only. It is reviewed beside 0084. Three code tickets implement it. Ticket 0097 builds the private interrupt check after 0085. Ticket 0086 exposes the 0084 inventory plus `interrupt`, `deadline_seconds`, and `deadline_millis`. Ticket 0098 adds the rest: labels, spec readers, `kind`, `members`, `Row::probability`, `ErrorKind::name`, and the JSON methods. The inventory check covers 0084 and 0095 once 0098 lands. The proposed ADR 0017 amendment on the 0084 branch lists these members as supporting forms.

The needs come from `sdlc/planning/surfaces-port-guide.md` (branch `planning/surfaces-port-guide`): gaps G1–G11 and its sixteen review questions. Queue item 7 of `sdlc/planning/one-line-plan-2026-09-24.md`, accepted by Ian on 2026-09-24, adds the interrupt check. The proposed ADR 0017 amendment on the 0084 branch records it. Ian can overturn any member here.

## Why 0084 changed

- `Batch<'a, T>`: a lazy batch holds the engine, question, options, and input iterator between pulls. A `Batch<T>` with no lifetime can hold none of them, and it could not hold the interrupt check below.
- `Evidence::evidence(&self) -> &str`: no outside crate can build an `Error`, so a fallible method served no implementor. Bindings check NULL, invalid UTF-8, and NUL bytes before the call and raise their own six-kind host error (G11, Q6).
- `deadline_after` returns `Result`: branch ADR 0041 (ported at its number) refuses a budget above 4,294,967,295 seconds with `usage`. `Duration::MAX` must neither panic nor wrap (R1-11, R4-12, G7).
- `RelationRule::both_ways(name, source, target)`: the relate file accepts `"either":true` on two kinds (`specification/relate.md`). A one-kind form would make `Relate::from_json` lossy (Q14).
- `EngineBuilder::width` refuses 0 and anything above 32, the range 0077 keeps. A host door can no longer ask for width 100,000 (R4-12).

## Added declarations

Normative after rustfmt, under the same rules as the 0084 block.

```rust
impl<'a> CallOptions<'a> {
    pub const fn interrupt(self, check: &'a (dyn Fn() -> bool + Sync)) -> Self;
    pub fn deadline_seconds(self, value: f64) -> Result<Self, Error>;
    pub fn deadline_millis(self, value: i64) -> Result<Self, Error>;
}
pub enum QuestionKind { Decide, Choose, Tag, Score, Rank, Find }
impl Question {
    pub fn choose_labels(text: &str) -> Result<LabelBuilder, Error>;
    pub fn tag_labels(text: &str) -> Result<LabelBuilder, Error>;
    pub fn kind(&self) -> QuestionKind;
}
pub struct LabelBuilder { /* private */ }
impl std::fmt::Debug for LabelBuilder {}
impl LabelBuilder {
    pub fn label(self, value: &str, description: Option<Description>) -> Result<Self, Error>;
    pub fn model(self, value: &str) -> Result<Self, Error>;
    pub fn build(self) -> Result<Question, Error>;
    pub fn cut_at(self, value: f64) -> Result<Question, Error>;
}
impl QuestionSet { pub fn members(&self) -> impl ExactSizeIterator<Item = (&str, QuestionKind)> + '_; }
impl Recognize {
    pub fn from_json(value: &str) -> Result<Self, Error>;
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, Error>;
}
impl Relate {
    pub fn from_json(value: &str) -> Result<Self, Error>;
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, Error>;
}
impl<T> Row<T, Answer> { pub fn probability(&self) -> f64; }
impl ErrorKind { pub const fn name(self) -> &'static str; }
impl Details { pub fn to_json(&self) -> String; }
impl<T> AnnotatedRecord<T> { pub fn value_json(&self) -> String; }
impl Recognized { pub fn to_json(&self) -> String; }
impl Edge { pub fn to_json(&self) -> String; }
```

`QuestionKind` is `Copy + Clone + Debug + Eq + PartialEq`.

## Meanings

**Interrupt check (G1, Q1).** The engine runs the check only on the calling thread, never on a worker. It runs once before the first send, then at every existing 50 ms poll while the call waits: the width gate, a retry wait, a recording or lock wait, and bulk results. A `Batch` runs it inside `next`. It does not run during one blocking send, because a sent attempt finishes under 0073 anyway. A `true` return is the call's cancel token firing at that moment: nothing new starts, sent attempts finish, and the call returns `Cancelled` with the existing stop metadata. A panic inside the check fires the call's cancel token, joins every worker, then resumes the payload unchanged to the caller. The panic belongs to the host. With no check set, behavior is 0084's. The check is `Sync` because `CallOptions` stays `Send + Sync` for the workers that share it. A host whose handle is not `Sync`, such as a SQLite database pointer, wraps it in its FFI module. The worker-thread pattern stays legal for a host that prefers it, and no binding needs it.

**Host deadline numbers (G7, Q5).** `deadline_seconds` and `deadline_millis` apply branch ADR 0041 whole: `-1` is no deadline, `0` is spent (the call returns `Deadline` and sends nothing), any other negative, NaN, or infinity is `Usage`, and a budget above 4,294,967,295 seconds is `Usage`. The message names the host's exact value. When options receive several deadline calls, the last one wins, and `-1` clears an earlier deadline. Host rules stay in the binding: refusing a bool (ADR 0041's Python amendment) and clamping a computed budget at zero. Ticket 0099 ports ADR 0041 to main before 0086, and its owner sentence then names these two methods in place of the retired contract crate.

**Runtime labels (G5, Q3).** `choose_labels` and `tag_labels` build an unbound `Question` of kind `Choose` or `Tag` from labels known only at run time. The value equals what `Question::from_json` returns for the same file. `build` keeps the verb's default rule, as `ChooseBuilder::build` and `TagBuilder::cut` do. The unbound value works with `details`, `into_choose`, `into_tag`, and `QuestionSetBuilder::question`. A binding's scalar `choose` or `tag` calls `details` and reads `value()`. That is one judgment with the typed call's request identity, so it adds no send.

**Bulk forms (G4, Q2, Q12).** `Row::probability` returns the yes probability behind each `decide_many` answer, so DuckDB's `thinkthen_probability` streams. The ruled bulk form for `choose`, `score`, and `tag` is `annotate` over a one-question set. It closes R2-23. Bulk score returns the position only. The nearest level stays in `details`, as ADR 0017 section 6 item 6 rules.

**Reading specs (G6, Q4).** `kind` and `members` read the parsed value; `members` follows set order, and a banded member reads `Decide`. `Recognize::from_json` reads the `recognize @FILE` form and `Relate::from_json` reads the version-one relate file, each through the 0080 or 0081 parser. A member the library cannot honor, such as a non-default relate `fields` pointer, is `Usage`. `from_json` reports a broken rule as `Usage` and `load` as `Local`, the same split 0084 gives questions (Q16). A binding that reads a named file itself (PostgreSQL's `open_beneath`, SQL's `'@refund.json'`) and then calls `from_json` maps that `Usage` to its host's `Local`.

**Result JSON (G8, Q7).** One serializer serves the command and every JSON door (C, TypeScript, R, PostgreSQL). `Details::to_json` emits the `thinkthen.result/1` document that `--details` prints for one evidence text. `value_json` emits the bare `annotate` value from `specification/result.md`, with a failed member as `{"failed":{"kind":"backend","cause":…}}`. `Recognized::to_json` emits the bare `recognize` object. `Edge::to_json` emits one member of the bare `relate` array. The bytes equal the command's for the same result. The command calls these methods, so details, annotate, recognize, and relate have one serializer. Bare values for `filter`, `rank`, `find`, scalar `choose`, `score`, and `tag`, and `usage` get no method: each is a literal, a list, or an index, and ticket 0094's small door serializer writes them, checked byte for byte against the command. Record mode (`input`) stays with the command.

**A bad row mid-stream (G11).** A streaming host that meets a NULL, invalid UTF-8, or NUL row ends its iterator there, keeps its host error, and raises it after the batch drains. Every binding uses this one pattern.

**Names and handles (Q11, Q15).** `ErrorKind::name` returns the conformance word: `usage`, `backend`, `local`, `cancelled`, `deadline`, or `defect`. Each binding keeps one table from that word to its host error. Cloning an `Engine` copies a handle. The clone shares settings, cache, counters, and width state. `Counters` count one loaded library in one process and start at zero in a forked child (0078). Conformance case 17 asserts counter differences around a call, never absolute totals.

## Where each open question lands

| Q | Answer | Owner |
|---|---|---|
| 1, 2, 3, 4, 5, 6, 7, 12 | As above | this ticket; built by 0097, 0086, and 0098 |
| 8 | Bindings are unpublished workspace crates over the public API. The C door is one of them. Draft ADR 0047 | 0093, 0094 |
| 9 | Awaiting Ian: ADR 0047 item 5 gives the options and recommends a per-copy cap for 0.1 | Ian |
| 10 | A loopback backend serves the shared cases and each fault. No null backend or public fault hook | 0092 |
| 11 | Counter differences, per meanings above | 0091 |
| 13 | TypeScript converts scalar offsets to UTF-16 units. Case 68 (accent and emoji) carries both | 0091, TypeScript ticket |
| 14 | A database binding dedupes rows by name and kind in first-seen order, calls `relate` once, and maps each edge back to every row with that pair. The 255 cap counts unique pairs | ADR 0047, each database ticket |
| 15 | Per meanings above. `default_engine` is built lazily in the child, or built in a parent (PostgreSQL shared preload) and rebuilt by 0096's guard | 0086 test |
| 16 | "unsure" in cases and libraries (ADR 0017 section 6 item 4). `find` with nothing selected is null in values and JSON, and `none` stays the answer's pick word. Usage/Local split as above | 0091 |

## Deferred past 0.1

- Nearest level in bulk score. ADR 0017 puts it in `details`, and no row needs more.
- Label and level getters on `Question`. No binding row needs them.
- A public fault hook or null backend. 0092's loopback backend covers every test.
- A cross-image width cap, if Ian takes ADR 0047 item 5's recommendation.

## Acceptance

Design only. 0084 and this ticket agree; rustfmt parses the extracted blocks of 0084 and 0095; each name has one owner and shape; `wc -m` stays under 16,000; `git diff --check` passes.

## Complexity

Contract 3; state and timing 1; reach 3; proof 1; cost of error 2; total 10. Final level: 3. Every binding depends on these members, and a wrong interrupt rule would reach every host.

## Routing

Design only. The code tickets use Claude builders (Opus subagent) and fresh Claude reviewers.

## Review

- Design review: the 2026-09-24 review (`sdlc/records/2026-09-24-spine-review-contract.md`) asked for an ADR record of the added members and four clarifications, all applied. The re-review (`sdlc/records/2026-09-24-rereview-contract.md`) asked for the rustfmt wording, cancel before join on a panic, and the `Sync` reason; all applied.
- Code review: not applicable; design records only.
