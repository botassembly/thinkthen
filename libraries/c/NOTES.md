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
