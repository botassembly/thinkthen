# Lessons from the polyglot verification program

Fresh Medium read-only review accepted655e9592e against69fcdc819. Focused pages, tickets and diff checks pass. This records conventions and follow-ups; it closes no product issue.

Source check: main e7a899e4a, with standards ownership claimed at69fcdc819. The independent experiment302 reports describe their sealed pins. Their final 14 PASS / 10 FAIL total is historical; the later JVM correction and reviewed Dart/R correction at61350a253 do not constitute a fresh 24-surface run. Privacy and canary successes in those reports remain evidence for their tested bytes.

## Adopted conventions and remaining work

The [standards](../planning/rust-standards.md#tests) now require independent public-name expectations, classification of intended negative failures and explicit package environments. Literal export counts should not duplicate a declared name set. Exact request multisets, ABI inventories and contractual messages remain valuable. TypeScript's current test compares CommonJS, ESM and declaration names as well as a literal twenty; replacing the count must preserve an independent public-contract check. No TypeScript code changed in this follow-up.

Dart's recent negative failure was a diagnostic that interpolated the new result wrapper instead of its value. Its positive comparison had already migrated. The reviewed fix preserves the exact intended assertion marker. Treating every message mismatch as harmless would have missed the loss of useful diagnostics.

Minimal environments must cover the launching gate and its children. Objective-C startup masking and JVM locale are observed examples. Explicit UTF-8, tool paths, wrapper overrides and pinned locks are required inputs. `env -i` does not remove Cargo configuration inherited through parent directories. The existing package issue owns enforcement gaps across the 0269–0272 release families; no Actions or broad package campaign ran here.

The standards also remove the stale full-ladder-per-handback instruction and describe the actual routine `test` script. This restores agreement with AGENTS and Ian's focused-check ruling. It changes no gate implementation or lint threshold.

## R and shared counters

Ticket 0288 on the preserved, reviewed sixteen-ticket preparation branch already specifies per-call `batch`, threshold, meanings, context and `deadline_ms`. Its builder should prove two different per-call settings in one R session. The counter test's fresh-child fix is valid for the current immutable engine configuration and does not settle that API migration.

At the source pin, SQLite's settings counter counts every file except the backend marker. PostgreSQL already checks the JSON suffix; Python selects JSON and excludes internal names; R and Ruby use their native filename filters. The package issue now owns a common entry fixture and shared Python counting helper after the active host claims clear. Do not add a cross-language runtime dependency merely to count fixture files. Preserve exact entry and observed-send assertions.

## Git maintenance

The common Git directory's old `gc.log` disabled automatic collection because too many unreachable loose objects remained. A connectivity check passed before maintenance; this was not evidence of repository corruption. The coordinator saved the warning and object inventory, then ran:

```sh
git -c gc.reflogExpire=never -c gc.reflogExpireUnreachable=never gc --cruft --no-prune --keep-largest-pack
```

Collection passed. A `cat-file` check found all 73,514 pre-existing objects, with zero missing, and the final connectivity check passed. The result had zero loose objects, three packs and zero garbage. The coordinator removed only the unchanged saved warning after verification. No prune or reflog expiration ran. Local receipts remain in codex-4's ignored `target/codex-builds/git-maintenance/`; subsequent commits naturally create new loose objects.

Experiment302's registry caches remain untouched. The reported 122 GB is the verifier's measurement. Reports, logs, manifests and sealed sources must remain, and each removable cache needs an ownership and active-use check before cleanup. This maintenance record authorizes no cache deletion.
