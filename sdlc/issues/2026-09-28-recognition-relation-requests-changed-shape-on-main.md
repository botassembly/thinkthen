# Recognition relation requests changed shape on main

Status: open, drift report. Filed 2026-09-28 from local experiment 274's post-J1 Go run; not a defect claim. The queue owner confirms intent and refreshes strict wire fixtures.

## What changed

The exact backend-request multiset of the accepted four-consumer Go gate is otherwise identical at pin `6dbdf03ff0fe290e503f0dbf4e09c51b5a964d22`, but each consumer's two recognize relation-pair request state objects changed: the old arrivals carried `{"entities":[...],"relation":{"name":"caused_by","source":"alert","target":"alert","reads":"caused by","either":false}}`; the new arrivals carry only `{"entities":[...]}` with the same entities. Eight replacements across four consumers; zero count changes (47 arrivals per consumer, two requests per recognized name); all output assertions still pass. Commit `c27fc529` replaced the relation-specific planner with a shared pair planner and is the plausible cause.

## Ask

Confirm the new wire shape is intended and the relation contract survives it, then refresh any product or site fixtures that assert the old request shape. Consumer gates that pin exact wire multisets (the language ports, record/replay tooling) need the updated baseline; the Go port's post-J1 report shows the mechanical pattern (`drift_check.py` comparing full multisets between pins).

## Evidence

Local experiment 274, `post-j1/POST-J1-REPORT.md`, `post-j1/logs/drift-check.log` (`POST_J1_EXACT_MULTISET_DRIFT_PASS`), gates `post-j1/logs/gate-20260928T022928Z/` versus `post-fix/logs/gate-20260927T140304Z/`. Also noted there: the C header now exports twenty symbols (`thinkthen_engine_new_with`), which any consumer ABI preflight pinning nineteen must update.
