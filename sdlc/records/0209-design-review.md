# 0209 design review brief

Status: awaiting fresh read-only design review. Draft at main `30d340d1`; no runtime source or tests changed.

Review `sdlc/tickets/0209-stable-frame-columns.md` against the authoritative type-contract issue, ADR 0082, `specification/types.md`, and current frame constructors. Check that typed answer null plus one failure column keeps not-sure distinct from failed, that a successful empty tag stays an empty list, and that a whole-call failure still raises. Check pre-send `failed` name and input-column collisions, all-success and empty frame schemas, the three host adapters, and the existing tests that currently require widening. Assess the proposed Polars fixed-key struct as the map representation and the explicit optional categorical deferral. Require exact runtime file claims and measured ratchets before implementation; do not treat this brief as code acceptance.
