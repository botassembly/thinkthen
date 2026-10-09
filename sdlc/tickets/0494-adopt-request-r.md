# 0494: Make R thin and first-class

Status: OPEN.

Milestone: 0.2

Depends on: 0511
Depends on: 0513

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

## Outcome

An R caller installs the package, calls the ten functions by name with ordinary R values, and gets typed R results that print, index and compare the way R users expect. Rust admits every request through 0511 and owns every rule and result fact. The R package keeps only naming, value conversion, conditions, interruption and cleanup.

## Evidence

- Starts from: the [2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). The R adapter repeats admission and result construction. It holds a copy of the complete-call input grammar in `libraries/r/thinkthen/src/rust/src/complete`, and its hand-copied reader lacked facts fields added by 0461 and 0468. 0520 repaired that reader as a narrow bridge; it does not complete this migration.
- Keeps: R indexing and native engine ownership. All ten functions and their input, result, error, cache and replay behavior. Missing stays distinct from null, and permitted unknown result fields are tolerated.
- Changes: Meet the caller acceptance and the R section of `../../libraries/BINDING-AUTHOR.md`. This ticket owns:
  - the R target template and generated outputs from 0513's common graph, extending that generator rather than copying a reader;
  - conversion of native R values into the shared Request, with no restated validation;
  - typed R conditions carrying Rust error kinds and facts;
  - interruption and cleanup in R's normal idiom; R is a synchronous host and gets no async runtime;
  - printing and documented class and field access on results;
  - the package README, with a short old-to-new call mapping;
  - removal of the old public names and copied readers after installed parity.
  One public API is one coherent family of named typed calls. Claim `libraries/r/**` and its installed typed consumer cases, narrowed to the actual files per slice before coding.
- Proof: The full shared cases run through the installed package's typed interface. They cover files and images where supported, context and options, original positions, facts, failures, invalid input with zero sends, interruption, printing and field access. Raw JSON pass-through does not count. Record handwritten code removed and added, counting generator templates, in the landing record.
- Defers: The proxy and any platform ruling change without evidence. Neither needs a ticket in 0.2.

### Added public declarations

```text
fn Engine::request_session_with_surface(&self, Request, Surface) -> Result<RequestSession, Error>
```
