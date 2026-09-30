# 0309: Docs and records hygiene

Status: landed. Fresh code review found a blocking gap: `release-managed-pair.py` allowed only two source symlinks, so source capture refused the header links. Both links are now allowed, and its self-test runs `tar_files` on a real `git archive HEAD`. Lane claude-1. Branch `ticket/0309-docs-and-records-hygiene`. Plan: `sdlc/planning/cleanup-2026-09-30.md`, ruling 8.

## Outcome

The front pages say what the tool does. The issue folder holds open issues only, each with a true status line. The planning index names the current plan. The Swift and Objective-C packages cannot drift from the C header.

## Evidence

- Starts from: main `1fc0075e9`, the cleanup plan's "next" list, and a read of `README.md`, `specification/README.md`, `sdlc/issues/` and `sdlc/planning/README.md`.
- Keeps: every specification rule, every issue's history, every historical plan, and `site/`, which marketing owns.
- Changes: the filter row in `README.md`; the command and audit rows in `specification/README.md`; the ADR 0050 citations; five closed issues move to `closed/`; five status lines added or corrected; one issue filed for marketing; the planning index; the vendored C header copies become symlinks.
- Proof: `sdlc/scripts/lint` up to its known main failures, `python3 sdlc/scripts/tickets`, a relative-link check over the touched pages and `sdlc/issues/`, both package checks, `release-pack` for `c swift objective-c`, and the archived-release self-test.
- Defers: the site fixes, which marketing owns; the open work each touched issue still names.

## Changes

1. `README.md` says filter prints kept records in input order, with CSV and TSV rows as JSON, as `specification/filter.md` says. `specification/README.md` lists `recognize`, `audit`, `diff` and `transform`, and says `audit` grades all ten commands.
2. ADR 0050 was planned by ticket 0147 and never written; ADR 0056 replaced it. The citations say so.
3. Issue `2026-09-30-reference-page-exit-codes-and-key-rule-drift.md` asks marketing to fix the reference page's exit codes and key rule.
4. Closed issues move to `sdlc/issues/closed/`, and links follow them. The panic, C++ and R issues gain status lines. The System One null-criteria and architect review 10 statuses say what remains.
5. `sdlc/planning/README.md` names `cleanup-2026-09-30.md` as the status authority and lists the older plans as historical.
6. `libraries/swift/Sources/CThinkThen/include/thinkthen.h` and `libraries/objective-c/Sources/thinkthen.h` are relative symlinks to `libraries/c/include/thinkthen.h`. The four `cmp` checks that compared each copy with the C header go, because a link compares a file with itself. The Swift local-archive fixture adds the header with `dereference=True`. In archived-source mode, `release-pack` compares a linked source with the file it names. `cp` follows the link, so each release archive still holds a regular header file.

## Build result

- `libraries/swift/check.sh` and `libraries/objective-c/check.sh` pass with the links, including the installed-consumer checks.
- `release-pack x86_64-unknown-linux-gnu OUT c swift objective-c` packs both wrappers with a regular `thinkthen.h` equal to the C header.
- `release-archive-self-test.py` passes after the linked-source fix. It failed before that fix with `swift source differs from archived commit`.
- `sdlc/scripts/lint` stops on main's known `cargo deny` advisories (ticket 0307). With that one step skipped, it reaches clippy, which fails on main in `crates/thinkthen/tests/version.rs:49` and `tests/backend/recording_durability.rs:96`. This ticket touches no Rust. `inventory` reports 0306's `contained` and `uncontained` as outside the frozen contract, also on main.
- Three pre-existing broken links in `sdlc/issues/` are fixed. Broken links in tickets 0162 and 0163 stay; they are history.

## What the build taught us

- A symlink is the least machinery for a shared header. SwiftPM, clang, GNU `cp` and Python's `copytree` and `copyfile` follow it. Python's `tarfile.add` and `git archive` keep it as a link, so each archive step needed a look.
- The archived-release self-test caught the one place that read the header from a tar member. Lint runs it, so the gap showed before review.
- A tracked symlink must also pass the release source-capture allow list in `release-managed-pair.py`. Its self-test now reads the real archive, so a new link fails lint.
- Issue statuses drift when a ticket lands and forgets the issue. `2026-09-28-binding-panic-hooks-can-print-caught-payloads.md` said open after 0306; this ticket settled it.
