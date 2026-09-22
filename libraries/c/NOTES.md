# NOTES

The C surface lane, 2026-09-21. The door lives here and owns every exported
symbol; the engine exports none.

## The shape

`src/lib.rs` builds `libthinkthen.so`, `libthinkthen.a`, and an rlib the
door's own tests link, against `contract/include/thinkthen.h`. The typed
`thinkthen_decide` answers with both the outcome and the probability in
one request by riding the engine's bulk path with one record, because the
contract's single `decide_opts` returns the answer without the
probability and `details` would ask twice.

A question string that is a JSON object must parse in the question-file
grammar or the call fails with the usage kind; a string that is not JSON
at all is the bare text of a decide question at the default cut, which is
what the slide passes. The first draft fell back to the bare question on
any parse failure, which silently turned broken grammar questions into
plain decide questions; the door suite caught it and the fallback now
applies only to non-JSON strings.

The JSON door's request is the question file's own shape with the
evidence beside it, plus the small markers the grammar cannot carry on
its own: `records` for filter and annotate, `units` for find, `rank: true`
beside a decide question for rank, `details: true` for the audit view,
`usage: true` for the counters. Replies: `{"answer": ...}` for the ask
verbs (unsure is `null`), `{"indexes": [...]}` for filter (the conformance
file's own shape), `{"answer": n, "nearest": "..."}` for score, the
counters object for usage, and one object a record for annotate, the
set's names in order with each verb's natural answer.

## Findings for the contract, not worked around here

1. **The header's doc promises the question-file grammar; the slide passes
   a bare string.** `thinkthen_decide`'s comment says "one question in
   the question-file grammar", and the C slide hands it a plain sentence.
   The door accepts both, and the header's doc line should say so.
2. **The JSON door's failure code is not retrievable.** The typed doors
   return their code; `thinkthen_call` returns null and the caller can
   read the message and the retryable flag but never the kind itself. The
   header's sentence "with the code as the return of the next
   `thinkthen_error_message`" does not hold: that function returns a
   string. A `thinkthen_error_code` entry point would close it.
   *Closed 2026-09-22: `thinkthen_error_code` exists; see the options
   design section at the end of this file.*
3. **The C header exposes no cancel token, no deadline, and no poll
   callback.** The ADR says the poll callback belongs to the binding, but
   the C door gives a host no way to install one, so a C program cannot
   stop a bulk call or cap one with a budget. Conformance case 18
   (cancel) cannot run on this surface for that reason.
   *Closed 2026-09-22 for the token and the budget: every `_opts` spelling
   carries both, and a C host fires the token from another thread. The
   poll callback stays deferred, with the reason in `DESIGN.md`.*

## Commands and output, as they happened

The build, both libraries:

```
$ cargo build --release
    Finished `release` profile [optimized] target(s)
$ ls target/release/libthinkthen.*
target/release/libthinkthen.a  target/release/libthinkthen.so
```

The slide, compiled with a plain cc and run exactly as drawn, null then
wire (stub on 8216 at 300 ms):

```
$ cc -std=c11 -Wall -Wextra -I../../contract/include examples/slide.c \
      -o build/slide -Ltarget/release -lthinkthen -Wl,-rpath,.../target/release
$ ENGINE_NULL=1 ./build/slide
slide ok: decide YES at 0.97, decide_many YES NO YES
$ ENGINE_BASE_URL=http://127.0.0.1:8216/v1 ./build/slide
slide ok: decide YES at 0.97, decide_many YES NO YES
$ curl -s http://127.0.0.1:8216/v1/stats
{"connections":4,"max_in_flight":3,"requests":4}
```

The slide's comment says 0.99, the Bash example's real-backend answer; the
stub's rule answers refund evidence with 0.97, and the sample's promise
is the answer class (THINKTHEN_YES), which holds on both. The drawn lines
run verbatim between the two markers in `examples/slide.c`.

The door's null suite (5 tests) and the wire suite (a dead address is the
backend kind, not retryable):

```
$ ENGINE_NULL=1 cargo test --quiet --test door
test result: ok. 5 passed; 0 failed
$ cargo test --quiet --test wire
test result: ok. 1 passed; 0 failed  (3.00s)
```

The conformance slice through ctypes, offline:

