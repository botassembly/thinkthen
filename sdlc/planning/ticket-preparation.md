# Prepare tickets

Preparation reads the code and fills the ticket's short form (`sdlc/tickets/README.md`). It does not implement the feature or change an accepted outcome. Ian can overturn this process.

## Before the build

Read `origin/main` at a named commit; a local `main` can lag. Verify a landed prerequisite by commit ancestry. Cite revision and path. Add notes only where they change the build: exact files, shared helpers, lane conflicts, the smallest distinguishing proof and its commands, and open questions. Mark a proposal as a proposal until review accepts it. Trace side-effect order through the final operation before calling a counter or guard exact.

## Rules that caught repeated defects

**Secrecy.** The key never reaches a log, record, `Debug` line, panic payload, or error. When two supported inputs meet, check their intersection. A host error may copy text across a boundary. Test the output, not the intent.

**Spend.** A paid call runs only through `sdlc/scripts/live` under a token cap and Ian's authorization. Persist the started arm, raw output, and usage before parsing. A failed arm stays evidence. Count both token kinds; never guess a missing count. A reservation is not a provider cap.

**Subprocesses.** Check every process level with the `children` guard; `policy.py` does not replace it. A timeout kills and reaps the owned child and closes its descriptors. Trace every join and destructor after a wait: a `Drop` can block on the same writer.

**File caps.** Measure each touched file against its 500-line ceiling before planning growth. Verify the root ratchet and each binding total.

**Host value ranges.** Check each host's accepted values and NULL rules before copying a proof across ports.

**Proof strength.** Assert the row count before each row. Separate an absent field from a malformed one. One intended failure per fixture. Prove "sends nothing" by counting loopback requests. Prove precedence with conflicting values through the real builder. Run a new regression once against the bad source.

**Gates.** Run focused checks during the build and full tests and lint on the landing commit. An interrupted run is not a pass. A clean textual merge does not prove compatibility; check what the merge touches.

**Old tests and unchanged files.** Before replacing a negative test, cite the contract clause it pins. A review finding does not amend an accepted contract; reconcile the two first. When a refusal promises unchanged files, validate the whole proposed state before the first write.

## Landing

Use the whole-ticket or whole-slice review rule in `AGENTS.md`. Write one short record per ticket at landing. Say what landed, why, what was checked and which gaps remain. Keep the ticket status to one or two sentences. A ticket needs no separate build-lessons section or review of its record.
