# Product rulings on the surfaces adversarial review

Status: Open. Written 2026-09-21 by the product side for the library team. Ian asked the product owner to decide what it can and send feedback. Ian can overturn every ruling here.

The review was honest and useful. It corrected its own overstated claims and confirmed the interrupt defect by command. Run the fix wave. The rulings below settle what the wave needs.

## Where the work happens

The library team fixes its own branch in its own worktree. The build team is working on main in theirs. Do not edit main's files, main's `conformance/cases.json`, or main's gate ladder from the surfaces branch. Anything that touches both sides becomes a note for the merge ticket, which the build team owns.

## Rulings

**1. The question file says `source` and `target`.** This was already ruled in `sdlc/planning/recognize-design.md`, and it covers every host, the command's JSON, and the question file. Change the parser. Every door accepts `source` and `target` only. PostgreSQL stops accepting `from` and `to`: one spelling, refused the same way at all nine doors. Nothing has shipped, so no alias is kept. The command-line rule stays `--relation NAME=FROM:TO`, because that is a position and no reserved word is involved.

**2. The defect case stays in the one shared file, and no public door gets a fault hook.** Main's case `25-defect-fault` reaches the defect through an injected fault. The branch is right that a public surface must never carry a way to inject a fault. The contract still promises that a defect arrives as its own error kind and never as a wrong answer, and every host maps that kind. So the merged file keeps the case and marks it engine-only. The private engine runner proves it. Each surface proves in its own binding tests that the `defect` kind maps to the host's error. The build team picks the file's name and owns the union at merge. The library team hands over its 72 cases and a list of what differs.

**3. Ctrl-C stops a batch promptly on every backend.** A user who interrupts a three-million-record batch and waits for the whole batch has been failed. Fix the poll loop, add the null-backend interrupt test, and correct FINDINGS answer 4. The promise to pin: an interrupt or a spent deadline surfaces within one poll tick, however fast the backend answers. The same promise holds for `AbortSignal`, `pg_cancel_backend`, and `statement_timeout`. ADR 0017 copies the corrected answer only.

**4. `reset_usage` is dropped.** No ruling admitted it and no demo needs it. A caller who wants fresh counters builds a new engine. Remove it from the three surfaces. Then make the public name list a check: a script compares each surface's exported names against the ruled list and fails on an extra name. A convention nobody enforces is how this one got in.

**5. The usage counter counts what left the machine.** A request counts when it is sent, and a retry that is sent counts again, because the vendor bills each one. A cache hit, a size refusal, and a usage error count nothing. Tokens come from the vendor's replies only. Fix the counter and the two comments so that code, comments, and notes say one thing.

**6. The surfaces checks join the gate ladder.** Agreed as proposed. Add one rung on the branch that runs the two offline checks now and `check_surfaces.sh` when the toolchains exist. Bringing `contract/`, `standin/`, the libraries, and the databases under fmt, clippy, deny, and the ratchet is a merge-ticket item for the build team, because the ratchet ceiling moves.

**7. Fix the generator's number key before anyone reruns it.** The settled name is `strength`. A tree that passes only because a stale generator was never rerun is a trap for the next person. Rerun it after the fix and commit the result. Add case-id uniqueness to the validator in the same change.

**8. Sweep the stale prose.** The two 1 GiB lines, the rule-5 sentence, and the "settled but open" list in `relate-design.md` give way to the 100 MB ruling and the current rulings. Correct "three passes each" and "all findings filed" to what was run and filed.

**9. File the `thinkthen_relations` finding in this repo, on the branch.** Say what cannot run as drawn and on which databases. The marketing page `recognize-surfaces.md` draws `SELECT * FROM thinkthen_relations(body, '@names.json')`. The product side will redraw it once the finding says what shape can run. No deck slide shows that call today.

**10. The fork story states its two preconditions.** Write them into the embedding note: the settings lock is read before the fork, and the child holds the inherited pool's file descriptors for its lifetime. A reader can live with a stated limit. "By construction" with unstated conditions is a claim we cannot defend.

## Left to the library team's judgment

The minors: the C header's numeric code, the nested git clone in the DuckDB folder, the two doors that bypass `relate_checked`, the broken 207 case runner, and the derived numbers stated as measurement. Fix or record each one. For the numbers, label inference as inference.

## Left for the merge ticket, owned by the build team

One conformance file and its name, the union of cases, the hand-merge of the rulings issue that both sides edited, and the ratchet change. The library team prepares a short merge note listing each of these with the branch's side of it.

## What Ian may want to overturn

Ruling 1 refuses `from` and `to` outright with no alias. Ruling 4 removes a name some test code may like. Ruling 5 counts a sent retry as a request.
