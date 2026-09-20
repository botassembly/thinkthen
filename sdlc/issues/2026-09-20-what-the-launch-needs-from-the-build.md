# What the launch needs from the build

Status: Open

Ian asked on 2026-09-20 for a message he can send to the builders for their short-term planning. It comes from the agent that holds the marketing and documentation job. Nothing here changes the plan's order. Finish the command line first. These are the things the launch leans on, most important first.

## The message

1. **`annotate` is the launch.** A third of the use cases collected for the launch wait on it. It is also the first place the tool sends several questions in one request, which the vendor measured at twelve times cheaper and ten times faster. The flagship triage demo, the worker-supervision pattern, and tagging all stand on it. `find` comes second and carries extraction and form filling.
2. **Measure the cap on questions in one request as soon as paid probes reopen.** Try 5, 10, 20, and 40 yes or no questions over one piece of evidence, against one request each. Two decisions wait on that number: how large an `annotate` file can be, and whether `tag` is real.
3. **Ian wants `tag`.** It is a fourth question type for zero or more labels from a list, and an eighth verb. It is one yes or no question per label in one request. The proposal is in `2026-09-20-tag-a-fourth-question-type-for-many-labels.md`. It needs an ADR and the probe above. It lands with `annotate` or right after it.
4. **Make the first five minutes clean.** `2026-09-20-new-user-stumble-register.md` lists fourteen stumbles, eight of them observed by command. Four are cheap and matter most:
   - The README says how to install, where a key comes from, what a judgment costs, and what the four exit codes are.
   - A guessed verb gets a hint. The headline sells a semantic `if`, `grep`, and `sort`, so people will type `thinkthen grep`. The refusal should name `filter`.
   - A CSV file piped into `--jsonl` gets a hint in place of "not valid JSON".
   - A file path given as a second argument gets a hint that names standard input and `--input`.
5. **Look again at the hold on CSV reading.** Both of Ian's tagging demos start from a CSV file. A CSV with quoted commas or a line break inside a cell cannot be converted by the base system alone. That meets the roadmap's own condition for bringing it in.
6. **Three proofs the copy is waiting for.** Each one unlocks a claim the launch cannot make today: the how-to against a second backend turns green, one real file is run and its cost is read from the ledger, and the result of the hostile-text probe is written where a page can quote it.
7. **Four how-tos that need no code.** Split one file into piles by a `choose` label. A tool-call guard for a coding agent, with the three-line exit code mapping. The long-lived loop from ticket 0024. Text split into paragraphs before a verb reads it.
8. **Refresh `interface-audit.md`.** It says three things never reach the wire that the binary now sends. See `2026-09-20-are-we-using-everything-the-service-offers.md`.
9. **A `status` view**, from the issue filed earlier the same day. It matters after the launch more than before it.

## Not asked for

The libraries. Six are ruled in: Rust, Python, JavaScript, Ruby, R, and a C interface. Ruby, R, and C are part of the long-term plan, and Ian approved all three on 2026-09-20. None of the six is part of the launch. `sdlc/planning/libraries/` holds the goals and anti-goals for each. `annotate` is the launch. The libraries follow the command line, and the opinion on how they should read is written. One thing helps them now: keep every verb's core free of the terminal, so a binding calls the same function the command calls.

## What Ian can overturn

The order above is a recommendation from the marketing side. The plan in `sdlc/planning/plan.md` stays the authority.
