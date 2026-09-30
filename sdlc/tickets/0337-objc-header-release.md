# 0337: The Objective-C package ships no C header

Status: landed

## Outcome

The Objective-C archive holds no `thinkthen.h`. A consumer takes the C header from the separate C archive, as the package README already says. `release-pack` refuses a source package with two names that differ only in case. The `meta.usage` name is recorded as settled.

## Evidence

- Starts from: the rank 8 investigation and ready item 2 in `sdlc/planning/issue-priorities-2026-09-30.md`, and issue `2026-09-30-objective-c-headers-collide-on-macos.md`. Checked on main `6ceb4c464`: `release-pack` copied `libraries/c/include/thinkthen.h` into the package's `Sources/`; `release-go-cpp-pair` listed and compared that copy; `check.sh` copied the header into `Sources/` and compared the packaged copy. The package targets GNU Objective-C on Linux, and `release-workflow` builds no Objective-C archive on macOS.
- Keeps: every other member of the Objective-C archive; the C archive's header and its export check; the Swift package's header copy; the archived-source byte check for every copied member.
- Changes: `release-pack` stops copying the header, drops its archived-source mapping for that copy, and refuses case-only twins in a source package. `release-go-cpp-pair` stops expecting and comparing the copy. `ThinkThen.h` includes `<thinkthen.h>` with angle brackets, so the `-I` folders supply it and a case-blind volume cannot resolve it to `ThinkThen.h` itself. `check.sh` and the `installed.py` and `portable_batch.py` checks take the header from the C include folder (the checkout's, or the unpacked C archive's). `.gitignore` drops the old copy's line. The Objective-C Python ceiling rises from 525 to 527 for those two lines. The README drops the header comparison and fixes a link to a closed issue. The release self-test plants `thinkthen.h` beside `ThinkThen.h` and expects the refusal. Records: the issue moves to `closed/`; release issue item 7 and the language-packages row drop it; per-record token shares keep the name `meta.usage` (a coordinator decision Ian can overturn), and `remaining-batches-2026-09-28.md` drops register 61 from its open list.
- Proof: `libraries/objective-c/check.sh` passes after `git clean -fdX libraries/objective-c`, with `Sources/` free of `thinkthen.h`; `release-pack` of `c ada objective-c cobol` lists no `thinkthen.h` in the Objective-C archive; `release-go-cpp-pair ada-objective-c-cobol` passes on it; `check.sh` in installed mode passes against those archives; `release-archive-self-test.py`, `release-language-tools-self-test.py`, `release-managed-pair-self-test.py`, `release-smoke-source-packages-test` and `release-smoke-command-test` pass; `tickets`, `policy.py` and `lint` pass in a clean checkout.
- Defers: a case check for the command, C and SQL archives, whose members are fixed names; Apple Objective-C, which needs its own design.

## What the build taught us

- Dropping the copy alone did not fix the Mac case. `ThinkThen.h` used a quoted include, which looks in its own folder first, and a case-blind volume resolves `thinkthen.h` there to `ThinkThen.h` itself. Review caught it; the angle-bracket include fixes it and also stops a stale checkout copy from winning.
- The installed-mode check needed a real pack of `c` and `objective-c`. The release self-test fakes cargo, so it never compiles the unpacked wrapper.
