# 0496: Make Python, pandas and Python Polars thin and first-class

Status: OPEN.

Milestone: 0.2

Depends on: 0511
Depends on: 0513

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

## Outcome

A Python caller has one obvious way in. Named calls take ordinary Python values and data frames, and return Rust-owned typed results that print, compare, convert and pickle like ordinary Python objects. Async calls keep the event loop responsive and cancel cleanly. Rust owns every rule; Python keeps only naming, conversion, scheduling and cleanup.

## Evidence

- Starts from: the [2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md) and the [0521 assessment](../records/0521-surface-contract-assessment.md). Python has five entry points. `Call` has no repr, equality, pickling or `to_dict`, and `bool(call)` raises. The hand-copied reader in `libraries/python/thinkthen/_complete.py` lacked facts fields added by 0461 and 0468; 0520 repaired it as a narrow bridge. `libraries/python/src/worker.rs` releases the interpreter while it waits on the calling thread, which does not prove an asyncio loop stays responsive.
- Keeps: Native engine handles, mapping access, data frame row identity, null masks and whole-set semantics. All ten functions and their input, result, error, cache and replay behavior. Missing stays distinct from null, and permitted unknown result fields are tolerated.
- Changes: Meet the caller acceptance and the Python, pandas and Python Polars sections of `../../libraries/BINDING-AUTHOR.md`. This ticket owns:
  - the Python target template and generated `.pyi` stubs from 0513's common graph, describing the actual installed objects;
  - conversion of Python values and frames into the shared Request, with no copied validation or cache logic;
  - typed exceptions for top-level failures;
  - result behavior: printing, equality, mapping conversion and pickling preserve presence and failures and hold no engine handle;
  - truth values: `bool` of a successful value follows the ordinary Python value, including False, None, zero and empty collections; embedded failures keep their failure kind and facts and never become False or None; the reviewed result design defines the truth behavior of any separate failure carrier;
  - asyncio execution, task cancellation and context-manager cleanup;
  - the package README, with a short old-to-new call mapping;
  - removal of the extra entry points and copied readers after installed parity.
  One public API is one coherent family of named typed calls. Frames issue whole-set calls, never per-row aggregate calls. Claim `libraries/python/**` and its installed typed consumer cases, narrowed per slice before coding.
- Proof: The full shared cases run through the installed wheel's typed interface, including files and images, context and options, original positions, facts, failures and invalid input with zero sends. One installed held-provider case in the existing runner shows another coroutine progressing, cancellation stopping further reads and submissions, and cleanup returning before the provider is released. Pending final facts stay pending. Raw JSON pass-through does not count. Record handwritten code removed and added, counting generator templates, in the landing record.
- Defers: Rust Polars goes to 0527. The proxy and platform ruling changes need no 0.2 ticket.

### Retained aggregate repair

Closure also fixes the aggregate overwrite found during 0526 installed checks: `thinkthen/_calls.py` replaces `self.results` for every aggregate packet. Recognize emits incremental nonempty aggregates followed by a possibly empty remainder, so this drops earlier answers. Preserve every chunk and the completed prefix on failure, following the native session contract. Keep a small installed recognize case that yields nonempty output; do not change the shared expected answer.

## Progress

- 2026-10-10 landed 5959fcd40d3eda08d3cdf14b5b8b6ae87eacca9b; next: Python producer cleanup now preserves active native failures, completed rows and facts while standalone cleanup errors remain visible. Six red/green cases, three retained cases and fresh review pass in shared 0498 slice E. Current installed-wheel parity and platform qualification remain held.
