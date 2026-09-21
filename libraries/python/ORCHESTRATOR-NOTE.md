# READ BEFORE YOUR NEXT COMMIT — orchestrator, 2026-09-21

1. The TypeScript lane's `git add -A` swept three of your in-flight files into ITS commit `f1683e7`: `src/arrow.rs`, `src/lib.rs`, and `thinkthen/__init__.py`. Your work is intact in the tree and in history; your working tree reads clean for those files because they are now committed by that commit. Do not re-commit or rewrite them — pull --rebase as usual and let your own commit carry only what you changed since. Delete this file when read.

2. Conformance cases 71 and 72 differ by their `form` field (persubject versus pairs). A fix landing on the branch adds the missing form to 72 and keys the stand-in's replay on it — rebase before chasing any 71/72 divergence.

3. The deck's `located_in` rule on the Maria Chen sentence has no recording (C01 covers `works_for` and `based_in`). That divergence is being filed with the deck's owner. Pin it in your notes as the known deck gap; do not bend the library.

4. SETTLED WHILE YOU RUN: the number on a recognized name is now the field `strength` — Ian's ruling, recorded in sdlc/issues/2026-09-21-one-rule-for-every-number-the-tool-prints.md. The interim `number` field is renamed everywhere in contract/, standin/, and conformance/ (already pushed). Bind `strength` in your surface, docs, and tests; the vendor's `confidence` passes through under details only. Rebase before you commit.
