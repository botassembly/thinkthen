# Quick Fix: guarded review JSONL cleanup

Candidate branch: `ticket/qf-triage-cleanup`, based on
`ffe21b48c6b1b76e40827b37e03ce3987c0da0b7` and refreshed with main
`4670a902ae55eeed5a14e1bac87f6be90aa7244c` after the first review.
The changed paths are the demo 16 review script, its existing self-test and
README, a small demo-local publication helper, `scratch.sh`, this record,
and the reported issue. No runtime, SQL, site, or policy table changed.

## Behavior and proof

Before this fix, `review-jsonl` reserved the caller's output with `mkdir`
and used an unguarded `rm -rf` on that output after failures. Focused
`scratch_lint` failed on that line. It now stages the complete two-file result
in a same-parent directory made by `scratch_dir`; the helper's exit trap
cleans only that owned directory after failures. It publishes only after
annotate returns its expected exit 7, the six schemas validate, and both
transforms finish. The public result is one complete directory or no result.

The first candidate used GNU `mv -Tn`. Fresh High review found that Apple's
`mv` has no `-T` and that plain `mv -n` can treat an occupied destination as
a container. The corrected demo-local Python 3 helper calls the operating
system's atomic no-replace rename: Linux `renameat2` with
`RENAME_NOREPLACE`, or macOS `renamex_np` with `RENAME_EXCL`. It returns exit 2
on an occupied output and fails closed on an unsupported operation. Python 3
is now an explicit README prerequisite for this review route. The operation
is same-parent and has no cross-filesystem copy fallback.

The first candidate also rejected ordinary spaces in `scratch_dir` after
`mktemp` had already created a folder, leaking it before trap registration.
The shared guard now permits ASCII spaces in its newline-delimited ownership
list and checks an explicit unsafe template before creation. Newlines, glob
characters, other unsafe delimiters, the lane path, and foreign paths remain
refused. If an unexpected path fails the post-canonical guard, the helper
leaves it untouched even if it is empty; a rare scratch leak is safer than
deleting an unproven path.

Before either changed cleanup first ran, a planted lane path was passed to
`scratch_remove`; it refused the path and left it intact. The Linux first
plant also retained its `caller-data` file. The corrected candidate repeated
the lane refusal on Linux and M5 before exercising cleanup.
The next High review caught a remaining post-canonical `rmdir` before
ownership registration. The new focused plant makes `mktemp` return an
empty, owned stand-in for the current directory. Before the correction the
plant failed because `scratch_dir` removed that directory; after removing
`rmdir`, the guard refuses it and the directory survives. This plant runs
before the corrected cleanup's first real run. The existing caller-data and
foreign-path guard plants remain.

| Focused check | Result |
| --- | --- |
| `bash -n` on the review script and self-test; `sh -n` on `scratch.sh`; Python helper syntax | Passed |
| Focused `scratch_lint` after removing the recursive cleanup | Passed |
| Empty stand-in PWD returned by planted `mktemp` before real cleanup | Initial red: the post-canonical `rmdir` deleted it; corrected: refusal at exit 1 and directory survives |
| Demo 16 `self-test` with the warm Linux binary and saved recording | Passed: five decisions, one review, exact refusal statuses, caller bytes, race, cleanup, and successful and failed paths with spaces |
| Demo 16 executable `README.md` with the absolute warm binary on `PATH` | 3 passed, 2 skipped |
| M5 macOS 26.4, `/usr/bin/python3` 3.9.6, native `renamex_np` helper | Passed: same-parent atomic success, raced file/directory/symlink refusals at exit 2, caller bytes, space path, lane guard refusal, and failed-run scratch cleanup |
| `policy.py`, `pages`, `tickets`, syntax, focused `scratch_lint`, and diff checks after the final guard correction | Passed; the first policy attempt inherited `sccache` and failed before checking packages, then passed with `RUSTC_WRAPPER` cleared |

The M5 proof copied only the candidate Python publisher and scratch helper
into a fresh `/tmp/triage-publish.CtakHv` directory and removed that exact
temporary directory after verification. M5 had no compiled `thinkthen` on
`PATH` or in its checked repository build path, so the M5 result qualifies
the publication and cleanup primitives, not a full saved-recording replay.
The Linux demo self-test supplies the full replay. Neither run called a
provider. No SDK install, container, SQL/frame validation, or full
test/spec/surfaces rung ran.

The self-test wrapper only forces annotate exit 67 or pauses before the
publication race. Success uses the real compiled replay. Existing triage
request-disclosure, validation, move, and signal cases remain. No test was
retired. The old self-test did not exercise `review-jsonl`, macOS publishing,
or space paths; the new cases pin those previously missed boundaries.

## What the build taught us

- The standalone review route inherited an unsafe output cleanup pattern
  from the original triage demo. A guarded same-parent stage gives it a
  complete publication boundary without recursively deleting caller output.
- The first candidate's GNU `mv -Tn` was Linux-only. The High review's
  macOS counterexample changed publication to the native exclusive rename
  on both hosts; a plain `mv -n` would not preserve the directory boundary.
- `scratch_dir`'s ownership delimiter is a newline, so ordinary spaces can
  be accepted without admitting a newline or shell metacharacter. Checking
  an unsafe template before `mktemp` avoids the leak found by review.
- A path returned by `mktemp` has not yet passed this run's ownership guard.
  The final High review found that `rmdir` on a post-canonical refusal could
  delete an empty lane or foreign replacement. The bounded PWD plant proved
  the regression and the correction; the guard now leaves that path alone.
- The first page check used a demo-relative `PATH` and could not find the
  binary. The absolute warm build path passed. M5 lacked that binary, so its
  focused primitive proof cannot be reported as an M5 demo replay.

## Integration limit

The issue remains open until the coordinator integrates the reviewed
candidate and runs one named full lint checkpoint on main after 0275's
separate hidden-package campaign. Focused lint, policy, pages, tickets, and
diff checks are candidate evidence only. The
coordinator can decide whether an M5 full replay is needed after arranging a
source-compatible binary; it was not available in this focused proof.

## Independent acceptance and integration

Fresh independent High review accepted `da126032`. The coordinator integrated the unchanged cleanup and publisher and checked the recursive-cleanup guard on main. The full lint criterion remains open: issue0275 separates the hidden package campaign, and the separately filed port child-environment findings remain. No whole-tree lint pass is claimed.
