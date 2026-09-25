FINDINGS

# Design review: 0112, Port the Ruby surface

Reviewer: a fresh, read-only Claude session that did not write the ticket. Date: 2026-09-24.

Read: 0112 at `e5ba58a6` (fetch succeeded and matched the local ref). Also read: repo `AGENTS.md` (the `CLAUDE.md` target) and `sdlc/README.md`, `sdlc/planning/surfaces-port-guide.md`, ADR 0017 on main, tickets 0084, 0085, 0086, 0093, 0094, 0095, and 0098 on their branches, ADR 0047 on the 0093 branch, 0105 and its review record on its branch, sibling tickets 0110 and 0111, the tag `surfaces-wave7-frozen-2026-09-24b` (`9df8bae9`) with its freeze record, the error index, main's `deny.toml`, `sdlc/scripts/install`, `conformance/backend/src/arms.rs`, and `specification/recording.md`.

Observed by command on this machine: rustc 1.93.1; libclang 17 and 18; the listed `-dev` packages present; `libyaml-dev` absent; no `ruby` on `PATH`; glibc 2.39. `docker image inspect thinkthen-ruby-builder:local` shows `RUBY_VERSION=3.4.11` and `RUBY_DOWNLOAD_SHA256=f79c6e78…f4e4`. The tag's Dockerfile uses the floating `ruby:3.4-trixie` tag. The hash comes from the official image's metadata. Every crate in the tag's Ruby lock is in the cargo cache.

## What holds

- The 15 `ruby` rows are all accounted for. Eleven are re-proved (R1-20, R2-8, R2-15, R3-5, R3-15b, R3-16, R4-2, R5-9, R5-14, R7-3, R7-9). Four retire behind a file check (R4-8, R4-9, R5-13, R7-10).
- The binding uses only 0084, 0095, and 0098 members through magnus. It does not use `CallOptions::interrupt`, and the worker drains `Batch` on its own thread. This matches 0084's rule that `Batch` is neither `Send` nor `Sync`.
- Decision 4 carries every point of the 0105 final check. The caller's token is read in each slice, a failed send on the closed handoff is ignored, a closed handoff raises `DefectError`, and SIGINT waits until the count line reads 1 or W.
- Deny now names `sources` and `--config deny.toml`, so the git-dependency plant can turn red. The forbid-level stop rule is present.
- No private names appear.

## Findings

1. **Rows: two Ruby halves are missing, a count is wrong, and two retirements have no check.**
   - R3-32 and R4-10 each have a Ruby half. The wave-7 probe names "a hard-coded .so name at ruby/build.sh:48", and the new `build.sh` copies the `.so` again. Fix: add a row. Derive the file name from cargo's output or from `RbConfig::CONFIG["DLEXT"]`. Plant a literal `.so` name that a check step refuses. Or retire the half to the release ticket with R5-37.
   - The text says "twelve cross-surface rows". The table lists 14: R2-10, R2-25, R1-34, R4-19, R5-34, R3-29, R3-28, R1-28, R3-30, R5-32, R4-18, R6-12, R1-31, and R2-31. Fix the number.
   - R1-29's Ruby half retires "since the Ruby source is pinned by checksum". Nothing checks that. Fix: `setup-ruby.sh` writes a stamp file into the prefix with both hashes and the configure line. `check.sh` compares the stamp with the hashes pinned in the script. On a mismatch it prints "not run". Plant a changed hash.
   - R5-35's half retires because `lint` no longer builds the crate. Fix: name the check that holds this. For example, the lint step for `libraries/ruby` runs only `cargo deny --offline` and the policy checks, and it passes with no `ruby` on `PATH`.
   - R5-37 moves to queue item 11. No ticket carries it yet. Fix: file it in `sdlc/issues/`, or name the existing record that holds it.
   - R2-8's plant borrows R7-9's plant. Fix: give R2-8 its own plant. Rust stores the tick proc as an unmarked `Value`, and the GC test turns red.

