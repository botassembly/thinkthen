# Python's `Entity` and `Edge` reprs print the caller's text

Status: Closed 2026-09-26 by Quick Fix qf-python-reprs-withhold-text.

Filed on 2026-09-26 by ticket 0134, found by its design review. No key was used, and no request left the machine.

## What happens

`libraries/python/src/asked.rs:268` gives `Entity.__repr__` as `Entity(name="Ada", kind="person")`, because it formats with Rust `{:?}`, with both in clear. `Edge.__repr__` at `asked.rs:301` prints its source and target entities through the same repr. A name and a kind are the caller's text. The same binding withholds question text in `Question.__repr__` (`asked.rs:116`). Ticket 0134 withheld the kind in the Rust `Entity`'s `Debug`, and that fix does not reach the Python repr.

`libraries/python/tests/test_secrecy.py:70` reprs `tt.Entity("Ada", "person")`, but it checks only that the key and URL credentials are absent.

## What would fix it

Print each entity's name and kind as a withheld byte count, as the Rust `Entity` does. Pin the whole repr line of an `Entity` and an `Edge` in `test_secrecy.py`.

## Done when

The repr of a Python `Entity` or `Edge` built with a marked name and kind holds no marker, and a test pins the whole line.

## Left for later

Ruby's `ThinkThen::Entity` and `ThinkThen::Edge` are plain `Struct`s, so their default `inspect` prints each name and kind in clear. The same holds for `Relation`, `Recognized`, `Ranked`, and `Found`, which also carry the caller's records. A Ruby fix is a choice about all its result values, larger than this Quick Fix. TypeScript returns plain objects and R returns data frames. Neither has a print form of its own, so neither prints through thinkthen code.
