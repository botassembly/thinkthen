# Notes

Write as you go; a note written later is a guess. Newest entry last.

## 2026-09-21 — lane start

Ruby is not installed on the host (205 found the same). Everything Ruby
runs in containers, removed after every run. No host install, no rc
change; the rc guard greps `~/.zshrc` at the end.

- **Tried:** `command -v ruby gem rbenv mise`
- **Saw:** no output, exit 1.
- **Means:** the same clean-machine start as 205; the container path is the
  honest one for this surface.

### Toolchain, decided after probing

- The host has no rustup shims for cargo (`~/.cargo/bin` holds only extra
  tools); the compiler lives in `~/.rustup/toolchains/stable-.../bin` and
  `/usr/bin`.
- The builder image is `ruby:3.4-trixie` plus one package, `libclang-dev`,
  which rb-sys's bindgen step needs; the full bookworm image 205 used
  carried it. A one-line `Dockerfile` bakes `thinkthen-ruby-builder:local`
  (removed with `docker rmi thinkthen-ruby-builder:local`). The host
  toolchain runs inside it mounted read-only (host glibc 2.39 < trixie
  2.41, forward-compatible).
- The cargo home is folder-local, `.runtimes/cargo` (gitignored), so the
  registry cache persists across container runs and nothing writes to the
  host.

## The build, and one real bug the crash caught

The extension follows 205's pattern: magnus 0.7, `rb_thread_call_without_gvl`
around every engine call, the GVL re-taken by the tick through
`rb_thread_call_with_gvl`. The contract types cross one way; no rule, no
retry, no sending lives in Ruby or the shim.

**Bug, found as a segfault, cause found by reading the C backtrace.** The
first `decide` crashed inside `magnus::error::raise` reading a garbage
string. The cause was mine: `single()` decoded the crossing's answer as a
`(answer, raised)` tuple while `single_body` boxed only the answer, so the
return read past the allocation. Single calls carry no tick and so no
`raised`; the tuple was bulk-only thinking pasted onto the single path.
Fixed by decoding exactly what the body boxed. Recorded here because the
same shape will tempt the next shim: the boxed type and the decoded type
must be spelled the same on both sides of `without_gvl`.

### Fixture lesson

The null backend answers by evidence substring, lowercase: "refund" 0.97,
"maybe" 0.55, else 0.03. My first fixtures wrote "Maybe" capital and the
band test read 0.03 as a no. The engine was right; the fixture was wrong.

## Results, all commands run inside the container, stub on 8214 at 300 ms

Surface tests (ENGINE_NULL=1):

```
20 runs, 48 assertions, 0 failures, 0 errors, 0 skips
```

Conformance slice, offline, ported case-for-case from the Rust runner:

```
12 ok, 4 skip with printed reasons (wire cases, the disk cache, the
pre-fired token divergence), 0 FAIL
```

The skips and the one divergence match the sibling surfaces exactly.

Slide sample, as drawn: green, with one finding reported, not hidden:

```
finding: score returned 1.7, the slide comment says 2.0; nearest level is 'Immediate.'
3 of 8 are complaints   (the filter comment's promise)
```

**The finding, stated for the slide's owner:** the Ruby slide's comment
promises 2.0 for `score`; the offline backend's level distribution puts
the probability-weighted position at 1.7 with "Immediate." still the
nearest level. The semantic promise holds — the top level wins — but the
exact number depends on the reply's level distribution, which the offline
backend synthesizes. The slide should say what it means without pinning
the number, or pin the distribution. The sample asserts the range and the
nearest level and reports the difference; the slide owns the change.

