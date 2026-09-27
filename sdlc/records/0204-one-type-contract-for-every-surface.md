# 0204: Publish one type contract for every surface

Status: implementation ready for fresh read-only code review on `ticket/0204-one-type-contract-for-every-surface`. Built 2026-09-27. Ian can overturn the cross-surface mappings in ADR 0082.

## Result

`specification/types.md` maps the settled inputs, ten answers, six call-error kinds, annotate failure marker, descriptions, and offset units to the existing surfaces. The hand-written `specification/result.schema.json` names separate bare-result definitions, single-answer details, aggregate details, record rows, usage, and the C JSON door request. It keeps the 0069 question-file schema as the input grammar. The C ABI and runtime remain unchanged.

`specification/fixtures/types/corpus.json` holds 38 focused structural cases. Valid door requests also pass through the public C `thinkthen_call` boundary against the local conformance backend. The checker compares independent expected values or usage errors and reuses conformance case 41 for the non-BMP scalar span. The `sdlc/scripts/test` rung calls this self-test once. This is the only shared test-rung edit in J1.

## Red and green proof

Before the second commit, the schema accepted a numeric `rank` question and a detailed `choose` result with a `yes_no` answer. It also rejected a valid detailed `find` result. The new corpus cases observed old/new verdicts of true/false, true/false, and false/true, respectively. The corrected schema ties the detailed answer kind and value to `question.verb` and types the door request's discriminating fields. The corpus separately checks six C request-error boundaries through the real door. Existing conformance cases remain the source of their backend exchanges; the fixture adds only type boundaries and does not copy captured recordings.

First fresh code review found that detailed annotate entries still accepted `{}` and that the checker assumed Python 3.12 and a Linux `.so` artifact. The schema now shares the single-judgment fields between ordinary details and successful annotate entries. It requires `question`, `failure`, and `request` in a failed entry and refuses `value`, `answer`, and `threshold` there. Focused cases reject an empty or partial success, a failed entry with a value, and a `find` member, while accepting a complete failed entry. The runner uses install's `python3` and selects Cargo's `.so` or `.dylib` artifact for Linux or macOS.

## Checks and limits

The focused self-test passed with 38 corpus cases, including 22 real C door requests and six usage errors. The first cold run took about 60 seconds to build the local C library and conformance backend. The repaired fixture took 0.71 seconds on a warm run, including Cargo's incremental build checks and the real door calls. Its preceding run waited about one minute for experiment 293's canonical heavy lock before passing. `sdlc/scripts/tickets`, `sdlc/scripts/children`, `python3 -m py_compile`, and `git diff --check` passed. The schema has 285 nonblank lines, under the 500-line ceiling; the corpus has 462 lines, below the 0069 corpus's 468. The self-test removes any inherited API key, supplies a fake key and loopback backend, gives each case a fresh temporary cache, and closes the backend process.

One earlier direct test invocation wrapped the self-locking fixture in another `flock`, causing a 23-second self-wait. The wrapper was stopped, and the fixture passed through its own canonical lock. No backend failure occurred. No full integration rung ran; the Batch J checkpoint owns that gate under the work plan's new focused-check policy.

The schema checks structure, not parser semantics such as known labels, probability distribution totals, threshold arithmetic, score range from the chosen levels, or detailed-result semantic relationships. The production parser and existing conformance tests own those checks. The type corpus does not claim parity for later port bindings; J8 runs it through each public port. No implementation path held by another batch was edited.

## Review route

The second fresh code reviewer should check the repaired annotate entry union and the interpreter and shared-library selection, alongside the settled result pages, C request parser parity, checker isolation, case-41 offsets, and test-rung hook. After ACCEPT, the coordinator can run the related-batch integration gates, record their results here, and land. The initial branch commits are `7443fd17`, `68daf774`, and `6d10c540` after the main claim merge.
