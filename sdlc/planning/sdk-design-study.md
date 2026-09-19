# thinkthen libraries: a design study

A second agent wrote this study on 2026-09-19. ADR 0017 takes its grammar, its backend seam, and its constraints on the core. ADR 0017 departs from section 3: it binds one core into every language, and it keeps the ports of this study as the fallback.

Four of the ten circulating use cases live inside a program: the next action in an agent loop, stuck checks, skill selection, per-turn guardrails. A shell tool is the wrong shape for those. One library per language, over the same rules and the same backend seam.

## 1. The grammar

One verb, one result, no ceremony. The six verbs keep their names, and each takes the question first and the evidence second.

| | Unresolved | Failure |
| --- | --- | --- |
| Python | `Outcome.UNRESOLVED` on a three-valued enum; `.value` is `None` | an exception from `ThinkthenError` |
| TypeScript | `outcome: "unresolved"` on a discriminated union; `value` is `null` | a rejected promise carrying `ThinkthenError` |
| Rust | `Outcome::Unresolved`, the core's existing enum | `Err(thinkthen::Error)` |

Unresolved is a value; a failure never is. No error carries an outcome and no outcome names an error. Two traps close mechanically: a Python decision refuses to be a boolean, and `__bool__` raises; in TypeScript every object is truthy, so the union is the only way in. Async is the default in TypeScript and Rust; Python ships `Client` and `AsyncClient` alike.

```python
from thinkthen import Client, Question, QuestionSet, Outcome, ThinkthenError
tt = Client()                                   # key and address from the environment

try:                                            # (a) gate, fail closed
    d = tt.decide("The command only reads files.", cmd, threshold="0.1:0.9")
except ThinkthenError:
    return REFUSE                               # a failure is not a no
match d.outcome:
    case Outcome.YES: return ALLOW
    case Outcome.NO: return REFUSE
    case Outcome.UNRESOLVED: return ASK_A_PERSON

pick = tt.choose("Which action moves this forward?", trace, options=steps, threshold=0.6)
act(pick.value) if pick.value else ask_a_person()    # (b) None if unresolved or tied

kept = tt.filter("Names a delivery date.", notes, field="/text", threshold=0.9, jobs=8)  # (c)
top5 = tt.rank("Answers the query.", rows, field="/passage", top=5, jobs=8)              # (d)
for row in tt.annotate(QuestionSet.load("triage.json"), tickets, field="/body", jobs=8): # (e)
    if row["kind"] == "bug" and row["impact"] >= 2: page_oncall(row)

def test_refund_gate():                         # (f) no network, no key
    offline = Client(replay="tests/recordings")
    q = Question.load("refund.json").with_threshold("0.2:0.8")
    assert offline.decide(q, MESSAGE).outcome is Outcome.YES
```

```ts
import { Client, Question, QuestionSet, ThinkthenError } from "thinkthen";
const tt = new Client();

let d;                                          // (a)
try { d = await tt.decide("The command only reads files.", cmd, { threshold: "0.1:0.9" }); }
catch (e) { if (e instanceof ThinkthenError) return REFUSE; throw e; }
switch (d.outcome) { case "yes": return ALLOW; case "no": return REFUSE; case "unresolved": return ASK; }

const pick = await tt.choose("Which action moves this forward?", trace, { options: steps, threshold: 0.6 });
pick.value ? act(pick.value) : askAPerson();    // (b)

const kept = await tt.filter("Names a delivery date.", notes, { field: "/text", jobs: 8 });  // (c)
const top5 = await tt.rank("Answers the query.", rows, { top: 5, jobs: 8 });                 // (d)
for (const row of await tt.annotate(set, tickets, { field: "/body", jobs: 8 }))              // (e)
  if (row.kind === "bug" && row.impact >= 2) pageOncall(row);

test("refund gate", async () => {               // (f)
  const offline = new Client({ replay: "tests/recordings" });
  const q = (await Question.load("refund.json")).withThreshold("0.2:0.8");
  expect((await offline.decide(q, MESSAGE)).outcome).toBe("yes");
});
```

```rust
use thinkthen::{Client, Outcome, Question, QuestionSet};
let tt = Client::from_env()?;

let gate = match tt.decide("The command only reads files.", &cmd)          // (a)
    .threshold("0.1:0.9")?.send().await {
    Ok(d) => match d.outcome() { Outcome::Yes => Allow, Outcome::No => Refuse, Outcome::Unresolved => Ask },
    Err(_) => Refuse,
};

let pick = tt.choose("Which action moves this forward?", &trace)           // (b)
    .options(&steps)?.threshold(0.6)?.send().await?;
match pick.value() { Some(label) => act(label), None => ask_a_person() }

let kept = tt.filter("Names a delivery date.", &notes).field("/text")?.jobs(8).send().await?;  // (c)
let top5 = tt.rank("Answers the query.", &rows).top(5).jobs(8).send().await?;                  // (d)
for row in tt.annotate(&set, &tickets).field("/body")?.jobs(8).send().await? {                 // (e)
    if row.label("kind") == Some("bug") && row.number("impact") >= 2.0 { page_oncall(&row); }
}

#[tokio::test] async fn refund_gate() {                                    // (f)
    let offline = Client::replaying("tests/recordings");
    let q = Question::load("refund.json").unwrap().with_threshold("0.2:0.8").unwrap();
    assert_eq!(offline.decide(q, MESSAGE).send().await.unwrap().outcome(), Outcome::Yes);
}
```

