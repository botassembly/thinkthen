# The fixtures

`refund.json` and `form.json` are byte-for-byte the two files the Bash
slides use, from `the product deck's examples directory`.
Every slide uses the same refund question and the same form set, so the
sample travels. `tickets.sql` builds the slide's `tickets` table: one row
that asks for a refund (the stub answers 0.97, a yes under the 0.2:0.8
band), one `maybe` row (0.55, unsure, the row a person should read), and
one plain row (0.03, a no).
