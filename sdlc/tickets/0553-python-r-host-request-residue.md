# 0553: Move the remaining Python and R request logic into Rust

Status: OPEN.

Milestone: 0.2

Depends on: 0551

Reviews: revision f23c075965dc12a966847fee80c13bfab3b29385, accept

## Outcome

Python and R only convert host values and decode results. Rust decides the question shape, label rules, whether an input is a scalar or a collection, and the column view. The finish line that 0534 and 0535 set for these two languages is met.

## Evidence

- Starts from: the [2026-10-11 adversarial review](../records/2026-10-11-0-2-adversarial-review.md), finding B10.
  - Python builds definition bodies in `thinkthen/_calls.py:29-64`. It repeats Rust label checks in `_labels.py:13-28,63-68` and adds host usage errors in `__init__.py:201,231`. The `on=` column checks at `__init__.py:31-43` stay in Python.
  - R picks scalar or collection from a hard-coded function list (`R/requests.R:37`). It sends native column positions back as attributes (lines 46-47) and fills missing slots for each function in `.tt_column_view` (lines 166-185).
  - R grows feed results quadratically with `c(results, value)` (`R/feeds.R:29`).
  - R splits errors on `\x1f` and signals itself (`thinkthen.R:76-113`). JavaScript parses JSON out of error messages (`native.js:25`).
- Keeps: every public Python and R call and its documented result.
- Changes:
  - Move question building, label checks and the column view behind the Rust request and column APIs.
  - Raise typed errors from the native glue, as Ruby does in `ruby/src/ffi.rs:54-97`.
  - Drop Python and R from the AGENTS.md line that keeps the handwritten complete readers.
  - Build feed results without quadratic growth.
- Proof:
  - The Python and R routine installed checks pass.
  - A search finds no function-name list in Python or R host code.
  - Each language's host source line count falls.
- Defers: nothing.
