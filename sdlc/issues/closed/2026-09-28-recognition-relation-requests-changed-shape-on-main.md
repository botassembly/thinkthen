# Recognition relation requests changed shape on main

Status: closed as non-issue after source verification at `fde46b37`. The request change is intentional under landed 0167. This closes the intent question, not the separately owned site and port fixture work. Filed 2026-09-28 from local experiment 274’s post-J1 Go run as a drift report, not a defect claim.

## What changed

The exact backend-request multiset of the accepted four-consumer Go gate is otherwise identical at pin `6dbdf03ff0fe290e503f0dbf4e09c51b5a964d22`, but each consumer's two recognize relation-pair request state objects changed: the old arrivals carried `{"entities":[...],"relation":{"name":"caused_by","source":"alert","target":"alert","reads":"caused by","either":false}}`; the new arrivals carry only `{"entities":[...]}` with the same entities. Eight replacements across four consumers; zero count changes (47 arrivals per consumer, two requests per recognized name); all output assertions still pass. Commit `c27fc529` replaced the relation-specific planner with a shared pair planner and is the plausible cause.

## Ask

Confirm the new wire shape is intended and the relation contract survives it, then refresh any product or site fixtures that assert the old request shape. Consumer gates that pin exact wire multisets (the language ports, record/replay tooling) need the updated baseline; the Go port's post-J1 report shows the mechanical pattern (`drift_check.py` comparing full multisets between pins).

## Evidence

Local experiment 274, `post-j1/POST-J1-REPORT.md`, `post-j1/logs/drift-check.log` (`POST_J1_EXACT_MULTISET_DRIFT_PASS`), gates `post-j1/logs/gate-20260928T022928Z/` versus `post-fix/logs/gate-20260927T140304Z/`. Also noted there: the C header now exports twenty symbols (`thinkthen_engine_new_with`), which any consumer ABI preflight pinning nineteen must update.


## Verified disposition

Ticket 0167 acceptance item 2 explicitly requires shared `{"entities":[…]}` state with no `relation` member. `crates/thinkthen/src/core/relation/pairs.rs` puts each rule's meaning in its pair questions and retains the pair-to-rule mapping, allowed source/target kinds and direction. Recognize asks what the text itself states; standalone relate asks whether the pair is true. The same planner serves both. This was the intended implementation landed with 0167 at `c2bc3538`.

Existing `core/relation/tests.rs` decodes request state with `deny_unknown_fields` and pins wording, rule order, IDs, evidence and entities. `tests/backend/recognize/rules.rs` checks captured state and pair questions. The 0167 build record carries their focused proof. A bounded independent source check confirmed these paths at `fde46b37`; no new build or provider call was needed to establish intent.

The remaining work stays open in [recognize site samples](2026-09-27-site-recognize-samples-after-the-three-steps.md), [site and bench pair examples](2026-09-27-site-and-bench-relate-pair-examples.md), and each port's supported-package integration issue. The [Go handoff](../2026-09-27-go-consumer-proof-needs-a-supported-package.md) already records this exact changed baseline. Each port must check its final native pin and strict request fixtures during integration; this disposition does not assert that every port has completed that check. No additional product defect was found, and no existing fixture task is closed here.