Interrupt proof, brief item 7, on the wire (ENGINE_WIDTH=8, 1,000 records,
watcher fires the token and Thread#raise(Interrupt) at 1.000 s):

```
raised: Interrupt
wall:   1.207 s from start to raise (signal at 1.000 s)
stub:   requests at return 32, after 2 s settle 32
```

The raise lands 0.207 s past the signal — inside the tick, which cancels
the token, lets the one in-flight round finish (32 requests), and
re-raises. Nothing served after the return. That is the 211 poll shape,
ported: the tick runs with the VM lock taken each wait interval, so
`Thread#raise` reaches a blocked bulk call.

The one check script, end to end with the stub up:

```
== ruby surface: surface tests, null backend        20 runs, 0 failures
== ruby surface: conformance slice, offline         green
== ruby surface: slide sample, as drawn             green, one finding printed
== ruby surface: interrupt proof on the wire        green
```

## Cleanup and the runtime guard

Containers are `--rm`; none remain. The built image stays for the parent's
check runs and is removed with `docker rmi thinkthen-ruby-builder:local`;
the base it was built from is the official `ruby:3.4-trixie`. The stub is
killed after each wire pass. The rc guard, after every install this lane
made (all inside the folder or the container):

```
$ grep -c deno ~/.zshrc
0
$ git -C repos/dotfiles status -s | wc -l
0
```

Nothing on the host changed.

## 2026-09-21 — recognize and relate, the deck's Ruby calls as drawn

**Tried:** the whole brief against the recordings — `cargo build --release` in the builder image, a smoke run of both deck calls, `tests/conformance.rb` (the recognize and relate cases), `tests/test_surface.rb`, and `./check.sh` with the stub on 8214; the vocabulary sweep over `lib/`, `src/`, and the two test files.

**Saw:**

```
smoke, the deck's recognize call:
[["Maria Chen","person",0,10,0.98], ["Northwind Freight","organization",18,35,1.0], ["Chicago","place",39,46,0.6693]]
"Maria Chen"                       (text[start...end], Ruby characters)
[["works_for",1,2,1.0]]
smoke, the deck's relate call (the four alerts in the arm):
[["caused_by",1,2,0.59], ["caused_by",1,4,0.94], ["caused_by",2,4,0.94], ["caused_by",3,4,0.84]]

surface tests: 25 runs, 64 assertions, 0 failures, 0 errors, 0 skips
conformance:   green; 71-relate-R03-persubject-10 skipped by name (engine-internal arm);
               40 recognize cases and the relate arms ok, including 68-recognize-C41
interrupt:     raised Interrupt 1.209 s from start (signal at 1.000 s), stub frozen at 32
sweep:         no confidence, certainty, likelihood, cutoff, gray zone, calibrated, accuracy;
               no "score" used for a probability — empty greps above
```

**Means:** the deck's Ruby calls run as written with the recorded answers; `text[start...end]` slices names out of the original text in Ruby characters, proven on `"Le café 😀 Maria Chen arrived."` from C41; `relate` crosses every record at once and refuses 256 with a usage error naming 255; the any-kind end is the one-character string `"*"` (C36's `located_in` from `*` to `place`); `strength` on names and `probability` on relations are bound per the settled rule; results of no fixed size come back as `ThinkThen::Entity`, `Relation`, `Recognized`, and `Edge` records parsed from the door's one JSON string (the C-door pattern).

**Finding for the parent, not touched here:** the contract's spec grammar still reads `from` and `to` keys in a relation rule (`relation_from_value` in `contract/src/lib.rs`), while the ruling says the question file says `source` and `target` too and no door converts. The Ruby wrapper builds `from`/`to` spec keys against the parser as it stands; when the contract flips, `lib/thinkthen.rb`'s `relation_rules` is the one place here to change.

**Finding, pre-existing, out of this change's scope:** the Ruby conformance runner has no arms for `rank` and `find`, so those three cases skip as "no case shape" even though the surface implements both verbs and its own tests cover them. The Python runner has a `rank` arm. A small follow-up for whichever lane owns the case runner shape.

**Pinned divergence, deck-side:** the deck's Python, TypeScript, and Rust sections ask a `located_in` rule on the Maria Chen sentence; the C01 recording covers `works_for` and `based_in`, so that rule cannot run as recorded. The Ruby section does not use `located_in` and is unaffected; the divergence is filed with the deck's owner (`the deck repository's issue`). Not bent here.

## 2026-09-21 — the rulings wave (languages lane)

Ruling 1 aftermath: the wrapper's relation rules build `source`/`target` keys, and the Hash ends form reads `:source`/`:target`; `tests/conformance.rb` reads the re-keyed rules.

Smaller item: `relate_body` now calls the contract's `relate_checked`, so the 255-record guard is inherited through the function the other doors use rather than resting on the stand-in's own repeat of it.

