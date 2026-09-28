# 0244 named question-file forms design review

Status: **ACCEPT** at `48ad825eba6e6a6c1ef12c59d640077275ab88a4` after a fresh, independent, read-only Sol Medium review. The coordinator approves routine implementation within this design and the plan's exact file claims.

The reviewer checked the original shared-conformance issue, the question-file contract and the actual C, TypeScript and Ruby edges. Named-file Local failures remain separate from inline Usage. The actual read is capped at 1 MiB plus one byte before the existing Rust parser runs. C returns its existing owned JSON string and uses the guarded per-engine error slot, with caller outputs unchanged on failure. TypeScript keeps the branded question's validated source rather than reordering rich metadata through another serialization. Ruby adds an explicit file keyword to its existing Question value; it does not reuse the multi-question set loader.

Required implementation proof includes one independently captured valid-file request per binding, selected case 30 with non-retryable Local and zero listener sends, a small invalid-file table, C ownership and symbol checks, and TypeScript rich-description identity. Existing literal strings, inline forms and text-only rank/find limits remain. DuckDB Q5 and recognize/relate forms stay open. The separate inherited Ruby source counter must be repaired against its previously reviewed source, not hidden in this feature's growth.

The reviewer returned no design-blocking findings. No runtime source or installed package was validated by this design review. Fresh code review follows implementation.
