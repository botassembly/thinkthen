# Notes

Write as you go; a note written later is a guess. Newest entry last.

## 2026-09-21 — lane start

Ruby is not installed on the host (205 found the same). Everything Ruby
runs in containers, removed after every run. No host install, no rc
change; the rc guard greps `~/.zshrc` at the end.

- **Tried:** `command -v ruby gem rbenv mise`
- **Saw:** no output, exit 1.
- **Means:** the same clean-machine start as 205; the container path is the
  honest one for this surface.

### Toolchain, decided after probing

- The host has no rustup shims for cargo (`~/.cargo/bin` holds only extra
  tools); the compiler lives in `~/.rustup/toolchains/stable-.../bin` and
  `/usr/bin`.
- The builder image is `ruby:3.4-trixie` plus one package, `libclang-dev`,
  which rb-sys's bindgen step needs; the full bookworm image 205 used
  carried it. A one-line `Dockerfile` bakes `thinkthen-ruby-builder:local`
  (removed with `docker rmi thinkthen-ruby-builder:local`). The host
  toolchain runs inside it mounted read-only (host glibc 2.39 < trixie
  2.41, forward-compatible).
- The cargo home is folder-local, `.runtimes/cargo` (gitignored), so the
  registry cache persists across container runs and nothing writes to the
  host.

## The build, and one real bug the crash caught

The extension follows 205's pattern: magnus 0.7, `rb_thread_call_without_gvl`
around every engine call, the GVL re-taken by the tick through
`rb_thread_call_with_gvl`. The contract types cross one way; no rule, no
retry, no sending lives in Ruby or the shim.

**Bug, found as a segfault, cause found by reading the C backtrace.** The
first `decide` crashed inside `magnus::error::raise` reading a garbage
string. The cause was mine: `single()` decoded the crossing's answer as a
`(answer, raised)` tuple while `single_body` boxed only the answer, so the
return read past the allocation. Single calls carry no tick and so no
`raised`; the tuple was bulk-only thinking pasted onto the single path.
Fixed by decoding exactly what the body boxed. Recorded here because the
same shape will tempt the next shim: the boxed type and the decoded type
must be spelled the same on both sides of `without_gvl`.

### Fixture lesson

The null backend answers by evidence substring, lowercase: "refund" 0.97,
"maybe" 0.55, else 0.03. My first fixtures wrote "Maybe" capital and the
band test read 0.03 as a no. The engine was right; the fixture was wrong.

## Results, all commands run inside the container, stub on 8214 at 300 ms

Surface tests (ENGINE_NULL=1):

```
20 runs, 48 assertions, 0 failures, 0 errors, 0 skips
```

Conformance slice, offline, ported case-for-case from the Rust runner:

```
12 ok, 4 skip with printed reasons (wire cases, the disk cache, the
pre-fired token divergence), 0 FAIL
```

The skips and the one divergence match the sibling surfaces exactly.

Slide sample, as drawn: green, with one finding reported, not hidden:

```
finding: score returned 1.7, the slide comment says 2.0; nearest level is 'Immediate.'
3 of 8 are complaints   (the filter comment's promise)
```

**The finding, stated for the slide's owner:** the Ruby slide's comment
promises 2.0 for `score`; the offline backend's level distribution puts
the probability-weighted position at 1.7 with "Immediate." still the
nearest level. The semantic promise holds — the top level wins — but the
exact number depends on the reply's level distribution, which the offline
backend synthesizes. The slide should say what it means without pinning
the number, or pin the distribution. The sample asserts the range and the
nearest level and reports the difference; the slide owns the change.

