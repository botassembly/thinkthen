# Ruby result values print the caller's text when inspected

Status: Closed on 2026-09-27 by Quick Fix qf-config-ruby-issue-status (`sdlc/records/qf-config-ruby-issue-status.md`). Every Ruby result value now prints each caller text field as its byte count through `inspect`, `pp`, `to_s` and interpolation, and `test_result_values_print_no_caller_text` in `libraries/ruby/tests/test_errors.rb` pins each whole line. `Ranked#to_s` stays the record by design. Filed 2026-09-26 by the queue owner from Quick Fix qf-python-reprs-withhold-text (`sdlc/records/qf-python-reprs-withhold-text.md`).

## What happens

Ruby's `Entity`, `Edge`, `Relation`, `Recognized`, `Ranked` and `Found` are plain `Struct`s. `inspect` and `p` print every field, so names, kinds and other caller text appear in clear in logs and error reports. The Rust `Entity` and the Python `Entity` and `Edge` print each name and kind as a withheld byte count.

## What would fix it

Give each Ruby result value an `inspect` that withholds caller text as a byte count, matching the Rust Debug form. Pin the whole line of each in the Ruby secrecy test. TypeScript returns plain objects and R returns data frames, so neither has a print form in thinkthen code.

## Done when

Inspecting any Ruby result value built with marked text shows no marker, and a test pins each whole line.
