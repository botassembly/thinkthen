# ADR 0123: Bound planned requests across repeated inputs

Status: Accepted by fresh Astra review of ticket 0460 on 2026-10-07.

## Decision

Ordinary native, command-line and annotate plans count every admitted wire-question occurrence with checked arithmetic as the initial-request admission bound. Every initial request consumes at least one occurrence. Pauses, window flushes and repeats after earlier answers complete cannot exceed that bound. Refusal splits and retries retain separate limits.

The prepared-body preview retains repeats, uses uninterrupted packing and drains closed batches after each input. It retains the first body and counters. Exact bytes and measured token bands describe that preview; they do not guarantee runtime byte or token admission. The existing upper_bound field marks an occurrence count above the packed body count and retains prior staged bounds. Annotate request_count and group_requests still count packed preview bodies.

This supersedes ADR 0111 section 2 and its amendment table's global once-per-call promise. Section 3's pending-key coalescing remains: equal keys already on their way share one answer. After completion, normal cache freshness, replay and recording eligibility applies. With caching disabled, a later equal key may send again. Runtime scheduling, cache identity, result positions, staged planners and public shapes remain unchanged.

## Evidence

The reported 164 unique records followed by 22 repeats exhaust the old eleven-request allowance with batch sixteen, one worker, no cache and no retries. The retained loopback regression uses the corrected bound to complete all 186 original positions. Native multi-label and annotate group tests distinguish wire-question occurrences from records and packed bodies.
