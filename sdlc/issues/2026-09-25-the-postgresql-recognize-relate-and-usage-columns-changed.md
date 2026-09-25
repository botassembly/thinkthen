# The PostgreSQL recognize, relate, and usage columns changed

Status: Open

Filed by ticket 0111 on 2026-09-25 for the site's owner.

## What changed

Ticket 0111 ported the PostgreSQL extension onto the public API. Three result shapes changed, and the site's `site/src/data/examples/*__postgresql.json` files still show the old ones. The ticket left `site/` untouched.

- `thinkthen_recognize` returns `(name, kind, start, end, strength)`. The `text` column is now `name`. It also takes a version-one spec, `thinkthen_recognize(body, '@names.json')`, beside the kinds array.
- `thinkthen_relations` returns `(relation, source_name, source_kind, target_name, target_kind, probability)`.
- `thinkthen_relate` returns `(relation, source, target, probability)`. The first column was `name`. It also takes a version-one relate spec in place of the rules array.
- `thinkthen_usage()` returns four columns: `requests_sent`, `cache_answers`, `input_tokens`, and `output_tokens`.

## What the owner does

Re-derive each PostgreSQL example from `databases/postgresql/examples.json`, which `check.sh` runs on every surface rung.
