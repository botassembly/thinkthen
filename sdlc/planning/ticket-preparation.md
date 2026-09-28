# Prepare tickets and learn from their builds

Ian requested this process on 2026-09-27 to reduce repeated discovery and rework. Preparation investigates the code and adds useful notes. It does not implement the feature, change an accepted outcome, or close a product gap. Ian can overturn this process.

## Before the build

The coordinator assigns the next ready ticket or a related family, such as the database tickets. Keep the implementation builder working where file claims permit. Reuse a preparation agent for related investigations; reviewers remain independent of authors. Claim the note and ticket files on main before editing.

Read current main for executable behavior, the active accepted branch for pending decisions, and prior experiment and build records. Cite the source revision and path. Compare these sources before calling a sentence stale or a behavior missing. Refresh only affected facts after another branch lands.

Add concise investigation notes to the ticket. A shared record may hold facts common to a family, with a link from each ticket. Include only information that helps the builder:

- What the code does, where it does it, and how the reported problem arises.
- The agreed outcome, retained behavior, prerequisites, exact files and live lane conflicts.
- Inventory every completion adapter, copied argument conversion and private parent export that the change mutates. A call path alone can omit files needed at the final handoff.
- Measure existing nonblank lines against each enforced file ceiling before treating a ticket's estimated growth as headroom. If a file is close, plan reuse or one coherent private extraction and review the actual growth.
- Reusable code, experiments, fixtures and test helpers. Name fixture hashing, copied examples, serialized fields or host contracts that can surprise the build.
- Keep one intended failure per fixture. Preserve valid digest names and inputs when testing a schema or file-open error, so an earlier failure cannot mask the target boundary.
- When two inputs are individually supported, check their intersection before calling a public-output or secrecy finding non-issue. Register 106's exact configured-key collision is owned by ticket 0210; it does not justify unrelated test combinations.
- Verify each claimed landed prerequisite by commit ancestry or explicit equivalent source comparison. A copied adapter file does not establish that its source and tests both transferred.
- Check each host's accepted value ranges and NULL rules before reusing a proof across ports; a value valid in one host may be refused in another.
- For port settings, inspect every current constructor and overload before copying an accepted ticket's sentence. A later landed adapter may already support a value that older prose called absent.
- The smallest relevant validation commands and what each proves. Keep setup, compile, execution and lock wait separate when measured. No load campaign belongs in preparation.
- For paid jobs, distinguish a reservation charged before execution from a runtime limit. Trace the wrapper's enforcement point and count every token kind it reserves; a missing usage report must not become a guessed count.
- Unresolved questions and missing evidence. Distinguish a proposed solution from an accepted decision; seek design review before changing the contract.

Review the notes against source and ask the builder whether they are actionable. Correct inaccurate or speculative claims before handoff. Keep accepted unbuilt tickets on their existing branches; do not land them early merely to add notes.

## Before landing

The builder updates the ticket under `## What the build taught us`. Use a few factual bullets, with build-record links for details:

- Assumptions corrected and unexpected code, fixture, test or host behavior.
- Preparation that helped and details it missed.
- Proof or implementation adjustments and why they were necessary.
- Remaining gaps, the next owner or ticket, and advice for the next related build.

Do not rewrite history to make an initial assumption look correct. Distinguish confirmed causes from hypotheses. Record substantive learnings; when none arose, say so briefly. A Quick Fix puts this section in its build record because it has no ticket.

The fresh code reviewer checks these lessons against the diff and evidence. The coordinator checks the section before marking the ticket complete and landing it. Routine test and documentation checks stay proportional to the change; adding lessons does not require a repeated full suite.

After adding lessons, read the ticket's Status, Closes, Deferred gaps, Evidence and Routing together, and check that numbered lists still sit under the right heading. A completed item must not remain described as deferred in a second section.

For prose and fixture sweeps, inspect executable-page word limits and derived byte or hash assertions. Keep the strongest exact boundary check when a duplicate scalar assertion goes stale, then run the affected documentation segment. A routine text replacement can exceed a page limit without changing code.