2. **The held arm releases once per backend, so the multi-round tests cannot run as written.**
   - `Backend::release` lets every held reply go "now and from here on" (`arms.rs:110`). The count also accumulates for the life of the backend.
   - R4-2 has four rounds that each "then release". After the first release, rounds two to four never hold.
   - Several files also assert "zero counted requests" after earlier sends.
   - Fix: copy 0111's rule. Each held phase starts its own backend through `tests/backend.rb`, so one `release` never reaches the next phase. Zero-send assertions compare counts before and after the call.
   - Each "once the count reads W" poll has its own bound, such as 5 s. A plant then fails an assertion. It does not wait out `timeout 120`.

3. **Interrupts: two paths are not prompt, and the detach cost is understated.**
   - A raising tick fires the call's own token. The call raises only "once the crossing returns", which can take up to the request timeout on a held single send. Fix: each slice also reads the call's own token. When the token has fired, the call detaches the worker and raises the tick's error at once. Add a held single `decide` test whose tick raises. Its plant is waiting for the worker.
   - Decision 4 says a detached send "can hold one width permit". A detached batch holds up to W permits and W sends until the request timeout ends them. The next call in the process can wait on the width gate for that long. ADR 0017 says "Zero threads outlive a call". Fix: state W. Record the departure in the Ruby section of ADR 0047, with its cost and Ian's lever. After the R1-20 release, add a test that a following `decide` answers.
   - R3-5's plant covers only the watchdog row. The row's defect is a raise that jumps across Rust frames. Fix: add a plant that drops `rb_protect` around `rb_thread_check_ints`. Run the file in a child process, as R4-2 does. Raise the loop count toward the probe's 3,000, or say why 300 is enough to catch it.

