# 0246 cache binding design review request

Status: awaiting a fresh independent High design review. Candidate is the notes-only ticket 0246 and `0246-cache-binding-preflight.md`; this file records no acceptance.

Review the exact `Some(0)` pre-admission sequence and its second read-only probe against accepted ADR 0099. Check that a bound hit, malformed/legacy/mismatched marker, competing writer, and the final `SendBudget::reserve` keep their authority and precedence. Check that the design does not promise an atomic durable marker and socket send, remove stable folder/lock inodes, change the closed version-one marker, or expose a raw bound URL.

Separately judge the disposition of experiment 284/10's literal all-zero-send/no-touch and both-address mismatch sentence. The proposed narrow fix satisfies neither universally. The current CLI requested-address+cure sentence follows the accepted safe policy; the library lacks a cure. A fixed library cure and a possible read-only current-address comparison can improve usability but cannot reconstruct the bound URL. Flag any requirement for a new privacy, output-schema, or old-reader ruling before runtime claims. Keep register 10 open unless a reviewed criterion reconciliation explicitly closes it.