Interrupt proof, brief item 7, on the wire (ENGINE_WIDTH=8, 1,000 records,
watcher fires the token and Thread#raise(Interrupt) at 1.000 s):

```
raised: Interrupt
wall:   1.207 s from start to raise (signal at 1.000 s)
stub:   requests at return 32, after 2 s settle 32
```

The raise lands 0.207 s past the signal — inside the tick, which cancels
the token, lets the one in-flight round finish (32 requests), and
re-raises. Nothing served after the return. That is the 211 poll shape,
ported: the tick runs with the VM lock taken each wait interval, so
`Thread#raise` reaches a blocked bulk call.

The one check script, end to end with the stub up:

```
== ruby surface: surface tests, null backend        20 runs, 0 failures
== ruby surface: conformance slice, offline         green
== ruby surface: slide sample, as drawn             green, one finding printed
== ruby surface: interrupt proof on the wire        green
```

## Cleanup and the runtime guard

Containers are `--rm`; none remain. The built image stays for the parent's
check runs and is removed with `docker rmi thinkthen-ruby-builder:local`;
the base it was built from is the official `ruby:3.4-trixie`. The stub is
killed after each wire pass. The rc guard, after every install this lane
made (all inside the folder or the container):

```
$ grep -c deno /home/ian/.zshrc
0
$ git -C repos/dotfiles status -s | wc -l
0
```

Nothing on the host changed.

## 2026-09-21 — recognize and relate, the deck's Ruby calls as drawn

**Tried:** the whole brief against the recordings — `cargo build --release` in the builder image, a smoke run of both deck calls, `tests/conformance.rb` (the recognize and relate cases), `tests/test_surface.rb`, and `./check.sh` with the stub on 8214; the vocabulary sweep over `lib/`, `src/`, and the two test files.

**Saw:**

```
smoke, the deck's recognize call:
[["Maria Chen","person",0,10,0.98], ["Northwind Freight","organization",18,35,1.0], ["Chicago","place",39,46,0.6693]]
"Maria Chen"                       (text[start...end], Ruby characters)
[["works_for",1,2,1.0]]
smoke, the deck's relate call (the four alerts in the arm):
[["caused_by",1,2,0.59], ["caused_by",1,4,0.94], ["caused_by",2,4,0.94], ["caused_by",3,4,0.84]]

surface tests: 25 runs, 64 assertions, 0 failures, 0 errors, 0 skips
conformance:   green; 71-relate-R03-persubject-10 skipped by name (engine-internal arm);
               40 recognize cases and the relate arms ok, including 68-recognize-C41
interrupt:     raised Interrupt 1.209 s from start (signal at 1.000 s), stub frozen at 32
sweep:         no confidence, certainty, likelihood, cutoff, gray zone, calibrated, accuracy;
               no "score" used for a probability — empty greps above
```

**Means:** the deck's Ruby calls run as written with the recorded answers; `text[start...end]` slices names out of the original text in Ruby characters, proven on `"Le café 😀 Maria Chen arrived."` from C41; `relate` crosses every record at once and refuses 256 with a usage error naming 255; the any-kind end is the one-character string `"*"` (C36's `located_in` from `*` to `place`); `strength` on names and `probability` on relations are bound per the settled rule; results of no fixed size come back as `ThinkThen::Entity`, `Relation`, `Recognized`, and `Edge` records parsed from the door's one JSON string (the C-door pattern).

**Finding for the parent, not touched here:** the contract's spec grammar still reads `from` and `to` keys in a relation rule (`relation_from_value` in `contract/src/lib.rs`), while the ruling says the question file says `source` and `target` too and no door converts. The Ruby wrapper builds `from`/`to` spec keys against the parser as it stands; when the contract flips, `lib/thinkthen.rb`'s `relation_rules` is the one place here to change.

**Finding, pre-existing, out of this change's scope:** the Ruby conformance runner has no arms for `rank` and `find`, so those three cases skip as "no case shape" even though the surface implements both verbs and its own tests cover them. The Python runner has a `rank` arm. A small follow-up for whichever lane owns the case runner shape.

**Pinned divergence, deck-side:** the deck's Python, TypeScript, and Rust sections ask a `located_in` rule on the Maria Chen sentence; the C01 recording covers `works_for` and `based_in`, so that rule cannot run as recorded. The Ruby section does not use `located_in` and is unaffected; the divergence is filed with the deck's owner (`repos/mktg/sdlc/issues/2026-09-21-the-decks-located-in-rule-has-no-recording.md`). Not bent here.

## 2026-09-21 — the rulings wave (languages lane)

Ruling 1 aftermath: the wrapper's relation rules build `source`/`target` keys, and the Hash ends form reads `:source`/`:target`; `tests/conformance.rb` reads the re-keyed rules.

Smaller item: `relate_body` now calls the contract's `relate_checked`, so the 255-record guard is inherited through the function the other doors use rather than resting on the stand-in's own repeat of it.

Ruling 2: two `#[cfg(test)]` unit tests in `src/lib.rs`: `the_defect_kind_names_its_error_class` (the name table maps `Defect` to `DefectError`) and `a_panic_inside_the_shim_becomes_the_defect_kind` (the shim's own `guarded` path produces the defect). They run in the builder container in `check.sh`: 2 passed. The host side asserts `ThinkThen::DefectError.new(..., "defect", false)` carries `kind` and `retryable` in `tests/test_surface.rb`.
