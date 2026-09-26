Status: open. Filed 2026-09-26 by the marketing lead from a fresh architect review.

# Architect review 09: record, replay and testing

A fresh reviewer tested recordings and offline replay as an architect who builds a test suite on them. The review ran against main `9d652bed` and the release binary. It rated the topic fair and found 0 severity 1, 3 severity 2 and 9 severity 3 issues. The full detail sits in the architect review report 273, 09. Work file names such as `09-work/demo27/` refer to the report's local work folder, which stays unpushed.

Severity 1 means a wrong answer, data loss, a security problem or a hang. Severity 2 means a broken guarantee or a misleading document. Severity 3 means a sharp edge or a missing feature an integrator needs.

## 1. A library cannot open a recording folder made before folder binding, and its message gives no way out (severity 2)

Evidence. R `tt_engine(cache = <copy of demos/27-test-with-no-network/recording>)` failed both a hit and a miss with `the recording folder uses a retired layout` (`public/error.rs:198`). On the same folder, the command's `--cache` says `the recording folder predates backend binding; replay it read-only or choose a new folder` (`cli/failure/recording.rs:58`). `recording.md` promises that message. Replaying read-only is impossible in a library. In this repo, 74 of 97 folders holding entries have no `.thinkthen-backend.json`, and demo 27 is one of them.

What an integrator hits. The shipped offline examples cannot be reused from any library. The message suggests that the recording format was retired, which is false. The only fix is to copy the folder and hand-write a marker, or to re-record live, and re-recording gives different answers (item 2).

Direction. Let a read-only library replay (ticket 0148) accept unmarked folders as the command does. Give the library error the command's actionable sentence. Consider a small subcommand that binds an old folder to its stored URL after checking that every entry names that URL.

## 2. The spec understates how far a repeated request moves (severity 2)

Evidence. `recording.md:78` and the ADR 0010 amendment quantify drift as borderline moves "up to 0.08" and "gaps up to 0.09 in 178 of 681". `threshold.md:45` says "A band narrower than about 0.1 on each side of a cut does not keep a flip out." The Beatles Bench recordings that the site uses show 4,075 of 5,290 repeated digests differing, 263 by more than 0.1, and a largest gap of 0.45. All come from `jev-1.13.0`. One digest replays as `john` 0.21, 0.25 and 0.66, and it flips across the default cut of 0.5. Live, six back-to-back sends of that request gave `john` from 0.22 to 0.49.

What an integrator hits. A team that sizes a tolerance band, a threshold margin or a "rerun until stable" budget from the spec's 0.08 will see decisions flip in production and in re-recorded suites. The recording is the only reproducible answer. The model version string does not pin behavior.

Direction. Replace the numbers with a measurement over the current corpus, and say plainly that a borderline answer can cross the cut from one call to the next. Point readers to `threshold.md`'s unsure band as the design answer, and resize the band advice from the new measurement.

## 3. Byte-exact keys make caches and recordings fragile to harmless input changes (severity 3, carried here for two reviews)

Reviews 08 (issue 6) and 09 (issue 11) both found this. A trailing newline, CRLF line ends, JSON key order, and `1` against `1.0` each change the key. Single-text stdin keeps the trailing newline, and `--lines` strips it. `records.md:35` and `:52-53` document parts of this. A cache warmed from one surface or mode misses from another and costs double. An upstream producer that reorders keys, such as a database JSON column, a `jq -S` step or a Python dict round trip, makes every entry miss. The model does see these bytes, so exactness is defensible. Direction: say so in one place next to the cache, recommend a canonicalizing step in pipelines that need stable fixtures, cover CRLF in demo 27, and add the cross-surface digest test.

## Already filed

- Libraries have no strict replay, and a library "replay" of a miss goes live (review 09 issue 1, severity 2). See `2026-09-26-libraries-cannot-replay-a-recording-strictly.md` and ticket 0148. This review adds two facts. The R run, with no key, answered a hit and then failed a miss with `no key is set … call EngineBuilder::api_key`, which names a Rust API to an R user. It also wrote `.locks/` into the fixture folder. `libraries/c/DESIGN.md:22` says recognize "follows the engine's cache and replay settings", and the C interface has no replay setting.

## Severity 3 titles

- Reformatting a recording file breaks it: `jq .` over a valid entry gives exit 5 and blames damage or hand edits.
- A replay miss does not say why it missed.
- No process-wide strict replay switch; the `THINKTHEN_CACHE` workaround misses at exit 4 and writes `.locks/` into the fixture folder.
- `--record` into a used folder pays again and then throws the answer away.
- Replay cannot tell a recording is stale against today's model. Carried in the architect review 08 file.
- Golden-file tests of ThinkThen output break across versions even when the answers replay. Carried in the architect review 05 file.
- A token in the base path is written into every recording and every `meta.url`.
- Byte-exact input keys. Carried above as item 3.
- No way to find or remove fixture entries a suite no longer uses.