```
$ ENGINE_NULL=1 python3 conformance_driver.py
ok  01..06, 08..16, 19  (15 ok)
skip 07, 20  (the backend kind needs the wire or a dead address)
skip 17      (the cache half and the reset need machinery the door does not carry)
skip 18      (the C header exposes no cancel token)
$ echo $?
0
```

The whole check in one command, with the stub up on 8216: `./check.sh`,
ending `slide ok` twice and both suites green.

## What was not run

- No free-threaded hosts, no Windows: the first release is Linux and
  macOS, and only Linux is on this machine.
- The static library is built but not linked into any example; the slide
  loads the shared library.
- No cancel, deadline, or poll behavior: the header does not carry them
  (finding 3).
- `find` and `rank` have no conformance cases yet, so the door's routes
  for them are exercised only by the door suite's own requests.

## The runtime-installer guard

No runtime or tool was installed: cc, cargo, and python3 were already on
the machine. The guard's proof, run once at close:

```
$ grep -c deno ~/.zshrc
0
```

Nothing changed in any rc file.

## 2026-09-21: the two semantic functions, recognize and relate

The library-team brief's C job: `thinkthen_recognize` and
`thinkthen_relate` in the header's out-param shapes, one returned string
each with the one free function, the JSON carrying `source`, `target`,
`probability`, and `strength` natively, offsets counted in code points
with the C byte conversion proven, `relate` refusing past 255 records,
the existing leak check over the two new strings. The stand-in answers
both from the harvest recordings; no paid call, no key.

### The door

`src/lib.rs` gained the two exports, each a pointer-marshalling wrap over
a safe body (`run_recognize`, `run_relate`), plus one `hand_over` that
writes the returned string and its byte length. The spec parses through
the contract's own `Recognize::from_json` / `Relate::from_json`; the door
checks nothing the core already checks. Exports, all `thinkthen_`:

```
$ nm -D --defined-only target/release/libthinkthen.so | awk '$2=="T"{print $3}'
thinkthen_call
thinkthen_decide
thinkthen_decide_many
thinkthen_engine_free
thinkthen_engine_new
thinkthen_error_message
thinkthen_error_retryable
thinkthen_free_string
thinkthen_recognize
thinkthen_relate
```

### The example, as drawn

`examples/recognize.c` runs both deck sections. `cc -std=c11 -Wall
-Wextra`, no warnings:

```
$ cc -std=c11 -Wall -Wextra -I../../contract/include examples/recognize.c \
      -o build/recognize -Ltarget/release -lthinkthen -Wl,-rpath,.../target/release
$ ENGINE_NULL=1 ./build/recognize
recognize: {"entities":[{"id":1,"text":"Maria Chen","kind":"person","start":0,"end":10,"strength":0.98},{"id":2,"text":"Northwind Freight","kind":"organization","start":18,"end":35,"strength":1.0},{"id":3,"text":"Chicago","kind":"place","start":39,"end":46,"strength":0.6693}],"relations":[{"name":"works_for","source":1,"target":2,"probability":1.0}]}
recognize (the deck's located_in rule): refused, no recorded answer for the rule located_in on this text; the recording covers works_for, based_in
offsets: code points 10..20 -> bytes 14..24 -> "Maria Chen"
relate: {"edges":[{"name":"caused_by","source":1,"target":4,"probability":0.94},{"name":"caused_by","source":2,"target":4,"probability":0.94}]}
recognize and relate ok
```

The offsets line is the byte arithmetic: the answer's code points 10..20
become C's byte offsets 14..24 through the one documented conversion, and
`memcmp` proves the slice is the name with the emoji and accent ahead of
it. The deck's `located_in` rule stays pinned: the C01 recording covers
`works_for` and `based_in`, the stand-in refuses an unrecorded rule, and
the example prints the refusal naming what the recording covers.

### The door's null suite, serialized

`cargo test --test door -- --test-threads=1` runs the suite on one thread
because several tests prove "a refusal sends nothing" against the
stand-in's process-global request counter; parallel tests made that
assertion racy (the counter read 10 where 0 was expected). Six tests were
added: the recording's answer, the deck's unrecorded-rule refusal with
the covered rules named, an unrecorded text refused, the emoji offset
proof, the relate answer at the 0.9 bar, and the 255 refusal.

```
$ ENGINE_NULL=1 cargo test --quiet --test door -- --test-threads=1
test result: ok. 11 passed; 0 failed
```

