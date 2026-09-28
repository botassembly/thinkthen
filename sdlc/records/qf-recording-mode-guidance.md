# Quick Fix: explain recording cost and byte-exact replay

Status: candidate for fresh independent review. Branch: `ticket/qf-recording-mode-guidance`, based on `origin/main` at `9236253f`. Scope: experiment 284 register 104 and the owned page portion of register 101. No runtime behavior or fixture attributes changed.

## Findings on current main

Register 104's guidance gap remained. In `crates/thinkthen/src/engine/recorder.rs::prepare_in`, a valid held entry replays only when `self.replaying` is true. With `--record` alone, even the same request prepares a live write. `install_or_compare` keeps the old entry when a fresh stored response differs and returns `RecordingConflict`. `crates/thinkthen/src/cli/failure/recording.rs::message` maps that conflict to exit 5. Existing `crates/thinkthen/tests/backend/recording_conflicts.rs::whitespace_padded_responses_with_different_values_conflict` pins the exact refusal and unchanged entry; `divergent_duplicate_records_stop_before_the_second_answer_prints` pins that a conflicting fresh answer is not printed. The Common and Find `--record` help and the page did not state the resend and possible second charge or show a safe fresh-run workflow.

Register 101's page warning was also absent. `crates/thinkthen/src/core/recording.rs::Exchange::digest` hashes the exact encoded request, and `Entry::replayed` compares the raw inner `request` bytes with the current request. Key order or whitespace changed inside that value can refuse replay even if the parsed JSON values agree. The existing `recording_the_same_whitespace_padded_response_again_is_idempotent` test reformats the outer envelope while leaving the inner values intact and passes. The page already said envelope formatting and whitespace outside the backend response value are tolerated, so the new warning preserves that distinction. The register's fixture-folder `.gitattributes` requirement belongs to the separate bench repository and remains open; this Quick Fix does not claim its closure.

## Change and retained behavior

Both `--record` help copies now say a held request sends again and may be billed again. They say a different fresh answer exits 5 before it is printed and keeps the old entry. They direct reuse to `--cache DIR` and deliberate fresh runs to a new empty folder. `specification/recording.md` repeats the mode distinction and gives a copy-first workflow: back up the old populated folder under a separate unused name, then record into another new folder. It explicitly warns that the populated copy still conflicts. The page also warns against changing the inner `request` value's bytes and distinguishes accepted outer-envelope formatting. No key, entry, replay, conflict, or output contract changed.

## Proof and checks

Compiled `decide --help` and `find --help` showed the new `--record` text. Existing focused conflict and envelope-formatting listener tests passed; no new test was needed for wording. Pages reported 1 coming and 21 green, tickets reported 0 evidence failures, and `git diff --check` was clean. No provider call, full suite, or broad build ran. The source ratchet measured 81,141 nonblank Rust lines, six above the previous 81,135. Those six lines are help comments in two existing CLI copies. I checked for a shared declaration first, but Common and Find own separate Clap argument structs; a new shared help constant or macro would add code for two short texts without reducing their duplication or making the command-specific wording clearer.

Recommend closing register 104 after fresh review and landing. Recommend keeping 101 open for the separate fixture-attribute work; its page portion is corrected here. The coordinator owns the plan and register.

## What the build taught us

The named folder is a storage destination in `--record` mode, not an implicit cache. Reusing that path can cost another backend call even when the old answer is present. If the answer moves, immutability preserves the first entry and discards the paid fresh result before output. A safe comparison needs two distinct folders; copying old entries into the new target recreates the conflict. A recording's outer JSON can be reformatted while its inner request remains exact, so a blanket “never format a recording” warning would overstate the contract. The two help structs need parallel copy, and the source-size ceiling counts that documentation.
