# 0246 cache binding design review request

Status: fresh independent High design review **ACCEPTED** the notes-only candidate `6a32887b`. The coordinator approved implementation on main `80228efe`. This acceptance covers the narrow explicit `Some(0)` admission check, not register 10's full original success criteria.

The review checked the exact `Some(0)` pre-admission sequence and its second read-only probe against accepted ADR 0099. A bound hit, malformed/legacy/mismatched marker, competing writer, and the final `SendBudget::reserve` retain their authority and precedence. The design promises no atomic durable marker and socket send, removes no stable folder/lock inode, changes no closed version-one marker, and exposes no raw bound URL.

The reviewer also checked the distinction between a successful key lookup and HTTP CR/LF validation. With explicit `Some(0)`, budget refusal may win even when the returned key contains a line break. This ticket proves the valid-key path and defers malformed-key behavior; implementation must not duplicate or move HTTP validation to expand the slice.

Experiment 284/10's literal all-zero-send/no-touch and both-address mismatch sentence remain separate. The narrow fix satisfies neither universally. The current CLI requested-address+cure sentence follows the accepted safe policy; the library lacks a cure. A fixed library cure and a possible read-only current-address comparison cannot reconstruct the bound URL. Register 10 stays open pending a separate original-criteria disposition. Public error wording and specification files are outside this runtime claim.