### The conformance slice

`conformance_driver.py` gained the two routes through ctypes. 60 of the
72 cases now pass, up from 16; the recognize and relate cases that were
skipped as unrouted now run, each with the byte-slice proof built into
the recognize route (a case passes only if the answer's code points
slice the name out of the case's text).

```
$ ENGINE_NULL=1 python3 conformance_driver.py  (tail)
ok       70-relate-R02-pickone
skip     71-relate-R03-persubject-10: the per-subject arm shares its input with the pairs arm and the stand-in serves the ruled pairs form; a conformance-data finding for the build team
ok       72-relate-R04-pairs-10
exit 0
```

The remaining skips are the recorded ones: the backend cases (wire or
dead address), the usage-and-cache case (no cache in the door), cancel
(finding 3), and `rank`/`find` (the driver has no routes for them; the
door itself carries both through the JSON door).

### The leak check

The address and leak sanitizers, the instrument this folder already
uses because valgrind is absent, over the example's two new returned
strings:

```
$ clang -std=c11 -Wall -Wextra -fsanitize=address -I../../contract/include \
      examples/recognize.c -o build/recognize_asan -Ltarget/release \
      -lthinkthen -Wl,-rpath,.../target/release
$ ENGINE_NULL=1 ASAN_OPTIONS=detect_leaks=1 ./build/recognize_asan >/dev/null
(no sanitizer output; exit 0)
```

`check.sh` now compiles and runs the example after the slide and runs the
sanitizer pass when `clang` is present. The whole check ends
`recognize and relate ok` and both suites green, exit 0.

### Findings for the contract, not worked around here

4. **The deck and the design page name the free function
   `thinkthen_string_free`; the header and this library name it
   `thinkthen_free_string`.** The deck's recognize snippet cannot link as
   written on that one line; the example calls the real name and the
   divergence is pinned for the deck's owner. Neither page was edited,
   and no alias export was added: the door owns one name per symbol.
5. **The two new functions' header docs say "Returns 0 on success and -1
   on failure"; the header's own return-codes block says one code per
   error kind, and this door returns the kind code (1..6), as the typed
   doors already do.** Returning `-1` would leave the six kinds
   unretrievable, which the acceptance brief rules out; following the
   block keeps the kinds addressable through `thinkthen_error_message`
   and the retryable flag. One line per function should be corrected, or
   a ruling made.
6. **The question file's relation ends are `source`/`target` on the
   design pages, and the contract's spec parser reads `from`/`to` keys**
   (the conformance cases carry `from`/`to` too). The door hands
   `spec_json` to the core parser unchanged — one parser in the core
   checks the shape — so a question file written with `source`/`target`
   is refused as a missing `from` end today. One of the two must change;
   the door will follow the core either way.

### The vocabulary sweep

Over the user-facing strings this lane owns — the door's error messages
(`src/lib.rs` doc comments and messages), the example's prints
(`examples/recognize.c`), the driver's lines (`conformance_driver.py`),
and `README.md`:

```
$ grep -rinE "certainty|likelihood|confidence|accuracy|calibrated|cutoff|gray zone|uncertain" \
      src/lib.rs examples/recognize.c conformance_driver.py README.md
examples/recognize.c:137:    ok &= !contains(out, "\"confidence\"");
```

The single line is an assertion that the returned JSON does NOT contain
the vendor's word — a guard, not a use. The door's messages say `strength`
on names and `probability` on relations; `not sure` stays `UNSURE`; the
restricted and banned words appear nowhere else. `test door` also asserts
`confidence` stays out of both returned shapes.

### The repo-wide check, and one incident to report

`scripts/check_surfaces.sh` (run without per-surface stubs) walks every
surface; the C section runs its steps green, zero FAIL lines, and ends
`== c surface: wire twin skipped, no stub on 8216` as expected in a run
without a stub. The script's overall exit is 1, caused outside this
surface: the R lane's conformance slice carries 44 failures against the
new recognize/relate cases while that lane's runner is mid-landing, and
the known `17-usage-and-cache` divergence (the stand-in holds no disk
cache) prints in the Python section. Neither touches this folder.

**Incident, reported for the orchestrator.** My closing commit
(`692ccbf`) used `git add -A` and swept six of the R lane's in-flight
files into it: `libraries/r/check.sh`, `conformance.R`,
`recognize_check.R`, `thinkthen/DESCRIPTION`, `thinkthen/R/thinkthen.R`,
and `thinkthen/src/rust/src/lib.rs`. The content is intact in both the
tree and history — nothing was lost or rewritten, the branch was not
force-pushed — but the R lane should be told before its next commit that
its earlier work already landed under the C lane's commit message, and it
should commit only what it changes from here. The root cause is mine:
`git add -A` in a shared worktree. The remedy for the rest of this lane
was explicit paths; that is what the commit below uses.

## 2026-09-21 — the rulings wave (languages lane)

Ruling 1 aftermath: `tests/door.rs`'s specs, `examples/recognize.c`, and `conformance_driver.py` use `source`/`target`.

Ruling 2: the `src/lib.rs` unit test constructs a contract `Error` with kind `defect`, runs it through the engine's `fail`, and asserts code 6, the message through `thinkthen_error_message`, and the zero retry signal. `check.sh` now runs `cargo test --lib` beside `--test door`: 1 passed.

Smaller item: `thinkthen_call`'s doc no longer promises a numeric code the door never returns, and `thinkthen_recognize`/`thinkthen_relate` say the kind's code 1..6 rather than `-1` — the header now matches `code_of`.

## 2026-09-21 — the shapes from lane B item 2 (languages lane)

The three shapes `e44d492` landed in `contract/` and `standin/`, carried
into this door, plus the fast-backend deadline proof and the examples
file. Commands and output as they happened.

**`Details.requests` and `failed_questions` (0053, 0054).** The JSON
door's `details` reply carries both beside the trail it already had:

```
$ ENGINE_NULL=1 cargo build --release --quiet && python3 - <<'PY' ...
"requests": ["d3476c41..."], "failed_questions": 0
```

`tests/door.rs` pins one 64-figure digest and the zero count on the audit,
and `the_new_shapes_ride_the_json_door` pins the failed marker and the
record row.

**The failed marker, this host's spelling: the ruled JSON object.** An
`annotate` field whose question failed comes back as
`{"failed":{"kind":"backend","cause":"missing_answer"}}`, never `null`,
while the neighbour's good answer rides beside it:

```
$ ENGINE_NULL=1 ./build/functions   (annotate line for the synthesized record)
{"answer":[{"refund":true,"topic":{"failed":{"kind":"backend","cause":"missing_answer"}}}]}
```

**The record row (go-ahead item 4).** The conformance driver builds the
ruled `{"input", "value"}` objects from the door's own outputs and checks
them where the cases carry rows (`05`, `06`, `19`); no C function was
added.

**Conformance, offline: 73 and 74 run green.**

```
ok       73-details-carries-requests
ok       74-annotate-preserves-good-answers
```

Case 73 checks the audit's identity fields (the null backend's own rule
cannot reproduce its recorded probability); case 74 checks the marker and
its count through the JSON door.

