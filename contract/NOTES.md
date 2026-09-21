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
