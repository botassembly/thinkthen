# 0221 code review

Status: corrections pushed for the same fresh review; acceptance is pending. A fresh Medium reviewer examined source `0387db24` in session `01a0e6b8-90cc-78a0-a462-e0f3f1fad3a7` and found two stale exact assertions. The reviewer confirmed the producer, row/range wrappers, facts order and command-plus-digest fallback by source inspection. Their read-only sandbox could not acquire Cargo's lock, so the build lane ran the executable cases.

The old exact prefix remained in `tests/backend/cache_identity.rs` for a legacy entry and in `tests/backend/default_cache.rs` for a missing directory. Both now expect `the decide request for one document` and retain their read-only, missing-folder and digest checks. The missing-directory case also pins the full explanatory sentence ending. A source search found no other unadorned exact prefix; the batch-specific `the decide request` assertions remain accurate because a batch is not one document. Both corrected cases pass as focused compiled tests. The earlier replay table and secrecy proof remain valid.

## What review taught us

The new outside-in table covered the added shape, but two older tests also owned the exact public sentence. Source-wide searches for copied diagnostic assertions must include cache and legacy fixtures before freezing a wording change. This correction changes expectations only; it does not change the producer or weaken the independent storage guarantees. Register 102's digest-component explanation remains open.
