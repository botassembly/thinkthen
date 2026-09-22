# contract NOTES

## 2026-09-21 — R1: recognize and relate on the contract

The two functions' shape, from `sdlc/planning/recognize-design.md` and
`relate-design.md` (rulings sections included) and the update-for-the-library-
team brief.

What landed:

- `Kind` (`Any`/`Named`), `RelationRule` (name, from, to, either) with the
  missing-end refusal and `check_kinds` for "a named end outside the asked
  kinds"; the rule shape is checked once here and no binding checks it again.
- `Recognize` (kinds default person/organization/place, relation rules, both
  bars at 0.5) with `from_json` for the question file's section; `Relate`
  (rules, optional `kind_field`, bar 0.5) with its own `from_json`.
- `Entity`, `Relation`, `Recognized`, `Edge`, `Recognized::to_json`, and
  `edges_json`; the host-facing ends are `source` and `target` on every
  surface including C's returned JSON, the wire keeps `from`/`to`, and the
  number on a name is the interim field `number` (see the doc comment and
  the vocabulary note in `../conformance/DIVERGENCES.md`).
- `MAX_RELATE_RECORDS = 255`, `guard_relate_records`, and `relate_checked`,
  so every surface that enters through the contract inherits the refusal.
- The `Engine` trait gained `recognize_opts` and `relate_opts` plus the
  plain `recognize`/`relate` wrappers; the stand-in implements them.

Commands and output:

```
$ cargo test
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The five new tests: the design page's question file reads whole; a missing
end, a kind outside the asked kinds, and a threshold out of range are usage
errors; the relate file reads bare names and either lists; the record limit
refuses at 256 and passes at 255; the door JSON spells `source`/`target`,
`probability`, and `number`, and never `from`, `to`, or `confidence`.

The C header (`include/thinkthen.h`) declares `thinkthen_recognize` and
`thinkthen_relate` in the out-param shapes the acceptance page shows, with
the returned string freed by `thinkthen_free_string`. The C surface crate
implements them in its own lane; R1 touched no surface folder.

## 2026-09-22 — the review fix wave, the contract's two pieces

Group 2 of the surfaces branch review: one checked deadline conversion,
owned here. And item 2 of the wave: the connector door the surfaces bind
through.

**The checked deadline conversion.** `deadline_from_seconds` and
`deadline_from_millis` are the one door a host's floating-point budget
passes through, and `Options::with_deadline_seconds` /
`with_deadline_millis` are the one call a host door makes. A NaN, a
negative other than the sentinel, or a budget past `MAX_DEADLINE_SECONDS`
(about 136 years) is the usage kind, so no host ever meets the panic
`Duration::from_secs_f64` raises on infinity or a huge value. `NO_DEADLINE`
(-1) means no deadline; zero stays a spent deadline, the settled rule.
Before this, all four host surfaces copied the unchecked conversion, and a
large budget crashed Node and aborted Ruby.

**The connector door.** `EngineConfig` is the settings struct's
construction-time name (`Settings` stays an alias, so nothing that imports
it changes), now carrying `timeout` and `max_retries` too. `Connector` is
the trait whose `connect` returns `Arc<dyn Engine>`; the stand-in's
`StandinConnector` implements it and honors every field the config sets,
with the environment as the fallback for what it leaves unset. Pointing a
surface at the real engine's connector is the one line the merge changes.

Commands and output:

```
$ cargo test
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The two new tests: a host's deadline is converted checked (NaN, negative,
infinity, 1e300, the sentinel, zero, and the largest accepted budget, in
both seconds and milliseconds), and the Options helper arms a call without
panicking on a hostile value.