Ruling 2: two `#[cfg(test)]` unit tests in `src/lib.rs`: `the_defect_kind_names_its_error_class` (the name table maps `Defect` to `DefectError`) and `a_panic_inside_the_shim_becomes_the_defect_kind` (the shim's own `guarded` path produces the defect). They run in the builder container in `check.sh`: 2 passed. The host side asserts `ThinkThen::DefectError.new(..., "defect", false)` carries `kind` and `retryable` in `tests/test_surface.rb`.

## 2026-09-21 — the shapes from lane B item 2 (languages lane)

The three shapes `e44d492` landed in `contract/` and `standin/`, carried
into this surface and proven offline, plus the fast-backend cancel proof
and the examples file. Commands and output as they happened, all inside
the builder container.

**`Details.requests` and `failed_questions` (0053, 0054).** `ThinkThen.details`
now carries both beside the trail it already had:

```
$ ENGINE_NULL=1 ruby -I lib -e "...ThinkThen.details('Is this a complaint?', 'i want a refund')..."
{"probability" => 0.97, "answer" => true, "model" => "jev-latest", ..., "requests" => ["d3476c41..."], "failed_questions" => 0}
```

**The failed marker, this host's spelling: a Hash.** `ThinkThen.annotate`
returns the ruled `{"failed" => {"kind" => ..., "cause" => ...}}` for a
field whose question failed while its neighbours answered:

```
$ ENGINE_NULL=1 ruby -I lib -e "...ThinkThen._parse_set(...); ThinkThen.annotate(set, ['order 4471: charged twice, please refund'])..."
{refund: true, topic: {"failed" => {"kind" => "backend", "cause" => "missing_answer"}}}
```

**The record row (go-ahead item 4).** The conformance slice builds the
ruled `{"input" => record, "value" => answer}` Hash from the surface's own
outputs and checks it where the cases carry rows (`05`, `06`, `19`); no
Ruby method was added. Cases 73 and 74 run green:

```
ok       73-details-carries-requests
ok       74-annotate-preserves-good-answers
conformance slice green for the Ruby surface
```

Case 73 checks the audit's identity fields (the null backend's own rule
cannot reproduce its recorded probability); case 74 checks the marker and
its count.

**A real bug found and fixed while proving the tick.** `ThinkThen.with_tick`
is the documented interrupt path, and it stored its block in an `@tick`
ivar that no native code ever read — the only working shape was passing
the tick as the fifth positional argument to `Native::Engine` directly.
`src/lib.rs` now falls back to the ivar when the argument is nil:

```
$ ENGINE_NULL=1 ruby -I lib tests/test_cancel_fast.rb
raised: Interrupt after 1.074 s (interrupt set at 1.0 s, 18 ticks)
the fast-backend tick raise holds
```

The test runs a two-million-record null batch (about 4.3 s deaf), raises
from the tick one second in, and requires the interrupt within 1.5 s.

**The examples file.** `examples.json` holds all ten functions keyed by
name, each a runnable call and the answer the null backend gives;
`tests/examples.rb` runs each in a fresh process (a private directory for
the question files) and `check.sh` calls it:

```
10 of 10 examples ok
```

**Full check, `./check.sh`:** build, the shim tests, 27 surface tests (74
assertions), the fast cancel proof, the examples, the conformance slice
green (73 and 74 included), and the slide sample. The wire interrupt proof
skips with no stub on 8214; exit 0.


## 2026-09-21 — the settle wave

Wired to the contract's settlements (`1fe8173`): `rank` and `find` return
the ruled pair — `Ranked` (place, record, probability) and `Found`
(place, unit, probability) structs; `Ranked#to_s` prints its record so the
deck's `puts mail` line runs as drawn; a built question plus members
refuses naming both; `nearest` rides in `details`. The examples file's
rank and find entries now show the pair, and the slide's rank assertion
compares `map(&:record)`. `check.sh` green; the public-name check admits
`Ranked`, `Found`, and `Ranked#to_s` with their citations.

## 2026-09-21 — punch-list item 1: probabilities from the one call

`decide_many_with_probabilities` no longer makes a details call a record.
The native `decide_many_with_probabilities` carries each judgment's
probability beside its answer from the same bulk call (commit c666251).
Acceptance: `tests/test_pairs_one_crossing.rb` against the counted stub:
20 records, 20 requests, none after the return -- a details pass a record
would have shown 40. `libraries/ruby/NOTES.md` also inherits punch-list
item 3 there: `recognize` and `relate` answers now arrive as typed Ruby
records, built natively; no serialized answer JSON is parsed in Ruby.

