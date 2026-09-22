# Synthetic parser fixtures

Every file here is synthetic: written 2026-09-21 while preparing the
question-set parser's replacement (the architect's punch list, wait-list
item 1). Since 2026-09-22 the contract's parser hands every set to the
core's own parser first, so the contract accepts exactly what the command
line accepts: each `bad-*` case is refused with the core's own words, and
file order is kept. `contract/tests/parser_fixtures.rs` pins each verdict.

| file | why it is invalid | production |
| --- | --- | --- |
| `bad-version.json` | a version the grammar does not know | refuse |
| `missing-version.json` | no version at all | refuse |
| `duplicate-names.json` | the same name twice (raw JSON duplicate key) | refuse |
| `unknown-top-key.json` | a top-level key no grammar names | refuse |
| `empty-set.json` | a set with no questions | refuse |
| `empty-name.json` | a question named with the empty string | refuse |
| `on-collision.json` | two members sharing one `on` pointer; the core ACCEPTS them and groups them (measured 2026-09-22: `thinkthen_core::QuestionSet::parse` returns two questions, `["a", "b"]`), so the contract accepts them too | accepted, grouped |
| `file-order.json` | valid; the order expectation: names stay in file order (zeta, alpha) | keep file order |
