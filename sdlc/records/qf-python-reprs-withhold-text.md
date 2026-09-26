# Quick Fix qf-python-reprs-withhold-text: withhold names and kinds in Python Entity and Edge reprs

Status: built, awaiting landing. It closes `sdlc/issues/closed/2026-09-26-python-entity-and-edge-reprs-print-caller-text.md`. No key was used, and no request left the machine.

## Result

- `Entity.__repr__` in `libraries/python/src/asked.rs` prints `Entity(name=<8 bytes withheld>, kind=<9 bytes withheld>)`. It counts UTF-8 bytes, as the Rust `Entity`'s `Debug` from ticket 0134 does.
- `Edge.__repr__` reuses the `Entity` repr for its source and target. It prints the relation in clear, as the Rust `Edge`'s derived `Debug` does. The relation is the caller's rule name, a setting, not evidence.
- The recognize design at `sdlc/issues/2026-09-26-recognize-design.md` showed the old clear repr as its example. The example now shows the withheld form, so a builder following the design does not bring the leak back. Ian can overturn this edit to his sent design.
- The issue records the other hosts for later. Ruby's `Entity` and `Edge` are plain `Struct`s whose default `inspect` prints each name and kind, as `Relation`, `Recognized`, `Ranked`, and `Found` print theirs. A Ruby fix is a choice about all its result values, larger than a Quick Fix. TypeScript returns plain objects and R returns data frames, and neither prints through thinkthen code.

## Tests

`test_entity_and_edge_reprs_withhold_names_and_kinds` in `libraries/python/tests/test_secrecy.py` reprs an `Entity` and a `relate` edge, all built with marked names and kinds, in a child against the loopback backend. It pins both whole lines.

- It protects the rule that no public repr prints the caller's text.
- A credible regression is the old `{:?}` form or a new field printed in clear.
- The existing secrecy test reprs `tt.Entity("Ada", "person")` but checks only for the key and the address credentials.
- It needs no test-only hook. It runs the real module through `tt.relate`.

The test also runs in the pandas 2 lane, which reruns `test_secrecy.py`.

Plant: restoring `format!("Entity(name={:?}, kind={:?})", ...)` turned the test red at index 0 with `'Entity(name="MARK-Ada", kind="MARK-kind")' != 'Entity(name=<8 bytes withheld>, kind=<9 bytes withheld>)'`. The other four secrecy tests passed. The fixed tree was restored and rebuilt.

## Ratchet

The root ceiling holds at `crates + conformance 69097/69097`. The Python source ceiling in `libraries/python/ratchet.json` rises from 4629 to 4634, for the doc line and the wrapped `format!` call. The Python test ceiling in `libraries/python/ratchet.py.json` rises from 2301 to 2316, for the new test. No Python helper formats a withheld count to reuse, and no existing test covered the repr text. The change is 22 nonblank lines net before this record.

## Review

A fresh read-only Claude reviewer found two things. The record did not exist yet, and this file answers it. The recognize design still showed the clear repr, and that example now shows the withheld form. It checked the byte counts, the relation left in clear, every other Python repr, the four questions, the other hosts' note, the links, and public hygiene.

## Checks

With `THINKTHEN_API_KEY` unset, after merging `origin/main` at `3c344b97`, with the one-minute load between 4.5 and 10:

- `lint`: exit 0, `ratchet: crates + conformance 69097/69097`, `src 4634/4634`.
- `test`: exit 0, 955 passed, 0 failed, 13 ignored across 36 result lines, `live-test: all cases passed`.
- `spec`: exit 0, `demos: 21 green, 0 red`.
- `libraries/python/check.sh` under the heavy lock: exit 0, 57 Python tests passed, conformance 50 passed with 4 not run of 54, and the pandas 2 lane 13 passed with 1 skipped.
- `sdlc/scripts/live` did not run.
