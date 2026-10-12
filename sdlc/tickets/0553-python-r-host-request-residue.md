# 0553: Move the remaining Python and R request logic into Rust

Status: OPEN.

Milestone: 0.2

Depends on: 0551

Reviews: revision f23c075965dc12a966847fee80c13bfab3b29385, accept

Reviews: revision 59bd6ddb93bd23234eb6e4b11c692b6ec5f90dc9, reject

Reviews: revision 92301107709327a1ae85b4fe8453361af6dc8059, reject

Reviews: revision aae01270d365676386c2b195dccbd51bb584d291, accept

Reviews: revision 6ce71b8b9f22bb2f36a8985cecc5a921c75b6aef, accept

Reviews: revision 6712593, accept

## Outcome

Python and R only convert host values and decode results. Rust decides the question shape, label rules, whether an input is a scalar or a collection, and the column view. The finish line that 0534 and 0535 set for these two languages is met.

## Evidence

- Starts from: the [2026-10-11 adversarial review](../records/2026-10-11-0-2-adversarial-review.md), finding B10.
  - Python builds definition bodies in `thinkthen/_calls.py:29-64`. It repeats Rust label checks in `_labels.py:13-28,63-68` and adds host usage errors in `__init__.py:201,231`. The `on=` column checks at `__init__.py:31-43` stay in Python.
  - R picks scalar or collection from a hard-coded function list (`R/requests.R:37`). It sends native column positions back as attributes (lines 46-47) and fills missing slots for each function in `.tt_column_view` (lines 166-185).
  - R grows feed results quadratically with `c(results, value)` (`R/feeds.R:29`).
  - R splits errors on `\x1f` and signals itself (`thinkthen.R:76-113`). JavaScript parses JSON out of error messages (`native.js:25`).
- Keeps: every public Python and R call and its documented result.
- Changes: Python and R request logic moves into Rust.
  - Move question building, label checks and the column view behind the Rust request and column APIs.
  - Raise typed errors from the native glue, as Ruby does in `ruby/src/ffi.rs:54-97`.
  - Drop Python and R from the AGENTS.md line that keeps the handwritten complete readers.
  - Build feed results without quadratic growth.
- Proof: these checks.
  - The Python and R routine installed checks pass.
  - A search finds no function-name list in Python or R host code.
  - Each language's host source line count falls.
- Defers: nothing.

## Progress

- 2026-10-11 started
- 2026-10-11 next: Final Python, R and JavaScript clippy passed, installed Python and R routine checks passed, and the owned JavaScript archive run is still awaiting exit before its results are read.
- 2026-10-11 next: Final Python, R and JavaScript clippy and installed routine checks passed; whole-ticket checks and landing await the approved dependency order.
- 2026-10-11 next: Fresh source search found R explicit selectors still interpreting serde diagnostic text; pushed checkpoint 8cf307a87 uses the shared typed decoder and adds exact context refusal and zero-send checks, with core measured 187627 against unchanged 187611 and compiler plus installed proof held behind the canonical campaign.
- 2026-10-12 next: Shared typed decoding and restored binding assertions are integrated in frozen 6712593f6 with matching limits, passing mandatory gates and fresh review acceptance; the combined rc.12 campaign precedes landing.
