# The rulings fix wave, 2026-09-21

The wave's verification outputs, pasted as they ran. The rulings themselves
are `sdlc/issues/2026-09-21-product-rulings-on-the-surfaces-adversarial-review.md`;
the cross-side handoff is `MERGE-NOTE.md`; the review that produced it all
is `sdlc/issues/2026-09-21-adversarial-review-of-the-surfaces-and-experiments.md`.

## Ruling 4: the public-name check

```
$ python3 scripts/check_public_names.py
ok python: 25 names, all ruled or documented
ok typescript: 15 names, all ruled or documented
ok ruby: 29 names, all ruled or documented
ok r: 13 names, all ruled or documented
ok rust: 20 names, all ruled or documented
ok c: 10 names, all ruled or documented
ok duckdb: 12 names, all ruled or documented
ok sqlite: 10 names, all ruled or documented
ok postgresql: 13 names, all ruled or documented
every surface's public names are ruled or documented
```

The injection proof: `reset_usage` added back to the Python `__all__`, and
the check fails naming it (exit 1):

```
$ python3 scripts/check_public_names.py   # with reset_usage injected
     FAIL python: 26 public names vs 25 expected
  extra:   reset_usage
  a new public name needs a ruling and a line here
```

The injection was removed after the proof. The check caught real gaps when
first run: Python's two stream helpers left `__all__`; R's
`exportPattern("^tt_")` exported twenty internals (`tt_call`, `tt_raise`,
the `tt_*_one`/`tt_*_column` wrappers) — the NAMESPACE now names the
thirteen public functions explicitly.

## Ruling 6: the rung

```
$ sh sdlc/scripts/surfaces           # offline half, then the full check
... generated files match functions.toml (14 functions)
... every surface's public names are ruled or documented
... OK: 72 cases validated: schema, grammar, digests, wire contract, offline replay
... all landed checks green
```

The skip path, with cargo absent from PATH (exit 0):

```
$ env PATH=/tmp/rt /bin/sh sdlc/scripts/surfaces
... the offline checks above ran
surfaces: cargo is not on PATH; the offline checks above ran, the full surfaces check skipped
```

## The full run with stubs on every port the surfaces expect

Stubs on 8211 through 8219 and 8231, `STUB_DELAY_MS=300`, then:

```
$ sh sdlc/scripts/surfaces
... every surface section ran with its wire suite ...
== postgres surface: wire suite against the stub on 8219
wire green: decide answers on the wire, usage counts sends and tokens
all landed checks green
```

Pasted output is in `/tmp/full-rung.log` at the time of writing; the
verdict line is the done bar.
