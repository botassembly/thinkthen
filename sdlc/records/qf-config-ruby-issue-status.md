# Quick Fix qf-config-ruby-issue-status: name the configuration field, withhold caller text in Ruby prints, and bring issue status lines up to date

Status: built and checked, waiting for a fresh code review and landing. Owner: Claude. Base: `origin/main` at `a057c594`. Lane: `thinkthen-lane-2`. Branch: `qf/config-ruby-issue-status`.

## Evidence it started from

- Local experiment 273, reports 06 (I-5) and 08 (issue 4). A configuration file holding `{"cache":"/tmp/somewhere"}` stopped every command, `status` included, at exit 5 with `the configuration file is not valid closed JSON`. `sdlc/issues/2026-09-26-architect-review-06-backends-and-config.md` item 4 holds it.
- Quick Fix qf-python-reprs-withhold-text found that Ruby's result values are plain `Struct`s whose `inspect` prints names, kinds, records and units in clear. `sdlc/issues/closed/2026-09-26-ruby-result-values-inspect-caller-text.md` holds it.
- Several issue status lines still named work as waiting after tickets 0110, 0158, 0160 and 0161 and Quick Fix qf-review-273 had landed it.

## Retained behavior

- The configuration file's closed shape, its five fields, and every rule on their values are unchanged. Every refusal is still a usage error at exit 5, and an unreadable file is still a local failure.
- Broken JSON, a document that is not an object, and a repeated field keep the sentence `the configuration file is not valid closed JSON`.
- Ruby's result values keep their fields, accessors, equality, `to_a` and `to_h`. `Ranked#to_s` stays the record, so `puts ranked` still prints it.

## Change

1. `crates/thinkthen/src/config.rs`. When the typed read fails, the file is read again as a JSON object, and the refusal names the first field that breaks the shape. It never prints a value. A missing or non-text `schema` reads the existing schema sentence. A `url` or `model` that is not text reads ``configuration field `url` must be a string``. A `cache` that is not a boolean reads ``configuration field `cache` must be true or false``. A negative, fractional or non-number `cache_bytes` reads ``configuration field `cache_bytes` must be a whole number greater than zero``, and 0 now reads the same sentence. A field outside the five reads ``the configuration file holds a field other than `schema`, `url`, `model`, `cache`, and `cache_bytes` ``. That sentence lists the allowed names and does not echo the unknown one, because an unknown key is the caller's text. Ian can overturn that choice.
2. `libraries/ruby/lib/thinkthen.rb`. One private `Withheld` module gives `Entity`, `RecognizedEntity`, `Relation`, `Recognized`, `Edge`, `Ranked` and `Found` one print form. A field named `name`, `kind`, `text`, `record` or `unit` prints as `<N bytes withheld>`, counting UTF-8 bytes as the Rust Debug and Python repr forms do. A list prints as its length, as Python's `Recognized` repr does. Places, probabilities and rule names print in clear. `to_s` and `pp` use the same form.
3. Issue status lines. Review 03 records items 1 to 3 as done by 0161 and item 5 by 0158, and item 4 stays open as the mixed-model cache issue. Review 04 item 2 and review 12 item 3 name qf-review-273. Review 06 records items 1 and 2 as done by 0160, and item 4's code half as done here. The settings page half stays with H4, and item 8 of the settings-table issue says so. Review 08 records item 2 as done by 0158. The scalar-bind issue no longer says 0110 has not landed. It says how 0110 works around the broken bind path and that the decision stays Ian's. Row 15 of the new-user stumble register closes on ticket 0044's commit `ce96895b`, which made the `cost` transform read an id only from an object input.
4. Three issues moved to `closed/`. The site samples issue closes on site commit `648a965f`, which exists on main and follows `bd9594ff` and `494f28ea`. The named-answers landing issue closes: `af0cd7dd` brought `AGENTS.md` under its cap, item 2 is the move above, and item 3 now holds the answer read from the registration code. DuckDB registers `thinkthen_recognize` and `thinkthen_relations` as scalars returning lists and `thinkthen_relate` as a table function. PostgreSQL registers all three as set-returning functions. SQLite registers `thinkthen_recognize` and `thinkthen_relate` as table-valued functions and no `thinkthen_relations`. The site's two wrong recognize samples are added to the open site recognize issue, which marketing owns. The Ruby issue closes on fix 2. Links to all three moved files point into `closed/`.

## Proof

`each_refusal_names_its_field_and_never_its_value` in `config.rs` replaces a table that only checked each case failed. It pins the whole sentence for fourteen refusals and checks each is a usage refusal. Plant: sending every failed read back to the one closed-JSON sentence failed it at the `{}` row, which read `the configuration file is not valid closed JSON` in place of the schema sentence. The fixed file was restored. The four questions. It protects the rule that a shape refusal names its field and never its value. Collapsing the diagnosis back to one sentence, or printing the value, fails it. No test pinned any configuration refusal sentence before. It needs no hook, because `Config::parse` is the function `Config::read` calls on the file's bytes.

`test_result_values_print_no_caller_text` in `libraries/ruby/tests/test_errors.rb` runs real `recognize`, `relate`, `rank` and `find` calls on marked text against the loopback backend, in a scrubbed child. It builds a `Relation` and a `Recognized` from the found names, because the loopback backend's recognize finds no relation. It pins the whole `inspect` line of every value and checks that `pp`, `to_s` and interpolation print no marker. Plant: with the old `thinkthen.rb` in place, the first line read `text="x MARK"` and `kind="MARK-kind"`, and the test failed. The fixed file was restored. The four questions. It protects the rule that no Ruby result value prints the caller's text. The old default `Struct#inspect`, or a new text field left out of the withheld list, fails it. The existing secrecy test checked only the key and the address credentials. It needs no hook.

## Size

`sdlc/ratchet.json` rises from 72168 to 72224. About 25 lines are the diagnosis in `config.rs`, and the rest is the pinned table. No other closed-shape refusal in the crate names its field, so nothing existed to reuse. `libraries/ruby/ratchet.rb.json` rises from 1579 to 1635: about 20 lines for the module and 36 for the test. One module serves all seven structs.

## Checks

With `THINKTHEN_API_KEY` unset. `sdlc/scripts/live` did not run, and no paid call was made.

The rungs ran at `b2e17154`. The later commit adds only this section.

- `lint` with the private-names list: exit 0, `ratchet: crates + conformance 72224/72224`, Ruby `lib + tests 1635/1635`.
- `test`: exit 0, 973 passed, 0 failed, 13 ignored across 38 result lines, `live-test: all cases passed`.
- `spec`: exit 0, `demos: 21 green, 0 red`.
- `surfaces`: exit 0, 19 `surfaces: pass` lines, `check ruby: pass` and `check ruby: pass, installed`.

## Deferred

- The settings page still says the configuration file's `cache` takes a folder. H4 corrects it.
- The site's SQLite and DuckDB recognize samples still call `thinkthen_relations` wrongly. Marketing owns the fix, and the open site recognize issue carries it.
- Review 03 item 4, review 06 items 3 and 5, and review 08 item 3 and the rest of item 1 stay open.