## Improve the next preparation

After the preparation pass, evaluate its accuracy and usefulness. After each ticket finishes, compare the notes with the builder's lessons. Record what reduced discovery, what was missed, and what caused rework in the ticket or shared preparation record. Use that evidence to give the retained agent a specific next brief. Do not measure success by note length or by counting prepared tickets as completed issues.

The first database pass caught useful SQL budget and warm-settings dependencies. It also incorrectly flagged an old main sentence about `LIMIT` as an unresolved DuckDB design defect; the active accepted branch had already corrected it. The coordinator verified the branch and the preparer withdrew the finding. This is why the source comparison above is required. Independent review also caught a shared zero-total proof that SQLite cannot run because it refuses zero. The corrected notes use a spent positive SQL total there and keep a direct typed-budget proof separate. Future briefs must check each host's input domain before copying a test across ports. The first build-usefulness evaluation is still pending.

The 0170 preparation review caught another overstatement: a hook named `attempt_sent` ran before a final deadline check, so it could count a request that never reached transport. Trace the order of waits, final stop checks, counter marks and transport calls before claiming a counter is exact. A proposed move must preserve both no-send-on-expired-deadline and visibility of requests while their responses are held; a single zero-connection test proves only the first half.

Use the [incident log](../records/2026-09-27-ticket-friction-since-1300.md) when briefing the next agent. Add each new substantive review rejection, misunderstanding or build surprise with its evidence and known or unknown cause. Send the retained preparation agent to check the named upcoming tickets for the same failure pattern, claim those note files, and record what it corrected or left unresolved. Keep the pass limited to the affected family.

The first batching pass found complete argument paths and the mixed replay/live counter constraint. Review caught its mistaken assertion that an attempt hook necessarily counts a started transport: current code still checks the deadline after that hook. Check side-effect order through the final operation, not just method names. Pin accepted branch evidence to a commit so later branch cleanup cannot remove the reference. The corrected notes name the existing 0149 owner and a narrow prerequisite for 0170; they do not invent another issue or count the gap as fixed.

The 0171 build found two gaps in an otherwise useful handoff: a shared `Run` value had a library initializer outside the CLI files, and the existing JSON splice helper could not remove a stale member while preserving other bytes. For the next ticket, search all constructors of each changed shared value and list the exact edit operations a file mutation needs before treating the source inventory as complete. Preparation value is checked against the later build and review; these catches alone do not establish a speed gain.

The 0207 SQLite build found that a shared JSON round trip sorted `recognize` kinds and changed strict captured request bytes. The correction landed on main `30d340d1`. Before reusing a JSON adapter, trace order-sensitive fields through parse and serialization and compare the final request body and digest where order matters. This is a bounded preparation check, not a claim that all JSON member order is significant.

The 0171 builder started a full `surfaces` run for its own CLI warning change. It rebuilt the unchanged R package and held the shared heavy lock while DuckDB waited. The builder stopped the run; its exit 130 is not a passing gate, even though completed library segments passed. Require an explicitly named related-ticket checkpoint in the coordinator brief before a full `test`, `spec`, or `surfaces` run. Use focused commands freely for the ticket's changed behavior, and retain completed broad segments only as partial evidence.

Fresh review of 0171 caught two proof holes: the shared saved-setting reader dropped a present invalid `meta.batch.setting` as if it were absent, and a per-row assertion did not first establish how many rows existed. For future metadata readers, separate absent legacy fields from present malformed fields before applying a default. In outside-in row tests, assert the expected count before testing every row's contents. The 0171 correction has a numbered audit refusal and unchanged-file proof; its follow-up review is pending.

For retained CLI workers, launch each resume from its explicit assigned worktree with the intended approval and sandbox settings. Verify cwd and branch before editing. After a scripted completion edit, assert that the target matched and read the resulting status; a no-op replacement can otherwise leave a landed ticket marked pending.