Rust needs a builder because it has no keyword arguments. Python and TypeScript do not, and should not get one.

`Question.load(path)` returns an immutable question, and `with_threshold`, `with_options`, `with_levels`, `with_model`, `with_field` each return a new one. Precedence is the shell tool's rule unchanged: call site over file over default, a list replacing a list, the text always from the file. `d.row()` returns the exact `thinkthen.result/1` object the shell tool prints, so both feed the same `jq`. `dry_run`, `record`, and `replay` are client options beside `THINKTHEN_RECORD` and `THINKTHEN_REPLAY`, so one variable takes a whole test suite offline. `dry_run` returns the plan, never an answer.

One real third-party caller hand-rolled a concurrency limiter, a sync wrapper, an uncertainty measure, a digest, a result cache, and fakes for tests, and still has no retries, so one failed row loses a batch. That is the job list. At volume the verbs raise, and the exception carries the records already completed.

## 2. The backend seam

Competing services and local models are expected, so the seam comes first and the wire format sits behind it. A backend takes a neutral request — evidence, model name, a list of questions — and returns neutral answers: a probability of yes, per option, or per level, plus usage.

```python
class Backend(Protocol):                        # Python
    name: str                                   # "systemone", "logprobs", ...
    def ask(self, request: Request) -> Reply: ...
```

```ts
interface Backend { readonly name: string; ask(request: Request): Promise<Reply>; }
```

```rust
pub trait Backend {
    fn name(&self) -> &str;
    fn encode(&self, plan: &Plan) -> Result<Vec<u8>, EncodeError>;
    fn decode(&self, plan: &Plan, body: &[u8]) -> Result<Reply, DecodeError>;
}
```

Rust splits encode from decode so the pure core keeps both and the binary keeps the socket. The default adapter is the System One shape. A chat-completions adapter renders a decide as one constrained token and normalizes the log probabilities of the yes and no tokens, a choose as one token per option, a score as one per level. Its `name` enters the recording key, so recordings never collide. A local server speaking System One needs only an address.

**On the vendor SDKs: not a dependency, at any tier.** A hard dependency ties the library to the vendor the seam exists to escape. An optional extra is still a second encoder for the same wire shape, so the digest would follow the SDK's serializer and version, and an upgrade would miss every recording ever made. The official Python SDK also skips an answer type it does not know. The adapter needs one POST with a bearer header, retries honoring `Retry-After`, and the bytes back.

**Identical across backends:** the question file and set, the three kinds and every refusal, the threshold rule, the score arithmetic, `question_sha256`, the result row, the names. **Names the backend:** the recording key, which already digests the adapter name with the URL and request bytes; the entry's `adapter` field; and `meta`, which gains it.

**Leaks today.** `Question`, `Answer`, `Value`, `Threshold`, and `Outcome` carry no vendor word, so the vocabulary is clean. A grep finds these leaks outside `systemone/`.

- `backend.rs`: the vendor address as `DEFAULT_BASE`, the vendor model as the public `DEFAULT_MODEL`, and a URL built by appending `systemone::NAME`. All three belong to the adapter.
- `recording.rs`: `systemone::NAME` in the digest, the entry, and the replay check. The `adapter` field is right; the constant is the leak.
- `plan_document.rs` calls `systemone::encode_raw`, binding `--dry-run` to one adapter; `lib.rs` exports `pub mod systemone` with no trait beside it.
- Tests in `result.rs`, `text.rs`, `recording.rs`, and the binary use the vendor URL and `jev-*` names as bare literals. Better as one adapter-owned fixture.

## 3. One core or three ports

**A, bind the Rust core everywhere.** No drift, by construction. The cost is a build matrix: a Python wheel per platform, plus a source distribution needing a Rust toolchain; and for JavaScript a different wasm-bindgen loader per target, asynchronous loading in several, and a serialization boundary on every value. Debugging stops at the extension edge. The seam makes A worse, because a Python adapter would call back through the binding.

**B, pure ports held by a conformance suite.** Idiomatic, debuggable, and a contributor can fix a band-edge bug or write an adapter without learning Rust. The rules at risk are small and total: a band is three comparisons with inclusive edges, a cut of zero is refused, an exact tie is unresolved, a score is a dot product, precedence is three ordered sources. Each is a table of cases, and a table of cases is a fixture. The hard one is the canonical encoding of a request, since the digest covers exact bytes: fix key order, number formatting, whitespace, escaping, and whether a level's key is a number or a string, which the vendor's own SDKs already differ on.

