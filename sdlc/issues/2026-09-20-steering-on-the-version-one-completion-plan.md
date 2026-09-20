# Steering on the version-one completion plan

Status: Open

Ian shared the builder's proposed completion plan on 2026-09-20 and asked the agent that holds the marketing and library-design job whether anything in it needs steering. The plan is good. It took in the tag output shape, the CSV pass-through for `filter` and `rank`, the four launch lessons, the stumble register, and the probe list. Six points follow, most important first. None changes the plan's order.

## 1. One published crate named `thinkthen`, before anything is packaged

The plan is silent on this, and its last step prepares a release-ready package. Ian ruled on 2026-09-20 that every public name is `thinkthen` and that no `thinkthen-core` is ever published. crates.io makes every dependency of a published crate public. The repository still has two crates, and `CLAUDE.md` still names both. The plan needs a ticket that folds them into one crate: a library target, the binary behind a default `cli` feature, the pure core as an inner module, and the purity rule held by a lint over that module. It has to land before the package step. `2026-09-20-libraries-ruled-in-and-every-public-name-is-thinkthen.md` has the ruling. Ian confirmed later the same day that he wants the single crate and no second landing zone. The command installs through a `curl` installer that downloads a release from GitHub, and `cargo install thinkthen` is welcome if the one crate can carry both.

## 2. New machinery lands where a library can call it

Ian ruled the same day that the libraries must be as fast as possible with the least code to maintain, and that everything possible is pushed down into Rust. `sdlc/planning/libraries/README.md` records it. It reverses item 3 of ADR 0017, which had every host language write its own sending, retries, scheduling, and recording. Seven surfaces are now ruled in: the command, Rust, Python, JavaScript, Ruby, R, and C.

The plan says the binary owns file reading and scheduling. That is fine if "the binary" means the crate's library half. The ask costs nothing now. The multi-question request plan, the CSV and TSV reader, the scheduler, the per-digest cache locks, and record and replay each land in a library module that does not depend on the argument parser, the terminal, or printing. `main` stays a shim that parses arguments, calls one function, and prints. The plan's own rule says no abstraction without two callers. This adds no abstraction. It is a choice of which file the code sits in, and the second caller is already ruled in. Code that lands tangled with the terminal gets rewritten once for the libraries.

## 3. Write new tests as data where the effort is the same

Ian wants one set of tests that works across all seven surfaces. The how-tos and the recordings are already close to that: an input, a recorded exchange, and an expected output. For `annotate`, `tag`, `find`, and the CSV framing, prefer a case that is data over an assertion written in Rust wherever either would do: the arguments, the input, the recording, the expected standard output, and the exit code. Those cases become the shared conformance suite with no second effort. A test of a Rust-only detail stays a Rust test.

## 4. A failed twenty-label probe should not block version one

The plan stops the expanded scope if twenty labels fail. Twenty came from Ian's example of a blog with twenty topics. Nobody promised it. If the probe holds at ten and fails at twenty, `tag` ships with a ceiling of ten and the page says so. The launch copy can sell "up to N labels in one request, measured". It cannot sell a missing command.

A live run later on 2026-09-20 held at twenty labels and at twenty-two, so this point is moot. `2026-09-20-live-probe-findings-packing-tagging-status-and-cost.md` has the numbers. The same run answered point 5.

## 5. Add the status look to the multi-question probe

Ian asked what status the service can report. Two calls answer it while the guard is open, at almost no cost: the documented models listing, and the names of the response headers on one ordinary judgment, with no values saved. `2026-09-20-a-live-probe-plan-for-tagging-many-questions-and-status.md` has the detail. The product `status` command stays after version one, as the plan says.

## 6. Three small things the launch needs from step 7

- **Prebuilt binaries and one install line.** The third line of the home page is the install line, and most shell users have no Rust toolchain. Name the targets in the plan: Linux and macOS, on both common processor families, with a static Linux build.
- **One page of quotable numbers.** The site shows three numbers: the time of one judgment, the cost of one real file, and the accuracy on one labeled set with its size. The plan produces the evidence for each. One short page that lists every number marketing may quote, each with the record that measured it, keeps the copy honest with no hunting.
- **A speed bench for the command.** Ian's first priority is speed. Against a stub backend on the loopback address, measure the time from start to the first request byte, and the overhead per record in a stream. It guards against a slow start creeping in, and it gives the site a fourth number.

## What Ian can overturn

All six. Point 1 rests on his naming ruling and point 2 on his library ruling, both from 2026-09-20.