4. **The dependency list is incomplete.** The tag's lock brings in these crates beyond the five the ticket names: cexpr 0.6.0, nom 7.1.3, minimal-lexical 0.2.1, itertools 0.13.0, either, regex 1.13.1, regex-automata, aho-corasick, rustc-hash 1.1.0, glob, lazy_static 1.5.0, shell-words 1.1.1, seq-macro 0.3.6, and shlex 1.3.0. The root lock has shlex 2.0.1, so shlex 1.3.0 is a second major version. I read each license from the cached crate, and all are in the allow list.
   - Fix: list the whole build tree with versions. Name libc as 0.2.189.
   - Record deny's observed result on the tag's lock, as 0111 does.
   - Add one static assertion in the binding's tests: `Question`, `LoadedQuestion`, `QuestionSet`, `Recognize`, `Relate`, and `Entity` are `Send + 'static`. The worker design depends on this, and 0086 asserts only "every frozen shared handle".

5. **The toolchain plan works in outline. Five points need fixing.**
   - *Folder.* `${XDG_CACHE_HOME:-$HOME/.cache}/thinkthen` is the product's own answer cache (`specification/recording.md:18`), and `~/.cache/thinkthen` already exists on this machine. `status` counts allocated bytes there. Fix: use `~/.cache/thinkthen-dev/ruby/3.4.11`, the same pattern as 0110 and 0111.
   - *Where it runs.* The ticket says the script runs "once on a networked machine". The built prefix is tied to its path and to this machine's glibc, so it cannot move between machines. Fix: say "on this machine, once, with the network". The "not run" line should say the same.
   - *Pins.* The libyaml hash is left to the builder: "from a second published source". Fix: put both download URLs and both SHA-256 values in the ticket. For Ruby that is `cache.ruby-lang.org/pub/ruby/3.4/ruby-3.4.11.tar.xz`. For libyaml, use the release tarball `yaml-0.2.5.tar.gz`, which ships a `configure` script. Say that the Ruby hash comes from the official image's metadata. Cross-check it against ruby-lang.org's release page.
   - *Repeatable.* Fix: build in a temporary folder and rename it into place only after checks pass. Refuse to overwrite a complete prefix. Fail the setup when `ruby -rpsych -rzlib -ropenssl -rfiddle -e1` fails, because Ruby's configure skips a missing extension with only a warning. Write the stamp from finding 1. `check.sh` checks the stamp and `ruby -v`.
   - *Authorization.* This machine has no Ruby or libyaml source, so the first run has to download both. It writes a toolchain outside the repository. The workspace `CLAUDE.md` excuses only repo-local checks that write build or test output. Fix: the ticket names who approves the one fetch, with its URLs, sizes, and hashes. Either Ian approves it with the ticket, or it goes to him as a `notes/todos/` item. The fetch then runs by hand, never from a rung, and the record logs it.
   - *Fair comparison.* Option (c) rejects Docker for "the no-Docker requirement". No record states that requirement. It was a condition of the freeze run, and ADR 0047 says the surface rung runs heavy Docker checks. Fix: reject (c) on grounds that can be checked. The base tag floats (R1-29). The image pins its own Rust toolchain (R7-10, R4-8). The check would start a container on every run. Also name the options the ticket left out:
     - Copying Ruby out of the cached image fails. It was built on Debian trixie, whose glibc 2.41 is newer than this host's 2.39.
     - A prebuilt `ruby/ruby-builder` tarball needs no compile but has no published checksum.
     - `ruby-build` does the same work as (a) and adds a tool.
     Option (b) is compared fairly: Ubuntu 24.04 ships Ruby 3.2, and installing it needs sudo.

6. **Budgets: realistic, and tests are tight.** I measured the tag in nonblank lines. `src/lib.rs` has 941 lines: 902 of production and 39 of tests. The 850 cap fits the stated cuts. `thinkthen.rb` and `version.rb` measure 429 against a cap of 450. `check.sh` and `build.sh` measure 180 against a cap of 240 that includes `setup-ruby.sh`, which fits. The 19 test files measure 1,582 against a cap of 1,650. That leaves 68 lines for about twenty new proofs, `tests/backend.rb`, and the rank and find arms. Fix: name the stub lines that leave and count them against the new proofs, or raise the test cap now. The stop rule covers any other overrun.

7. **Small fixes.**
   - The R4-9 and R5-13 step should refuse a `[features]` table. The manifest keeps `features = ["rb-sys"]` on magnus, which "declares any feature" would also match.
   - The R4-19 step should list the scripts it reads (`check.sh`, `build.sh`). `setup-ruby.sh` must fetch, so it has to stay outside the step.
   - Tests read W from the backend's count or from the engine setting, and never use a literal 4. An open issue asks whether the default width should drop to 3.
   - Writing: four trailing "since" clauses (decision 1, decision 3, "Where the check runs", and "Not carried"). Split each into two sentences. The rest follows the writing rules.

ACCEPT once findings 1 through 5 are applied. Finding 6 needs one number or one sentence, and finding 7 is small edits.

## Confirmation

Read 0112 at `562f0b8f`. Also read the shared rules on main at `6eb1303e` (`surfaces-port-guide.md` lines 9 to 16), the 0084 amendment at `f19cf437`, ticket 0117 on its branch, the filed Mac issue, main's `specification/backends.md`, and main's `crates/thinkthen/src/engine/http.rs`.

### ACCEPT

**The rejected finding.** My finding 3 did not ask for one slot. It asked the ticket to state that a detached batch holds up to W slots, record the departure from ADR 0017, name Ian's lever, and test a following call. Decision 5 now does all four. So the "rejected as written" line in the ticket's Review section misreads the finding. The substance is accepted, and I count finding 3 as fixed. The cost is stated honestly.
- Under 0073 and 0095, a cancelled call starts nothing new, and requests already sent finish. The width gate covers the whole process (0077). A detached call therefore keeps at most W sends and W slots.
- Each attempt ends within 30 s. `http.rs:340` sets `Duration::from_secs(30)`, and `backends.md:59` defines that limit as one attempt from connect to the last byte. No retry starts on a cancelled token, and a library has no environment setting that lengthens the limit.
- Repeated Ctrl-C does not add up past W, because every new call waits at the same gate. The waiting call is itself in 50 ms slices, so a second Ctrl-C still returns at once.
- Both levers are real and correctly costed. One changes 0073, and the other gives up the prompt Ctrl-C that the shared rule requires. The R1-20 test's check that a following call answers proves the slots come back.

**The libyaml plan is sound.** The recalled hash is marked unverified and cannot land as written. buildroot's `libyaml.hash` and Alpine's `APKBUILD` are two separate published sources for the same `yaml-0.2.5.tar.gz`. Pinning both the SHA-256 and the SHA-512, checking both during setup, and stopping when they disagree is stronger than one checksum. Note for the builder: `toolchain.env` and the stamp carry both libyaml hashes. The record names the two pages that were read.

**Other findings.** All are fixed in the text.
- Rows: R3-32 and R4-10 are carried with a portability step and a plant. The count reads sixteen, which matches the table. R1-29 is held by the stamp against `toolchain.env`. R5-35 is held by the no-Ruby `lint` block and its `cargo check` plant. R5-37 has a filed issue. R2-8 has its own unmarked-`Value` plant.
- Held arm: each test has its own backend and cache. R4-2 uses 0117's `round`. Zero-send checks compare counts. Every poll uses `wait N`, which 0117 bounds at 5 s.
- Interrupts: the slice reads the call's own token, so a raising tick is prompt, with a test and a plant. R3-5 runs in a child at 3,000 calls with an `rb_protect` plant.
- Dependencies: all 22 crates are listed. I re-read the tag's lock, and the ticket's rustc-hash 2.1.3 is correct. My 1.1.0 was wrong. Deny's result is recorded, and the `Send + 'static` assertion is added.
- Toolchain: the prefix lives in `~/.cache/thinkthen-toolchains/`, per the shared rule. It is built on each machine, both URLs are in `toolchain.env`, the setup builds in a temporary folder and renames it into place, it checks the extensions it needs, and it writes the stamp. Decision 13 records the no-Docker rule and the one-time fetch rule as Claude decisions that Ian can overturn, which satisfies the durable-record rule. The Docker rejection now rests on checkable grounds, and options (d) to (f) are weighed fairly.
- Budgets rise to 900 lines of Rust, 490 of Ruby library, 1,950 of tests, and 260 of scripts. Each rise has a stated cause.
- `ThinkThen::Engine` is built on `EngineBuilder::from_env()`, which matches the amendment. It has a plant for the address and one for the width.
- The writing rules hold, and no private names appear.