**Recommendation: B, with the Rust crate as the reference implementation.** The cost: the rules get written three times, a change touches four places, and a rule the suite misses drifts in silence. Pay for it two ways — the suite gates every repository, and a ticket that changes a rule adds its case first.

## 4. The conformance suite

Under `conformance/` in the thinkthen repository, vendored at a pinned version by each library. Five families, none touching the network. **Threshold**: a rule and a probability in, an outcome out, every boundary and refusal. **Reading**: a question and answers in, a value, an outcome, and a row out. **Resolution**: a question file plus overrides in, the effective question, each setting's source, and the digest out. **Encoding**: a question and evidence in, exact bytes and the recording digest out, per adapter. **Adapter**: a raw body in, neutral answers out.

## 5. What does not carry over

Exit codes, `--quiet`, and `--raw` go: a library returns values, with no stream to keep clean and no shell to signal. Byte-for-byte passthrough becomes an identity, and `filter` returns the very objects it was handed, in input order, never a copy.

The transforms stay `jq`, because the rows are identical and every transform already written works on library output. No metrics functions in version one; revisit when a caller has written the same twenty lines twice. `find` carries over, last, and loses its line framing, since a library caller already has a list.

## 6. Names and packages

- PyPI `thinkthen`, imported as `thinkthen`. Modules `client`, `question`, `result`, `backend`, `errors`, `recording`.
- npm `thinkthen`, ESM with CommonJS beside it, no runtime dependency beyond `fetch`.
- crates.io: `thinkthen-core` stays the pure crate, and its `systemone` module becomes one implementation of a `Backend` trait beside it. The `thinkthen` package becomes a library and a binary in one, with command-line dependencies behind a default `cli` feature a library user turns off. The binary is then the library's first caller.

One vocabulary: the tool's. `Noul` and `criteria` never appear in a public signature, type, option, error, or page.

## 7. An agent skill

**Frontmatter.** `name: thinkthen`, a license line, and a `description` triggering on a program or script that must branch on the meaning of text: gate a risky action, route a request, screen an output, triage, filter or rerank by meaning.

**Sections.** What it is, with one example and the four outcomes. The five rules, first. The verbs, one table. Writing a question that works: one visible fact, never a judgment of quality. Thresholds, and what a cut leaves out. Question files and sets, with precedence. Details rows, saved runs, transforms. Record and replay. Backends and adapters. What it will not do: no acting, no text, no real-time loop. A library section when the libraries ship.

**The five rules.** Word the question so that yes permits the action, and every other outcome leaves it undone. Use a band for anything risky: a cut has no unresolved middle, and a no under a cut is not a confident no. Never treat a failure as a no. Keep the details rows, or nothing can be measured later without paying again. Test with replay, so no test needs a key or a network.

## 8. Constraints on the core, for the tickets being built now

- **0013.** Concurrency, the connection pool, and ordering live in the binary. The core exposes independent plans and assumes nothing about the order answers return in. No `--jobs` value reaches it.
- **0017.** The question-file parser takes text already in memory, never a path. Precedence is a pure function from a parsed file plus overrides to a resolved question and a source per setting, callable with no command line. Messages name the key and the source, never a path.
- **The digests.** Put the canonical encoding in the specification as a rule, not as whatever the serializer happens to do. `question_sha256` digests the resolved question, never the file's bytes, under every adapter alike.
- **One inconsistency to fix now.** `meta.questions_sha256` is the digest of the definition file, but a library caller may build a set in memory. Make it the digest of the canonical encoding of the resolved set.
- **0014 and 0015.** The cut, the sort by probability of yes, the input-order tiebreak, the `--top N` slice, and the grouping of questions by distinct `on` in file order are pure functions in the core.
- **The adapter name** is a value, never a constant, wherever the recording or `meta` touches it, and replay keeps taking response bytes rather than a directory.
- **The outcome enum never gains a failure variant, and the error type never gains an unresolved one.**

## 9. Order to build in

1. **Close the vendor leaks, add the `Backend` trait.** Proves the seam while one adapter has to move.
2. **Write the conformance suite against the shipped tool.** Proves the rules state as data, and surfaces the ones living only in code.
3. **Split the Rust package into a library and a binary.** Proves the surface suffices for the only caller that exists.
4. **Python: decide, choose, score, question files, replay, rows.** Proves the four in-program use cases.
5. **Python: filter, rank, annotate, concurrency, the failure rule.** Proves volume and record identity.
6. **A second adapter, in one language.** Proves the seam against a wire shape that is not the vendor's.
7. **TypeScript, steps 4 and 5.** Proves the suite catches drift, where drift first shows.
8. **`find`, and the skill's library section.** Proves nothing new, so it is last.

Each step ships only when the conformance suite passes in its repository.
