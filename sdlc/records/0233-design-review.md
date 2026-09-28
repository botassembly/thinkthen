# 0233 Python type design review

Status: Accepted for implementation at corrected source `3d44d4dd`; coordinator approved the routine ADR 0082 outcome. This record reports the independent Sol review, not a code review of the implementation.

The first review of `b7262133` found two contract mistakes. First, the existing Python recognition keyword path calls `_Recognize._build` with `Vec<(String, Option<String>)>` and `Description::text`. That cannot preserve structured Enum, Pydantic or override descriptions. The corrected ticket sends keyword forms through the existing closed version-one recognize JSON and `_Recognize._from_json`, while retaining ordered relations, threshold defaults, no-kinds `ENTITY`, and `on=` eligibility. It requires captured request equivalence for old and structured forms. No second Rust parser or broader builder is proposed.

Second, Python canonicalizes `Literal["a", "a"]` before `typing.get_args()` can inspect it. The corrected contract refuses duplicates still visible after introspection, checks Enum aliases separately through `__members__`, and leaves a collapsed one-label `choose` set to the native count rule. It makes no source-inspection promise.

The same independent reviewer accepted the corrected design at `3d44d4dd`. The coordinator then granted the exact runtime files on main `f261fd11`. No additional Ian approval was requested. Fresh independent code/API review is still required after the source and focused proof are frozen.
