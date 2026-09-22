# 0069: Structured question descriptions and state

Status: Landed on main as `e261e4e` after independent acceptance and the final sequential local ladder. Main was pushed and ancestry verified. GitHub Actions stays manually disabled under Ian's local-gate ruling; no hosted success is claimed.

## Result

Question files and question-set entries accept ordered object/array instructions and string, object, array, or null criteria. Top-level numbers and booleans remain invalid. Score accepts an ordered map from level names to descriptions while results retain names. Tag uses its historical sentence bytes for string-only questions and one ruled instruction array when any part is structured. Existing string request bytes and pinned digests remain unchanged.

`specification/question-file.schema.json` publishes the Draft 2020-12 structural grammar. Four reusable single-question-file definitions and one shared corpus are checked offline by Python `jsonschema` and the production parser. The schema does not replace runtime semantic validation. No Rust runtime dependency was added.

Selected object/array evidence now reaches System One as JSON `state`; selected scalars and unpointed records retain text state. Multiple pointers preserve written order. The core holds evidence once as a validated private text-or-structured union, rejects scalar structured construction in every build, and redacts Debug. Profile limits count compact evidence bytes and exact request bytes. Text request identities remain stable; structured states intentionally receive new identities.

The evidence-shape probe keeps its historical text arm by constructing the same compact object before an unpointed call. Its object helper now consumes native object state. All original probe recordings, answers, cases, and measured rows remain unchanged. Eighteen mechanically derived demo entries pair new object-state requests with byte-identical historical responses; no provider call occurred.

## Review and verification

- Independent design review accepted the level-2 SWE-2 route, then separately accepted the incoming structured-state and narrow probe-preservation amendments.
- Independent code review accepted the description half after reusable schema definitions, counted-listener annotate refusal proof, and specification corrections. The coordinator's first complete ladder passed 601 Rust tests plus doctests, schema, replay, transform, and nineteen demo checks.
- Incoming main added structured state before landing. SWE-2 implemented it and stopped when the historical probe no longer replayed. The reviewed scope correction preserved the measurement rather than changing recorded evidence.
- Amendment review rejected the first `Evidence` representation because it duplicated structured content and enforced its shape only with a debug assertion. SWE-2 replaced it with a validated private union and added scalar-construction rejection. The same reviewer accepted the remediation.
- One implementer backend run reported a transient failure in `annotate::scheduling::one_global_queue_bounds...`; its exact timing assertion was unrelated to evidence representation. The isolated test and a later full backend rerun passed. The independent reviewer found no plausible amendment path into scheduling. This evidence is retained rather than called harmless.
- Final coordinator command: `sdlc/scripts/install && sdlc/scripts/lint && sdlc/scripts/test && sdlc/scripts/spec && git diff --cached --check`. Exit 0: policy and package checks, exact ratchet `35909/35909`, audit, format, Clippy, docs, 622 Rust tests passed with one intentional ignored child-harness test, doctests, schema/probe/transform checks, all sixteen replay comparisons, and nineteen green how-tos.

The Rust ceiling rose from 33,840 to 35,909 across both halves. Production growth provides validated ordered description/evidence types, parser ownership, wire encoding, and exact limit handling. Most growth is compatibility, secrecy, framing, no-send, digest, schema, probe, and replay proof. The implementation reuses the existing ordered JSON tree, compact serializer, resolver, question-set parser, and listener harness. Complete diff review found no material duplicate block to remove.

No historical recording or measured row was rewritten. No paid call, credential handling change, workflow change, publication, site change, library/database change, scheduler change, cache-policy change, or recorder change was made. Typed description builders remain with the library team.