## 2026-09-22 — the review fix wave: interrupts, the connector, the checked deadline

The Ruby lane of the surfaces branch review: group 4's interrupt finding,
plus the phase-1 adoption (connector, checked deadline, panic guard).

**The interrupt (group 4).** Before this, `rb_thread_call_without_gvl` was
called with no unblock function, so `Thread#raise` and Ctrl-C could not
reach a running call: the VM lock was released with no way to wake the
waiting thread, and a plain call ran to completion before the raise
landed. Reproduced against HEAD's shim: `raised: Interrupt after 4.378 s
(signal at 0.3 s)` on a two-million-record null batch — the full deaf
batch, about 4.3 s.

The fix is MRI's unblock function, firing the call's own token. Every
crossing now arms the call with a token (the caller's, or a fresh one) and
passes Ruby `stop_call` as the unblock function's data: when an interrupt
arrives, MRI calls it from the interrupting thread, it sets the token's
atomic, the engine returns its cancelled error at its next stop check, and
the pending exception re-raises when the call returns. Semantics were
proven first with a 30-line C extension against MRI 3.4 in the builder
container, because the docs alone were ambiguous: the unblock function
fires (`ubf=1`), the body returns because it polled the stop flag
(`body=1`), and MRI raises by long jump (`after=0`) — so the code after
the VM call never runs on an interrupt. That last fact is why the answer
box and the unblock function's token copy leak, one small allocation pair
per interrupted call; the token's own refcount is dropped on every
non-interrupted path.

**The checked deadline (group 2).** `options_for` used
`Duration::from_secs_f64(seconds.max(0.0))` outside the panic guard.
Reproduced against HEAD: a 1e300 budget aborted the process —
`thread caused non-unwinding panic. aborting`, exit 134. It now goes
through the contract's `Options::with_deadline_seconds`, inside the
guarded region: NaN, infinity, any negative other than the sentinel, and
oversized budgets raise `UsageError`; minus one means no deadline; zero
stays a spent deadline. A Ruby test and two shim unit tests pin it.

**The connector (phase 1).** `EngineValue` holds `Arc<dyn Engine>` built
by `StandinConnector` through the contract's `Connector`; the job structs
hold the `Arc` instead of a raw pointer to the concrete type. Pointing the
surface at the real engine is now the one connector line the merge
changes. The panic guard also wraps `connect` and the option build, so a
panic on either side of the engine call cannot reach the host.

**Case 74's opt-in.** The stand-in's synthesized partial failure now fires
only under `ENGINE_SYNTHETIC_PARTIAL` (phase 1), and the engine reads it
at construction, so `tests/test_surface.rb` and `tests/conformance.rb` set
it before the require. Without that, `74-annotate-preserves-good-answers`
read `true` where the marker belongs.

Commands and output (all inside the builder container; the wire runs
against the stub on 8214 at 300 ms):

```
$ cargo test --quiet --lib
test result: ok. 4 passed (the two new ones: the unblock function fires
the call's own token; a hostile budget is a usage error not a panic)

$ ENGINE_NULL=1 ruby -I lib tests/test_interrupt_fast.rb
raised: Interrupt after 0.440 s (interrupt set at 0.3 s)
a plain call hears Thread#raise

$ ruby -I lib tests/test_interrupt_wire.rb   # stub up, 300 ms delay
raised: Interrupt after 0.604 s (signal at 0.5 s, one round 0.302 s)
stub:   requests at return 16, after 2 s settle 16
deaf would be about 15.1 s over 50 rounds; the bound is 2.1 s

$ ./check.sh
surface tests: 32 runs, 99 assertions, 0 failures
conformance slice: green, 74 included
slide sample: green; interrupt proofs: green; pairs: green
```

The wire test skips, with a printed reason, when no stub is up or the stub
runs without a delay; the timing proof needs one.

## 2026-09-22 — wave 3: the second review's Ruby items

The second review's Ruby findings, each with the command that proves it
before and after. All runs inside the builder container on the null
backend; the wire sections skip without a stub.

**The unblock function is gone; the poll hears interrupts (item 15).** The
crossing passed Ruby an unblock function, and Ruby calls one for *every*
interrupt at all — a spurious `Thread#wakeup`, a trapped signal, a real
raise alike. The callback fired the token the engine watched, which was the
caller's own token when one was given, so a wake-up cancelled the batch and
one call's interrupt cancelled every sibling sharing the token. Now the
engine watches a fresh per-call token; the poll checks the caller's token
(and fires only the call's own), then calls MRI's pending-interrupt
handling with the VM lock taken (`rb_thread_check_ints` under
`rb_protect`, so a raise cannot unwind across the Rust frames), then runs
the caller's tick. A real `Thread#raise`, Ctrl-C, or kill cancels the call
and re-raises; a wake-up and a trapped signal raise nothing and cancel
nothing. Single verbs take no poll from the contract, so their channel is
the engine's own stop checks on the token the caller gave; an interrupt
that lands during one surfaces when the call returns. The trade-off is
stated here, not hidden: no surface can stop a send mid-flight.

Pre-fix, `tests/test_harmless_wakeups.rb` failed with all three:

```
a wake-up cancelled the call: ThinkThen::CancelledError: the wait was cancelled
a trapped USR1 cancelled the call: ThinkThen::CancelledError: the wait was cancelled
the interrupt cancelled the call sharing the token: ThinkThen::CancelledError: the wait was cancelled
```

Post-fix: `wake-ups and trapped signals leave calls alone; an interrupt
fires only the call's own token`. The existing proofs still hold:
`test_interrupt_fast.rb` raises in 0.467 s against a 0.3 s signal, and
`test_cancel_fast.rb` in 1.045 s against a 1.0 s tick (17 ticks).

**The tick is a GC root for the call's length (item 8).** The block is
copied into Rust memory, and a bare copy there is not a root. `bulk` and
`annotate` now hold it in a `magnus::value::BoxValue` — a registered
address, unregistered when the call returns — and the poll reads the live
value through that slot. Honest limit: `tests/test_tick_gc.rb` cannot
provoke the old hole, because Ruby's conservative stack scan keeps the
block alive by accident in this shape (two threads, `GC.start` and
`GC.compact` every 5 ms, 88 ticks, no collection both before and after).
The fix removes the reliance on that accident; the test is the two-thread
GC stress test the review asked for, and it pins that the registered slot
is released when the call ends (a `register_mark_object` "fix" would leak
the block forever).

**One error base, a refused bad deadline, records as JSON (leftovers).**
`ThinkThen::Error < StandardError` is now the base of the six kind
classes, and the wrapper's own refusals raise `UsageError`, so one
`rescue ThinkThen::Error` hears them all (the old `UsageError` inherited
`ArgumentError`; the test that pinned that now pins the base).
`optional_deadline` raised instead of `.ok()`-dropping: `deadline: "soon"`
is a `UsageError` naming the deadline. Records and evidence that are not
strings cross as their JSON text (`ThinkThen.text_of`), so a Hash keeps its
fields and a record object can speak its own `to_json`; Ruby's `to_s` form
is never sent. Pre-fix, `tests/test_error_classes.rb` died on
`ThinkThen::Error` (no such constant) and the JSON-record case answered
`false` where the JSON text answers `true`.

**The -1 sentinel stays, deliberately.** Ruby's deadline is seconds, its
absent spelling is `nil`, and the contract's `NO_DEADLINE` (`-1`) still
means no deadline here, as the wave-1 test pins. Python and Node refuse
`-1` because their absent spelling (`None`, `null`) is the only way to say
none; the C door keeps the sentinel because a number has to stand for
none. If the ruling changes, this surface changes with it.

**The gate needed three cross-lane repairs to go green** (the phase-1 and
wave-2 commits landed after this surface's last gate run):

- The stand-in's synthesized partial failure is compile-time now
  (`standin/Cargo.toml`, `synthetic-partial`), so the old
  `ENGINE_SYNTHETIC_PARTIAL` opt-in in `test_surface.rb` and
  `conformance.rb` was dead. `Cargo.toml` declares the feature,
  `build.sh synthetic` builds the gate's copy with it, `check.sh` uses
  that build, and the tests no longer set the dead variable. The plain
  `build.sh` (the packaging path) never carries the fixture.
- The conformance runner wrapped case sets without `version`, which the
  core's parser refuses now: it carries `"version" => 1` like the Python
  runner.
- The annotate example's expected field order was the old sorted order;
  the ruled order is the file's, so it reads `{refund, complaint}`.

**Commands and output (final gate).**

```
$ ./check.sh            # exit 0
surface tests: 32 runs, 99 assertions, 0 failures, 0 errors, 0 skips
deadline bounds: usage for hostile budgets, minus one means none, zero stays spent
interrupt (no tick): raised Interrupt after 0.467 s (signal at 0.3 s)
harmless wake-ups: green
tick GC stress: green (67 ticks, 2,000,000 answers, no collection)
error classes: green (one base, refused bad deadline, records as JSON)
fast-backend cancel: raised Interrupt after 1.045 s (tick at 1.0 s, 17 ticks)
fork: green; examples: 10 of 10; conformance slice: green; slide: green
```

`cargo test --lib` is 3 passed (the unblock-function test is gone with the
unblock function; the panic test now asserts the contract's boundary
message).

## 2026-09-22 — the wire interrupt proof's segfault: the leaked workers (fixed)

**The finding.** The final verification run of the gate segfaulted in the
wire step `tests/test_cancel.rb`: exit 139, five runs in six. The shape is
the one the test pins — a watcher fires the caller's token and
`Thread#raise` back to back, which is exactly what the second review's
fix wave taught the poll to hear.

**The diagnosis, by command.** An `LD_PRELOAD` shim that interposes
`sigaction` (kept separate from the branch, in session scratch) caught the
first fault: `segv addr=10`, RIP inside
`lib/thinkthen/thinkthen.so`, and the executing threads' names read from
`/proc/self/task/<tid>/comm`. Every faulting thread was a `ttb-worker` —
the stand-in batch's own workers — and the fault was
`thinkthen_contract::Error::guard` dereferencing a pointer that was `0x10`,
seconds after `test_cancel.rb`'s call had already returned. The workers had
outlived the call.

**The mechanism.** The caller's token and the raise land in the same
breath. The poll's caller-token branch cancelled the call's token and
returned **without hearing the interrupt**; the raise stayed pending, and
MRI later delivered it at its own checkpoint — inside the batch, during a
GVL reacquisition — where the jump skips Rust frames, including the
`thread::scope` that joins the batch's workers. The workers lived on over
the freed `Options` (its `&Cancel` read back as `0x10` from reused stack),
and the process died at the next checkpoint. Raise alone and token alone
were each clean; only the pair crashed, which is what the bisected probe
matrix showed (`none`, `token`, `raise`: survived; `both`: crashed).

**The fix.** `hear_interrupts` now runs on every poll path: the
caller-token branch drains the pending interrupt under `rb_protect`
before it returns, the regular path still drains first, and a second
drain follows the tick's own Ruby code. `without_gvl` also drains once
after the call returns, with the VM lock held, so a raise that arrives
between the last poll and the return surfaces as the call's error — the
clean channel — instead of firing inside the raise that follows.

**Evidence, pre-fix versus post-fix, by command.** Same stub (8214, 300 ms
delay), same test:

| probe | pre-fix | post-fix |
| --- | --- | --- |
| `ruby -I lib tests/test_cancel.rb` | exit 139, 5 of 6 runs | exit 0, 10 of 10 runs, `interrupt proof green` |
| `tests/test_interrupt_wire.rb` | exit 0 | exit 0 |
| `./check.sh` (whole surface) | died at this step | exit 0, 84 ok-lines |

The regression pin is `tests/test_cancel.rb` itself: it crashed against
the pre-fix build and passes after, so the gate now fails if the drain
regresses.

## 2026-09-23: the third review's Ruby items, each with its probe

Item 5 (the lock-exit window, HIGH): the whole crossing now runs under one
`rb_protect` (`protected` in src/lib.rs) with the tick registration and the
job's answer slot owned OUTSIDE it, and the poll's own with-gvl body under a
nested protect. Probe (wire stub, 300 ms delay, `/tmp/rb3/probe_drain8.rb`):
30 calls, one `Thread.main.raise Interrupt` mid-call each, raised at 0.05-0.15 s
into every call - pre-fix: SEGV storm (`SEGV received in SEGV handler` x4,
process death); post-fix: `raises landed: 30, delivered: 0, interrupted: 30,
errored: 0, PASS settled, no crash`, exit 0. Retention (probe_drain2, class-
counted via ObjectSpace after full GC): `live tick objects after GC: 0`.

Item 19a (ticks ran on the wrong thread): the tick rides
`Thread.current[:thinkthen_tick]`, and every module verb passes it explicitly;
the shared `@tick` ivar and its Rust fallback are gone (`tick_from` is now the
explicit argument only). Probe (probe_tick_owner.rb, wire stub): pre-fix
`ticks ran: 3448, on owning thread: 2335, on foreign thread: 1113, FAIL`;
post-fix (probe_tick2.rb) `ticks ran: 2, on owning thread: 2, on foreign
thread: 0, PASS`.

Item 19b (nil record crossed as "null"): `text_of` refuses nil. Probe
(probe_nil.rb): pre-fix `ANSWERED (nil crossed as a record)`; post-fix
`refused: ThinkThen::UsageError - a record is text or a JSON-able value, not nil`.

Item 18 (annotate overwrote input columns): with `on:`, a question landing on
any existing key refuses before any request. Probe (probe_annotate.rb):
pre-fix `OVERWRITTEN: body is now false (original text lost)`; post-fix
`refused: ... annotate cannot add a question named 'body'`.

Item 19c (single-verb honesty): the with_tick doc now states it plainly - a
single verb cannot stop the request already sent; the interrupt surfaces when
the crossing returns. No claim of an unblock function remains.

Skip-table adoption: tests/conformance.rb and libraries/r/conformance.R both
decide through `conformance/skiptable.py lookup` (the one reader); the
private matchers are gone. One shared fix was required: the CLI parsed
`--none`/`--error` as strings, so boolean facets never matched
(`python3 conformance/skiptable.py lookup r 25-find-none-fits --verb find
--none true` printed RUN before, SKIP with the reason after). Both runners
pass the verb, kind, form, record, none, and error facets. R conformance:
`conformance slice green for the R surface` (76 ok). Ruby conformance:
`conformance slice green for the Ruby surface` (72 ok, fixture build).

Packaging: the gemspec stamps `Gem::Platform::CURRENT` and finds the
extension by glob (`.so` or `.bundle`), so a Mac-built gem carries its own
name; `required_ruby_version` is now `>= 3.4` - what is built and tested, the
review caught the untested `>= 3.1`. The builder image installs the pinned
Rust toolchain (1.93.1) itself and build.sh/check.sh no longer mount the
host's `~/.rustup`, which could not work from a Mac; the image rebuild
(`docker build -q -t thinkthen-ruby-builder:local` -> sha256:83143d0...) and
`./build.sh synthetic` then `./check.sh` (exit 0, 90 PASS/ok lines) prove the
self-contained image. check.sh ends with `./build.sh` so the gate leaves the
production shape, mirroring the R surface's restore step.

Full gate for this surface at this commit: `./check.sh` exit 0 (wire stub on
8214), ending `restored: thinkthen-0.0.1-x86_64-linux.gem`.

## 2026-09-23: the seventh review's Ruby items

R7-9 (and the Ruby half of R4-18): `tests/test_tick_gc.rb` could not fail. The tick runs on the watchdog thread, so its drop cleared the watchdog's thread-local and left the caller's hold in place. The test now clears the caller's thread-local by name and checks the drop took. Evidence: the weak-row mutant (the row holds a `WeakRef`, the crossing drops its local) fails the new test with "the collector took the tick" after 1 tick. The old test passed the same mutant with 133 ticks. The production build passes the new test with 134 ticks.

R7-10: the builder image installs its toolchain inside the container at image build time and cannot read `rust-toolchain.toml` there. Decision: keep the literal in the Dockerfile and hold it equal to the pin by check. `sdlc/scripts/policy.py` (lint rung) fails when the Dockerfile's `--default-toolchain` differs from the channel or its components differ from clippy and rustfmt. A pin bump therefore edits the Dockerfile, and `build.sh` rebuilds an image older than its Dockerfile. A build argument fed from the pin would give one source, but the edit would force an image rebuild that needs the network, and the check gives the same guarantee. Evidence: a scratch copy with the Dockerfile at 1.92.0 passes the old policy and fails the new one with "libraries/ruby/Dockerfile installs exactly the pinned channel". Ian can overturn this in favor of a build argument.
