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
3. **The C header exposes no cancel token, no deadline, and no poll
   callback.** The ADR says the poll callback belongs to the binding, but
   the C door gives a host no way to install one, so a C program cannot
   stop a bulk call or cap one with a budget. Conformance case 18
   (cancel) cannot run on this surface for that reason.

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
