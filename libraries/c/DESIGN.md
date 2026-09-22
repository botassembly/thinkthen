# The C door's options and ownership design

Decided 2026-09-22 by the library team, under Ian's ruling that the C door
serves every language that can call a C library — C, C++, Go, Java — and
that the library team owns this design. It settles the punch list's line
"C needs an accepted options/ownership design covering cancellation,
deadlines, partial completion, null/length inputs, allocated results, and
concurrent callers before its ABI freezes"
(`sdlc/planning/library-team-architecture-punch-list.md`).

The shape that stays: one engine value built from the environment; the JSON
door carries any request; the typed doors carry the hot paths; every result
of open size crosses as JSON text. The design behind the option arguments
is the A-shape approved on 2026-09-22: the drawn signatures freeze and the
control spellings sit beside them, because the C slide is the surface's
acceptance sample and it must keep compiling as drawn.

## 1. Cancellation

**Decision.** The door owns a token handle: `thinkthen_cancel_token_new`
creates it, `thinkthen_cancel(token)` fires it, and
`thinkthen_cancel_token_free` frees it. Every `_opts` spelling carries a
`thinkthen_cancel_token *cancel` argument; a null token means no cancel.
A fired token is one-shot: it stays fired, a second fire is ignored, and no
call re-arms it, so one token can stop many calls. The promise is the
engine's own, no larger: no new request starts after the fire, requests
already sent finish, and the calls that carried the token return
THINKTHEN_ECANCELLED with no results.

**Why.** C has no standard interrupt channel, and every host that binds
this door has threads. The token is one pointer and one atomic store, so Go,
Java, and C++ hosts fire it from whatever thread already receives their
stop gesture; a POSIX signal handler can fire it too, because the fire
allocates nothing. The poll callback from ADR 0017 belongs to the binding,
and for C that binding is the host program: a door-supplied callback would
need one dispatch per host language, and none of the three ruled consumers
needs it. The callback stays deferred (see the end of this page).

## 2. Deadlines

**Decision.** Every `_opts` spelling takes a flat `long deadline_ms`
argument, beside the token. THINKTHEN_NO_DEADLINE (-1) sets none; zero is
a spent budget, exactly as the conformance file's own row
(`27-deadline-spent-budget`, `budget_ms: 0`) rules, so the call refuses
before anything is sent with THINKTHEN_EDEADLINE; a positive value is that
many milliseconds from the call. The engine's own tick check enforces it,
so a budget spent mid-batch ends the wait within one tick. Every plain
spelling is exactly its `_opts` twin called with THINKTHEN_NO_DEADLINE and
a null token; the header says so in one sentence and
`the_plain_call_is_its_opts_twin` proves it on the answer path and the
error path alike.

**Why flat arguments, not an options struct.** Go's cgo and Java's FFI
marshal scalars and pointers directly and struct-by-value awkwardly; C++
can wrap two flat arguments in its own default-argument overload without
the door freezing a layout. A struct would also cross the ABI (section 8),
and the drawn slide calls keep the four plain signatures, so the control
arguments arrive on the `_opts` twins.

## 3. Partial completion

**Decision.** A cancelled or deadline-expired call returns its code and no
results. Every out parameter holds what it held before the call; the door
writes no rows. A host that wants "everything that arrived" asks with no
token and batches to its own taste. Re-asking a cancelled call is safe: a
judgment is a function of the question and the evidence, and the engine's
cache answers a repeat without a new send (an engine property, not a door
promise).

**Why no partial rows.** The engine reports a bulk call as one unit: its
settled contract stops the wait, discards the completed judgments, and
returns the cancelled or deadline kind. The boundary rule forbids the door
from becoming a second scheduler, so it does not chunk the call itself to
fake partial delivery. Partial rows would also need a count the header
does not carry, and a half-filled caller array under a failure code is
exactly the ambiguity the failure rule removes.

**One guard.** On success the door requires one judgment a record; a short
list is THINKTHEN_EDEFECT with every slot untouched, never a success code
over stale slots.

**Ownership.** The `out` array is the caller's from before the call to
after it. On success the door writes every slot once, in input order; on
any failure it writes none.

**Deferred.** Partial rows arrive only with an engine capability that
exposes the completed judgments of a stopped call, which does not exist
today.

## 4. Null and length inputs

**Decision.** The door checks its pointers before it asks the engine, so a
refusal sends nothing:

- `engine` null: THINKTHEN_EUSAGE (NULL from `thinkthen_call` and its
  `_opts` twin), with no message, because no engine holds one.
- `question_json`, `request_json`, `spec_json` null or not UTF-8:
  THINKTHEN_EUSAGE. These strings are NUL-terminated and carry no length.
- `text` null with `text_len` zero: the empty text, and the engine's own
  blank-evidence rule refuses it with the usage kind. `text` null with a
  nonzero length: THINKTHEN_EUSAGE. A non-null `text` reads exactly
  `text_len` bytes; no terminator is read.