**The fast-backend deadline.** The C header exposes no cancel token and no
deadline (finding 3, above), so `tests/deadline_fast.rs` drives the same
engine the door calls — `thinkthen_standin::BlockingEngine` through the
contract — and proves the shape the door inherits when the header grows a
budget: a one-second deadline spent mid-way through a two-million-record
null batch (about 5.6 s deaf) surfaces with the deadline kind within 1.5 s.

```
$ ENGINE_NULL=1 cargo test --quiet --test deadline_fast
test result: ok. 1 passed   (1.31 s)
```

**The examples file.** `examples.json` holds all ten functions keyed by
name; `examples/functions.c` runs every one of them through the door, and
`examples.py` checks that every snippet appears in the C file verbatim,
compiles it with a plain cc against the built library, runs it offline,
and compares each expected line:

```
$ ENGINE_NULL=1 python3 examples.py
10 of 10 examples ok
```

**Full check, `./check.sh`:** the build, the slide and the recognize
example as drawn, the leak check, the null suite (12 door tests), the
deadline proof, the examples, and the conformance slice — all green; the
wire twin skips with no stub on 8216; exit 0.


## 2026-09-21 — the settle wave

Wired to the contract's settlements (`1fe8173`): the no-verb message now
lists all ten verbs including `filter` and `rank`; the audit JSON carries
`nearest` (null on a decide question, the level's name on a score
question, both asserted in `tests/door.rs`). The recorded cancel/deadline
gaps stand as recorded. `check.sh` exit 0; the door's twelve tests green.

