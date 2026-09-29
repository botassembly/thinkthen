# Frozen Rust API inventory disagrees with the release line

Status: Open. The selected lint checkpoint on main `7053849e1` completed the pinned API extraction and reported 36 missing declarations and 103 additional public declarations. The four existing planted API mutations were refused.

Many visible differences concern the landed Call result carriers and batching types. Reconcile every difference with accepted tickets and ADRs before updating the frozen contract. A passing extraction does not prove every export was intentional. Correct accidental exports or missing declarations in code when that is what the accepted contract requires; do not blindly regenerate the inventory or weaken the checker. Preserve all existing mutation plants and verify the exact public declaration set afterward.

The saved extracted comparison is `target/codex-builds/integrated-lint-checkpoint/inventory.log` in codex-2. This is a Rust no-default-feature API reconciliation, not permission to change held SQL/DataFrame contracts or repeat an all-port campaign. Keep any feature-specific remainder explicit.