- `texts` and `lengths`: read for `count` entries. Null is
  THINKTHEN_EUSAGE when `count` is nonzero; a count of zero reads nothing,
  so null is accepted and the engine's own empty rule answers.
- The bulk `out` array: written for `count` entries, so null is
  THINKTHEN_EUSAGE when `count` is nonzero, and a count of zero accepts
  null because it writes nothing.
- `out` and `out_len` of recognize and relate, and `out` of decide: always
  written on success, so null is THINKTHEN_EUSAGE.
- A cancel token: null means no token, at every `_opts` spelling.

**Why.** Every ruled consumer passes host-owned buffers, and each host
spells "missing" differently: Go passes a nil slice, Java a null reference,
C++ a null pointer. A crash on any of those would be the door's defect, not
the host's. The matrix gives each one a code and a message instead, and the
refusal-before-the-engine rule keeps "nothing was sent" true for every row.
`tests/null_matrix.rs` pins the matrix row by row.

## 5. Allocated results

**Decision.** One freer per returned pointer, and the door never frees
what it did not allocate:

- A string from `thinkthen_call`, `thinkthen_recognize`, or
  `thinkthen_relate` (or their `_opts` twins) is freed with
  `thinkthen_free_string`, exactly once.
- The token from `thinkthen_cancel_token_new` is freed with
  `thinkthen_cancel_token_free`, after every call that carried it has
  returned.
- The engine from `thinkthen_engine_new` is freed with
  `thinkthen_engine_free`, after every call on it has returned.
- The message from `thinkthen_error_message` is borrowed: valid until the
  next call on the same engine, never freed by the host.
- Every buffer the caller passes in is borrowed for the call, never
  retained, and never freed by the door. The door writes only the out
  parameters the header names.

**Why.** The caller owns every lifetime, and each rule is one sentence. The
free function's shipped name is `thinkthen_free_string`; the deck and one
design page spell it `thinkthen_string_free`, and their owners correct the
line. The door owns one name per symbol and adds no alias.

## 6. Concurrent callers

**Decision.** Any number of threads may call on one engine at once, and
each caller sees the answers it would get alone. The host must not free the
engine or a token while a call that uses it is in flight. The door's only
shared mutable state is the last-failure slot, which is guarded and
last-writer-wins: `thinkthen_error_code`, `thinkthen_error_message`, and
`thinkthen_error_retryable` name the most recent failure any thread
recorded, and the failing call's own return value is that call's
authoritative code.

**Proof.** `tests/concurrency.rs` runs four threads over one engine,
twenty-five calls each: the plain typed call, its `_opts` twin, the JSON
door, and
a failing caller. Every thread asserts its own answers and codes, and the
engine's counter shows one send a call, seventy five in all, none lost and
none doubled. `tests/cancel.rs` fires a token from a second thread while a
two-million-record batch runs and watches the call end within a tick.

## 7. The error surface

**Decision.** `thinkthen_error_code(const thinkthen_engine *)` returns the
code of the last failure on the engine, THINKTHEN_OK before any failure.
Success does not clear it. A null engine returns THINKTHEN_EUSAGE, because
no engine holds a failure.

**Why.** The JSON door returns NULL, so before this function a host could
read the message and the retryable flag but never the kind (NOTES finding
2). Every ruled consumer wants a number to switch on: a Go or Java binding
cannot decide a fallback from a message string. The typed doors keep
returning their code directly, and the three functions together are the
whole error surface: code, message, retry signal.

## 8. ABI stability

**Decision.** Version 0.1.0 freezes:

- the symbol names (the nineteen in the header, held by
  `scripts/check_public_names.py`),
- the `thinkthen_answer` layout: `int` then `double`, `repr(C)`, never a
  third field,
- the return codes 0 through 6, with new kinds appended and old ones never
  renumbered,
- THINKTHEN_NO_DEADLINE, and the `thinkthen_` prefix on every name.

Adding a symbol or a code is a minor bump. Changing a signature, a struct
layout, or an existing code is a major bump. No symbol is removed without
a major bump. The version macros in the header carry the version at compile
time; a runtime version accessor is deferred, because no consumer asked for
one and the header and the `.pc` file answer the same question.

**Why no structs cross.** Every open-shaped result crosses as JSON text, so
a new field never changes a layout. `thinkthen_answer` is the one struct,
and it is two fixed fields — which is what lets Go (a cgo struct), Java (two
fields in a memory layout), and C++ (a trivially copyable value) read it
without glue. Options are flat scalars for the same reason, so the door has
no options struct to grow.

## Deliberately deferred

- **The poll callback.** ADR 0017 gives the binding the callback that hosts
  run their interrupts in. For C the binding is the host program itself, and
  the token covers the stop gesture from another thread; a door-supplied
  callback would need one dispatch per consumer language. The header's old
  sentence about an installed callback was corrected to say what ships.
- **Partial rows.** They need an engine capability (section 3).
- **Per-thread error slots.** The header promises one slot per engine, and
  the last-writer-wins rule is stated instead of a second mechanism.
- **Windows.** The first release is Linux and macOS, as the packaging notes
  say; the design has no Windows-specific shape.