## Spike amendment check (experiment 256)

### Round 1: 0112 at 1c6229c3 (diff against 5a40feca): findings

1. **The R5-35 plant cannot fail as written.** The guard puts the prefix's `bin` first on `PATH` and then checks that `command -v ruby` prints the prefix path. The plant puts a stub `ruby` "ahead of the prefix on the block's `PATH`". A stub placed before the guard runs ends up behind the prefix after the guard's prepend, so the guard passes and `lint` stays green. The plant turns red only if it edits the guard's own `PATH` line, and the ticket does not say so. On a normal run, the `command -v` check passes by construction. Fix: name a plant that reaches the real checks, for example pointing the guard's prefix at a stub prefix whose `ruby` prints 3.2.3 so the version check fails, or a stamp mismatch.
2. **The lint block now depends on the Ruby prefix, and it lost its old proof.** The block runs the file checks, deny, the ratchets, and the policy checks. None of them needs Ruby. Before the amendment, the block ran with no Ruby on any machine. Now a machine without the prefix prints "not run" for the whole Ruby lint block, deny and the ratchets included. The old plant (add `cargo check` to the block, and rb-sys finds no Ruby) also proved that the block builds no crate. With the pinned Ruby on `PATH`, that `cargo check` would pass, and "It builds no crate" has no check left. Fix: let the `lint` block run with a `PATH` built from named tool folders that excludes every Ruby. The spike offered that option. Keep "`command -v ruby` finds nothing" and the old `cargo check` plant there. Keep the pinned-Ruby guard for `check.sh`, which needs Ruby.
3. Changes 2 to 4 are sound. The `link-ruby` dev-dependency adds no crate. The extension still links no libruby, provided the resolver keeps dev-dependency features out of the release build, which the spike observed. The FFI module is the one `missing_docs` allow. `tests/backend.rb` gives each child a fresh `HOME`, `XDG_CACHE_HOME`, and `XDG_CONFIG_HOME`, and the cargo steps keep the real `HOME`. Test children start `ruby` from the pinned prefix by path, so a scratch `HOME` does not hide it.
4. Two stale lines remain (does not block). Line 55 still says the libyaml hash is not on this machine, and the "Defers" line at 233 still says the libyaml checksum stays unverified. The amendment says the Defers line no longer applies, but neither line was edited or points to the amendment.

