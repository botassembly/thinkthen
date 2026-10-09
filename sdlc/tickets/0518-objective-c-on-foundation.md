# 0518: Build Objective-C on Apple Foundation

Status: OPEN.

Milestone: 0.2

Depends on: 0516
Depends on: 0517

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

## Outcome

Objective-C is Apple-only and uses Foundation: `NSError`, ARC, blocks for async and cancellation, nullability annotations and Foundation collections. A caller installs an Apple package carrying the native library and reads generated Foundation result objects. GNU Objective-C support ends.

## Evidence

- Starts from: The [2026-10-09 decision](../decisions/2026-10-09-thin-first-class-bindings.md) and the [0521 assessment](../records/0521-surface-contract-assessment.md). The binding is GNU Objective-C without Foundation, so callers free objects by hand and get no `NSError`.
- Keeps: Shared results and errors.
- Changes: Follow the 0516 pattern on 0503's session. Meet the caller acceptance and the Objective-C section of `../../libraries/BINDING-AUTHOR.md`. This ticket owns:
  - the Objective-C target template and generated Foundation objects from 0513's common graph;
  - Foundation input conversion, `NSError` failures, block callbacks, cancellation and ARC cleanup;
  - the Apple package slice under 0517's Apple binary design;
  - the README, declaring Apple-only support with a short old-to-new call mapping;
  - removal of `TTJSON.c`, the native view copies and old public names after their callers migrate.
  One public API is one coherent family of named typed calls. Initial claims: `libraries/objective-c/Sources/ThinkThen.h`, `libraries/objective-c/Sources/ThinkThen.m`, `libraries/objective-c/check.sh`, `libraries/objective-c/source-package.json`, `libraries/objective-c/README.md` and `libraries/objective-c/checks/installed.py`. Name the installed consumer source before coding. 0530 removes the Linux-only Objective-C routing in final assembly; coordinate with it.
- Proof: An actual Foundation consumer installs the local Apple package on macOS and passes the shared suite. It proves callback ownership, cancellation and ARC cleanup, plus one held-provider case where unrelated work progresses and cancel returns before the provider is released. Linux checks and declaration generation cannot replace this native proof. Record handwritten code removed and added, counting templates, in the landing record.
- Defers: GNU Objective-C support, which ends here. This ticket grants no machine or workflow permission; native qualification follows Ian's release hold.
