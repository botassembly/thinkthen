# Synthetic parser fixtures

Every file here is synthetic: written 2026-09-21 while preparing the
question-set parser's replacement (the architect's punch list, wait-list
item 1). The production parser must refuse each `bad-*` case and preserve
file order; the current contract parser accepts most of them and sorts
names. `contract/tests/parser_fixtures.rs` locks today's behavior so the
engine swap changes it deliberately, never by accident.

| file | why it is invalid | production |
| --- | --- | --- |
| `bad-version.json` | a version the grammar does not know | refuse |
| `missing-version.json` | no version at all | refuse |
| `duplicate-names.json` | the same name twice (raw JSON duplicate key) | refuse |
| `unknown-top-key.json` | a top-level key no grammar names | refuse |
| `empty-set.json` | a set with no questions | refuse |
| `empty-name.json` | a question named with the empty string | refuse |
| `on-collision.json` | two members, one `on` group, the same question twice | refuse |
| `file-order.json` | valid; the order expectation: names stay in file order (zeta, alpha) | keep file order |