Author response at the next commit. Findings 1 and 4 are fixed as proposed: three plants reach real checks, and the stale libyaml lines are edited. Finding 2 is fixed in substance and not in form. Ian ruled that the guard requires the pinned Ruby by absolute path and refuses any other Ruby, so the lint block does not return to a Ruby-free PATH. A missing prefix now marks only the guard "not run", and deny, the ratchets, and the policy checks still run. An empty CARGO_TARGET_DIR and a last step that fails when it fills prove the block builds no crate, and the old cargo check plant turns it red.

### Round 2: 0112 at f2defc43 (diff against 1c6229c3): one finding

- Finding 2 is resolved within Ian's ruling. The guard no longer gates the Ruby-free checks. The file checks, deny, the ratchets, and the policy checks still run without the prefix. The empty `CARGO_TARGET_DIR` and the last step that fails when it fills prove the block builds no crate. Deny and the ratchets write nothing into the target folder, so the last step does not fail on a normal run.
- Finding 4 is fixed. The libyaml line and the Defers line are edited.
- Finding 1 is fixed for plants two and three. Setting `RUBY` to the stub hits the refusal. An added `cargo check` fills the target folder.
- **Finding (plant one).** The guard checks the stamp against `toolchain.env` before it runs the version check. The R1-29 row says a stamp mismatch prints "not run" and never "fail", and a missing prefix also prints "not run" for the guard. A bare stub prefix with no matching stamp therefore stops at "not run" and never reaches the version check. If `lint` treats a "not run" guard as allowed, which it must for machines without the prefix, the plant stays green. Fix: have plant one give the stub prefix a copy of the real stamp so it reaches the version check, or have the guard report a stamp-matched version mismatch as "fail".
- Note that does not block: "The check it adds to the gate ladder" still says `lint` runs "behind the pinned-Ruby guard". The Ruby-free checks now run regardless, so "beside" is more accurate.
- 0108 `2b0ae3f9`: each R test child now unsets `R_LIBS_USER`, so my earlier note is applied.

Author response at the next commit. Both fixes are applied: plant one's stub prefix holds a copy of the real stamp, and a wrong version behind a matching stamp prints "fail". The gate-ladder line now reads "beside".

### Round 3: 0112 at 89db81ed (diff against f2defc43): ACCEPT

- Plant one's stub prefix now holds a copy of the real stamp and a `ruby` that prints 3.2.3. The guard prints "fail", never "not run", for a wrong version behind a matching stamp. The plant therefore reaches the version check and can turn red. The R5-35 row and change 1 agree.
- The gate-ladder line now reads "beside".
- Nothing else in the ticket changed. The commit also adds round 2 to the records file.
