# Language-port integration priority brief

Status: open, a prioritization request from the consumer-language program to the queue owner. Filed 2026-09-28. Ian asked for this brief so the integration tickets can be planned against the whole program at once.

## Who is asking and what exists

The consumer-language program finished twelve languages across ten experiments (local experiments 273, 274, 289, 290, 291, 292, 293, 294, 295, 300): Zig, Go, Java/Kotlin/Scala, C#, PHP, COBOL, Ada, Swift, Objective-C, and Dart. Every port has two gated stages, parent-verified runs, fresh reviews, fix cycles, and exact-evidence gates with planted negatives. Each has an issue in this folder. Nothing is published; every package is a rehearsal.

## What we need, in priority order, and why

**P1 — Final-pin rebuilds and per-language READMEs under `libraries/<lang>/`.** Why: every accepted proof runs at a pin that is now months of engine change old; evidence decays as the header grows (nineteen exports at the earliest pins, twenty-one now) and drift findings have already appeared once (the relation wire-shape change, closed as intentional). The Go post-J1 run shows a whole accepted gate passing unchanged, but only a rebuild proves that per language. The READMEs are the install story every registry entry and the website point at; they name the native-archive step, which the release issue's distribution decision already fixes.

**P2 — Integration tickets in registry-value order: C# (NuGet), JVM (Maven Central), PHP (Packagist), Dart (pub.dev), then Swift, Zig, Ada, Objective-C, COBOL.** Why: those first four carry real registry demand and Ian's one-time account setup for all four is already written in his to-dos; the accounts exist before the packages do, so wiring can start the moment a package lands. Each ticket should: copy only the experiment's `stage2/package/` folder, derive the export list from the sealed header per rebuild (never pin a count), bind `thinkthen_engine_new_with`, and run the shared J1 corpus through the public binding. The Dart and Swift issues carry the tested constructor and schema-parity patterns to copy. Per Ian's ruling 2026-09-28, the Dart ticket includes a real Flutter-embedder run, not just platform packaging.

**P3 — Contract deltas for C#, PHP, Swift, Go, JVM, and Zig** (named error kinds, map-form label sets, annotate failure typing, offset naming — listed per issue). Why: Ada, Objective-C, COBOL, and Dart already meet the type contract; the earlier six predate it and their issues name exactly what changes.

**P4 — One model-produced non-BMP offset case in the shared corpus.** Why: every port proves the shared synthetic case; a model-produced case would close the last gap between synthetic and live evidence for offset correctness.

**Compute: no constraint.** Ian's ruling (2026-09-28): fill this Linux machine and the M5 Mac with as many builds as the work needs. The only courtesy rule on the Linux box is not to overload it while other lanes hold the shared lock (the existing load gate already handles this). The experiments' single-host wording describes evidence provenance, not a capacity limit: rebuild tickets should run on both machines and in CI without asking. The one true second-environment finding so far is positive — the first macOS native package checks and the M5 proof landed cleanly. The Go and Swift post-J1 reports show what drift checks look like and cost.

## Decisions Ian can overturn

The registry order, the READMEs-before-packages order, and the thin-wrapper-plus-native-archive shape. The type contract and its section-2 wording observation (Ada experiment) remain the core team's ruling to make or delegate.
