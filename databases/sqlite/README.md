# The sqlite surface

Landed 2026-09-21. The acceptance sample, drawn in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`, runs
as drawn against the stand-in: `thinkthen_warm` fills the answers in one
pass at the process width, and the queries read them row by row.

## The eight functions

`thinkthen_decide`, `thinkthen_choose`, `thinkthen_score`, `thinkthen_tag`,
`thinkthen_annotate`, `thinkthen_details`, `thinkthen_usage`,
`thinkthen_warm` — the ruled names with the `thinkthen_` prefix. `NULL` is
"not sure". A question argument is plain text, a JSON question, or a
question file named with the command's spelling `'@refund.json'`, resolved
relative to the process working directory. `thinkthen_usage('reset')`
zeroes the counters and clears the session's saved answers.

In SQL, `filter` is the `WHERE thinkthen_decide` pattern the slide draws,
and the slide's second and third queries are that pattern reading the
warm pass's saved answers. The aggregate forms of `rank` and `find` are
the database ADR's to rule; this surface claims no names for them.

## The engine

The extension binds `thinkthen-contract` and calls the stand-in through
the trait; when the real engine lands, one dependency changes. Load-time
init registers the functions and touches no wire. The engine builds
lazily on the first call, and a fork is repaired by the engine's process
check — proven through this surface: a child forked after a call answers
on its own wire call. A stopped query ends between records, one in-flight
round deep, through the wait's poll callback hearing
`sqlite3_is_interrupted` — no watchdog thread. Idle connections are
pruned and retried by the engine, and the pool is sized to the gate.

## Building and checking

```
cargo build --release
cp target/release/libthinkthen0.so thinkthen.so
ENGINE_NULL=1 .runtimes/sqlite3 :memory: < tests/slide.sql   # the slide, as drawn
./check.sh                                                    # everything
```

The stock CLI is the official `sqlite-tools-linux-x64` bundle fetched
into `.runtimes/` (nothing outside the folder is touched, and no rc file
is modified). The entry point follows the basename rule: the artifact is
`libthinkthen0.so`, the entry is `sqlite3_thinkthen_init`, and
`.load ./thinkthen` resolves `thinkthen.so`.

The full record — the build traps, the wire numbers, the unchecked list —
is `NOTES.md`. The divergences the conformance slice reports are recorded
in `conformance/DIVERGENCES.md` as real-engine requirements.