## 2026-09-22 — the C options design (Ian's C ruling)

Ian's ruling: the C door supports everything that can call a C library —
C, C++, Go, Java — and the library team owns the C ABI design, not the
architect. The supervisor approved the A-shape the same day: the drawn
signatures freeze (the C slide is the surface's acceptance sample and
`examples/slide.c` runs it as drawn), and the control spellings sit beside
them. `DESIGN.md` holds the eight decided sections and the rationale.

### The decisions, one line each

1. **Cancellation.** `thinkthen_cancel_token_new`, `thinkthen_cancel`, and
   `thinkthen_cancel_token_free`; every `_opts` spelling carries
   `thinkthen_cancel_token *cancel`; null is no token; one-shot (a fire
   stays fired, a second fire is ignored); the promise is the engine's: no
   new request starts after the fire, sent requests finish, the calls
   return THINKTHEN_ECANCELLED with no results. A C host hears its own
   interrupt by firing from another thread; the fire is one atomic store
   and allocates nothing.
2. **Deadlines.** A flat `long deadline_ms` on every `_opts` spelling:
   THINKTHEN_NO_DEADLINE (-1) sets none, zero is a spent budget (the
   conformance file's own row 27), a positive value is the budget. Flat,
   not a struct, because Go and Java FFI marshal scalars and because no
   struct crosses the ABI. Every plain spelling is exactly its `_opts`
   twin with THINKTHEN_NO_DEADLINE and a null token, and one test proves
   the equivalence on the answer path and the error path.
3. **Partial completion.** No partial rows: a cancelled or
   deadline-expired call returns its code with every out parameter
   untouched, because the engine reports a bulk call as one unit and the
   boundary rule forbids the door from becoming a second scheduler. On
   success the door requires one judgment a record; a short list is a
   defect, never a success code over stale slots.
4. **Null and length inputs.** The full matrix, every row refused with
   the usage code before the engine is asked, so a refusal sends nothing;
   a null engine is the usage code with no message; a count of zero reads
   and writes nothing and accepts null arrays.
5. **Allocated results.** One freer a pointer: strings with
   `thinkthen_free_string`, tokens with `thinkthen_cancel_token_free`,
   engines with `thinkthen_engine_free`; the message is borrowed; caller
   buffers are borrowed for the call and never freed by the door. The
   name stays `thinkthen_free_string` (finding 4: the docs that spell
   `thinkthen_string_free` are corrected by their owners).
6. **Concurrent callers.** Any number of threads over one engine, each
   caller seeing its own answers; the last-failure slot is last-writer-wins
   and the call's own return value is authoritative.
7. **The error surface.** `thinkthen_error_code` returns the last
   failure's code, THINKTHEN_OK before any, unchanged by success; finding
   2 above is closed.
8. **ABI stability at 0.1.** Symbol names, the `thinkthen_answer` layout,
   the codes 0..6, and THINKTHEN_NO_DEADLINE freeze; additions are minor,
   changes are major; `thinkthen_answer` is the one struct that crosses,
   and options are flat scalars.

### What shipped

`contract/include/thinkthen.h`: the token block, the argument-rules block,
`thinkthen_error_code`, the five `_opts` spellings beside the drawn
signatures, and the corrected top block (the old sentence about a poll
callback the host installs is gone; the token is the channel). The
extractor in `scripts/check_public_names.py` learned the token return type
and the nineteen-name set.

`src/lib.rs`: the token type over the contract's `Cancel`, the `control`
helper that turns `(deadline_ms, cancel)` into the engine's `Options`, the
null checks (`str_required`, null engine, null out pointers, null arrays),
the defect guard on a short judgment list, and the last-failure code. The
plain functions are thin calls into their `_opts` twins, so the
equivalence is structural.

`tests/`: `null_matrix.rs` (5 tests, the matrix), `cancel.rs` (3 tests:
fired token over every entry point with nothing sent, one-shot and null
token, a cross-thread fire ending a two-million-record batch),
`concurrency.rs` (four threads, twenty-five calls each, one send a call),
`deadline_fast.rs` rewritten to drive the door (spent budget with nothing
sent, and the poll-bug shape through `deadline_ms`), `door.rs` +2 tests
(the retrievable JSON-door code; the plain/`_opts` equivalence over all
five entry points). `check.sh` runs them all, serialized where a test
reads the process-global counter.

`conformance_driver.py`: routes case 27 through `thinkthen_decide_opts`
with the case's own `budget_ms` and checks that nothing was sent; the
cancel skip now names the real reason (the case's cancel-after-two-replies
timing needs the wire stub; the token's paths live in `tests/cancel.rs`).

### Commands and output, as they happened

The whole surface check, exit 0:

```
$ ./check.sh
slide ok: decide YES at 0.97, decide_many YES NO YES
recognize and relate ok
asan and lsan clean
test result: ok. 14 passed; 0 failed   (door, with --test-threads=1)
test result: ok. 5 passed; 0 failed    (null_matrix)
test result: ok. 3 passed; 0 failed    (cancel, 0.51 s)
test result: ok. 2 passed; 0 failed    (deadline_fast, 1.43 s)
test result: ok. 1 passed; 0 failed    (concurrency)
10 of 10 examples ok
conformance slice: 63 ok, 11 skip, 0 fail
== c surface: wire twin skipped, no stub on 8216
```

The deadline case, and the two skips that remain on this surface:

```
ok       27-deadline-spent-budget: spent budget refused, nothing sent
skip     18-cancel-mid-batch: the mid-batch cancel needs the wire stub's
         reply timing; tests/cancel.rs proves the token's pre-fired and
         mid-batch paths through the door
```

The public names, from the repo root:

```
$ python3 scripts/check_public_names.py | grep "c:"
ok c: 19 names, all ruled or documented
```

The four-thread proof asserts what the counter shows: seventy-five sends
for seventy-five answering calls, none lost and none doubled, with the
failing thread's twenty-five refusals sending nothing.

### What was not run

- No wire stub on 8216 in this lane's run, so the wire twin skipped; the
  backend-kind test still runs alone with a dead address (`cargo test
  --test wire`, without `ENGINE_NULL`).
- Conformance case 18 stays a skip here, for the timing reason above.
- No Windows: the first release is Linux and macOS.
- The examples and the drawn slide are unchanged; the plain signatures
  they call did not move, which is the point of the A-shape.

## 2026-09-22 — the review fix wave (lane C)

The review's group 2 entries for this door, each fix with the test that
failed before it, plus the cross-side repair the stand-in's fixture
opt-in demanded.

**The per-thread error slot.** The old slot was one `Mutex` on the engine,
last-writer-wins, while the header promised both any number of threads and
a message that lives until the next call. A second thread's failure freed
the first thread's message mid-read. The slot is now thread-local
(`LAST_FAILURE`, keyed by the engine pointer): `fail` replaces only the
calling thread's entry, the three error accessors read only the calling
thread's entry, `thinkthen_engine_free` and `thinkthen_engine_new` forget
the calling thread's entry so a reused address starts clean, and the entry
drops at thread exit. The header's two promise spots and `DESIGN.md`'s
sections 5, 6, 7, and the deferred list now state that truth.

The repro, against the pre-fix library (the fix stashed, the new tests
kept), is the review's own: a saved message pointer read after the other
thread failed.

```
$ clang -fsanitize=address ... tests/error_threads.c && ./build/error_threads_prefix
==ERROR: AddressSanitizer: heap-use-after-free ... READ of size 2 ... thread T1
    #1 ... in strstr
freed by thread T2 here:
    #1 ... in thinkthen::thinkthen_engine::fail
previously allocated by thread T1 here:
    #3 ... in thinkthen::thinkthen_engine::fail

$ ENGINE_NULL=1 cargo test --test error_threads        # against the pre-fix library
two_threads_read_their_own_messages --- FAILED
```

With the fix, `tests/error_threads.c` prints `ok two threads read their
own messages, none crossed` under ASan and LSan, and
`tests/error_threads.rs` passes. `tests/concurrency.rs`'s four threads
each read their own code and message; the main thread, which recorded no
failure, reads the no-failure text, where the old test asserted the last
writer's failure on the main thread — that assertion was the shared-slot
bug written down.

**The panic guard.** Every exported symbol now runs its body behind
`guard`: `catch_unwind` catches a panic from anywhere beneath the door,
records the defect kind on the calling thread's slot with the panic's own
text, and returns the symbol's fallback (a code, null, or the defect
text), so a panic never aborts the host. The unit test
`a_panic_behind_the_door_comes_back_as_the_defect_kind` forces a panic
through the guard and reads code 6 and the recorded text back. A test hook
that panics was rejected: it would be test code in the shipped library,
the same defect finding 7 removed from the stand-in.

**The checked deadline.** `control()` now converts a nonnegative
`deadline_ms` through the contract's `deadline_from_millis`, so a budget
larger than the engine holds is the usage kind before anything is sent.
Before this, `deadline_ms = i64::MAX` overflowed the instant and panicked
inside the host process — the C door's share of the review's "large
deadline crashes the host" finding.
`an_impossible_budget_is_refused_not_a_panic` pins it (usage code, the
`larger than` message, nothing sent, nothing written).

**The cross-side repair.** The stand-in's annotate fixture is now armed
only by `ENGINE_SYNTHETIC_PARTIAL=1` (standin commit `7acb3da`), and the C
gate replayed that record in `the_new_shapes_ride_the_json_door` and in
conformance case 74. Without the opt-in the door suite failed at HEAD;
`check.sh` now exports the opt-in for the door suite and the conformance
driver, with a comment naming the reason. Verified: without the opt-in
exactly one door test fails (the fixture's), with it all fourteen pass.

**Commands and output.**

```
$ ./check.sh                                        # exit 0
asan and lsan clean
ok       two threads read their own messages, none crossed
lib: 3 passed; door: 14 passed; null_matrix: 5 passed; cancel: 3 passed;
deadline_fast: 3 passed; conformance: 1 passed; error_threads: 1 passed;
fork: 1 passed; examples and the drawn slide unchanged; wire twin skipped
(no stub on 8216)
conformance slice: 74-annotate-preserves-good-answers ok, 68 green lines
```

The four pre-existing `unsafe_op_in_unsafe_fn` warnings in
`tests/deadline_fast.rs` and `tests/door.rs` helpers are unchanged from
HEAD; this lane added none.

**The Rust surface.** `libraries/rust` has no FFI boundary and no
`from_secs_f64` of its own (grep over the crate: no matches), so item 3's
two clauses resolve to nothing to change there; its notes carry the grep.
It needed the same fixture opt-in repair as this gate — `tests/verbs.rs`
and conformance case 74 replay the armed record — and its `check.sh` is
green with it.

**Known corner.** The TLS slot is keyed by the engine pointer. A thread
that recorded a failure keeps its message string after another thread
frees the engine; if a new engine takes that address and is used on that
thread, the stale entry could be read as the new engine's last failure.
Bounded to one message string per thread, cleared on that thread's own
free and new, and stated here rather than hidden.

**2026-09-22 (contract lane).** The fixture door moved from
`ENGINE_SYNTHETIC_PARTIAL` to the stand-in's `synthetic-partial` cargo
feature; this gate forwards it (`synthetic-partial =
["thinkthen-standin/synthetic-partial"]`), builds `--features
synthetic-partial` for the door shape test and the conformance slice, and
rebuilds the production shape after. The inline annotate dialect, the
`functions.c` example, and `examples.json` carry `"version": 1` now, and
annotate columns come back in file order, so the example's expected line
follows.

**2026-09-22 (c lane, review wave 3).** Items 1, 3, 4, 5, 6, 7, and 16 of
the second review, one wave:

- **The failure table moved into the engine (items 1 and 16).** The old
  one-slot-per-thread table keyed by the engine pointer broke two ways: a
  failure on one engine erased another engine's entry (the first engine
  then read `no failure yet`), and a freed engine's entry survived on
  threads that did not free it, so a new engine at the reused address
  inherited the dead engine's message. The table now lives inside
  `thinkthen_engine`, keyed by the recording thread; `fail`, the three
  error readers, and `thinkthen_engine_free` all go through it, and the
  thread-local (`LAST_FAILURE`) and `forget_thread_failure` are gone.
  `tests/error_engines.rs` is new and pins both edges. Against the pre-fix
  code (a scratch worktree at `f532e9e`, the same test file copied in,
  removed after):

  ```
  $ ENGINE_NULL=1 cargo test --test error_engines -- --test-threads=1
  assertion `left == right` failed: the first engine still reports its own failure
    left: 0
   right: 1
  assertion `left == right` failed: a new engine at the old address starts with no failure
    left: 1
   right: 0
  test result: FAILED. 0 passed; 2 failed
  ```

- **The teardown abort (items 1 and 7).** glibc runs thread-local
  destructors before the atexit handlers, so the old error path's
  `LAST_FAILURE.with` panicked twice — once inside the body (caught by
  `catch_unwind`), once in the guard's own error arm (not caught) — and a
  panic escaping `extern "C"` aborts the host. The door now runs the
  contract's shared guard (`catch_panic`), which is also item 7: the local
  `catch_unwind` wrapper and `panic_text` are deleted, and the recorded
  message now reads `a panic crossed the C door: ...`. `tests/atexit_free.c`
  is new, registers `thinkthen_engine_free` with `atexit`, and `check.sh`
  runs it:

  ```
  $ ENGINE_NULL=1 ./build/atexit_free; echo $?          # at HEAD
  0
  # the same program against the pre-fix library:
  panicked ... cannot access a Thread Local Storage value during or after destruction
  panicked ... panic in a function that cannot unwind
  thread caused non-unwinding panic. aborting.
  thinkthen_engine_free
  free_at_exit
  __run_exit_handlers
  exit 134
  ```

  The unit test `a_panic_behind_the_door_comes_back_as_the_defect_kind`
  also pins the shared boundary's own spelling now.

- **The deadline sentinel (item 3).** `control()` dropped its
  `deadline_ms < 0` early return and passes every value through the
  contract's `deadline_from_millis`; only `THINKTHEN_NO_DEADLINE` (-1)
  yields no deadline. Zero stays a spent budget; `-2` and `i64::MIN`
  refuse with the usage kind before anything is sent.
  `tests/deadline_fast.rs` gained
  `every_other_negative_budget_refuses_before_the_wire` (pre-fix: 3 passed,
  1 failed, `a negative budget is the usage kind: no failure yet`, because
  `-2` answered), and the unit test
  `only_the_sentinel_means_no_deadline` covers the conversion table.

- **`details: false` (item 4).** The JSON door reaches the audit view
  only when `details` is exactly `true`; the old `contains_key("details")`
  turned the view on for `false`. The door suite asserts the plain decide
  shape and the absence of every audit field (pre-fix it failed with the
  full audit JSON printed).

- **The header (item 5).** `unsigned long` lengths and counts became
  `size_t`, `long` budgets became `int64_t`, `<stddef.h>`/`<stdint.h>` are
  included, and `THINKTHEN_NO_DEADLINE` is `INT64_C(-1)`; a C++ syntax
  check (`g++ -std=c++17 -fsyntax-only`) passes. The `.pc` promise is
  removed: no install tree ships in 0.1, so a `.pc` would name a prefix
  that does not exist (DESIGN.md section 8). SONAME: none for 0.1,
  recorded in section 8 — `readelf -d libthinkthen.so` shows no SONAME,
  and the release ships as an archive linked by path. The ctypes driver's
  deadline slot moved to `c_int64`, and the examples' length arrays to
  `size_t`.

- **The connector (item 6).** `thinkthen_engine` holds `Arc<dyn Engine>`
  built by `StandinConnector.connect(&EngineConfig::from_env())`;
  `BlockingEngine` is gone from the crate, and the stand-in is named
  exactly once below its `use`:

  ```
  $ grep -rn "BlockingEngine\|StandinConnector" libraries/c libraries/rust --include=*.rs
  libraries/c/src/lib.rs:53:use thinkthen_standin::StandinConnector;
  libraries/c/src/lib.rs:274:        let Ok(engine) = StandinConnector.connect(&EngineConfig::from_env()) else {
  libraries/rust/src/lib.rs:40:use thinkthen_standin::StandinConnector;
  libraries/rust/src/lib.rs:350:    StandinConnector.connect(config)
  ```

  `thinkthen_engine_new` returns null when the connector refuses the
  environment (the stand-in never refuses); the header's comment says so.

- **Commands.** `./check.sh` exits 0: lib 5, door 14, null matrix 5,
  cancel 3, deadline_fast 4, concurrency 1, error_threads 1, error_engines
  2, fork 1, the drawn slide and the recognize and relate examples as
  drawn, the ASan error-thread repro clean, the atexit probe 0,
  conformance 84 green (the wire twin skipped, no stub on 8216).
